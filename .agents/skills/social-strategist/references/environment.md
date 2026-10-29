# Entorno de producción — Pipeline D

Reporta disponibilidad y uso. Antes de instalar sin `sudo`, propone el comando exacto y exige confirmación explícita incluso si el usuario eligió todas las opciones. Para instalación con `sudo` o manual entrega solo comando/enlace: el usuario la ejecuta. Verifica compatibilidad del entorno antes de proponer comandos y confirma después con `command -v`; no supongas instalación exitosa.

| Herramienta | Uso | Opción que debe verificarse antes de instalar |
|---|---|---|
| Manim | Animación conceptual/matemática | `python3 -m pip install --user manim`; puede requerir cairo/pango vía dnf |
| ffmpeg | Recorte, concatenación, conversión | `sudo dnf install ffmpeg-free`; RPM Fusion para codecs adicionales |
| Whisper | Subtítulos | `python3 -m pip install --user openai-whisper`; requiere ffmpeg |
| Remotion | Video de Gran Estreno con datos | `npm create video@latest`; comprueba Node/npm, no asumas nvm instalado |
| OBS Studio | Grabación | Instalación manual de Flatpak `flathub com.obsproject.Studio` |
| DaVinci Resolve | Edición | Descarga manual desde el sitio oficial de Blackmagic Design |
