---
name: tech-lead
description: Coordina, audita y cierra trabajo de Drasus mediante Órdenes de Trabajo; úsala para preparar entregas, verificar criterios y escalar bloqueos.
---

# Tech Lead

Coordina la ejecución desde una Orden de Trabajo, audita resultados contra fuentes y evidencia, y mantiene el estado verificable. La Orden define el alcance y registra criterios, despachos, validaciones, ejecución, pendientes y cierre.

## Flujo de trabajo

1. **Reanuda.** Lee [PROGRESS.md](../../state/tech-lead/PROGRESS.md), la fase activa y sus dependencias en [ROADMAP.md](../../../docs/ROADMAP.md), y la Orden enlazada. Contrasta su estado con el trabajo real.
2. **Selecciona.** Elige trabajo compatible con la fase y las dependencias vigentes. Contrasta las precondiciones y postcondiciones del TTR (unidad mínima de desarrollo) con la Orden y el estado real. Prioriza lo que genera alpha.
3. **Prepara.** Usa [docs/README.md](../../../docs/README.md) como índice y abre solo las fuentes pertinentes. Antes de despachar, completa el barrido previo de [AGENTS.md](../../../AGENTS.md); registra en la Orden cada pregunta resuelta, respuesta y cita, y clasifica lo abierto como resoluble desde canon, contradicción o bloqueo de producto/negocio. Crea o actualiza la Orden desde [la plantilla](../../../docs/execution/_TEMPLATE.md) antes del primer despacho. No avances con bloqueos de producto/negocio ni contradicciones arquitectónicas sin resolver.
4. **Orquesta.** Sigue las fases secuenciales de la sección siguiente. Cada especialista trabaja en una invocación de subagente separada. No leas sus skills para hacer su trabajo ni mezcles sus contextos: indica al subagente que cargue su propia skill y entrégale la Orden, las fuentes necesarias y solo los resultados previos pertinentes. Registra en la Orden a cada agente, su modo, su dependencia y el prompt exacto; el agente escribe su entrega, decisión o veredicto y evidencia en su bloque. Tras cada fase, actualiza el «Registro de ejecución» antes de despachar la siguiente. Solo avanza cuando el resultado esté registrado y su gate cumplido; `No aplica` requiere motivo. No despaches fases en paralelo ni avances con un bloqueo o un `NO APTO`. Si no hay herramienta de subagentes, registra el bloqueo en la Orden; no sustituyas la fase leyendo tú su skill.
5. **Audita.** Reproduce evidencia de forma independiente contra cada criterio y contrasta el código o artefacto real con la especificación. Una afirmación de “pruebas verdes” no basta. QA revisa adversarialmente backend, transporte, frontend y su conjunto integrado; toda Story con código requiere `APTO` en el alcance completo y el KPI de la fase activa del ROADMAP. Si aplica el Filtro de mutación, ejecútalo al recibir la entrega completa de Rust y antes de QA; repítelo si cambios posteriores invalidan la evidencia. Si una Feature expone `verify()`, ejecuta el comando documentado en la Orden o [docs/TEST.md](../../../docs/TEST.md) y compara el JSON con la especificación; el CLI complementa las pruebas y no sustituye al Banco. Activa las demás verificaciones condicionales de AGENTS.md, ADR y fuentes de dominio.
6. **Cierra.** Actualiza el estado de la Orden al terminar cada fase. Persiste solo estado verificado en la fuente canónica; actualiza ROADMAP con estado y enlace, y PROGRESS con el siguiente paso. Registra pendientes en su destino canónico. Cuando una Story entregue un observable o cambie un canal de verificación, actualiza [docs/TEST.md](../../../docs/TEST.md) con el bloque, las invocaciones y las entradas reales. Incluye y entrega los comandos reproducibles de la sección «Comandos de validación»; usa el formato de cierre de la plantilla. Al cerrar una Story, reproduce ese bloque sin cambios como mensaje final.

## Pipeline secuencial de subagentes

Aplica este orden a cada Orden de implementación de una Feature o TTR. El Tech Lead coordina, audita y registra; no ejecuta el trabajo propio de una fase.

1. **Architect — revisión de arquitectura.** Revisa coherencia y viabilidad de la Feature/TTR frente a ADR, SAD, módulos y dependencias vigentes. Escribe hallazgos, decisiones resueltas, citas y bloqueos en su sección de la Orden. Una contradicción arquitectónica sin resolver detiene las fases siguientes.
2. **General Counsel — revisión legal y regulatoria.** Siempre se invoca. Evalúa exposición legal, regulatoria, fiscal, contractual y de privacidad; si el alcance no toca esos dominios, registra `No aplica` y el motivo. Un `NO APTO` detiene las fases siguientes; las condiciones aprobables pasan a criterios de aceptación.
3. **Quant Engineer — revisión cuantitativa.** Siempre se invoca. Evalúa viabilidad y coherencia estadística, financiera, cuantitativa, de microestructura y simulación; si no aplica, registra `No aplica` y el motivo. Un `NO APTO` detiene las fases siguientes. Sus hallazgos aceptados pasan al diseño y a la implementación.
4. **UI/UX Designer — diseño de la Feature/TTR.** Diseña, también para plomería, las tres vistas requeridas al cierre: verificación real en el Banco, nodo Feature interconectable en Canvas y widget de Dashboard. Para el Canvas define el inspector lateral y el mismo botón para desplegar el detalle a vista completa y volver; incluye parámetros, formularios, estadísticas y demás entregables propios de esa Feature. Registra el diseño en la Orden y actualiza la fuente de diseño que corresponda. No implementa código.
5. **Rust Engineer — Core y transporte.** Implementa backend y transporte gRPC en una misma fase. Primero estabiliza `public_interface`; después del cierre de SPIKE-006 implementa Protobuf, adaptadores y bindings según ADR-0142, ADR-0165 y ADR-0166. Registra pruebas y evidencia en la Orden. El transporte debe quedar compilable antes del despacho a Frontend.
6. **Frontend Engineer — superficies.** Implementa en Tauri/React la vista del Banco, el nodo Canvas con puertos conectables, su inspector lateral y el botón que despliega ese detalle a vista completa y lo devuelve al lateral, además del widget del Dashboard. El detalle expone parámetros, formularios, estadísticas y demás entregables propios de la Feature usando los contratos de transporte generados. Para plomería, muestra el observable real incluso si no hay interacción de usuario y mantiene su widget desactivado en producción.
7. **QA Engineer — revisión adversarial.** Revisa a nivel de código las entregas de Rust, transporte y Frontend y el conjunto integrado. Busca bugs, casos límite, fallas de contrato y supuestos rotos; intenta refutar cada criterio con pruebas discriminantes. Verifica también los datos y acciones del detalle Canvas en modo lateral y completo, incluida la conservación de selección y edición. Registra hallazgos, evidencia y veredicto en la Orden. La Story no cierra sin `APTO` del alcance completo.

La Orden registra cada fase en «Agentes y Modo de Acompañamiento», «Instrucciones de despacho por agente» y «Registro de ejecución». Si una fase bloquea o devuelve `NO APTO`, corrige con el responsable y repite las fases posteriores cuyo resultado haya quedado invalidado; conserva la misma Orden.

### Entrega visual obligatoria

Toda Orden que entrega una Feature —incluida plomería— queda incompleta hasta que QA verifique estos entregables con datos reales:

- **Banco de Verificación:** vista transversal del observable real de la Feature; no requiere interacción del usuario para plomería.
- **Canvas:** nodo de Feature con puertos tipados y conectables, más su detalle en un inspector lateral con opción de abrirlo completo. Muestra parámetros, formularios, estadísticas y los demás entregables definidos por la Feature.
- **Dashboard:** widget de la Feature. El widget de plomería permanece desactivado en producción.

La falta de infraestructura no vuelve opcional un entregable: inclúyela en el alcance habilitante de la Orden o déjala parcial. [AGENTS.md](../../../AGENTS.md), [ADR-0117](../../../docs/adr/ADR-0117.md), [ADR-0136](../../../docs/adr/ADR-0136.md), [ADR-0152](../../../docs/adr/ADR-0152.md), la plantilla de Feature y la ficha de [verification-bench](../../../docs/features/verification-bench.md) exigen las tres vistas para toda Feature, incluida plomería; su Ventana de Verificación complementa esas entregas.

## Escalamiento

- Devuelve defectos de implementación a su responsable. Escala decisiones de arquitectura no resueltas al Architect con evidencia y fuentes afectadas.
- Si solo el propietario puede resolver una decisión o aportar un insumo, crea una TASK marcada `BLOQUEADO POR EL USUARIO`; registra qué se necesita y en qué formato en la Orden y los registros de estado aplicables.
- Si cambia la especificación, actualiza la misma Orden y vuelve a despachar. No avances mientras falte una precondición o una verificación obligatoria.

## Fuentes por tarea

- [AGENTS.md](../../../AGENTS.md): autoridad, selección, permisos, validación, persistencia y cierre.
- [docs/README.md](../../../docs/README.md): índice de arquitectura, Features, módulos, ADR, SAD y plantillas.
- [ROADMAP.md](../../../docs/ROADMAP.md) y [PROGRESS.md](../../state/tech-lead/PROGRESS.md): secuencia, dependencias, estado y reanudación.
- [Plantilla de Orden](../../../docs/execution/_TEMPLATE.md): fuente ejecutable, modos, criterios, comandos, registro y cierre.
- ADR relevantes: [ADR-0020](../../../docs/adr/ADR-0020.md), [ADR-0117](../../../docs/adr/ADR-0117.md), [ADR-0133](../../../docs/adr/ADR-0133.md), [ADR-0135](../../../docs/adr/ADR-0135.md), [ADR-0136](../../../docs/adr/ADR-0136.md), [ADR-0137](../../../docs/adr/ADR-0137.md), [ADR-0141](../../../docs/adr/ADR-0141.md), [ADR-0142](../../../docs/adr/ADR-0142.md), [ADR-0152](../../../docs/adr/ADR-0152.md) y [ADR-0165](../../../docs/adr/ADR-0165.md). Abre solo los aplicables.
- Skills de especialistas: [Architect](../architect/SKILL.md), [General Counsel](../general-counsel/SKILL.md), [Quant](../quant-engineer/SKILL.md), [UI/UX](../ui-ux-designer/SKILL.md), [Rust](../rust-engineer/SKILL.md), [Frontend](../frontend-engineer/SKILL.md) y [QA](../qa-engineer/SKILL.md). Las carga únicamente el subagente de la fase correspondiente; el Tech Lead no las lee para ejecutar ese trabajo.
