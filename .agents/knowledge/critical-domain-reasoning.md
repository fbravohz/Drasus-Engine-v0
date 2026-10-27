# Razonamiento crítico de dominio

Obligatorio antes de fijar reglas cuantitativas, estadísticas, criptográficas,
financieras, fiscales o legales, y al revisar cambios de riesgo alto o crítico.
La selección de modelo sigue la petición vigente del propietario y `AGENTS.md`.
Una etiqueta de modelo no sustituye la evidencia especializada.

## CRITICAL-DOMAIN-REASONING-1 — Siete comprobaciones

1. **Límite degenerado:** empuja parámetros a extremos (saldo cero, tiempo cero,
   intentos o usuarios ilimitados). Documenta si aparecen divisiones por cero,
   condenas universales, costo infinito o bloqueo total; corrige el modelo.
2. **Especialidad:** usa la referencia de dominio apropiada, canon y fuentes
   primarias verificadas antes de convertir una conclusión generalista en regla.
3. **Afirmación fuerte:** cuanto mayor el peso de una afirmación en la seguridad
   o validez del sistema, mayor el esfuerzo para refutarla; la retórica no prueba.
4. **Supuestos rederivados:** enumera los supuestos del teorema, métrica o patrón
   adoptado y demuestra cuáles se cumplen aquí. No traslades una conclusión
   suponiendo independencia entre estrategias que comparten datos.
5. **Primitivo y política:** distingue almacenamiento (por ejemplo, ledger de
   intentos) de la política que agrupa, pondera, limita o interpreta esos datos.
6. **Cierre transitivo:** enumera todos los puntos que producen el mismo efecto:
   selección al minar, filtrar, optimizar, agrupar, ordenar o construir portafolios.
7. **Tipos y casos:** identifica qué reglas aplican a cada clase; no equipares
   selección con validación OOS, ni WFO con WFA puro, ni simulación con ejecución.

## CRITICAL-DOMAIN-REASONING-2 — Cierre

Conserva supuestos, evidencia, contraejemplos, corrección y prueba de cierre en
el documento que consume la decisión. Si falta una comprobación aplicable,
declara la decisión no verificada; no transfieras al propietario el último filtro
de errores caros. Una decisión legal depende también de jurisdicción y vigencia.

## Fuentes de recuperación

`6d92edd:.agents/knowledge/critical-domain-reasoning.md`, siete comprobaciones
de la regla 1 y cierre de la regla 2.
