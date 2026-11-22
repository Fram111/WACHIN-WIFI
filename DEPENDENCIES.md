# Dependencias

Dependencias iniciales: React y React DOM (MIT), Vite y plugin React (MIT), TypeScript (Apache-2.0), Tailwind CSS y plugin Vite (MIT), API/CLI Tauri y crates Tauri (MIT o Apache-2.0). Se usan para interfaz, tipado, estilos y contenedor de escritorio. Oxlint pertenece a la plantilla de Vite (MIT); paquetes de tipos incluyen sus respectivas licencias en node_modules. La lista transitiva y sus licencias deben auditarse con los archivos lock antes de distribuir. Las versiones exactas npm están en package-lock.json; Cargo.lock todavía no existe porque falta Cargo.

Para fases posteriores se evaluarán Serde, SQLite, Recharts y una biblioteca de captura/análisis tras comprobar licencias y necesidades. No se incluyen aún. Npcap tiene condiciones separadas; ver CAPTURE_ENGINE.md. La licencia del código propio está pendiente de elección por el titular.

La interfaz inicial usa lucide-react (ISC) para iconos. Rust se instaló en el perfil de usuario durante el desarrollo; aún se requiere verificar el compilador nativo y las dependencias transitivas de Cargo.

El proveedor de Windows usa libloading 0.8 (ISC) para cargar Npcap del sistema. Npcap es independiente, tiene restricciones específicas y no se distribuye con la aplicación; ver CAPTURE_ENGINE.md. Serde (MIT OR Apache-2.0) serializa comandos y eventos.

Recharts se utiliza para los graficos del frontend (licencia MIT). windows-sys (MIT OR Apache-2.0) permite consultar los contadores de interfaz de Windows sin ejecutar procesos externos.

Historial: rusqlite (MIT) con SQLite incluido (dominio publico) y tauri-plugin-dialog (MIT OR Apache-2.0) para seleccionar la ruta de exportacion.
