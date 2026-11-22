//! Contrato de captura; todavía no hay proveedores implementados.
#[derive(Debug, Clone)]
pub struct CaptureCapabilities {
    pub live: bool,
    pub capture_filter: bool,
    pub pause: bool,
}

pub trait CaptureProvider {
    type Error;
    fn capabilities(&self) -> CaptureCapabilities;
    fn interfaces(&self) -> Result<Vec<String>, Self::Error>;
    fn start(&mut self, interface: &str) -> Result<(), Self::Error>;
    fn receive(&mut self) -> Result<Option<Vec<u8>>, Self::Error>;
    fn stop(&mut self) -> Result<(), Self::Error>;
}

#[cfg(windows)]
pub mod providers;
