# WACHIN-WIFI

Aplicación local de análisis de red para Windows. La interfaz ofrece selección de interfaz Npcap, inicio y detención, tabla, búsqueda simple y vista hexadecimal/ASCII. El proveedor Npcap fue probado en este equipo con tráfico real generado durante una prueba breve. Utilizar únicamente en redes e interfaces autorizadas.

Npcap se instala por separado desde https://npcap.com/ y requiere revisar sus condiciones de licencia. La aplicación no lo incluye ni lo instala. Luego inicia la aplicación Tauri, abre Captura y pulsa Buscar interfaces. La versión en navegador no puede acceder al motor nativo. Consulta [DEVELOPMENT.md](DEVELOPMENT.md) y [CAPTURE_ENGINE.md](CAPTURE_ENGINE.md).

Solo se conservan los últimos 2.000 paquetes en memoria. Los filtros de captura y varias pantallas siguen pendientes. La captura puede incluir contenido sensible.


## Análisis en memoria

La pantalla Paquetes admite búsqueda libre y filtros de visualización `tcp`, `udp`, `icmp`, `icmpv6`, `arp`, `ethernet`, `ip.src == IPv4`, `ip.dst == IPv4`, `tcp.port == número` y `udp.port == número`. No son filtros de captura ni implementan toda la sintaxis de Wireshark. Dispositivos, Protocolos y Estadísticas agrupan únicamente los últimos 2.000 paquetes en memoria; los dispositivos son direcciones IP observadas y no un inventario completo de red. El analizador reconoce IPv6 base y transporte inmediato, sin recorrer cabeceras de extensión ni reconstruir fragmentos. Historial, almacenamiento y exportación siguen pendientes.

## Velocidad de red

Durante la captura, Inicio y Panel de captura muestran la velocidad de entrada y salida de la interfaz seleccionada y un grafico de los ultimos 60 puntos. Se calcula a partir de los contadores de bytes de Windows y el tiempo transcurrido entre muestras; representa el trafico total del adaptador y puede diferir del trafico entregado a Npcap. Para interfaces sin contadores compatibles se muestra que no estan disponibles. La animacion de busqueda aparece mientras se enumeran interfaces.


## Guardar, analizar y exportar

Deten la captura y pulsa **Guardar sesion** en Captura o Panel de captura. La seccion **Historial** permite abrir la sesion en la aplicacion, exportar un archivo PCAP para abrirlo en Wireshark y eliminar la sesion guardada mediante confirmacion. El guardado es explicito: se almacenan como maximo los ultimos 2.000 paquetes; los anteriores no se recuperan. El historial indica las cantidades capturada, guardada y retirada. Los tiempos del archivo tienen resolucion de milisegundos. No hay importacion general de archivos PCAP externos ni limite automatico del historial.

Los archivos PCAP contienen datos potencialmente sensibles. Se guardan junto con una base de metadatos en la carpeta de datos de la aplicacion. Al eliminar una sesion se eliminan su archivo interno y sus metadatos; las copias exportadas y los paquetes aun visibles en memoria no se eliminan automaticamente.

## Seleccion automatica de interfaz

La aplicacion consulta la ruta IPv4 que Windows elegiria para el destino de referencia 1.1.1.1, sin enviarle paquetes; busca su adaptador activo en Npcap. No usa el primer elemento enumerado. Antes de capturar valida de nuevo la ruta y, si cambia durante la sesion, detiene la captura para evitar mezclar adaptadores. Las velocidades y el grafico usan la misma interfaz. Si no hay ruta IPv4 o Npcap no expone ese adaptador, se muestra un error. Una ruta especifica para otro destino, una VPN o una red solo IPv6 pueden usar otra interfaz; no se afirma que este metodo abarque todo el trafico del equipo.
