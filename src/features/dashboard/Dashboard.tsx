import { lazy, Suspense } from 'react'
import { Activity, ArrowDownToLine, ArrowUpFromLine, Cable, CircleDot, Radio, ShieldCheck } from 'lucide-react'
import type { Section } from '../../components/layout/sections'
import type { Packet, Status } from '../../hooks/useCapture'
import { summarizePackets } from '../../lib/summarizePackets'
const TrafficChart = lazy(() => import('../../components/charts/TrafficChart'))
import { formatRate, type TrafficPoint } from '../../hooks/useTrafficRate'

export function Dashboard({ navigate, status, packets, interfaceName, traffic }: { navigate: (section: Section) => void; status: Status; packets: Packet[]; interfaceName: string; traffic: { points: TrafficPoint[]; unavailable: boolean } }) {
  const protocols = summarizePackets(packets).protocols
  const last = traffic.points.at(-1)
  const tiles = [
    { label: 'PAQUETES CAPTURADOS', value: status.captured.toLocaleString(), detail: status.active ? 'Captura activa' : 'Captura detenida', icon: CircleDot },
    { label: 'VELOCIDAD DE ENTRADA', value: formatRate(last?.incoming), detail: traffic.unavailable ? 'No disponible para esta interfaz' : 'Trafico del adaptador', icon: ArrowDownToLine },
    { label: 'VELOCIDAD DE SALIDA', value: formatRate(last?.outgoing), detail: traffic.unavailable ? 'No disponible para esta interfaz' : 'Trafico del adaptador', icon: ArrowUpFromLine },
    { label: 'INTERFAZ SELECCIONADA', value: interfaceName || 'Ninguna', detail: 'Selección actual', icon: Cable },
  ]
  return <>
    <div className="page-heading"><div><div className="eyebrow">RESUMEN GENERAL</div><h1>Tu red, en perspectiva.</h1><p>Observa y comprende el tráfico de tus interfaces de red.</p></div><span className="status"><span /> {status.active ? 'CAPTURANDO' : 'SIN CAPTURA ACTIVA'}</span></div>
    <div className="metrics">{tiles.map(({ label, value, detail, icon: Icon }) => <article className="metric" key={label}><div className="metric-top"><span>{label}</span><Icon size={17}/></div><strong className="metric-value">{value}</strong><p>{detail}</p></article>)}</div>
    <div className="dashboard-grid"><section className="panel activity"><div className="panel-heading"><div><h2>Actividad de red</h2><p>Contadores de la sesión</p></div><Activity size={18}/></div><Suspense fallback={<div className="empty-chart">Cargando grafico...</div>}><TrafficChart points={traffic.points} active={status.active} unavailable={traffic.unavailable}/></Suspense><div className="chart-footer"><span>ENTRADA <b>{formatRate(last?.incoming)}</b></span><span>SALIDA <b>{formatRate(last?.outgoing)}</b></span><span>PAQUETES <b>{status.captured.toLocaleString()}</b></span></div></section>
    <section className="panel protocols"><div className="panel-heading"><div><h2>Protocolos principales</h2><p>Últimos {packets.length.toLocaleString()} paquetes en memoria</p></div></div><div className="protocol-empty">{protocols.length ? protocols.slice(0, 5).map(p => <div className="stat-row protocol-row" key={p.protocol}><span>{p.protocol}</span><strong>{p.packets.toLocaleString()}</strong></div>) : <><div className="ring"><span>—<small>PAQUETES</small></span></div><p>Los protocolos aparecerán aquí<br/>al analizar una captura.</p></>}</div></section></div>
    <div className="bottom-grid"><section className="panel bottom-panel"><div className="panel-heading"><div><h2>Actividad reciente</h2><p>Último paquete conservado</p></div></div><div className="inline-empty"><Radio size={19}/><div><strong>{packets.length ? `${packets[packets.length - 1].protocol} · ${packets[packets.length - 1].length} bytes` : 'Aún no hay actividad'}</strong><p>{packets.length ? new Date(packets[packets.length - 1].timestamp_ms).toLocaleTimeString() : 'Cuando inicies una captura, aparecerá aquí.'}</p></div></div></section><section className="panel bottom-panel"><div className="panel-heading"><div><h2>Captura segura</h2><p>Antes de comenzar</p></div><ShieldCheck size={19}/></div><p className="notice">Analiza únicamente redes e interfaces para las que tengas autorización.</p><button className="text-button" onClick={() => navigate('captura')}>Ir a captura <span>→</span></button></section></div>
  </>
}
