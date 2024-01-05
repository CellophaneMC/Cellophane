use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct ByteArray(Vec<i8>);

impl ByteArray {
    pub fn new(data: Vec<i8>) -> Self {
        Self(data)
    }

    pub fn into_inner(self) -> Vec<i8> {
        self.0
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        let data: &[i8] = unsafe { std::mem::transmute(data) };
        Self(data.to_owned())
    }

    pub fn from_buf(data: Vec<u8>) -> Self {
        let data: &[i8] = unsafe { std::mem::transmute(data.as_slice()) };
        Self(data.to_owned())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.iter().map(|&b| b as u8).collect()
    }
}

impl Deref for ByteArray {
    type Target = [i8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ByteArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
