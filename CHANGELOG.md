# Registro de cambios

## 0.1.0 — En desarrollo

- Base React, TypeScript, Vite, Tailwind y configuración Tauri 2.
- Documentación de arquitectura, captura y requisitos.
- La captura y la compilación nativa todavía no están verificadas.

## Avance posterior

Navegación inicial con ocho secciones y estados vacíos. Se añadió un contrato de proveedor y un analizador parcial de Ethernet/VLAN e IPv4, aún sin integración ni proveedor de captura. Rust se instaló en el perfil de usuario sin añadirlo al PATH; la instalación de Build Tools detectada está marcada como incompleta y se está verificando.

- `cargo check` verificado con Rust y MSVC. Se corrigió la codificación de la configuración Tauri y se generó un icono.

- Proveedor Npcap opcional, controles de captura, lotes de eventos y tabla con detalles hexadecimales. La captura real aún no se ha verificado con Npcap instalado.

- Npcap 1.90 instalado por separado; se verificó la carga de la biblioteca, la enumeración y recepción de tráfico con el proveedor en este equipo.

- Se agregaron filtros de visualización validados, resúmenes de direcciones y protocolos, contadores de memoria y soporte parcial de IPv6.

- Se corrigio el estado obsoleto del motor y se agrego un panel de captura que reune los controles y las vistas de analisis.

- Grafico en tiempo real y velocidades de interfaz obtenidas mediante contadores de Windows; pantalla de busqueda de interfaces animada.

- Historial local, reapertura, exportacion PCAP y eliminacion con confirmacion de las sesiones guardadas.

- Seleccion automatica del adaptador correspondiente a la ruta IPv4 de salida y detencion segura si cambia durante la captura.

- La compilacion release usa el subsistema grafico de Windows y se prepara una carpeta de entrega con una guia breve.
