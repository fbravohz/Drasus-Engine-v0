# Bibliotecas para la interfaz de Drasus

Esta comparación prepara un piloto visual entre HeroUI y shadcn/ui + Radix para Tauri/React. Antes del piloto se definirán en [DESIGN.yaml](../DESIGN.yaml) los tokens y comportamientos visuales que se compararán. Los archivos CSS y la configuración Tailwind del cliente implementarán las clases y recetas reutilizables; ambos kits consumirán la misma capa Drasus. La biblioteca ganadora sigue por decidir. TanStack Table y Virtual quedan adoptados para los casos que requieren sus capacidades. La caché de datos de interfaz queda aplazada hasta que una superficie real demuestre su necesidad. La autorización general de TanStack está en [DESIGN.yaml](../DESIGN.yaml) y la elección de React Flow para el Canvas está en [ADR-0136](../adr/ADR-0136.md). Estas decisiones no significan que los paquetes estén instalados o que su integración esté verificada.

El sistema de iconos también sigue sin selección. Los SVG locales del Dashboard describen la implementación actual; el piloto debe comparar cobertura, accesibilidad, consistencia visual, licencia y costo de mantenimiento antes de elegir una biblioteca.

## Componentes visuales

| Opción | Qué aporta | Costo o límite para Drasus |
|---|---|---|
| [shadcn/ui](https://ui.shadcn.com/docs) + [Radix Primitives](https://www.radix-ui.com/primitives/docs/overview/introduction) | Componentes con estilos y código editable sobre una base de interacciones sin estilo. | Hay que mantener y actualizar el código incorporado. El acabado de Drasus requiere adaptación. |
| shadcn/ui + [Base UI](https://base-ui.com/react/overview/about) | El mismo enfoque visual de shadcn con otra base de interacciones. Es la opción predeterminada actual de shadcn; [Radix sigue admitido](https://ui.shadcn.com/docs/changelog/2026-07-base-ui-default). | La elección de base debe contrastarse con los componentes y comportamientos necesarios; no basta con comparar la apariencia. |
| [HeroUI v3](https://heroui.com/en/docs/react/getting-started) | Componentes de paquete con estilos personalizables, Tailwind CSS y React Aria. La biblioteca mantiene su código y publica actualizaciones. | Adaptar sus estilos al lenguaje de Drasus. El núcleo usa Apache-2.0; [HeroUI Pro](https://heroui.com/en/docs/react/releases/v3-0-0) ofrece componentes y plantillas adicionales de pago, cuya disponibilidad y términos se revisarán cuando se necesiten. |
| [Material UI](https://mui.com/material-ui/getting-started/) | Catálogo amplio y tema configurable con componentes listos para usar. | Hay que adaptar Material Design y coordinar su sistema de estilos con Tailwind. MUI X tiene [ediciones comerciales](https://mui.com/x/introduction/licensing/) para algunas capacidades avanzadas; no es necesario elegirlas para usar TanStack Table. |
| [Chakra UI](https://chakra-ui.com/) | Componentes componibles y un sistema de temas con tokens y variantes. | Su sistema de estilos usa Emotion; integrar sus estilos con Tailwind requiere definir una única fuente de tokens. |
| [Mantine](https://mantine.dev/) | Catálogo amplio de componentes y hooks, con tema y estilos personalizables. | Hay que adaptar su apariencia y coordinar sus estilos con los tokens de Drasus. |

Shadcn permite elegir Radix, Base UI o React Aria como base. Esa elección afecta la API y el comportamiento de los componentes, pero no decide por sí sola la identidad visual. El piloto autorizado compara **shadcn/ui + Radix** y **HeroUI v3** con los mismos controles de Drasus. Base UI permanece como alternativa futura; no forma parte de este piloto.

Después de definir los tokens y de implementarlos en el tema CSS/Tailwind del cliente, conviene probar en Tauri una barra lateral, un campo numérico, un menú y un diálogo con el estilo de Drasus. La prueba debe cubrir teclado y foco, accesibilidad, densidad, personalización mediante tokens, funcionamiento local y costo de actualización. Esas características no quedan verificadas por la documentación de una biblioteca.

## Posibles usos de TanStack

| Biblioteca | Posible aplicación | Estado |
|---|---|---|
| [Table](https://tanstack.com/table/latest/docs/overview) | Ordenar, filtrar y configurar tablas de resultados u operaciones. | Adoptada para tablas que necesiten estas capacidades; la capa visual se construye con la biblioteca que resulte del piloto. |
| [Virtual](https://tanstack.com/virtual/latest/docs/introduction) | Dibujar solo las filas o columnas visibles de listas extensas. | Adoptada para listas o tablas cuyo volumen justifique virtualización. No reduce por sí sola los datos transferidos. |
| [Query](https://tanstack.com/query/latest) | Gestionar el estado y la caché de consultas asíncronas. | Aplazada. No se selecciona hasta que una superficie real demuestre una necesidad de caché; Local-First por sí solo no la exige. |
| [Form](https://tanstack.com/form/latest) | Manejar formularios de configuración, errores y estado de envío. | Candidata; la validación de interfaz no sustituye las reglas del Core. |
| [Router](https://tanstack.com/router/latest) | Organizar la navegación y el estado de las superficies React. | Candidata; no cambia la regla del Canvas único. |
| [Hotkeys](https://tanstack.com/hotkeys/latest/docs/overview) | Gestionar atajos y sus contextos para Dashboard, Canvas y diálogos. | Candidata en beta; hay que verificar su comportamiento en Tauri, sistemas y teclados. |
| [Pacer](https://tanstack.com/pacer/latest) | Regular eventos de interfaz como una búsqueda mientras el usuario escribe. | Candidata en beta; no sustituye el control de flujo ni descarta eventos del Core. |
| [Store](https://tanstack.com/store/latest) | Estado reactivo local de interfaz si aparece una necesidad que React no cubra con suficiente claridad. | Candidata en alpha; no es una base de datos ni la fuente de verdad del dominio. |
| [DB](https://tanstack.com/db/latest/docs/overview) | Colecciones y consultas reactivas en el cliente. | No se propone incorporarla ahora. Su compatibilidad con la propiedad de datos del Core y la recuperación por gRPC requeriría evaluación. |

### Caché de datos (aplazada)

Local-First no determina por sí solo que React necesite una segunda caché de datos. El Core conserva la propiedad y persistencia de dominio según [ADR-0165](../adr/ADR-0165.md); por ahora no se elige TanStack Query ni una política de caché para la interfaz.

El tema se reabre cuando una superficie real muestre una necesidad concreta que no resuelva directamente el contrato gRPC y el estado propio de la pantalla. Si entonces existe incertidumbre técnica, se prepara un SPIKE acotado; un ADR registra una decisión arquitectónica aprobada. Solo se registra deuda si ya hay una carencia comprobada en una capacidad construida.

TanStack Start agrega funciones de servidor y renderizado de aplicaciones web; no se necesita para justificar el cliente de escritorio Tauri. Elegir una biblioteca para la interfaz no cambia la propiedad del dominio, la persistencia, los contratos Protobuf ni los streams gRPC de [ADR-0165](../adr/ADR-0165.md).
