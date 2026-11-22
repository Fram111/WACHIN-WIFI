# Desarrollo de WACHIN-WIFI

Esta guía es para compilar y modificar el proyecto. Si solo quieres ejecutar el programa, consulta el [manual de usuario](MANUAL_DE_USUARIO.md).

## Requisitos para compilar en Windows x64

1. Instala [Node.js](https://nodejs.org/) y npm, con una versión compatible con Vite indicada por las dependencias del proyecto.
2. Instala Rust estable para `x86_64-pc-windows-msvc` mediante [rustup](https://rustup.rs/).
3. Instala [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/), con desarrollo de escritorio C++ y Windows SDK, y Microsoft Edge WebView2 Runtime. Consulta los [requisitos oficiales de Tauri](https://v2.tauri.app/start/prerequisites/).
4. Instala [Npcap](https://npcap.com/) por separado si vas a probar captura real y revisa su licencia. No se redistribuye con este proyecto. Wireshark es opcional para examinar PCAP exportados.
5. Git es útil para clonar el repositorio y registrar cambios; GitHub Desktop es opcional.

## Preparar y verificar

En una terminal ubicada en la raíz del proyecto:

```powershell
npm ci
npm test
npm run lint
npm run build
cargo check --locked --manifest-path src-tauri/Cargo.toml
cargo test --locked --manifest-path src-tauri/Cargo.toml
npm run tauri dev
```

Cargo y MSVC deben estar disponibles en el entorno de la terminal. En Windows se puede abrir una consola de desarrollador de Visual Studio o ejecutar `VsDevCmd.bat` de la instalación correspondiente antes de los comandos. Rust instalado mediante rustup suele estar en `%USERPROFILE%\.cargo\bin`.

`npm run dev` abre solamente la interfaz en el navegador: no tiene acceso al motor nativo. La aplicación de escritorio se ejecuta mediante Tauri.

## Crear el ejecutable

Desde un entorno con Cargo y MSVC configurados:

```powershell
npm run release:windows
```

El script compila con `tauri build --no-bundle` y copia el ejecutable a la raíz y a `WACHIN-WIFI - Entrega`, junto con una guía breve. El empaquetado de instalador está desactivado. El script sobrescribe esas copias generadas; no incluye Npcap.

En la computadora donde se preparó el proyecto, MSVC se encontraba en `C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\Common7\Tools\VsDevCmd.bat`; esa ruta puede ser distinta en otras computadoras.

## Verificación y límites

El proveedor Npcap se ha probado con tráfico real en el equipo de desarrollo. Las pruebas automatizadas cubren partes del analizador, filtros y PCAP, pero no sustituyen una prueba completa de la interfaz, captura, historial y exportación en Windows. La aplicación sigue en desarrollo; consulta [CAPTURE_ENGINE.md](CAPTURE_ENGINE.md).

No incluyas capturas reales, bases de historial, credenciales, `node_modules`, `target`, ejecutables generados ni el instalador de Npcap en los commits. Revisa licencias antes de distribuir.
