import { useEffect, useState } from 'react'
import { invoke, isTauri } from '@tauri-apps/api/core'

type Counters = { received: number; sent: number }
export type TrafficPoint = { time: string; timestamp: number; incoming: number; outgoing: number }
export function useTrafficRate(interfaceId: string, active: boolean) {
  const [points, setPoints] = useState<TrafficPoint[]>([])
  const [unavailable, setUnavailable] = useState(false)
  useEffect(() => {
    if (!active || !interfaceId || !isTauri()) return
    let cancelled = false
    let timer: ReturnType<typeof setTimeout> | undefined
    let previous: { counters: Counters; at: number } | undefined
    async function sample() {
      try {
        const counters = await invoke<Counters>('interface_counters', { interface: interfaceId })
        if (cancelled) return
        const at = performance.now()
        if (!previous) setPoints([])
        if (previous) {
          const seconds = (at - previous.at) / 1000
          if (seconds > 0 && counters.received >= previous.counters.received && counters.sent >= previous.counters.sent) {
            const now = Date.now()
            const point = { time: new Date(now).toLocaleTimeString(), timestamp: now, incoming: (counters.received - previous.counters.received) / seconds, outgoing: (counters.sent - previous.counters.sent) / seconds }
            setPoints(current => [...current, point].slice(-60))
          } else setPoints([])
        }
        previous = { counters, at }; setUnavailable(false)
      } catch { if (cancelled) return; previous = undefined; setPoints([]); setUnavailable(true) }
      if (!cancelled) timer = setTimeout(sample, 1000)
    }
    void sample()
    return () => { cancelled = true; if (timer) clearTimeout(timer) }
  }, [interfaceId, active])
  return { points: active ? points : [], unavailable: active && unavailable }
}
export function formatRate(value: number | undefined) {
  if (value === undefined || !Number.isFinite(value)) return '—'
  if (value >= 1024 * 1024) return `${(value / (1024 * 1024)).toFixed(2)} MB/s`
  if (value >= 1024) return `${(value / 1024).toFixed(1)} KB/s`
  return `${Math.round(value)} B/s`
}
