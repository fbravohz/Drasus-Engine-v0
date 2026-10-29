# Producción por formato

Consulta solo el flujo seleccionado y el canon `CONTENT-STRATEGY.md`. Si falta, no sustituyas sus reglas con supuestos.

## Pulso — Pipeline A

Por cada Historia/Riesgo/Tarea cerrada: redacta Story Core en español con Hook, Problema, Solución, Resultado y Siguiente paso; tradúcelo completo al inglés. Genera por idioma los cinco formatos del canon —Twitter, Discord, Instagram, TikTok, Facebook— con prompts de imagen si no hay captura. Guarda ambos documentos y actualiza cierre procesado y `último_commit_procesado`.

## Episodio — Pipeline B

Se activa al elegir el episodio detectado por masa narrativa. Confirma ángulo y plantilla del Pilar: primer episodio frente a Gran Estreno con datos reales según canon. Antes del guion informa herramientas críticas faltantes y ofrece configurar entorno. Produce guion minuto a minuto, scripts Manim, componentes Remotion solo para Gran Estreno con datos exportados, sugerencias ffmpeg/OBS y checklist de pantalla, voz y cámara que el usuario grabará. Guarda bajo `episodios/[slug]/`, aplica idioma canónico y registra historias/riesgos/decisiones cubiertos para no repetirlos.

## Postura — Pipeline C

Elige un mito o afirmación del trading algorítmico hispano del Pilar F. Usa argumentos técnicos verificables ligados a lo que Drasus resuelve o tiene previsto resolver; distingue capacidades construidas de futuras, sin promesas falsas. Produce corto de 30–60 s y variantes para las demás redes, ES y EN. Guarda como Pulso y etiqueta `Pilar F / postura` en progreso.

## Cápsula educativa — Pipeline E

Usa la semilla elegida. Con `--capsula [concepto]`, localiza primero la Orden que implementó ese concepto y lee su feature/ADR pertinente: qué hace, qué problema resuelve y cómo lo usa Drasus. No inventes diseño de features sin desarrollar; si explicas una parte futura vinculada a la semilla, identifícala como tal con EPIC y propósito.

Conserva cuatro bloques: contexto histórico —quién, cuándo, problema—; idea sin cálculo completo y analogía solo si ayuda; relevancia quant/trading con situación reconocible; uso concreto en Drasus y estado de implementación.

Evalúa granularidad: concepto simple admite thread de 4–6 tweets y carrusel; uno rico, Short/Reel de 60 s con indicaciones Manim y thread de soporte. Genera los formatos completos en ES y EN separados:

- Twitter: 4–7 tweets, primer hook y último uso en Drasus.
- Discord: 600–900 palabras, contexto técnico y enlace a feature/ADR.
- Instagram: 6–8 slides; 1 hook, 2–5 bloques, 6 CTA; adapta las adicionales al concepto.
- TikTok/Short: 0–3 s hook, 3–45 s concepto, 45–60 s uso en Drasus y CTA; incluye indicaciones de animación si Manim aporta valor.

Guarda ambos idiomas, mueve la semilla a cápsulas producidas y anota si un episodio ya cubrió el concepto.
