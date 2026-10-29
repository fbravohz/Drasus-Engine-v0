#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
# Gate de mutación (ADR-0133 capa 8) — punto de entrada ÚNICO y determinista.
#
# Uso:
#   scripts/mutation-gate.sh <crate> [jobs]
#   scripts/mutation-gate.sh hybrid-data-transformer
#   scripts/mutation-gate.sh sovereign-data-fetcher 4
#   Requiere jq para validar los resultados de cargo-mutants.
#
# Por qué existe este script en vez de escribir el comando a mano:
#
#   `cargo-mutants` copia el árbol de fuentes COMPLETO al directorio temporal
#   del sistema y compila ahí una copia por cada trabajo en paralelo. En esta
#   máquina `/tmp` es tmpfs — es decir, RAM disfrazada de disco. El árbol
#   arrastra Polars, cuyos binarios de depuración pesan cientos de MB y cuyo
#   enlazado necesita varios GB de memoria de trabajo por proceso. Con el
#   paralelismo por defecto (= número de núcleos) arrancan tantos enlazadores
#   simultáneos como núcleos haya, se agota la RAM, y el enlazador muere con
#   `ld terminated with signal 7 [Bus error]`.
#
#   Ese error APARENTA un fallo de compilación grave y cuesta un buen rato
#   diagnosticar que en realidad era falta de memoria (ocurrió el 2026-07-18).
#
# Las tres medidas que aplica este script:
#   1. TMPDIR en disco real (NVMe), no en tmpfs.
#   2. Paralelismo acotado — por defecto 2 enlazadores concurrentes.
#   3. Perfil sin símbolos de depuración: los binarios de mutación no se
#      depuran nunca, y quitar la info de debug reduce drásticamente tanto el
#      tamaño como la memoria que el enlazador necesita.
#
# Salida: imprime el resumen y deja el detalle en el directorio de resultados.
# Código de salida: 0 si no hay supervivientes; 1 si los hay (gate NO APTO).
# ─────────────────────────────────────────────────────────────────────────────

set -euo pipefail

CRATE="${1:?Uso: scripts/mutation-gate.sh <crate> [jobs]}"
JOBS="${2:-2}"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# Permite aislar la evidencia; por defecto usa el caché local del usuario.
MUTANTS_BASE="${DRASUS_MUTANTS_BASE:-${HOME}/.cache/drasus-mutants}"

mkdir -p "${MUTANTS_BASE}/tmp"

# Comprobación explícita: si la base cayera en tmpfs, abortamos con un mensaje
# que explica el problema, en vez de dejar que el enlazador muera sin contexto.
FS_TYPE="$(df --output=fstype "${MUTANTS_BASE}" | tail -1 | tr -d ' ')"
if [[ "${FS_TYPE}" == "tmpfs" ]]; then
    echo "GATE NO APTO"
    echo "ABORTADO: ${MUTANTS_BASE} está en tmpfs (RAM), no en disco real." >&2
    echo "El enlazado de Polars agotaría la memoria y moriría con Bus error." >&2
    exit 2
fi

# Cada corrida tiene evidencia propia; un resultado anterior no puede aprobarla.
RUN_DIR="$(mktemp -d "${MUTANTS_BASE}/${CRATE}.XXXXXXXX")"

echo "── Gate de mutación · crate '${CRATE}' · ${JOBS} trabajos en paralelo ──"
echo "   Temporales en: ${MUTANTS_BASE}/tmp  (${FS_TYPE})"
echo "   Resultados en: ${RUN_DIR}"
echo

cd "${REPO_ROOT}"

set +e
TMPDIR="${MUTANTS_BASE}/tmp" \
CARGO_PROFILE_DEV_DEBUG=0 \
CARGO_PROFILE_TEST_DEBUG=0 \
    cargo mutants -p "${CRATE}" --jobs "${JOBS}" --output "${RUN_DIR}"
MUTANTS_EXIT=$?
set -e

MISSED_FILE="${RUN_DIR}/mutants.out/missed.txt"
OUTCOMES_FILE="${RUN_DIR}/mutants.out/outcomes.json"

echo
echo "──────────────────────────────────────────────────────────────"
if [[ "${MUTANTS_EXIT}" -ne 0 && "${MUTANTS_EXIT}" -ne 2 && "${MUTANTS_EXIT}" -ne 3 ]]; then
    echo "GATE NO APTO"
    echo "cargo mutants falló con código ${MUTANTS_EXIT}"
    exit 1
fi

if [[ ! -s "${OUTCOMES_FILE}" || ! -f "${MISSED_FILE}" ]]; then
    echo "GATE NO APTO"
    echo "Resultados incompletos (código ${MUTANTS_EXIT})"
    exit 1
fi

if ! command -v jq >/dev/null 2>&1 || ! jq -e '
    type == "object" and (.end_time | type == "string") and
    (.outcomes | type == "array") and
    any(.outcomes[]; .scenario == "Baseline" and .summary == "Success") and
    ([.total_mutants, .missed, .caught, .timeout, .unviable] | all(type == "number")) and
    (.total_mutants == (.missed + .caught + .timeout + .unviable)) and
    ((.outcomes | length) == (.total_mutants + 1))
' "${OUTCOMES_FILE}" >/dev/null 2>&1; then
    echo "GATE NO APTO"
    echo "Resultados JSON inválidos o incompletos; verifica que jq esté instalado."
    exit 1
fi

if ! jq -e '.total_mutants > 0' "${OUTCOMES_FILE}" >/dev/null; then
    echo "GATE NO APTO"
    echo "No se generaron mutantes; no hay evidencia de cobertura."
    exit 1
fi

if [[ "${MUTANTS_EXIT}" -eq 3 ]] && { [[ ! -s "${RUN_DIR}/mutants.out/timeout.txt" ]] || ! jq -e '.timeout > 0' "${OUTCOMES_FILE}" >/dev/null; }; then
    echo "GATE NO APTO"
    echo "El código 3 no tiene evidencia de timeout."
    exit 1
fi

MISSED_COUNT="$(awk 'END { print NR }' "${MISSED_FILE}")"
if ! jq -e --argjson listed "${MISSED_COUNT}" '.missed == $listed' "${OUTCOMES_FILE}" >/dev/null; then
    echo "GATE NO APTO"
    echo "La lista de supervivientes no coincide con los resultados JSON."
    exit 1
fi

if [[ "${MUTANTS_EXIT}" -eq 2 && ! -s "${MISSED_FILE}" ]]; then
    echo "GATE NO APTO"
    echo "cargo mutants reportó supervivientes sin detalle (código 2)"
    exit 1
fi

if [[ ! -s "${MISSED_FILE}" ]]; then
    echo "GATE APTO"
    echo "0 supervivientes en '${CRATE}'"
    exit 0
fi

echo "GATE NO APTO"
echo "${MISSED_COUNT} supervivientes en '${CRATE}':"
echo
sed "s|crates/[^ ]*/src/||; s| in [0-9].*||" "${MISSED_FILE}"
echo
echo "Detalle completo: ${MISSED_FILE}"
echo
echo "Recordatorio: un superviviente solo se acepta como EQUIVALENTE con una"
echo "demostración escrita de por qué ninguna prueba puede matarlo (el bit que"
echo "la mutación altera no alcanza el valor de retorno). 'Es trivial' NO vale."
exit 1
