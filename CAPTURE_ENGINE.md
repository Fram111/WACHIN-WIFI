# Motor de captura: evaluación inicial

- PktMon integrado: captura y filtra en distintos puntos de la pila, admite Ethernet y Wi-Fi y conversión ETL a PCAPNG. La captura por defecto puede truncarse; observar múltiples puntos puede producir varias observaciones de un paquete. La CLI tiene modo en tiempo real y la API documenta streaming y múltiples sesiones, pero la compatibilidad exacta de API en Windows 10/11 debe comprobarse. La API requiere administrador. No confundir lectura periódica de archivos con streaming. https://learn.microsoft.com/en-us/windows/win32/pktmon/pktmon y https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon-syntax
- ETW: mecanismo de eventos y seguimiento que puede transportar eventos de proveedores; su disponibilidad, contenido, pérdida y formato dependen del proveedor y configuración. No equivale automáticamente a PCAP completo.
- WFP: infraestructura de filtrado y eventos; inspección profunda mediante callouts puede requerir un controlador. No se desarrollará uno propio. https://learn.microsoft.com/en-us/windows-hardware/drivers/network/introduction-to-windows-filtering-platform-callout-drivers
- Npcap: alternativa potencial para captura continua mediante interfaz compatible con libpcap. La edición estándar tiene licencia y restricciones propias; OEM ofrece licencias de redistribución. No se incluye ni redistribuye y debe revisarse el contrato de uso concreto antes de su integración o distribución. El modo Wi-Fi crudo depende de adaptador, controlador y configuración. https://npcap.com/oem/redist y https://npcap.com/guide/

Decisión preliminar: investigar PktMon integrado como primera prueba; no se afirma que cumpla todos los requisitos de captura continua o Windows 10. Comparar con Npcap instalado por el usuario después de validar licencia, soporte y rendimiento. Posteriormente se implementó el proveedor Npcap opcional descrito abajo; PktMon sigue sin implementar.

## Implementación en desarrollo: Npcap opcional

La aplicación carga `wpcap.dll` desde el directorio de Npcap del sistema; no instala ni redistribuye el controlador. Es necesario instalar Npcap por separado y aceptar sus condiciones. La edición gratuita tiene restricciones de uso y no permite redistribución: https://npcap.com/guide/ . Con Npcap 1.90 instalado en este equipo se enumeraron siete interfaces, se abrieron cinco interfaces Ethernet y se observaron 41 paquetes durante una prueba breve con tráfico DNS generado. Esta prueba verifica el proveedor, pero no todos los flujos de la interfaz.

La selección de interfaz se valida contra las interfaces enumeradas por Npcap. Se admiten interfaces que entreguen Ethernet (DLT_EN10MB), sin modo promiscuo; el adaptador, controlador y sistema determinan el tráfico visible. El lector es no bloqueante y trabaja en un hilo; emite lotes y conserva hasta 2.000 paquetes en memoria, eliminando los más antiguos. No hay exportación, filtros BPF, pausa, descartes del controlador ni estadísticas de dirección. Los datos pueden contener información sensible y permanecen en memoria hasta iniciar otra sesión o cerrar la aplicación. No se garantiza captura de tramas Wi-Fi 802.11 crudas.



El analizador reconoce direcciones IPv6 y transporte inmediato cuando no hay cabeceras de extensión. No recorre extensiones ni reconstruye fragmentos; un encabezado de fragmentación inmediato se indica como fragmentado. Las estadísticas y filtros de visualización operan sobre la ventana limitada del frontend y no sobre toda la sesión.

El guardado explicito conserva hasta 2.000 paquetes en PCAP Ethernet con resolucion de milisegundos. El archivo omite paquetes ya retirados de memoria. Es posible reabrir capturas propias y exportarlas; no hay importacion general.

La seleccion automatica consulta la ruta IPv4 a 1.1.1.1 sin enviar trafico; el adaptador debe estar activo y ser accesible mediante Npcap. La captura se detiene si la ruta cambia. En este equipo se comparo el indice elegido con Find-NetRoute y se abrio la misma interfaz en Npcap.
