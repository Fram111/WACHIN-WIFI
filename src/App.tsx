import { useState, lazy, Suspense } from 'react'
import { CircleHelp, Radio } from 'lucide-react'
import { Sidebar } from './components/layout/Sidebar'
import { sections, type Section } from './components/layout/sections'
import { Dashboard } from './features/dashboard/Dashboard'
import { CaptureView } from './features/capture/CaptureView'
import { PacketsView } from './features/packets/PacketsView'
import { AnalysisView } from './features/analysis/AnalysisView'
import { HistoryView } from './features/history/HistoryView'
import { useCapture } from './hooks/useCapture'
import { useTrafficRate, formatRate } from './hooks/useTrafficRate'
const TrafficChart = lazy(() => import('./components/charts/TrafficChart'))
import { ScanningOverlay } from './components/ui/ScanningOverlay'

const descriptions: Record<Exclude<Section, 'inicio'>, string> = {
  panel: 'Controles, paquetes y an?lisis en una sola vista.',
  captura: 'Selecciona una interfaz y controla la captura de tráfico.',
  paquetes: 'Inspecciona los paquetes observados y sus cabeceras.',
  dispositivos: 'Consulta los dispositivos observados en el tráfico.',
  protocolos: 'Explora la distribución de los protocolos detectados.',
  estadisticas: 'Analiza la actividad y las conversaciones de la sesión.',
  historial: 'Consulta y administra las sesiones guardadas.',
  configuracion: 'Administra las preferencias de la aplicación.',
}
function App() {
  const [section, setSection] = useState<Section>('panel')
  const capture = useCapture()
  const traffic = useTrafficRate(capture.selected, capture.status.active)
  const latestRate = traffic.points.at(-1)
  const label = sections.find(item => item.id === section)?.label
  return <div className="app-shell">{(capture.provider === 'checking' || capture.scanning) && <ScanningOverlay/>}<Sidebar current={section} onChange={setSection} provider={capture.provider} active={capture.status.active}/><main className="main"><header className="topbar"><span>ESPACIO DE TRABAJO <span className="separator">/</span> <b>{label}</b></span><span className="top-right"><CircleHelp size={17}/> WACHIN-WIFI <span className="version">v0.1.0</span></span></header><div className="content">{section === 'inicio' ? <Dashboard navigate={setSection} status={capture.status} traffic={traffic} packets={capture.packets} interfaceName={capture.interfaces.find(i => i.id === capture.selected)?.name ?? ""}/> : section === 'panel' ? <div className="unified-workspace"><CaptureView capture={capture}/><section className="panel"><div className="panel-heading"><div><h2>Actividad de red en tiempo real</h2><p>Velocidad total de la interfaz seleccionada</p></div><span className="status">ENTRADA {formatRate(latestRate?.incoming)} ? SALIDA {formatRate(latestRate?.outgoing)}</span></div><Suspense fallback={<div className="empty-chart">Cargando grafico...</div>}><TrafficChart points={traffic.points} active={capture.status.active} unavailable={traffic.unavailable}/></Suspense></section><PacketsView key={capture.revision} packets={capture.packets}/><AnalysisView packets={capture.packets} section="estadisticas"/><AnalysisView packets={capture.packets} section="protocolos"/><AnalysisView packets={capture.packets} section="dispositivos"/></div> : section === 'captura' ? <CaptureView capture={capture}/> : section === 'paquetes' ? <PacketsView key={capture.revision} packets={capture.packets}/> : section === 'historial' ? <HistoryView active={capture.status.active} onOpen={async id => { await capture.openSession(id); setSection('panel') }}/> : section === 'dispositivos' || section === 'protocolos' || section === 'estadisticas' ? <AnalysisView packets={capture.packets} section={section}/> : <><div className="page-heading"><div><div className="eyebrow">ESPACIO DE TRABAJO</div><h1>{label}</h1><p>{descriptions[section]}</p></div><span className="status"><span/> EN DESARROLLO</span></div><section className="panel feature-empty"><div className="empty-icon"><Radio size={22}/></div><h2>Función en desarrollo</h2><p>Esta sección estará disponible cuando se implemente y verifique su funcionalidad.</p></section></>}</div><footer className="app-footer"><span><i/> {capture.status.active ? 'CAPTURANDO' : 'SIN CAPTURA ACTIVA'}</span><span>Herramienta de análisis local · Uso autorizado</span></footer></main></div>
}
export default App




