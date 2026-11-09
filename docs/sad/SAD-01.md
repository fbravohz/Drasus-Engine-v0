## 1. Introducción y Objetivos

### 1.1 Propósito y estado

El Documento de Arquitectura de Software (SAD) es el mapa del sistema: explica sus componentes, responsabilidades, conexiones y propiedades que condicionan el diseño. Describe la arquitectura objetivo, no el avance implementado. El [Roadmap](../ROADMAP.md) y las Órdenes de Trabajo reflejan entregas; los registros de decisiones arquitectónicas (ADR) desarrollan las decisiones técnicas concretas.

### 1.2 Propósito de Drasus Engine

Drasus Engine es un motor local de investigación cuantitativa. Recibe datos de mercado y criterios del usuario, genera estrategias candidatas, las somete a pruebas estadísticas y simulación, y habilita órdenes solo después de aplicar controles de riesgo. Registra qué corridas produjeron y evaluaron cada candidata para que el usuario pueda revisar la evidencia y decidir qué operar. La [Definición de Producto](../PRODUCT.md) establece capacidades y límites; Drasus no garantiza rentabilidad.

### 1.3 Objetivos arquitectónicos

- **Permitir distintos métodos de búsqueda:** cada método usa un contrato común —qué recibe, qué devuelve y cuántas evaluaciones puede consumir— y reporta sus evaluaciones. Ese conteo permite ajustar la estadística por elegir una ganadora después de probar muchas variantes ([ADR-0113](../adr/ADR-0113.md), [ADR-0151](../adr/ADR-0151.md)).
- **Hacer reproducible y auditable la investigación:** las corridas conservan su configuración y sus vínculos con datos y estrategias. Así se puede reconstruir qué produjo cada resultado y repetirlo con las mismas entradas ([Roadmap](../ROADMAP.md), [ADR-0150](../adr/ADR-0150.md)).
- **Validar antes de operar:** una ruta rápida compara muchas candidatas; las finalistas vuelven a probarse en una simulación que modela la ejecución con más detalle. La ruta rápida no puede inflar sus resultados frente a esa revisión, y cada orden pasa por controles de riesgo antes de enviarse ([ADR-0114](../adr/ADR-0114.md), [ADR-0025](../adr/ADR-0025.md)).

### 1.4 Partes y límites del sistema

| Parte | Responsabilidad |
|---|---|
| **Core de ejecución** | Mantiene las reglas, los datos y la lógica de simulación y operación. Corre en el equipo o servidor privado virtual (VPS) del usuario. Cada feature (componente funcional) declara entradas y salidas para conectarse con otras ([ADR-0137](../adr/ADR-0137.md), [ADR-0143](../adr/ADR-0143.md)). |
| **Aplicación de escritorio** | Usa Tauri 2 y React/TypeScript para presentar controles y resultados. Se comunica con el Core por gRPC (protocolo de comunicación) y Protobuf (formato de mensajes) ([ADR-0165](../adr/ADR-0165.md)). |
| **Cabina de Mando del proveedor** | Administra identidad, licencias y telemetría (datos de uso y estado). No ejecuta estrategias, pruebas históricas (backtests) ni órdenes ([ADR-0143](../adr/ADR-0143.md)). |

ADR-0143 y ADR-0154 definen qué datos fluyen entre estos límites, cómo depende la telemetría del plan y qué secretos permanecen en el entorno del usuario. [SAD-02](SAD-02.md) y [SAD-04](SAD-04.md) desarrollan la arquitectura y sus vistas; [SAD-20](SAD-20.md) y [SAD-22](SAD-22.md) detallan soberanía de datos y Cabina de Mando.

### 1.5 Metas y estado de entrega

La arquitectura debe producir corridas repetibles, probar muchas candidatas con eficiencia, controlar el tiempo entre una señal y su orden, y reducir selecciones afortunadas. El [Roadmap](../ROADMAP.md) fija fases y valores objetivo; [ADR-0114](../adr/ADR-0114.md), [ADR-0063](../adr/ADR-0063.md) y [ADR-0151](../adr/ADR-0151.md) definen sus políticas técnicas.

El Roadmap lista ≤5 s para watchdog (supervisor de salud) y Kill Switch (corte de emergencia). [ADR-0026](../adr/ADR-0026.md) fija el umbral de detección: activar el corte si falta el latido por más de 5 s. El Roadmap no aclara si mide ese umbral o el tiempo de respuesta posterior; queda pendiente definir qué intervalo representa el KPI.
