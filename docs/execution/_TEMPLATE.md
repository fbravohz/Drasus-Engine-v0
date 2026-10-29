# <ID> · <Título llano>

> **Plantilla de Orden de Trabajo (Spec-Driven).** Copia este archivo a `docs/execution/<ID>-<slug>.md`.
> La Orden de Trabajo es la **especificación ejecutable**: contiene la instrucción EXACTA que recibió el agente,
> los comandos para que el usuario valide por su cuenta, y el registro de lo que pasó. Vive en git, NO en el chat.
> Si la especificación cambia, se EDITA aquí y se re-despacha — así el cambio queda reflejado y versionado.

| Campo | Valor |
|---|---|
| **ID** | STORY-000 |
| **Título** | <llano, sin códigos> |
| **Tipo** | Story \| Spike \| Bug \| Task |
| **Épica (Fase)** | EPIC-0 — Fundación |
| **Sprint** | <n> |
| **Estado** | Pendiente \| En curso \| Bloqueado \| 🟡 Parcial \| ✅ Implementado |
| **Responsable** | <rol> (<modelo>) · auditó <Tech-Lead / QA> |
| **Creada** | YYYY-MM-DD |
| **Completada** | YYYY-MM-DD |

> La tabla «Agentes y Modo de Acompañamiento» es la fuente de verdad para los participantes y sus modos. Toda Story incluye también a QA.

## 0. Resumen de la Orden

> Escrito por el Tech-Lead al crear la Orden. Responde tres preguntas en lenguaje llano, sin códigos internos:
> - **Qué problema resuelve** este ticket (una frase).
> - **Qué se va a construir** (lista de 2-4 puntos concretos y verificables).
> - **Por qué ahora** (qué desbloquea o qué gate cierra).

<Resumen aquí — sin límite de líneas, pero sin verbosidad: si necesitas más de 10 líneas, la descripción es demasiado detallada para un resumen.>

---

## 1. Especificación de origen (qué specs implementa)
Enlaces a las unidades de especificación que gobiernan este trabajo:
- **Feature(s):** [`nombre`](../features/nombre.md)
- **TTR(s):** TTR-001, TTR-002…
- **Módulo(s):** [`nombre`](../modules/nombre.md)
- **ADR(s):** ADR-XXXX

## 2. Objetivo (una frase llana)
<qué logra este trabajo, en lenguaje de negocio>

## 3. Agentes y Modo de Acompañamiento (ADR-0120)
> Declara una fila por agente participante y su modo. Cada agente lee su fila en esta Orden; sin modo declarado, asume **Autónomo**. El modo no reduce los criterios ni la validación.

| Agente | Etapa del pipeline | Depende de | Modo |
|---|---|---|---|
| <Rust-Engineer / Frontend-Engineer / Bridge-Engineer / Quant-Engineer / Refactoring-Engineer> | Etapa <n> | <agente del que depende, o "ninguno"> | Autónomo \| Mentor \| Docente \| Revisión |
| **QA-Engineer** | **Etapa 5 — obligatoria en toda Story** | **Entregas que audita** | **Autónomo**, salvo declaración distinta |

> En toda Story, QA revisa cada entrega aplicable y el conjunto final, sin excepción por fase o capa. La Story no se cierra sin `APTO` de QA sobre el alcance completo. Las pruebas concretas se activan según sus propios criterios de aplicación, conforme a ADR-0133. En otras Órdenes, conserva la fila de QA cuando corresponda a su alcance o criterios.

**Significado de cada Modo** (contrato completo en `<agente>/SKILL.md` §"Modos de Acompañamiento"):
- **Autónomo:** el Agente implementa y entrega código + pruebas terminadas. El usuario solo audita el resultado.
- **Mentor:** el usuario teclea. El Agente explica el concepto y dicta el fragmento EXACTO a escribir, bloque por bloque, verificando cada bloque antes de avanzar al siguiente.
- **Docente:** el Agente implementa un bloque, explica sus decisiones desde cero y responde preguntas antes de avanzar.
- **Revisión:** el usuario entrega código ya escrito por su cuenta. El Agente audita y explica el porqué de cada corrección, sin dictar la solución de antemano.

## 4. Instrucciones de despacho por agente (la spec ejecutable)
> El prompt EXACTO que recibe cada Agente. Si cambia la spec, se edita ESTO y se re-despacha. Bajo Modo Mentor o Revisión, el Agente llena aquí mismo (no en el chat) su Plan de Implementación Guiada o Checklist de Revisión, antes/mientras lo ejecuta en el chat — esta Orden es la fuente de verdad, no el chat.

### 4.1 <Nombre del Agente>
```
<prompt completo dado al agente>
```

**Plan de Implementación / Revisión** (lo llena el Agente al ser invocado, según su modo en «Agentes y Modo de Acompañamiento»):
<Modo Mentor → secuencia concepto → fragmento exacto a teclear → punto de verificación, bloque por bloque. Modo Revisión → checklist de auditoría aplicado y veredicto por bloque. Modo Autónomo → resumen de lo implementado.>

<Repetir el bloque de despacho por cada agente declarado en «Agentes y Modo de Acompañamiento».>

## 5. Criterio de aceptación (cada criterio ↔ su prueba)
> Regla de proceso: ningún criterio se da por cumplido sin una prueba nombrada que lo ejerza. El ingeniero entrega ya en verde; el Tech-Lead reproduce y verifica cobertura del criterio (no solo "tests verdes").
> Si solo el propietario puede ejecutar un criterio, incluye en «Comandos de validación» el recorrido exacto, el resultado esperado y las trampas conocidas.

| # | Criterio verificable | Prueba que lo demuestra |
|---|---|---|
| 1 | <condición verificable 1> | `<nombre_del_test>` |
| 2 | <condición verificable 2> | `<nombre_del_test>` |

## 6. Comandos de validación (para el usuario — copy/paste)
> Sustituye los ejemplos por los comandos y entradas reales de esta Orden. Incluye pasos manuales cuando sean necesarios.
```bash
# comandos exactos para reproducir y validar por tu cuenta
cargo test -p <crate>
cargo clippy --workspace --all-targets -- -D warnings
cargo llvm-cov --workspace --summary-only          # % de cobertura de líneas
```

## 7. Registro de ejecución (bitácora cronológica)
> Registra el trabajo del responsable y cada invocación de subagente, directa o anidada, incluidos reintentos y continuaciones. Anota encargo, agente/modelo, etapa, resultado real, evidencia, bloqueos y siguiente paso. Si una invocación no produjo resultado, indícalo; no lo inventes. Pide a quien delegue que informe sus invocaciones anidadas y resultados.
>
> Registra cada veredicto de QA con su alcance y evidencia. Un `APTO` parcial no certifica el conjunto; si hay cambios posteriores, QA verifica su impacto antes del cierre. Distingue el veredicto de QA, el del Filtro de mutación cuando aplique y la aceptación final del Tech Lead.
>
> Al terminar la iteración, registra los errores, su causa raíz, el filtro que los detectó o dejó pasar y la mejora propuesta; si no hubo hallazgos, déjalo explícito.

| Fecha | Etapa y responsable o subagente/modelo | Encargo | Resultado y estado | Evidencia, bloqueo o siguiente paso |
|---|---|---|---|---|
| YYYY-MM-DD | <etapa · agente/modelo> | <objetivo> | <resultado o sin resultado> | <enlace o pendiente> |

## 8. Pendientes derivados / decisiones
- <pendiente o bloqueo · impacto · responsable · siguiente paso · destino: Orden, Roadmap o deuda>

---

## 9. Cierre ejecutivo (para el usuario — CEO)

> Al finalizar o pausar una iteración, redacta este bloque desde «Registro de ejecución» y «Pendientes derivados / decisiones». Resume toda la iteración en lenguaje llano: trabajo y resultados verificados, cada invocación y su resultado, bloqueos, impacto y siguiente paso. Incluye las invocaciones fallidas, interrumpidas o sin resultado; si no hubo subagentes, indícalo.
>
> Conserva el formato de la skill Tech Lead. En el cierre de una Story, copia este bloque sin cambios como mensaje final y usa 🟢 solo si está completa y QA emitió `APTO`. En las demás Órdenes, usa 🟢 cuando sus criterios aplicables estén cumplidos. Usa 🟡 para parcial o decisión pendiente y 🔴 para bloqueo. Si no hay decisión pendiente, escribe `Ninguna.` en «INPUT REQUERIDO DEL CEO».

```text
ESTADO: 🟢 COMPLETADO | 🟡 PARCIAL O REQUIERE DECISIÓN | 🔴 BLOQUEADO

PROGRESO MACRO:
- <resultado general verificado de toda la iteración y veredicto de QA, si aplica>
- <una viñeta por invocación de subagente: encargo y resultado; o Sin invocaciones.>
- <siguiente paso y responsable, o Ninguno.>

FRICCIONES Y DEUDA:
- <bloqueos o deuda y su impacto; omitir esta sección si no hay ninguno>

INPUT REQUERIDO DEL CEO:
- <decisión o insumo concreto, o Ninguna.>
```
