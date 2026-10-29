#!/usr/bin/env bash
set -euo pipefail

TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${TEST_DIR}/../.." && pwd)"
GATE_SCRIPT="${DRASUS_GATE_UNDER_TEST:-${REPO_ROOT}/scripts/mutation-gate.sh}"
TEST_ROOT="$(mktemp -d)"
trap 'rm -rf -- "${TEST_ROOT}"' EXIT
FAKE_BIN="${TEST_ROOT}/bin"
TESTS_RUN=0
FAKE_JQ_REAL="$(command -v jq || true)"
export FAKE_JQ_REAL
mkdir -p "${FAKE_BIN}"

cat > "${FAKE_BIN}/df" <<'EOF'
#!/usr/bin/env bash
if [[ "${MUTATION_TEST_SCENARIO:-}" == 'tmpfs' ]]; then
    printf 'Type\ntmpfs\n'
else
    printf 'Type\next4\n'
fi
EOF

cat > "${FAKE_BIN}/jq" <<'EOF'
#!/usr/bin/env bash
if [[ "${MUTATION_TEST_SCENARIO:-}" == 'jq_missing' || -z "${FAKE_JQ_REAL}" ]]; then
    exit 127
fi
exec "${FAKE_JQ_REAL}" "$@"
EOF

cat > "${FAKE_BIN}/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
OUTPUT_DIR=''
while (($#)); do
    if [[ "$1" == '--output' ]]; then
        OUTPUT_DIR="$2"
        break
    fi
    shift
done
[[ -n "${OUTPUT_DIR}" ]] || exit 70

case "${MUTATION_TEST_SCENARIO}" in
    tool_missing) exit 127 ;;
    usage_error) exit 1 ;;
    baseline_failure) exit 4 ;;
    diff_mismatch) exit 5 ;;
    invalid_diff) exit 6 ;;
    internal_error) exit 70 ;;
    interrupted) exit 137 ;;
    missing_output) exit 0 ;;
    timeout_missing_output) exit 3 ;;
esac

mkdir -p "${OUTPUT_DIR}/mutants.out"
write_outcome() {
    printf '{"outcomes":[{"scenario":"Baseline","summary":"Success"},{"scenario":{"Mutant":{}},"summary":"%s"}],"total_mutants":1,"missed":%s,"caught":%s,"timeout":%s,"unviable":%s,"end_time":"2026-01-01T00:00:00Z"}\n' "$1" "$2" "$3" "$4" "$5" > "${OUTPUT_DIR}/mutants.out/outcomes.json"
}
write_outcome CaughtMutant 0 1 0 0
: > "${OUTPUT_DIR}/mutants.out/missed.txt"
case "${MUTATION_TEST_SCENARIO}" in
    success|jq_missing) exit 0 ;;
    zero_mutants) printf '%s\n' '{"outcomes":[{"scenario":"Baseline","summary":"Success"}],"total_mutants":0,"missed":0,"caught":0,"timeout":0,"unviable":0,"end_time":"2026-01-01T00:00:00Z"}' > "${OUTPUT_DIR}/mutants.out/outcomes.json"; exit 0 ;;
    timeout) write_outcome Timeout 0 0 1 0; printf '%s\n' 'mutante agotado' > "${OUTPUT_DIR}/mutants.out/timeout.txt"; exit 3 ;;
    timeout_without_evidence) exit 3 ;;
    unviable) write_outcome Unviable 0 0 0 1; printf '%s\n' 'mutante inviable' > "${OUTPUT_DIR}/mutants.out/unviable.txt"; exit 0 ;;
    survivor) write_outcome MissedMutant 1 0 0 0; printf '%s\n' 'mutante superviviente' > "${OUTPUT_DIR}/mutants.out/missed.txt"; exit 2 ;;
    survivor_no_newline) printf '%s' 'mutante superviviente' > "${OUTPUT_DIR}/mutants.out/missed.txt"; exit 0 ;;
    timeout_and_survivor) write_outcome Timeout 0 0 1 0; printf '%s\n' 'mutante agotado' > "${OUTPUT_DIR}/mutants.out/timeout.txt"; printf '%s\n' 'mutante superviviente' > "${OUTPUT_DIR}/mutants.out/missed.txt"; exit 3 ;;
    inconsistent_exit) exit 2 ;;
    missing_outcomes) rm "${OUTPUT_DIR}/mutants.out/outcomes.json"; exit 0 ;;
    empty_outcomes) : > "${OUTPUT_DIR}/mutants.out/outcomes.json"; exit 0 ;;
    invalid_json) printf '%s\n' 'no es JSON' > "${OUTPUT_DIR}/mutants.out/outcomes.json"; exit 0 ;;
    incomplete_json) printf '%s\n' '{"outcomes":[]}' > "${OUTPUT_DIR}/mutants.out/outcomes.json"; exit 0 ;;
    missing_missed) rm "${OUTPUT_DIR}/mutants.out/missed.txt"; exit 0 ;;
    *) exit 70 ;;
esac
EOF
chmod +x "${FAKE_BIN}/df" "${FAKE_BIN}/cargo" "${FAKE_BIN}/jq"

assert_gate() {
    local scenario="$1" expected_status="$2" expected_text="$3" error_code="${4:-}"
    local result status cache_dir
    if [[ -n "${DRASUS_MUTATION_CASE:-}" && "${DRASUS_MUTATION_CASE}" != "${scenario}" ]]; then
        return
    fi
    TESTS_RUN=$((TESTS_RUN + 1))
    cache_dir="${TEST_ROOT}/cache/${scenario}"
    mkdir -p "${cache_dir}/demo/mutants.out"
    printf '{"outcomes":[]}\n' > "${cache_dir}/demo/mutants.out/outcomes.json"
    : > "${cache_dir}/demo/mutants.out/missed.txt"

    set +e
    result="$(env PATH="${FAKE_BIN}:${PATH}" DRASUS_MUTANTS_BASE="${cache_dir}" \
        MUTATION_TEST_SCENARIO="${scenario}" bash "${GATE_SCRIPT}" demo 1 2>&1)"
    status=$?
    set -e

    if [[ "${status}" -ne "${expected_status}" ]] || ! grep -Fxq "${expected_text}" <<< "${result}"; then
        printf 'FALLÓ %s: estado=%s; se esperaba estado=%s y %s\n%s\n' \
            "${scenario}" "${status}" "${expected_status}" "${expected_text}" "${result}" >&2
        exit 1
    fi
    if [[ -n "${error_code}" && "${result}" != *"${error_code}"* ]]; then
        printf 'FALLÓ %s: falta el código de error %s\n%s\n' \
            "${scenario}" "${error_code}" "${result}" >&2
        exit 1
    fi
    printf 'APTO %s\n' "${scenario}"
}

assert_concurrent() {
    if [[ -n "${DRASUS_MUTATION_CASE:-}" && "${DRASUS_MUTATION_CASE}" != 'concurrent' ]]; then
        return
    fi
    TESTS_RUN=$((TESTS_RUN + 1))
    local cache_dir="${TEST_ROOT}/cache/concurrent"
    local success_pid survivor_pid success_status survivor_status success_dir survivor_dir
    mkdir -p "${cache_dir}"

    env PATH="${FAKE_BIN}:${PATH}" DRASUS_MUTANTS_BASE="${cache_dir}" \
        MUTATION_TEST_SCENARIO=success bash "${GATE_SCRIPT}" demo 1 > "${TEST_ROOT}/success.log" 2>&1 &
    success_pid=$!
    env PATH="${FAKE_BIN}:${PATH}" DRASUS_MUTANTS_BASE="${cache_dir}" \
        MUTATION_TEST_SCENARIO=survivor bash "${GATE_SCRIPT}" demo 1 > "${TEST_ROOT}/survivor.log" 2>&1 &
    survivor_pid=$!

    set +e
    wait "${success_pid}"
    success_status=$?
    wait "${survivor_pid}"
    survivor_status=$?
    set -e
    success_dir="$(sed -n 's/^   Resultados en: //p' "${TEST_ROOT}/success.log")"
    survivor_dir="$(sed -n 's/^   Resultados en: //p' "${TEST_ROOT}/survivor.log")"
    if [[ "${success_status}" -ne 0 || "${survivor_status}" -ne 1 || \
          -z "${success_dir}" || -z "${survivor_dir}" || "${success_dir}" == "${survivor_dir}" ]]; then
        printf 'FALLÓ concurrent: las corridas no conservaron resultados separados\n' >&2
        cat "${TEST_ROOT}/success.log" "${TEST_ROOT}/survivor.log" >&2
        exit 1
    fi
    printf 'APTO concurrent\n'
}

assert_gate tool_missing 1 'GATE NO APTO' 'código 127'
assert_gate tmpfs 2 'GATE NO APTO'
assert_gate jq_missing 1 'GATE NO APTO'
assert_gate usage_error 1 'GATE NO APTO' 'código 1'
assert_gate baseline_failure 1 'GATE NO APTO' 'código 4'
assert_gate diff_mismatch 1 'GATE NO APTO' 'código 5'
assert_gate invalid_diff 1 'GATE NO APTO' 'código 6'
assert_gate internal_error 1 'GATE NO APTO' 'código 70'
assert_gate interrupted 1 'GATE NO APTO' 'código 137'
assert_gate missing_output 1 'GATE NO APTO'
assert_gate timeout_missing_output 1 'GATE NO APTO'
assert_gate timeout_without_evidence 1 'GATE NO APTO'
assert_gate missing_outcomes 1 'GATE NO APTO'
assert_gate empty_outcomes 1 'GATE NO APTO'
assert_gate invalid_json 1 'GATE NO APTO'
assert_gate incomplete_json 1 'GATE NO APTO'
assert_gate missing_missed 1 'GATE NO APTO'
assert_gate inconsistent_exit 1 'GATE NO APTO'
assert_gate zero_mutants 1 'GATE NO APTO'
assert_gate survivor 1 'GATE NO APTO'
assert_gate survivor_no_newline 1 'GATE NO APTO'
assert_gate timeout_and_survivor 1 'GATE NO APTO'
assert_gate success 0 'GATE APTO'
assert_gate timeout 0 'GATE APTO'
assert_gate unviable 0 'GATE APTO'
assert_concurrent

if [[ "${TESTS_RUN}" -eq 0 ]]; then
    printf 'No se ejecutó ningún caso para %s\n' "${DRASUS_MUTATION_CASE:-}" >&2
    exit 2
fi
