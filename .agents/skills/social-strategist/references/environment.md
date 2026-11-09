# Entorno de producción

Verifica las herramientas necesarias para el flujo elegido. Consulta [la estrategia de contenido](../../../../docs/CONTENT-STRATEGY.md), sección 6, para usos y límites. Informa disponibilidad antes de proponer instalaciones. Al proponer una instalación, consulta las instrucciones oficiales vigentes para esa herramienta y plataforma.

Pide confirmación explícita antes de ejecutar cualquier instalación. Para instalaciones con sudo o pasos manuales, presenta el comando o enlace oficial y deja la ejecución al usuario. Comprueba compatibilidad antes de proponer un comando y vuelve a verificar disponibilidad después; no supongas que tuvo éxito.

| Herramienta | Uso | Verificación |
|---|---|---|
| Manim | Animación matemática y conceptual | Verifica con `command -v manim`. |
| Python 3 | Runtime de Manim | Verifica con `command -v python3`. |
| Motion Canvas | Animación de diagramas y AST | Verifica las dependencias del proyecto de video; no asumas un comando global. |
| ffmpeg | Recorte, concatenación y conversión | Verifica con `command -v ffmpeg`. |
| Whisper | Subtítulos | Verifica con `command -v whisper` y comprueba que ffmpeg esté disponible. |
| Node y npm | Remotion y proyectos de video | Verifica con `command -v node` y `command -v npm`. Confirma antes de crear archivos de proyecto. |
| OBS Studio | Grabación de pantalla | Comprueba la instalación desde su fuente oficial. |
| DaVinci Resolve | Edición final | Comprueba la instalación desde el sitio oficial de Blackmagic Design. |
