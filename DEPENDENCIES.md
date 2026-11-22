# Dependencias y licencias

Las versiones exactas de paquetes npm están en `package-lock.json` y las de Rust en `src-tauri/Cargo.lock`. Esta tabla resume dependencias directas; antes de distribuir se requiere una auditoría de las licencias transitivas y de los términos aplicables.

| Dependencia | Uso | Licencia declarada |
| --- | --- | --- |
| React, React DOM | Interfaz | MIT |
| Vite, plugin React | Compilación de interfaz | MIT |
| TypeScript | Tipado | Apache-2.0 |
| Tailwind CSS y plugin Vite | Estilos | MIT |
| Recharts | Gráficos | MIT |
| lucide-react | Iconos | ISC |
| Tauri, API y CLI | Aplicación de escritorio | MIT o Apache-2.0 |
| tauri-plugin-dialog | Diálogo para exportar | MIT o Apache-2.0 |
| Serde | Serialización | MIT o Apache-2.0 |
| rusqlite | Metadatos de historial | MIT; SQLite incluido es de dominio público |
| libloading | Carga de biblioteca Npcap | ISC |
| windows-sys | Contadores e interfaces de Windows | MIT o Apache-2.0 |
| Oxlint | Análisis estático | MIT |

Npcap se instala por separado y tiene [condiciones propias](https://npcap.com/guide/). La edición gratuita no autoriza redistribuir su instalador; no se incluye en este repositorio ni en la entrega. WebView2 y las herramientas de Microsoft tienen sus propios términos.

El titular aún no ha elegido una licencia para el código de WACHIN-WIFI. La presencia de licencias permisivas en dependencias no otorga por sí sola una licencia de distribución del código del proyecto.
