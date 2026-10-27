---
name: drasus-memory-compacta
description: Contexto persistente mínimo de Drasus Engine: preferencias del propietario, decisiones del producto, referencias canónicas y pendientes verificables.
metadata:
  node_type: memory
  type: project
  consolidated: 2026-10-25
---

# Memoria compacta de Drasus Engine

Esta memoria resume los otros archivos de `.agents/memory/`. Es contexto, no autoridad: manda la instrucción actual del usuario, después `AGENTS.md` y luego la documentación canónica. Verifica estados en `docs/ROADMAP.md`, `docs/DEBT.md`, la Orden y el ADR correspondiente. Los originales permanecen temporalmente para que el propietario compare el resultado; no los cargues como contexto rutinario.

## Colaboración con el propietario

- Responde en español mexicano, directo y con impacto primero. Usa vocabulario normal de negocio y software; evita traducciones torpes, analogías infantiles y detalle de código no solicitado.
- Para dos o más elementos, escribe una oración breve de contexto y después una lista. Separa cada bloque estructurado con un encabezado propio; explica solo la sintaxis técnica no evidente.
- Evita el campo semántico prohibido por `AGENTS.md` en nombres y documentos. Corrige una aparición de inmediato; no la registres como deuda.
- Un default recomendado no sustituye una elección que el usuario se reservó. Si la decisión cambia materialmente el resultado, espera una respuesta real.
- El usuario elige roles especialistas y delegación. Al delegar, entrega problema, restricciones, fuentes y resultado esperado; presenta cualquier método sugerido como hipótesis para validar.
- No hagas commits sin autorización explícita en el turno actual. Propón grupos separados por tipo.
- Marca personal: método de dos niveles, con nombre de canal magnético y handle personal. Handles finalistas históricos: `LFBravo`, `Febravo`, `fbravoh` y `lfbravoh`; falta decidir. Evita reutilizar un identificador ligado al correo por riesgo de correlación.

## Código, documentación y deuda

- Los comentarios documentan contratos públicos no evidentes y el motivo de decisiones complejas. `ponytail:` solo registra una simplificación local con umbral medible. Un aplazamiento externo pertenece a `docs/DEBT.md`.
- `docs/DEBT.md` es el registro canónico. Al cerrar un hito o decidir el siguiente trabajo, promueve toda deuda cuyo disparador ya se cumplió. Un hueco en la Definición de Terminado precede al siguiente hito.
- ADR-0118: entrega el núcleo completo de cada módulo; construye una feature en su primer consumidor; usa `docs/ROADMAP.md` como guía, no como bitácora. La ejecución vive en `docs/execution/`.
- ADR-0133 y `AGENTS.md` gobiernan las pruebas. Funciones numéricas de dominio abierto requieren propiedades; dominios finitos pueden usar enumeración exhaustiva. Cambios de correctitud en `domain/` o persistencia pasan el gate de mutación vigente.
- El Banco de Pruebas es gate de cierre por ADR-0152. Cada feature verificable debe exponer entrada válida o inválida y respuesta real a través de la integración vigente. No confundas esta superficie con la galería de componentes, el Dashboard o el Canvas.
- Antes de canonizar decisiones cuantitativas, estadísticas, financieras, legales o criptográficas, prueba límites, deriva supuestos, separa dato de política, revisa el alcance completo y usa al especialista. El caso guía es DSR: el número de pruebas no puede ser un acumulado vitalicio que condene toda estrategia futura; debe corresponder al punto de selección y considerar dependencia efectiva.

## Arquitectura y datos

- ADR-0020 define 25 campos como vocabulario lógico, no como 25 columnas copiadas en cada tabla. Grupo I es universal; los demás dependen del perfil A–D. Los perfiles son acumulativos cuando su definición lo permite; B cubre D, mientras B y C no se contienen. Consulta `docs/adr/ADR-0020.md` antes de diseñar persistencia.
- Todo valor crudo conserva su unidad junto al dato. El sufijo `_raw` no basta; declara la unidad en el registro o en un contrato fijo de la fuente. Rechaza el dato si la unidad no puede establecerse. El caso guía fue el spread de MT5: puntos y pips difieren por un factor de diez.
- Los ledgers append-only siguen el molde vigente de `AGENTS.md` y ADR-0141: escritura atómica, reintento solo de contención, error tipado, inmutabilidad física, hash encadenado y pruebas concurrentes discriminantes. No copies implementaciones antiguas como canon.
- ADR-0137: cada feature es una unidad hexagonal autónoma con puertos tipados; los módulos son presets de composición. Una feature depende de `shared`, no de otra feature.
- ADR-0136: la UX objetivo usa Dashboard de monitoreo y un Canvas único con zoom y nodos componibles. `MACRO/MESO/MICRO` quedó retirado. El nombre comercial del Canvas es paramétrico; código e identificadores permanecen neutrales.

## Estrategias, portafolios y procesos

- ADR-0153 separa dos ejes: entidad (`Cluster → Portfolio → Strategy → Logic Block`) y proceso (`Workspace → Pipeline → módulo/feature`). Se conectan por puertos; una feature no posee las estrategias que procesa.
- `Cluster` es una sola entidad. Una Selección es un conjunto mutable y efímero de referencias; solo se convierte en Portfolio cuando necesita pesos o reutilización versionada.
- Descartar un artefacto conserva su trazabilidad. Quitar un nodo del Pipeline solo cambia su topología versionada; no cambia el ciclo de vida del artefacto.
- El Pipeline recurrente ya tiene diseño: estados `RUNNING`, `PAUSED` y `STOPPED`, con una Expedition nueva por iteración. Su implementación continúa abierta como DEBT-024 y pertenece a EPIC-7.
- El versionado de estrategias es un DAG inmutable con ramas, diferencias y reversión. Consulta ADR-0005; no reproduzcas el modelo de copias entre bancos de StrategyQuant X.

## Monetización, privacidad y Cabina de Mando

- ADR-0143 define tres planos: UI local, ejecución en hardware del usuario y Cabina de Mando del proveedor. La Cabina autentica, licencia, factura y agrega telemetría; nunca ejecuta el trabajo del usuario.
- La soberanía depende del plan: el plan gratuito permite telemetría de trabajo; el pago al corriente la suprime en origen; al vencer el pago se reactiva. Credenciales de bróker e IPs operativas nunca salen.
- El substrato documenta 14 cimientos: identidad, licencias, cuotas, medición de uso, consentimiento, eventos enriquecidos, reportes, API de terceros, agregación, cuentas verificadas, continuidad de instancia, jerarquía de cuentas, portabilidad y roles de operador. ADR-0144 a ADR-0149 son la fuente.
- Consentimiento por plan y consentimiento opt-in son categorías distintas. Los roles se autorizan por puerto de feature, aplican igual a humanos y agentes y preservan al menos una capacidad administradora.
- La Cabina y la web pública están asignadas al repo hermano planificado `../drasus-portal/`, con stack libre. Esa ruta no existe en este workspace; verifica su ubicación antes de usarla. Su disparador documentado es el primer cobro real después de EPIC-5, con Spike gRPC previo.

## Estado histórico útil

- La auditoría retroactiva de EPIC-0 terminó con correcciones concentradas en el esquema base. TASK-049 está resuelta. DEBT-005, DEBT-018 y DEBT-022 están pagadas; DEBT-019 y DEBT-024 siguen abiertas al 2026-10-25. Para cualquier otra deuda, consulta `docs/DEBT.md`.
- El substrato de monetización cerró 14 backends y el Banco de Pruebas conectó esos 14 más el fetcher. Los estados resumidos del ROADMAP aún contienen texto rezagado; usa las Órdenes y fichas de deuda para confirmar cierres.
- `docs/DATA-MODEL.md` y `docs/data-model/` documentan el esquema. Toda migración debe actualizar su ficha en el mismo cambio.
- `../quantforge-docs/` conserva el historial previo al fork. Úsalo solo para recuperar artefactos eliminados mediante Git y adáptalos a las plantillas actuales.
- `prompt-engineer` se renombró a `context-engineer`. Los documentos de conocimiento nuevos o reescritos usan la plantilla vigente y declaran tamaño real en caracteres más un estimado de tokens; no se exige migración retroactiva.

## UI histórica: no usar como stack vigente

- Las memorias de Flutter, `custom_ui`, `Gx`, SVF en Dart y la galería describen trabajo histórico. `AGENTS.md` ordena React/TypeScript dentro de Tauri para toda UI nueva; no amplíes Flutter ni lo uses como referencia arquitectónica.
- STORY-025 dejó una biblioteca Flutter neutral y funcional; STORY-026 separó superficies del directorio de galería. Conserva esos hechos solo para entender documentos antiguos.
- Aprendizajes transferibles: el vidrio sobre fondo oscuro necesita tinte claro; evita desenfoques por elemento en animaciones grandes; precalcula geometría estable y reutiliza fondos estáticos. Revalida la técnica en el renderer actual.
- La documentación visual histórica contiene nombres y mapas ya desincronizados. DEBT-037 registra ese desfase; no inventes equivalencias React a partir de clases Flutter.

## Ambigüedades verificadas

- `docs/ROADMAP.md` mezcla estados históricos con actuales: su resumen menciona 13 cimientos y una entrega pendiente, aunque su tabla incluye 14 y las Órdenes registran el backend cerrado.
- Varios ADR y documentos aún hablan de Flutter mientras `AGENTS.md` fija React/Tauri. Hasta la migración documental, `AGENTS.md` gana para trabajo nuevo.
- Fechas, conteos de componentes, métricas de pruebas, hashes y listas de archivos se omitieron porque envejecen rápido y no cambian decisiones. Consúltalos en las Órdenes cuando sean necesarios.
