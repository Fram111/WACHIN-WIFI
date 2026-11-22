import { useEffect, useState } from 'react'
import { invoke, isTauri } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export type Interface = { id: string; name: string }
export type Packet = { id: number; timestamp_ms: number; length: number; captured_length: number; source: string | null; destination: string | null; protocol: string; source_port: number | null; destination_port: number | null; truncated: boolean; fragmented: boolean }
export type Status = { active: boolean; captured: number; bytes: number; evicted: number; error: string | null }
type Batch = { packets: Packet[]; status: Status }
export type Detail = { packet: Packet; hex: string; ascii: string }
const empty: Status = { active: false, captured: 0, bytes: 0, evicted: 0, error: null }
export function useCapture() {
  const [interfaces, setInterfaces] = useState<Interface[]>([])
  const [selected, setSelected] = useState('')
  const [status, setStatus] = useState<Status>(empty)
  const [packets, setPackets] = useState<Packet[]>([])
  const [error, setError] = useState<string | null>(null)
  const [busy, setBusy] = useState(false)
  const [scanning, setScanning] = useState(false)
  const [sessionMessage, setSessionMessage] = useState('')
  const [routeError, setRouteError] = useState<string | null>(null)
  const [revision, setRevision] = useState(0)
  const [provider, setProvider] = useState<'checking' | 'ready' | 'unavailable' | 'browser'>(isTauri() ? 'checking' : 'browser')
  useEffect(() => {
    if (!isTauri()) return
    let disposed = false
    let unlisten: (() => void) | undefined
    listen<Batch>('capture-batch', event => { setStatus(event.payload.status); if (event.payload.packets.length) setPackets(current => [...current, ...event.payload.packets].slice(-2000)) }).then(fn => { if (disposed) fn(); else unlisten = fn })
    invoke<Status>('capture_status').then(setStatus).catch(e => setError(String(e)))
    return () => { disposed = true; unlisten?.() }
  }, [])
  useEffect(() => {
    if (!isTauri() || status.active) return
    let disposed = false
    let timer: ReturnType<typeof setTimeout> | undefined
    async function detect() {
      try {
        const found = await invoke<Interface>('active_interface')
        if (!disposed) { setInterfaces([found]); setSelected(found.id); setProvider('ready'); setRouteError(null) }
      } catch (e) {
        if (!disposed) { setInterfaces([]); setSelected(''); setProvider('unavailable'); setRouteError(String(e)) }
      }
      if (!disposed) timer = setTimeout(detect, 5000)
    }
    void detect()
    return () => { disposed = true; if (timer) clearTimeout(timer) }
  }, [status.active])
  async function refresh() { if (!isTauri()) { setError('Abre la aplicacion de escritorio para acceder a las interfaces.'); return } setBusy(true); setScanning(true); try { const found = await invoke<Interface>('active_interface'); setInterfaces([found]); setSelected(found.id); setError(null); setRouteError(null); setProvider('ready') } catch (e) { setRouteError(String(e)); setInterfaces([]); setSelected(''); setProvider('unavailable') } finally { setBusy(false); setScanning(false) } }
  async function start() { if (!selected || busy) return; setBusy(true); setError(null); try { setPackets([]); setStatus(empty); setSessionMessage(''); setRevision(n => n + 1); await invoke('start_capture', { interface: selected }); setStatus(await invoke<Status>('capture_status')) } catch (e) { setError(String(e)) } finally { setBusy(false) } }
  async function stop() { setBusy(true); try { await invoke('stop_capture') } catch (e) { setError(String(e)) } finally { setBusy(false) } }
  async function save() { setBusy(true); setError(null); setSessionMessage(''); try { await invoke('save_session'); setSessionMessage('Sesion guardada. Puedes abrirla o exportarla desde Historial.') } catch (e) { setError(String(e)) } finally { setBusy(false) } }
  async function openSession(id: number) { const result = await invoke<Batch>('open_session', { id }); setPackets(result.packets); setStatus(result.status); setRevision(n => n + 1); setError(null); setSessionMessage('Sesion guardada abierta para analisis.') }
  return { interfaces, selected, status, packets, error: error || routeError, busy, refresh, start, stop, provider, scanning, save, openSession, sessionMessage, revision }
}

