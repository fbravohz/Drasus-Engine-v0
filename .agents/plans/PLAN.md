# Plan maestro consolidado

> **Propósito:** sustituir los trece planes fragmentados por un solo mapa de trabajo.
>
> **Estado:** 🟡 Pendiente de auditoría de implementación. Este archivo sintetiza lo declarado entre 2026-06-21 y 2026-07-26; no prueba que código, migraciones, ADRs, pruebas ni documentación sigan aplicados.
>
> **Precedencia:** si este plan contradice una fuente canónica vigente, prevalecen ADR, feature, orden, Roadmap, modelo de datos y código. La siguiente fase verifica; no vuelve a decidir desde este resumen.

## 1. Invariantes

1. Drasus no provee un lago de datos de mercado. El usuario aporta datos mediante API autenticada, credenciales de bróker o exportación local.
2. OHLC, bid, ask, precio negociado y volumen son primitivos. Todo cálculo derivado, incluido VWAP, es indicador y no se persiste en la ingesta cruda.
3. La identidad y el linaje nunca fusionan procedencias. La serie conserva instrumento, temporalidad, venue/bróker, proveedor, método y declaración aplicable.
4. La simulación compra al ask y vende al bid. Nunca usa precio medio ni uno genérico.
5. Una capacidad solo se habilita si el dato requerido existe y tiene semántica declarada. Si no, queda visible, apagada y con motivo.
6. Ausencia o ambigüedad de dato no se rellena como hecho: se rechaza, se declara nula o requiere confirmación del usuario.
7. Secretos de bróker no salen del equipo del usuario. No publicitar esa promesa sin una auditoría técnica de rutas de excepción.
8. No se descarta una clase de activo por límite de diseño. Se corrige el límite o se difiere la población hasta tener fuente y consumidor.

## 2. Ingesta, normalización y linaje

### Contrato de datos

- Separar físicamente QuoteTick y TradeTick: cotización no es ejecución.
- Usar enteros ×10⁸ para precio/volumen y nanosegundos UTC para tiempo.
- Persistir spread_reported solo cuando la fuente lo observa; derivar ask − bid en lectura.
- Mantener separados volume_ticks, volume_traded, trade_count, volume_buy y quote_asset_volume.
- Reservar el esquema de barras, profundidad, futuros y opciones; no poblar sin fuente que defina la semántica.
- No materializar derivados sin observación independiente: precio medio, rango, precio típico, VWAP calculado, volume_sell, cierre ajustado ni contrato continuo.

### Barras y acumuladores

- Catálogo: TIME, TICK_COUNT, VOLUME, RENKO, RANGE y DOLLAR.
- Heikin Ashi es indicador. Kagi, Point & Figure y Line Break quedan diferidos hasta tener consumidor y tipo de dato propio.
- volume_traded y volume_buy acumulados requieren i128; conservar ×10⁸. Verificar soporte efectivo en dominio, Parquet/Polars, bridge y límites f64/2⁵³.
- DOLLAR depende del acumulador ancho. Las demás barras no deben esperar esa dependencia.

### MT5 y archivos locales

Los CSV M1 y tick analizados son candidatos de ingesta. Los .dat no se adivinan: requieren identificar la herramienta de origen o reexportarse a CSV con plantilla validada.

- Aceptar TAB y CRLF; escalar sin pérdida; declarar reloj, zona, venue y sesión.
- Exigir instrumento y plantilla de sesión declarados antes de importar. El Gate de Calendario compara huella declarada y observada; no infiere una sesión.
- Reconstruir BBO unilateral mediante carry-forward acotado. Marcar cada lado como observado o arrastrado.
- No emitir cotización hasta observar ambos lados desde la apertura; contar descartes iniciales.
- Detectar cotización cruzada y barras con rango real y volume_ticks = 0.
- No asumir unidad para <VOL>: guardar el valor crudo y marcarlo no verificado.
- Usar spread_reported M1 para reconstruir ask solo bajo la política canónica y su validación aplicable.

### Mapeo y trazabilidad

- Rechazar por defecto un formato sin plantilla reconocida ni mapeo declarado.
- Una sugerencia automática solo puede ser borrador. La transformación se acepta con vista previa explícita del usuario, guardarraíles propios y plantilla sometida a control de calidad.
- Sellar una declaración de mapeo inmutable. Dataset y corrida de transformación la referencian por identificador estable; nunca copiar su contenido a cada fila de mercado.

### Declaración por tramo

Un archivo no es necesariamente homogéneo.

- Declarar el reloj como zona IANA ancla más desplazamiento fijo en minutos.
- Declarar rangos explícitos y permitir varias declaraciones no solapadas para el mismo archivo.
- Si cambia una característica, declarar varios tramos. Si cambia origen real, usar otro venue/source_provider.
- Ejecutar Gate de Homogeneidad: fractura de magnitud/disponibilidad sobre umbral configurable y fecha anterior al inicio operativo declarado del venue.
- No versionar la unidad de volumen en el catálogo de bróker: la variación pertenece al tramo importado.

### Frontera cuantitativa

- Volumen Forex OTC representa el flujo propio del bróker, no interés global. volume_ticks puede medir actividad relativa de la misma serie.
- Sin trades clasificados por agresor, CVD/OFI no se habilita.
- La señal futuro→contado modela dos instrumentos y relojes separados. Requiere contrato específico, rolo como vista y prueba fuera de muestra contra una base de contado.
- No iniciar topología cross-market hasta cerrar TASK-059, contrato del momento de conocibilidad.

## 3. Conectores y legal

### Conectores

- La importación local es un tercer canal junto a Bulk y Delta. Lee, valida, normaliza y particiona; no descarga.
- El puerto de descarga acepta canal autenticado, archivo local y descompresión conectable; no fuerza HTTP al orquestador.
- Fuente y método de obtención son parte del linaje y de la política legal de cada partición.
- Futuros, L2/L3 y opciones se difieren hasta tener feature consumidora y feed contratado por el usuario.
- Binance pasa a credenciales del usuario o sale del producto. No se solicita permiso a Binance; DEBT-027 es de prioridad baja.

### Límites legales

- La agregación externa solo usa particiones con permiso explícito. Revisar fuente, canal y uso.
- Regime y Correlation no se venden externamente sin derechos de la fuente. BrokerFriction requiere comprobar también el origen de su precio; Sentiment requiere definir su fuente antes de adaptar datos.
- Un conector Binance anónimo para producto de pago no es apto hasta revisar términos de usuario y desarrollador o cambiar el modelo. Credenciales del usuario reducen riesgo, no otorgan derechos ajenos.
- Revisar por separado histórico/Bulk y vivo/Delta.
- Sustituir fixtures reales por sintéticos antes de distribuir repositorio o binarios.
- T&C/EULA debe declarar: dato de terceros sin garantía, resultados dependientes de insumo y herramienta, no asesoría.
- Aclarar que la titularidad del trabajo del usuario no transfiere derechos de mercado que el usuario no posea.
- El nivel gratuito es una cooperativa transparente: solicita consentimiento separado, muestra qué aporta el usuario y entrega una contraprestación visible. Requiere estrategia completa y resultados; los agregados anónimos no sustituyen ese intercambio.
- Mantener bloqueo por jurisdicción como capacidad de producto antes de ampliar distribución.

## 4. Secuencia de validación e implementación

No ejecutar esta lista como prueba de estado. Cada paso inicia con una auditoría de fuentes canónicas y árbol de trabajo.

1. **Auditar realidad.** Inventariar ADR-0114, 0141, 0154–0156, 0158 y 0162–0163; TASK-059/060/061/062/063/064/065/069; STORY-051/057/058; deudas asociadas; features y crates implicados. Clasificar cada punto como implementado, parcial, sustituido o pendiente con evidencia reproducible.
2. **Cerrar contrato de importación.** Verificar e implementar, si falta: tipos de tick, identidad/linaje, i128, declaración por tramo, reloj ancla+offset, broker_server_clock y Gate de Homogeneidad. Migración y modelo de datos se actualizan juntos.
3. **Construir lector MT5.** Empezar por CSV M1. Agregar tick unilateral después. Mantener .dat fuera hasta contar con plantilla validada. Probar CRLF, escala, DST, arrastre, inicio de sesión, cruces y anomalías de volumen.
4. **Barras y validadores.** Implementar barras sin i128; después DOLLAR. Ejecutar gates de calendario, punto-en-el-tiempo y semántica de spread.
5. **Activar conectores legalmente.** Registrar decisión legal por proveedor, canal y uso antes de ampliar una fuente.
6. **Datos avanzados.** Solo con datos y relojes verificables: rolo, señal futuro→contado, L2/L3 y opciones. Exigir prueba robusta fuera de muestra y aceptar “sin mejora”.

## 5. Bloqueos e insumos externos

| Insumo | Efecto |
|---|---|
| Herramienta o especificación de .dat | Impide su ingesta hasta identificarla o reexportar |
| Declaración de origen, venue, reloj, unidad y sesión | Determina semántica verificable; no se infiere |
| Catálogo inicial de plantillas | Limita formatos soportados; verificar estado de TASK-060 |
| Términos de usuario y desarrollador por fuente | Habilita conector, agregación y redistribución |
| Feed de futuros, opciones o L2/L3 del usuario | Habilita trabajo avanzado; queda diferido |

## 6. Calidad, proceso y contexto

- Aplicar Doc-First antes de proponer regla, contrato o fórmula.
- No precocinar soluciones al despachar. El ejecutor recibe problema, restricciones, fuentes y resultado; debe cuestionar premisas débiles.
- Defecto de correctitud se corrige. Deuda tiene ficha y disparador. Tarea espera trabajo, decisión o insumo.
- Antes de iniciar una feature, revisar juntas las deudas activas y promover las que cumplieron su disparador.
- Aplicar ADR-0133 a cambios de correctitud en domain/ o persistence.
- Pruebas deben discriminar comportamiento; cuando aplique, demostrar rojo antes de verde.
- Mantener migración y modelo de datos consistentes columna por columna.
- Para contexto, usar núcleo breve más gates/referencias releídos al momento de usar; conservar índices ligeros para deudas, progreso y órdenes.
- Auditar la reforma pendiente antes de repetirla: núcleo y gates para Tech Lead, UI, QA, Flutter, Bridge y Architect; índice para deuda y progreso; nombres de órdenes y lecciones ordenados por número; memoria global; una sola regla de lectura progresiva y frontera memoria/progreso.

## 7. Contenido público

1. Auditar estado y commits antes de publicar. No presentar trabajo en curso como cerrado.
2. Instalar captura de código (silicon o carbon-now-cli) solo con autorización explícita.
3. Producir por bloques: pulsos ES/EN de cierres verificados, short de bug multiplataforma, episodio de fundamentos y mitos de trading algorítmico.
4. Actualizar estrategia y progreso después de cada bloque para evitar duplicación.

## 8. Fuentes reemplazadas

| Fuente | Conservada en |
|---|---|
| Sesiones de 2026-07-18, 2026-07-19 y 2026-07-22/23 | §§1, 2, 4 y 6 |
| Esquema canónico de datos de mercado | §2 |
| Decisión de fuentes de mercado | §3; se descarta la parte sustituida por ADR-0154 |
| Plan de escala, barras e ingesta | §2 y §4 |
| Informes Quant/Architect de MT5, volumen, reloj y tramos | §2 |
| Veredicto legal de datos de mercado | §3 |
| Reforma de contexto y conocimiento | §6 |
| Arranque de bitácora pública | §7 |

## 9. Cierre

La consolidación termina cuando .agents/plans/ contiene únicamente este archivo. El siguiente trabajo es auditoría de implementación (§4.1), no una nueva decisión de arquitectura.
