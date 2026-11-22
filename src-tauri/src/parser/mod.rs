//! Lectura segura de tramas Ethernet sin reconstrucciÃ³n de fragmentos.
#[derive(Debug, PartialEq, Eq, serde::Serialize)]
pub struct PacketSummary {
    pub source: Option<String>,
    pub destination: Option<String>,
    pub protocol: &'static str,
    pub source_port: Option<u16>,
    pub destination_port: Option<u16>,
    pub truncated: bool,
    pub fragmented: bool,
}

pub fn parse_ethernet(data: &[u8]) -> PacketSummary {
    let mut result = PacketSummary { source: None, destination: None, protocol: "Ethernet", source_port: None, destination_port: None, truncated: false, fragmented: false };
    if data.len() < 14 { result.truncated = true; return result; }
    let mut kind = u16::from_be_bytes([data[12], data[13]]);
    let mut offset = 14;
    while kind == 0x8100 || kind == 0x88a8 {
        if data.len() < offset + 4 { result.truncated = true; return result; }
        kind = u16::from_be_bytes([data[offset + 2], data[offset + 3]]);
        offset += 4;
    }
    match kind {
        0x0806 => result.protocol = "ARP",
        0x0800 => {
            result.protocol = "IPv4";
            let Some(ip) = data.get(offset..) else { result.truncated = true; return result; };
            if ip.len() < 20 { result.truncated = true; return result; }
            let header_len = usize::from(ip[0] & 15) * 4;
            if ip[0] >> 4 != 4 || header_len < 20 { return result; }
            if ip.len() < header_len { result.truncated = true; return result; }
            let total_len = usize::from(u16::from_be_bytes([ip[2], ip[3]]));
            if total_len < header_len { return result; }
            result.truncated = ip.len() < total_len;
            result.source = Some(format!("{}.{}.{}.{}", ip[12], ip[13], ip[14], ip[15]));
            result.destination = Some(format!("{}.{}.{}.{}", ip[16], ip[17], ip[18], ip[19]));
            let fragment = u16::from_be_bytes([ip[6], ip[7]]);
            result.fragmented = fragment & 0x3fff != 0;
            result.protocol = match ip[9] { 6 => "TCP", 17 => "UDP", 1 => "ICMP", _ => "IPv4" };
            if fragment & 0x1fff == 0 && (ip[9] == 6 || ip[9] == 17) {
                let end = total_len.min(ip.len());
                if end >= header_len + 4 {
                    result.source_port = Some(u16::from_be_bytes([ip[header_len], ip[header_len + 1]]));
                    result.destination_port = Some(u16::from_be_bytes([ip[header_len + 2], ip[header_len + 3]]));
                } else { result.truncated = true; }
            }
        },
        0x86dd => {
            result.protocol = "IPv6";
            let Some(ip) = data.get(offset..) else { result.truncated = true; return result; };
            if ip.len() < 40 { result.truncated = true; return result; }
            if ip[0] >> 4 != 6 { return result; }
            let payload_len = usize::from(u16::from_be_bytes([ip[4], ip[5]]));
            let total = 40 + payload_len;
            result.truncated = ip.len() < total;
            let mut source = [0u8; 16]; let mut destination = [0u8; 16];
            source.copy_from_slice(&ip[8..24]); destination.copy_from_slice(&ip[24..40]);
            result.source = Some(std::net::Ipv6Addr::from(source).to_string());
            result.destination = Some(std::net::Ipv6Addr::from(destination).to_string());
            result.fragmented = ip[6] == 44;
            // Extension headers are not walked; avoid interpreting them as transport headers.
            result.protocol = match ip[6] { 6 => "TCP", 17 => "UDP", 58 => "ICMPv6", _ => "IPv6" };
            if ip[6] == 6 || ip[6] == 17 {
                if total.min(ip.len()) >= 44 {
                    result.source_port = Some(u16::from_be_bytes([ip[40], ip[41]]));
                    result.destination_port = Some(u16::from_be_bytes([ip[42], ip[43]]));
                } else { result.truncated = true; }
            }
        },
        _ => (),
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn short_frames_never_panic() { for n in 0..14 { assert!(parse_ethernet(&vec![0; n]).truncated); } }
    #[test] fn parses_ipv4_udp_ports() {
        let mut p = vec![0; 38]; p[12..14].copy_from_slice(&[8,0]); p[14] = 0x45; p[16..18].copy_from_slice(&24u16.to_be_bytes()); p[23] = 17; p[26..30].copy_from_slice(&[192,168,1,1]); p[30..34].copy_from_slice(&[8,8,8,8]); p[34..38].copy_from_slice(&[0x30,0x39,0,53]);
        let r = parse_ethernet(&p); assert_eq!(r.protocol, "UDP"); assert_eq!(r.destination_port, Some(53)); assert!(!r.truncated);
    }
    #[test] fn non_initial_fragment_has_no_ports() {
        let mut p = vec![0; 38]; p[12..14].copy_from_slice(&[8,0]); p[14]=0x45; p[16..18].copy_from_slice(&24u16.to_be_bytes()); p[20..22].copy_from_slice(&1u16.to_be_bytes()); p[23]=6;
        let r = parse_ethernet(&p); assert!(r.fragmented); assert_eq!(r.source_port, None);
    }
    #[test] fn parses_ipv6_udp_and_rejects_truncated_payload() {
        let mut p = vec![0; 58]; p[12..14].copy_from_slice(&[0x86,0xdd]); p[14] = 0x60; p[18..20].copy_from_slice(&4u16.to_be_bytes()); p[20] = 17; p[37] = 1; p[53] = 2; p[54..58].copy_from_slice(&[0x30,0x39,0,53]);
        let r = parse_ethernet(&p); assert_eq!(r.protocol, "UDP"); assert_eq!(r.destination_port, Some(53)); assert_eq!(r.source.as_deref(), Some("::1")); assert!(!r.truncated);
        p.truncate(56); assert!(parse_ethernet(&p).truncated);
    }
    #[test] fn ipv6_extensions_are_not_misread_as_ports() {
        let mut p = vec![0; 58]; p[12..14].copy_from_slice(&[0x86,0xdd]); p[14] = 0x60; p[18..20].copy_from_slice(&4u16.to_be_bytes()); p[20] = 44;
        let r = parse_ethernet(&p); assert_eq!(r.protocol, "IPv6"); assert_eq!(r.source_port, None);
    }

}



