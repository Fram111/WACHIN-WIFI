# Seguridad y privacidad

Captura únicamente en interfaces y redes para las que tengas autorización. Npcap se instala por separado y la aplicación no activa modo promiscuo ni ofrece inyección o modificación de paquetes.

Los paquetes pueden contener información privada. La vista hexadecimal y ASCII muestra los bytes recibidos. El historial solo se escribe al pulsar **Guardar sesión**, en la carpeta de datos de la aplicación del usuario actual. Los PCAP exportados son copias independientes: trátalos como información confidencial.

Eliminar una sesión borra el archivo interno y sus metadatos, pero no borra copias exportadas ni garantiza borrado físico seguro. No hay un límite automático de almacenamiento del historial; revisa y elimina sesiones que ya no necesites.

La aplicación no eleva permisos por sí sola. El acceso a Npcap depende de su instalación y de las políticas del sistema. No compartas contraseñas, tokens ni capturas sin revisarlas. Para informar un problema de seguridad, contacta al titular del repositorio sin incluir datos sensibles en una incidencia pública.
