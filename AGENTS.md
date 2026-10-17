# AGENTS.md — Mapa operativo

Mapa de arranque. Contiene solo reglas que deben estar activas siempre; el detalle vive en `docs/README.md`, ADRs, SAD, skills y knowledge.

## 1. Arranque y selección de agente

Para cualquier tarea de Drasus, consulta primero `docs/README.md` como índice y abre solo los documentos relevantes. Antes de actuar, clasifica **ambigüedad, impacto, novedad, riesgo** (datos, dinero, seguridad o contratos), **alcance** y **validación**.

**Luna es el caballo de batalla:** primera opción para implementación diaria, debugging, tests, documentación, UI, gRPC, refactors claros y exploración.

| Agente | Úsalo para | Razonamiento |
|---|---|---|
| **Luna** | Tareas acotadas, reversibles, mecánicas o con diseño claro; exploración y subagentes | `medium/high/max` |
| **Sol** | Implementación compleja, vertical slices y cambios multiarchivo con arquitectura/contratos definidos | `high/xhigh/max` |
| **Astra** | Arquitectura nueva, conflictos, decisiones irreversibles, trading, riesgo, seguridad y errores caros | `high/xhigh/max/ultra` |

Reglas:

- Si hay dos o más señales de ambigüedad alta, impacto transversal, novedad, riesgo alto o validación end-to-end, usa Astra. Si el diseño está cerrado, usa Sol para complejidad y Luna para lo demás.
- Usa Astra **antes de codear** para validar arquitectura, datos, contratos, reglas cuantitativas, seguridad y ejecución; úsalo **después** para auditar invariantes, backend/frontend/DB, fugas, riesgo financiero y fallos graves. Entrega evidencia, severidad, corrección y pruebas de cierre.
- Resuelve el ID real disponible en el host (`gpt-6-astra`, `gpt-5.6-sol`, `gpt-5.6-luna` son ejemplos); usa el máximo nivel soportado y decláralo.
- El agente debe producir un resultado verificable, no solo un plan. Antes de empezar, declara: `tarea → clasificación → agente/nivel → motivo → escalamiento → subagentes`.
- Los barridos grandes van a Luna. Sol/Astra solo para síntesis arquitectónica o decisiones de dominio, con autorización del usuario o justificación explícita. En entregas críticas: barrido Luna + auditoría Astra. Los subagentes devuelven solo la conclusión y nunca escriben concurrentemente en los mismos archivos.

## 2. Guardas de arranque

- **Stack:** Rust + React/TypeScript/Tauri 2; no Python/FastAPI en Core. Los bindings/contratos se generan desde Rust; no dupliques DTOs a mano.
- **Motor y seguridad:** UI delgada; motor independiente, Local-First, Zero-Docker, gRPC autenticado y checkpoints reanudables. La ejecución queda en el hardware del usuario; la Cabina central autentica, licencia y agrega, pero nunca computa; credenciales de bróker e IPs live nunca salen. El portal central es `../drasus-portal/`.
- **Arquitectura:** FCIS + hexagonal; las Features contienen la lógica y los módulos las componen. Una Feature depende solo de `shared`; no acoples Features ni accedas a tablas ajenas fuera de su interfaz pública.
- **Documentación:** antes de cambiar arquitectura, datos, portal, riesgo o esquema, consulta `docs/README.md` y los ADR/SAD relevantes. El documento canónico prevalece sobre este mapa.
- **Estado y commits:** mientras no haya builds distribuidas, trata el sistema como GREENFIELD; antes de cambiar esquema consulta los ADR de migraciones y después del primer release usa migraciones forward-only. Haz commits solo con autorización explícita, separados por tipo y nunca masivos o automáticos.

## 3. Fuentes y rutas

- `docs/README.md` es el índice completo para localizar módulos, Features, ADRs, SAD, producto, roadmap, plantillas y lecciones. No dupliques ese índice aquí.
- Detalle operativo: `.agents/skills/<rol>/ROLE.md`, `.agents/knowledge/README.md` y el contexto `drasus_context` según `.agents/context-runtime/catalog.json`.
- `governance.md` y `hard-prohibitions.md` son obligatorios; consulta `critical-domain-reasoning.md` antes de cerrar reglas cuantitativas, estadísticas, cripto, fiscales o legales. Declara las lecturas y reglas aplicadas según `GOVERNANCE-1.5`; los reportes van a `.agents/sessions/`.

## 4. Recuperación eficiente de contexto

1. Localiza primero en los índices; no leas repositorios completos.
2. Usa `rg` antes de abrir archivos y lee solo la sección relevante.
3. Delega barridos amplios; no releas archivos sin cambios.
4. En archivos grandes, continúa desde el offset exacto; no repitas rangos.

## 5. Governance, memoria y comunicación

- La memoria automática está desactivada en `.codex/settings.local.json`. No escribas nuevas entradas en `.agents/memory/`; los archivos existentes no se modifican. Los aprendizajes persistentes van al documento de dominio que los consume.
- `governance.md` gobierna anti-alucinación, herramientas, cierre, persistencia, doc-first y provider bypass; `hard-prohibitions.md` contiene las prohibiciones absolutas. Ambos prevalecen sobre este mapa; la instrucción explícita del usuario en el turno actual gana.
- No inventes nombres, rutas ni snippets. Prosa en español; identificadores en inglés; comentarios de código en español.
- Habla al propietario a nivel CEO: claro, directo y con vocabulario normal de negocio/tech. Lidera con impacto y decisión; deja el detalle técnico para cuando aporte valor.
