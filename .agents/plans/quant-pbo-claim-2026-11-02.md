# Revisión cuantitativa acotada: afirmación sobre PBO

## Análisis del Quant-Engineer

**Alcance:** comprobar en SAD-01 la afirmación de que un PBO ≤0.10 garantiza que el Alpha no es suerte. No se audita la implementación ni se reevalúa la metodología CPCV/PBO.

**Veredicto:** NO APTO para la afirmación original. ADR-0063 define `PBO_Threshold` configurable con valor predeterminado 0.10 y rechaza resultados que lo exceden; ADR-0151 precisa que PBO/CSCV es la puerta primaria, agnóstica a la métrica, para decisiones de selección entre variantes y la distingue del DSR específico de Sharpe y de Monte Carlo, que estima ruina y percentiles. Un umbral de aceptación no demuestra rentabilidad futura ni elimina toda posibilidad de suerte.

**Corrección aplicada en SAD-01:** describir `PBO_Threshold` como filtro configurable en los puntos de selección pertinentes, conservar 0.10 como predeterminado documentado y retirar la garantía de que el Alpha no es suerte.

**Fuentes:** [ADR-0063](../../docs/adr/ADR-0063.md), [ADR-0151](../../docs/adr/ADR-0151.md).

**Preguntas abiertas:** ninguna para esta corrección. La revisión no certifica fórmulas, implementación ni resultados estadísticos.
