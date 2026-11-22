import type { Packet } from '../hooks/useCapture'
export type ProtocolCount = { protocol: string; packets: number; bytes: number }
export type DeviceCount = { address: string; packets: number; bytes: number; protocols: string[] }
export function summarizePackets(packets: Packet[]) {
  const protocols = new Map<string, ProtocolCount>()
  const devices = new Map<string, { address: string; packets: number; bytes: number; protocols: Set<string> }>()
  for (const packet of packets) {
    const entry = protocols.get(packet.protocol) ?? { protocol: packet.protocol, packets: 0, bytes: 0 }
    entry.packets++; entry.bytes += packet.length; protocols.set(packet.protocol, entry)
    for (const address of new Set([packet.source, packet.destination])) {
      if (!address) continue
      const device = devices.get(address) ?? { address, packets: 0, bytes: 0, protocols: new Set<string>() }
      device.packets++; device.bytes += packet.length; device.protocols.add(packet.protocol); devices.set(address, device)
    }
  }
  return {
    protocols: [...protocols.values()].sort((a, b) => b.packets - a.packets),
    devices: [...devices.values()].map(d => ({ ...d, protocols: [...d.protocols].sort() })).sort((a, b) => b.packets - a.packets),
  }
}
