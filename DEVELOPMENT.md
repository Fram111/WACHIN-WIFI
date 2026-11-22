# Desarrollo

Requisitos Windows: Node.js compatible con Vite, Rust estable para x86_64-pc-windows-msvc, Microsoft C++ Build Tools con Windows SDK y WebView2. Véase https://v2.tauri.app/start/prerequisites/ . La instalación de herramientas puede requerir privilegios; no se ha realizado.

Ejecutar `npm ci`, `npm run build`, `cargo check --manifest-path src-tauri/Cargo.toml`, `cargo test --manifest-path src-tauri/Cargo.toml` y `npm run tauri dev`. Para distribución, configurar iconos y empaquetado y ejecutar `npm run tauri build` una vez verificada la base. El empaquetado está desactivado por ahora.

Inspección inicial: Windows 11 Home de 64 bits, Node 24.20.0, npm 11.19.0, WebView2 158.0.4370.33. Rust, Cargo y Git no estaban en PATH; vswhere no encontró instalaciones de Visual Studio. Falta comprobar la compilación nativa.

Actualización: Rust 1.100.0 se instaló en el perfil de usuario sin modificar PATH. Para usarlo en una terminal PowerShell: `$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"`. Se encontraron `cl.exe` y Windows SDK dentro de una instancia de Build Tools marcada como incompleta; la compilación se prueba desde VsDevCmd. Las tres pruebas aisladas del analizador pasaron con `rustc --test`; la verificación completa de Tauri sigue en curso.

Verificación posterior: `cargo check --manifest-path src-tauri/Cargo.toml` pasó al invocarlo desde VsDevCmd con Rust 1.100.0. La ejecución de la aplicación completa y la distribución siguen pendientes.

Para captura se requiere Npcap instalado separadamente; consulta su licencia en https://npcap.com/guide/ . Inicia con `npm run tauri dev` en un entorno con Cargo y MSVC. Dentro de la aplicación usa Captura > Buscar interfaces. Si Npcap falta, la aplicación muestra el error; el proveedor se probó en este equipo con tráfico real; la interfaz completa aún requiere pruebas de extremo a extremo.



Pruebas de frontend sin dependencias adicionales: `npm test`. Incluyen validación de filtros y agregación de paquetes.

## Generar entrega para Windows

Desde PowerShell en la carpeta del proyecto:

```powershell
cmd /c 'set "PATH=%USERPROFILE%\.cargo\bin;%PATH%" && call "C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\Common7\Tools\VsDevCmd.bat" -arch=x64 -host_arch=x64 >nul && npm run release:windows'
```

El script compila Tauri en modo release y, si tiene exito, copia el ejecutable a la carpeta principal y a `WACHIN-WIFI - Entrega`, junto con una guia breve para usuarios. El comando anterior refleja la ruta de MSVC de este equipo; en otro equipo se debe usar su propio entorno de compilacion MSVC. No genera un instalador ni incluye Npcap.
