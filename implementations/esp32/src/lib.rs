use esp_idf_sys as _;
use rand_core::{impls, CryptoRng, Error, RngCore};

pub struct EspRng;

impl RngCore for EspRng {
    fn next_u32(&mut self) -> u32 {
        unsafe { esp_idf_sys::esp_random() }
    }

    fn next_u64(&mut self) -> u64 {
        unsafe {
            let msw = esp_idf_sys::esp_random() as u64;
            let lsw = esp_idf_sys::esp_random() as u64;
            msw << 32 | lsw
        }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl CryptoRng for EspRng {}
