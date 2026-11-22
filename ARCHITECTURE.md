# Arquitectura propuesta

React/TypeScript renderiza vistas y consume comandos y eventos Tauri. Rust concentra estado, captura, análisis, filtros y persistencia; no habrá servidor HTTP de aplicación. El motor expondrá un trait CaptureProvider con capacidades explícitas y errores tipados. Los proveedores alimentarán un canal acotado; el procesamiento y los lotes al frontend tendrán límites y contadores diferenciados. El analizador validará límites y reconocerá truncamiento y fragmentación. SQLite guardará configuraciones y metadatos; PCAP/PCAPNG guardará capturas.

Estas son decisiones de diseño, no funcionalidades implementadas. La API nativa y sus garantías de permisos, fiabilidad y retrocompatibilidad se validarán antes de elegir definitivamente el proveedor. La interfaz, filtros, analizador, almacenamiento y eventos llegarán en fases posteriores.

Plan: 1) base y compilación; 2) interfaz y estados vacíos; 3) contratos Rust; 4) captura real con pruebas autorizadas; 5) analizador; 6) integración; 7) persistencia; 8) pruebas y licencias; 9) distribución. Cada etapa requiere verificaciones antes de continuar.

## Estado de implementación

Existe un contrato preliminar `CaptureProvider` y un analizador parcial de Ethernet/VLAN e IPv4; hay un proveedor Npcap e integración inicial; el proveedor se verificó con tráfico real en este equipo. No hay persistencia. IPv6 base se analiza sin recorrer cabeceras de extensión. No se reconstruyen fragmentos. Los puertos se extraen únicamente cuando hay suficientes bytes y no es un fragmento posterior.

## Captura Npcap en desarrollo

Se implementaron comandos de enumeración, inicio, detención, detalles y eventos agrupados con un proveedor Npcap opcional. No sustituye todavía al contrato preliminar CaptureProvider. El lector captura en un hilo no bloqueante; conserva un máximo de 2.000 tramas en memoria. El proveedor se verificó con Npcap instalado y tráfico autorizado; aún faltan pruebas de extremo a extremo de la interfaz, estadísticas del proveedor, persistencia y filtros avanzados.




El historial utiliza SQLite para metadatos y archivos PCAP separados para paquetes; la exportacion se realiza mediante un dialogo nativo. Solo se reabren archivos creados por el historial de la aplicacion.
