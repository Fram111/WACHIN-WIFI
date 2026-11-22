# WACHIN-WIFI

Analizador local de tráfico de red para Windows, desarrollado con Rust, Tauri 2, React y TypeScript. Permite observar paquetes entregados a la interfaz activa, consultar estadísticas y guardar sesiones para analizarlas en la aplicación o en Wireshark.

**Versión en desarrollo:** úsala solo en redes e interfaces donde tengas autorización. Una captura puede contener información privada.

## Quiero usar la aplicación

1. Necesitas Windows 10 u 11 de 64 bits, Microsoft Edge WebView2 Runtime y Npcap instalado por separado.
2. Descarga `WACHIN-WIFI.exe` desde una versión de este repositorio cuando haya una disponible. También puedes compilarlo siguiendo [DEVELOPMENT.md](DEVELOPMENT.md).
3. Abre el ejecutable, espera la detección de la interfaz y pulsa **Iniciar captura**.

La aplicación se distribuye como ejecutable, no como instalador. Npcap no está incluido y tiene [condiciones de licencia propias](https://npcap.com/guide/). Wireshark es opcional.

**[Manual de usuario completo](MANUAL_DE_USUARIO.md)** · [Guía de desarrollo](DEVELOPMENT.md) · [Motor de captura](CAPTURE_ENGINE.md) · [Seguridad](SECURITY.md)

## Qué funciona

- Detección automática de la interfaz usada por una ruta IPv4 de referencia, captura con Npcap y detención.
- Tabla de paquetes, búsqueda, filtros de visualización y detalle hexadecimal y ASCII.
- Gráfico y velocidades del adaptador; resúmenes de direcciones y protocolos observados.
- Guardado explícito de la ventana de paquetes en un historial local, reapertura, eliminación y exportación PCAP para Wireshark.

## Límites importantes

- Se mantienen hasta **2.000 paquetes** en memoria. Al guardar se conserva únicamente lo que siga en memoria; el historial informa los paquetes retirados.
- La interfaz se detecta mediante la ruta IPv4 que Windows elegiría hacia `1.1.1.1`, sin enviarle paquetes. Una VPN, una ruta específica o una red solo IPv6 pueden usar otra interfaz. Si la ruta cambia, la captura se detiene.
- Los gráficos usan contadores de Windows del adaptador completo y pueden diferir de los paquetes capturados. Los dispositivos son direcciones observadas, no un inventario de toda la red.
- El analizador es parcial: no reconstruye fragmentos ni recorre cabeceras de extensión IPv6. No hay filtros de captura, importación general de PCAP externo ni descifrado TLS.
- El proyecto aún no tiene una licencia de código propia elegida. Antes de redistribuir o reutilizar el código, consulta con su titular. Las licencias de terceros se detallan en [DEPENDENCIES.md](DEPENDENCIES.md).

## Para desarrolladores

La compilación requiere Node.js, Rust, las herramientas C++ de Microsoft, Windows SDK y WebView2. Npcap se instala aparte para probar capturas. Consulta [DEVELOPMENT.md](DEVELOPMENT.md) para las instrucciones y [ARCHITECTURE.md](ARCHITECTURE.md) para la organización.
