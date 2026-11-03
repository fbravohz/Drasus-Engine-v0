# Integración de Impeccable en la skill de diseño

## Alcance y revisión

Se inventariaron los 62 recursos excluidos de Impeccable y se revisaron procedimientos de diseño para incorporarlos en `ui-ux-designer`. Los ejecutables y archivos de datos no recibieron una auditoría detallada. La revisión estructural aplicó la skill `architect` en el mismo contexto de Codex; no fue una revisión independiente ni se despacharon subagentes. El canon de Drasus y la solicitud del propietario prevalecen sobre las reglas genéricas del material de origen.

El inventario registra el destino de cada recurso o el motivo de su exclusión. La clasificación es por recurso: no afirma una equivalencia literal de cada instrucción del paquete original. Las guías existentes se conservaron; los ajustes a cifras y umbrales corrigieron afirmaciones no sustentadas o criterios que se presentaban como universales.

## Estado de cada recurso

Las rutas de origen son relativas a la raíz del respaldo. Los destinos de diseño indicados son relativos a `.agents/skills/ui-ux-designer/references/`; las referencias a guías existentes incluyen sus recursos enlazados.

| Recurso original | Tratamiento |
|---|---|
| `agents/impeccable_asset_producer.toml` | combinar: visual/direccion-y-prototipos.md; sin subagentes |
| `agents/impeccable_documenter.toml` | combinar: visual/direccion-y-prototipos.md; canon de Drasus prevalece |
| `agents/impeccable_finish_reviewer.toml` | combinar: visual/revision.md; sin subagentes |
| `agents/impeccable_manual_edit_applier.toml` | pendiente: decisión del propietario sobre Impeccable Live |
| `agents/openai.yaml` | excluir: metadatos de la skill anterior; no describen ui-ux-designer |
| `reference/adapt.md` | adaptar: desktop-responsive/guide.md (escritorio, contenido y controles) |
| `reference/adapt.native.md` | excluir: adaptación iOS/Android nativa; primera versión es escritorio Tauri |
| `reference/android.md` | excluir: plataforma Android nativa fuera del alcance de Drasus Engine |
| `reference/animate.md` | combinar: motion/guide.md |
| `reference/audit.md` | combinar: visual/revision.md y accesibilidad existente; detector pendiente de decisión |
| `reference/audit.native.md` | excluir: auditoría móvil nativa; conservar criterios transversales en accesibilidad y revisión |
| `reference/bolder.md` | combinar: visual/criterios-visuales.md, intensidad dentro del alcance |
| `reference/clarify.md` | combinar: ux-writing/guide.md, respetar ADR-0160 y no inventar causas |
| `reference/colorize.md` | combinar: visual/criterios-visuales.md y significados protegidos |
| `reference/component-review.md` | adaptar: visual/direccion-y-prototipos.md, revisión de regiones y activos |
| `reference/craft-floor.md` | adaptar: visual/criterios-visuales.md; descartar prohibiciones que contradicen la dirección |
| `reference/craft.md` | excluir: alias obsoleto que remite a trabajo nuevo |
| `reference/critique.md` | combinar: visual/revision.md y evaluación heurística existente; sin herramientas ni puntuaciones infundadas |
| `reference/degraded/asset-producer.md` | combinar: visual/direccion-y-prototipos.md; sin subagentes |
| `reference/degraded/documenter.md` | combinar: visual/direccion-y-prototipos.md; canon de Drasus prevalece |
| `reference/degraded/finish-reviewer.md` | combinar: visual/revision.md; sin subagentes |
| `reference/degraded/manual-edit-applier.md` | pendiente: decisión del propietario sobre Impeccable Live |
| `reference/delight.md` | combinar: interaction/onboarding-y-resiliencia.md y motion/guide.md |
| `reference/distill.md` | combinar: visual/criterios-visuales.md; conservar funciones críticas y densidad acordada |
| `reference/doctor.md` | adaptar: visual/direccion-y-prototipos.md, detectar discrepancias con el canon sin imponer sidecar |
| `reference/document.md` | adaptar: visual/direccion-y-prototipos.md y design-system/guide.md existente |
| `reference/extract.md` | combinar: design-system/guide.md existente y revisión de evidencia |
| `reference/generate.md` | pendiente: decisión del propietario sobre Impeccable Live |
| `reference/harden.md` | combinar: interaction/onboarding-y-resiliencia.md y desktop-responsive/guide.md |
| `reference/hooks.md` | pendiente: decisión del propietario sobre el detector y su hook |
| `reference/init.md` | adaptar: brief en visual/direccion-y-prototipos.md; usar PRODUCT.md existente |
| `reference/ios.md` | excluir: plataforma iOS nativa fuera del alcance de Drasus Engine |
| `reference/layout.md` | combinar: visual/criterios-visuales.md y desktop-responsive/guide.md |
| `reference/live-setup.md` | pendiente: decisión del propietario sobre Impeccable Live |
| `reference/live.md` | pendiente: decisión del propietario sobre Impeccable Live |
| `reference/mode-operate.md` | adaptar: visual/direccion-y-prototipos.md para superficies de trabajo |
| `reference/mode-persuade.md` | adaptar: brand/guide.md para solicitudes de marca o persuasión |
| `reference/mode-read.md` | adaptar: visual/direccion-y-prototipos.md para superficies de lectura |
| `reference/new-work.md` | adaptar: visual/direccion-y-prototipos.md; respetar canon, sin maquinaria de decisiones |
| `reference/onboard.md` | combinar: interaction/onboarding-y-resiliencia.md y plantilla existente |
| `reference/operate.md` | combinar: visual/criterios-visuales.md, visual/revision.md y movimiento |
| `reference/optimize.md` | adaptar: movimiento, visualización de datos y revisión; código productivo en frontend |
| `reference/overdrive.md` | adaptar: movimiento, visualización de datos y revisión; no agregar efectos o dependencias por defecto |
| `reference/polish.md` | combinar: visual/revision.md y criterios por dominio |
| `reference/quieter.md` | combinar: visual/criterios-visuales.md |
| `reference/region-map.md` | adaptar: visual/direccion-y-prototipos.md; sin CLI ni automatización ficticia |
| `reference/routing.md` | excluir: menú de comandos de la antigua skill, sustituido por el enrutador actual |
| `reference/shape.md` | combinar: visual/direccion-y-prototipos.md y entrevistas existentes |
| `reference/typeset.md` | combinar: visual/criterios-visuales.md, Geist y perfiles aprobados |
| `reference/visualize.md` | adaptar: visual/direccion-y-prototipos.md, comparación y revisión de activos |
| `scripts/VERSION` | pendiente: decisión del propietario sobre Impeccable Live y su kit |
| `scripts/command-metadata.json` | pendiente: decisión del propietario sobre Impeccable Live y su kit |
| `scripts/data/font-index-failures.json` | pendiente: decisión del propietario sobre Impeccable Live y su kit |
| `scripts/data/font-index.json` | pendiente: decisión del propietario sobre Impeccable Live y su kit |
| `scripts/engine-checksums.txt` | pendiente: decisión del propietario sobre Impeccable Live y su kit |
| `scripts/impeccable` | pendiente: decisión del propietario sobre Impeccable Live y su kit |
| `scripts/impeccable.cmd` | pendiente: decisión del propietario sobre Impeccable Live y su kit |
| `scripts/live-browser-dom.js` | pendiente: decisión del propietario sobre Impeccable Live y su kit |
| `scripts/live-browser-ignores.js` | pendiente: decisión del propietario sobre Impeccable Live y su kit |
| `scripts/live-browser-session.js` | pendiente: decisión del propietario sobre Impeccable Live y su kit |
| `scripts/live-browser.js` | pendiente: decisión del propietario sobre Impeccable Live y su kit |
| `scripts/modern-screenshot.umd.js` | pendiente: decisión del propietario sobre Impeccable Live y su kit |

## Pendiente

- El propietario tiene preguntas pendientes sobre el modo Live y el detector. Sus scripts, definiciones específicas y hook no se han integrado ni ejecutado hasta recibir esas decisiones.
- El respaldo original verificado se conserva en `/var/home/fbravohz/.local/share/drasus/skill-recovery/impeccable-toolkit-0.1.18.tar.gz`; también queda una copia temporal en `/tmp/drasus-ui-ux-merge-cwa3q_bk/`.

## Verificación

La skill pasó `quick_validate.py`. Se verificaron los enlaces y el espacio final en los archivos modificados o nuevos, y `git diff --check` en los cambios rastreados. Se revisó el alcance de las guías nuevas y no se conservan invocaciones a las rutas de la skill antigua. No se ejecutaron el CLI, los scripts ni el detector de Impeccable; esta revisión no acredita su seguridad ni su funcionamiento.

## Actualización de decisiones posteriores

El propietario eligió adaptar el método de Impeccable Live a las herramientas de Drasus. Se integró la exploración de variantes en ejecución en `references/visual/direccion-y-prototipos.md`: conserva la comparación sobre una misma superficie y sus criterios, y excluye el helper, el servidor, la inyección y el protocolo propietario. React/Vite está documentado como compatible por Impeccable; la comprobación en la ventana nativa de Tauri queda para cuando exista el cascarón de desarrollo de STORY-074. No se instaló ni ejecutó el kit.

| Recurso antes pendiente | Tratamiento actualizado |
|---|---|
| `agents/impeccable_manual_edit_applier.toml` | Excluir: agente ligado al protocolo de aplicación automática; la skill de diseño no modifica componentes productivos. |
| `reference/degraded/manual-edit-applier.md` | Excluir por el mismo motivo; la versión alternativa conserva el contrato del agente. |
| `reference/generate.md` | Adaptar la comparación de variantes y la selección de región a `direccion-y-prototipos.md`; excluir el protocolo CLI, el overlay y la espera de eventos. |
| `reference/live-setup.md` | Adaptar la distinción entre navegador de desarrollo y ventana Tauri; excluir configuración, inyección y cambios CSP del helper. |
| `reference/live.md` | Adaptar criterios de identidad, comparación y elección visual; excluir el protocolo de sesión y la escritura automática a fuentes. |
| `scripts/VERSION`, `scripts/command-metadata.json`, `scripts/data/font-index-failures.json`, `scripts/data/font-index.json`, `scripts/engine-checksums.txt`, `scripts/impeccable`, `scripts/impeccable.cmd`, `scripts/live-browser-dom.js`, `scripts/live-browser-ignores.js`, `scripts/live-browser-session.js`, `scripts/live-browser.js`, `scripts/modern-screenshot.umd.js` | Excluir: piezas del ejecutable, su catálogo y su runtime; no se requieren para adaptar el método visual. |
| `reference/hooks.md` | Pendiente: falta elegir entre detector manual, hook automático o excluirlo. El propietario pidió conocer sus funciones y activación; el sistema no se activa por esa consulta. |

El detector no se instaló ni ejecutó. La diferencia entre sus modos se explicó al propietario; no cuenta como autorización para activar el hook.

## Resolución del detector

El propietario excluyó el detector de Impeccable. La comprobación del workspace encontró que no están instalados `.agents/skills/impeccable`, `.impeccable` ni su lanzador; tampoco existe el binario en `PATH`, `/var/home/fbravohz/.impeccable/bin/impeccable` o `/var/home/fbravohz/.codex/skills/impeccable`. `.codex/hooks.json` sí existe, pero solo declara el hook `SessionStart` de Drasus y no contiene Impeccable. No instalé ni ejecuté el detector. Se conserva la revisión visual y las comprobaciones específicas con herramientas existentes, sin invocar el motor de Impeccable.

| Recurso antes pendiente | Tratamiento actualizado |
|---|---|
| `reference/hooks.md` | Excluir: el propietario descartó el detector y su hook automático. |
