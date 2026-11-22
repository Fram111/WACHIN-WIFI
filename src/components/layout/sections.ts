import { Activity, Boxes, ChartNoAxesCombined, FileClock, House, Network, Radio, Settings2, LayoutDashboard } from 'lucide-react'
export const sections = [
  { id: 'inicio', label: 'Inicio', icon: House },
  { id: 'panel', label: 'Panel de captura', icon: LayoutDashboard },
  { id: 'captura', label: 'Captura', icon: Radio },
  { id: 'paquetes', label: 'Paquetes', icon: Boxes },
  { id: 'dispositivos', label: 'Dispositivos', icon: Network },
  { id: 'protocolos', label: 'Protocolos', icon: Activity },
  { id: 'estadisticas', label: 'Estadísticas', icon: ChartNoAxesCombined },
  { id: 'historial', label: 'Historial', icon: FileClock },
  { id: 'configuracion', label: 'Configuración', icon: Settings2, LayoutDashboard },
] as const
export type Section = typeof sections[number]['id']


