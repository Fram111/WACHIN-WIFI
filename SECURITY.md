# Seguridad

Capturar solo con autorización. El tráfico puede contener datos sensibles; tratar capturas como información confidencial. No hay captura, elevación, comandos externos ni instalación de controladores implementados. La aplicación no incorporará ataques, inyección ni interceptación activa. Antes de cualquier captura o instalación se deberán aclarar permisos y almacenamiento.

## Captura y datos

El proveedor Npcap no activa modo promiscuo, pero los paquetes entregados pueden incluir contenido sensible. La vista hexadecimal y ASCII muestra los bytes capturados; evita compartirlos sin revisarlos. Los datos se mantienen en memoria, no se guardan automáticamente en archivos. El usuario debe instalar Npcap separadamente conforme a su licencia. No hay elevación automática.

Guardar una sesion escribe paquetes sensibles en la carpeta de datos del usuario. Eliminar la sesion borra el archivo interno y sus metadatos; no elimina copias exportadas ni garantiza borrado fisico seguro.
