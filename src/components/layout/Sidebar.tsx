import { Clock3, Wifi } from 'lucide-react'
import { sections, type Section } from './sections'

export function Sidebar({ current, onChange, provider, active }: { current: Section; onChange: (section: Section) => void; provider: 'checking' | 'ready' | 'unavailable' | 'browser'; active: boolean }) {
  const message = provider === 'browser' ? 'Abrir aplicacion de escritorio' : provider === 'checking' ? 'Comprobando Npcap...' : provider === 'unavailable' ? 'Interfaz no disponible' : active ? 'Captura activa' : 'Npcap disponible - sin captura'
  return <aside className="sidebar">
    <div className="brand"><span className="brand-icon"><Wifi size={21} /></span><span>WACHIN<span className="brand-accent">-WIFI</span><small>NETWORK ANALYZER</small></span></div>
    <div className="nav-label">ESPACIO DE TRABAJO</div>
    <nav aria-label="Navegación principal">{sections.map(({ id, label, icon: Icon }) => <button key={id} className={current === id ? 'nav-item active' : 'nav-item'} onClick={() => onChange(id)} aria-current={current === id ? 'page' : undefined}><Icon size={18} strokeWidth={1.8}/>{label}</button>)}</nav>
    <div className="sidebar-footer"><Clock3 size={15}/><span>Versión en desarrollo<br/><small>{message}</small></span></div>
  </aside>
}

