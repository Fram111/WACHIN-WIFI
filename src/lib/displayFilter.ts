import type { Packet } from '../hooks/useCapture'

export type DisplayFilter = (packet: Packet) => boolean
export function parseDisplayFilter(input: string): DisplayFilter {
  const text = input.trim().toLowerCase()
  if (!text) return () => true
  if (['tcp', 'udp', 'icmp', 'icmpv6', 'arp', 'ethernet'].includes(text)) return packet => packet.protocol.toLowerCase() === text
  const match = /^(ip\.src|ip\.dst|tcp\.port|udp\.port)\s*==\s*(\S+)$/.exec(text)
  if (!match) throw new Error('Filtro no válido. Ejemplos: tcp, udp, ip.src == 192.168.1.10, tcp.port == 443.')
  const [, field, value] = match
  if (field === 'ip.src' || field === 'ip.dst') {
    const parts = value.split('.')
    if (parts.length !== 4 || parts.some(part => !/^\d{1,3}$/.test(part) || Number(part) > 255 || (part.length > 1 && part.startsWith('0')))) throw new Error('Introduce una dirección IPv4 válida.')
    return packet => (field === 'ip.src' ? packet.source : packet.destination) === value
  }
  if (!/^\d+$/.test(value) || Number(value) > 65535) throw new Error('El puerto debe estar entre 0 y 65535.')
  const protocol = field === 'tcp.port' ? 'TCP' : 'UDP'
  const port = Number(value)
  return packet => packet.protocol === protocol && (packet.source_port === port || packet.destination_port === port)
}


