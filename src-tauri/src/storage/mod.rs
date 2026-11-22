//! PCAP Ethernet clásico, con resolución de milisegundos heredada del motor.
use std::io::{Read, Write};
#[derive(Debug)] pub struct Frame { pub timestamp_ms: i64, pub original_len: u32, pub bytes: Vec<u8> }
pub fn write_pcap(mut out: impl Write, frames: &[Frame]) -> Result<(), String> {
    out.write_all(&[0xd4,0xc3,0xb2,0xa1, 2,0,4,0, 0,0,0,0, 0,0,0,0, 0xff,0xff,0,0, 1,0,0,0]).map_err(|e| e.to_string())?;
    for f in frames {
        if f.timestamp_ms < 0 || f.timestamp_ms / 1000 > u32::MAX as i64 || f.bytes.len() > 65535 || f.bytes.len() > f.original_len as usize { return Err("Paquete no compatible con PCAP".into()); }
        for n in [(f.timestamp_ms / 1000) as u32, ((f.timestamp_ms % 1000) * 1000) as u32, f.bytes.len() as u32, f.original_len] { out.write_all(&n.to_le_bytes()).map_err(|e| e.to_string())?; }
        out.write_all(&f.bytes).map_err(|e| e.to_string())?;
    }
    out.flush().map_err(|e| e.to_string())
}
pub fn read_pcap(mut input: impl Read) -> Result<Vec<Frame>, String> {
    let mut global = [0u8; 24]; input.read_exact(&mut global).map_err(|_| "Cabecera PCAP incompleta")?;
    if global != [0xd4,0xc3,0xb2,0xa1,2,0,4,0,0,0,0,0,0,0,0,0,0xff,0xff,0,0,1,0,0,0] { return Err("Formato PCAP no admitido por el historial".into()); }
    let mut frames = Vec::new();
    loop {
        let mut header = [0u8; 16]; let n = input.read(&mut header[..1]).map_err(|e| e.to_string())?; if n == 0 { break; }
        input.read_exact(&mut header[1..]).map_err(|_| "Registro PCAP incompleto")?;
        let number = |offset| u32::from_le_bytes(header[offset..offset+4].try_into().unwrap());
        let sec = number(0); let micros = number(4); let cap = number(8); let original_len = number(12);
        if micros >= 1_000_000 || cap > 65535 || cap > original_len || frames.len() >= 2000 { return Err("Registro PCAP invalido o limite excedido".into()); }
        let mut bytes = vec![0; cap as usize]; input.read_exact(&mut bytes).map_err(|_| "Datos PCAP incompletos")?;
        frames.push(Frame { timestamp_ms: i64::from(sec)*1000 + i64::from(micros/1000), original_len, bytes });
    }
    Ok(frames)
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn pcap_roundtrip_and_truncation() {
        let frames = vec![Frame { timestamp_ms: 1700000000123, original_len: 9, bytes: vec![1,2,3] }];
        let mut data = Vec::new(); write_pcap(&mut data, &frames).unwrap();
        let read = read_pcap(&data[..]).unwrap(); assert_eq!(read[0].timestamp_ms, frames[0].timestamp_ms); assert_eq!(read[0].original_len,9); assert_eq!(read[0].bytes,frames[0].bytes);
        assert!(read_pcap(&data[..data.len()-1]).is_err());
    }
}
