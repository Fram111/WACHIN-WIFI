import test from 'node:test'
import assert from 'node:assert/strict'
import { parseDisplayFilter } from '../src/lib/displayFilter.ts'
import { summarizePackets } from '../src/lib/summarizePackets.ts'
const packet = { id: 1, timestamp_ms: 0, length: 80, captured_length: 80, source: '192.168.1.10', destination: '8.8.8.8', protocol: 'UDP', source_port: 12345, destination_port: 53, truncated: false, fragmented: false }
test('filtros válidos distinguen protocolo, dirección y puertos', () => {
 assert.equal(parseDisplayFilter('udp')(packet), true)
 assert.equal(parseDisplayFilter('tcp')(packet), false)
 assert.equal(parseDisplayFilter('ip.src == 192.168.1.10')(packet), true)
 assert.equal(parseDisplayFilter('ip.dst == 192.168.1.10')(packet), false)
 assert.equal(parseDisplayFilter('udp.port == 53')(packet), true)
 assert.equal(parseDisplayFilter('tcp.port == 53')(packet), false)
})
test('filtros inválidos no se aceptan', () => {
 for (const input of ['dns', 'tcp.port == 65536', 'ip.src == 999.1.1.1', 'udp.port == -1', 'tcp && udp']) assert.throws(() => parseDisplayFilter(input))
})
test('estadísticas cuentan una dirección solo una vez por paquete', () => {
 const result = summarizePackets([packet, { ...packet, id: 2, source: '8.8.8.8', destination: '8.8.8.8' }])
 assert.deepEqual(result.protocols, [{ protocol: 'UDP', packets: 2, bytes: 160 }])
 assert.equal(result.devices.find(d => d.address === '8.8.8.8')?.packets, 2)
 assert.equal(result.devices.find(d => d.address === '192.168.1.10')?.bytes, 80)
})
