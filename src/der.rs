use core::convert::{AsRef, AsMut};

pub struct SignatureArray([u8; 6 + 33 + 33], usize);

impl SignatureArray {
    pub fn new(size: usize) -> Self {
        SignatureArray([0u8; 6 + 33 + 33], size)
    }

    pub fn len(&self) -> usize {
        self.1
    }
}

impl AsRef<[u8]> for SignatureArray {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for SignatureArray {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}
