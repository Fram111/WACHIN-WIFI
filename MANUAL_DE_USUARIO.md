# Manual de usuario de WACHIN-WIFI

Este manual describe la versión 0.1.0 en desarrollo. Está pensado para usar la aplicación en Windows; no hace falta saber programar para seguirlo.

## 1. Qué necesitas

| Requisito | Para qué sirve | Instalación |
| --- | --- | --- |
| Windows 10 u 11 de 64 bits | Sistema donde se ejecuta la aplicación | La versión inicial está orientada a x64. |
| Microsoft Edge WebView2 Runtime | Muestra la interfaz | Suele venir instalado. Si falta, instala el **Evergreen Runtime** desde [Microsoft](https://developer.microsoft.com/microsoft-edge/webview2/). |
| Npcap | Proporciona la captura de paquetes | Instálalo por separado desde [npcap.com](https://npcap.com/). Revisa su licencia antes de instalarlo. |
| Wireshark, opcional | Abre y analiza archivos PCAP exportados | Descárgalo desde [wireshark.org](https://www.wireshark.org/download.html). No hace falta para usar WACHIN-WIFI. |

Para **ejecutar el `.exe`** no necesitas Rust, Node.js, Git ni Visual Studio. Esas herramientas son para compilar o modificar el proyecto; consulta [DEVELOPMENT.md](DEVELOPMENT.md).

## 2. Obtener e iniciar la aplicación

1. En GitHub, abre **Releases** y descarga el ejecutable o paquete de la versión que quieras usar, si hay una versión publicada. El código fuente no es por sí solo el ejecutable.
2. Si descargaste un ZIP, extrae su contenido en una carpeta de tu elección.
3. Haz doble clic en `WACHIN-WIFI.exe`. La aplicación no tiene instalador y la compilación disponible no está firmada digitalmente; Windows puede mostrar un aviso de reputación.
4. Si Npcap o WebView2 faltan, instala el componente correspondiente desde su sitio oficial y vuelve a abrir la aplicación. Npcap puede requerir permisos de administrador; sigue las indicaciones de Windows y del instalador.

Si el repositorio es privado, solo las personas autorizadas en GitHub podrán acceder a él y a sus versiones.

## 3. Antes de capturar

Usa la aplicación únicamente en interfaces y redes donde tengas autorización. Los paquetes y archivos guardados pueden contener información sensible. WACHIN-WIFI no activa el modo promiscuo: solo recibe el tráfico que el adaptador y Npcap entregan a la aplicación. Esto no significa que pueda ver toda la red ni las comunicaciones de otros equipos.

La aplicación busca automáticamente la interfaz correspondiente a la ruta IPv4 que Windows elegiría hacia `1.1.1.1`; esa consulta no envía paquetes. Si usas una VPN, rutas específicas o solo IPv6, puede no corresponder a todo tu tráfico. Si no encuentra una interfaz compatible, mostrará un error. Si la ruta cambia durante la captura, la detiene para no mezclar interfaces.

## 4. Iniciar y detener una captura

1. Abre **Panel de captura** o **Captura** y espera a que termine «escaneando wachines...».
2. Comprueba el nombre de la interfaz detectada. El selector es informativo; esta versión no permite elegir otra manualmente.
3. Pulsa **Iniciar captura**. La tabla mostrará los paquetes que lleguen.
4. Para terminar, pulsa **Detener**. La pausa no está implementada.
5. Para conservar los paquetes disponibles, pulsa **Guardar sesión** después de detener.

Si cambias de red y aparece un aviso de ruta, pulsa **Actualizar interfaz activa** y comienza una sesión nueva. Se muestran paquetes recibidos y retirados de memoria; los descartes del controlador aún no están disponibles.

## 5. Paquetes y filtros

En **Paquetes** encontrarás número, hora, origen, destino, protocolo y longitud cuando el analizador pueda identificarlos. Haz clic en una fila para consultar los bytes en hexadecimal y ASCII. Los datos pueden aparecer truncados o fragmentados; la aplicación no reconstruye fragmentos.

El primer campo busca texto en direcciones, protocolos y puertos. El segundo acepta **filtros de visualización**, que se aplican al pulsar **Aplicar filtro**:

```text
tcp
udp
icmp
icmpv6
arp
ethernet
ip.src == 192.168.1.10
ip.dst == 8.8.8.8
tcp.port == 443
udp.port == 53
```

Para quitar un filtro, vacía el segundo campo y vuelve a pulsar **Aplicar filtro**. Las direcciones en estas expresiones solo admiten IPv4. No se admiten combinaciones como `&&`, ni toda la sintaxis de Wireshark. Estos filtros no limitan los paquetes que captura Npcap ni modifican el archivo guardado.

## 6. Gráfico, dispositivos y estadísticas

**Inicio** y **Panel de captura** muestran velocidades de entrada y salida y un gráfico de los últimos 60 puntos durante una captura. Se necesitan dos lecturas para calcular la primera velocidad. Los valores provienen de los contadores de Windows para la interfaz completa, no únicamente de la tabla de paquetes. Si Windows no ofrece contadores compatibles, aparece «no disponible».

**Dispositivos** agrupa direcciones IP observadas: no es una lista completa de equipos conectados, y los nombres y MAC no están disponibles en esta vista. **Protocolos** y **Estadísticas** resumen únicamente los paquetes que permanecen en memoria. IPv6 se reconoce de forma parcial y las cabeceras de extensión no se recorren.

## 7. Guardar y volver a analizar

1. Detén la captura y pulsa **Guardar sesión**.
2. Ve a **Historial**. Cada fila muestra fecha, paquetes guardados, total capturado y cantidad retirada de memoria.
3. Pulsa **Analizar** para abrir una sesión guardada. Debes detener una captura activa antes de abrir otra sesión.

La aplicación conserva hasta **2.000 paquetes** en memoria. Si se capturaron más, los más antiguos ya retirados no se incluyen al guardar. Guardar es una acción explícita: iniciar otra captura reemplaza los paquetes que siguen en memoria. No hay limpieza automática del historial ni importación general de archivos PCAP externos.

El historial guarda metadatos y un PCAP local por sesión en la carpeta de datos de la aplicación para el usuario de Windows. No publiques capturas sin revisar su contenido.

## 8. Abrir una captura en Wireshark

1. Guarda la sesión como se explicó arriba.
2. En **Historial**, pulsa **Exportar PCAP** en la fila correspondiente.
3. Elige dónde guardar la copia `.pcap`.
4. Abre Wireshark y usa **File → Open** para seleccionar ese archivo.

El archivo contiene únicamente los paquetes que se pudieron guardar y registra tiempos con resolución de milisegundos. La compatibilidad de lectura se comprobó con Npcap/libpcap; el flujo completo desde la interfaz y Wireshark aún requiere más pruebas.

## 9. Eliminar una sesión

En **Historial**, pulsa **Eliminar** y confirma. Se borran los metadatos y el archivo interno de esa sesión. Las copias PCAP exportadas y los paquetes ya abiertos en memoria no se borran automáticamente; elimina las copias por separado si lo necesitas. La eliminación no garantiza borrado físico seguro.

## 10. Solución de problemas

| Situación | Qué comprobar |
| --- | --- |
| Npcap no disponible | Instálalo desde su sitio oficial, comprueba sus opciones y reinicia la aplicación. Algunas instalaciones limitan el acceso a administradores. |
| No se detecta la interfaz | Comprueba la conexión, la ruta IPv4 de salida y que Npcap pueda abrir el adaptador. Usa **Actualizar interfaz activa**. |
| La captura se detuvo | Revisa el aviso: la aplicación detiene la sesión si cambia la ruta activa. |
| No aparecen paquetes | Verifica que la captura esté activa y genera tráfico autorizado, por ejemplo abriendo una página. No todo el tráfico de la red llega al adaptador. |
| Velocidades o gráfico sin datos | Espera las primeras muestras; algunos adaptadores no exponen contadores compatibles. |
| Un filtro da error | Comprueba los ejemplos de la sección 5. No se admiten `&&` ni direcciones IPv6 en comparaciones. |
| Faltan paquetes en una sesión | Solo se guardan los 2.000 últimos que siguen en memoria; revisa la columna de retirados. |
| La ventana no abre | Comprueba Windows x64 y WebView2 Runtime. Si compilaste desde el código, consulta la guía de desarrollo. |

La aplicación está en desarrollo. Si informas un problema, incluye la versión de Windows y el mensaje de error, pero evita adjuntar capturas o direcciones privadas sin revisarlas.
