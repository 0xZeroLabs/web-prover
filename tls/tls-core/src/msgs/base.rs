use crate::{
    key,
    msgs::{
        codec,
        codec::{Codec, Reader},
    },
};

/// An externally length'd payload
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Payload(pub Vec<u8>);

impl Codec for Payload {
    fn encode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.0);
    }

    fn read(r: &mut Reader) -> Option<Self> {
        Some(Self::read(r))
    }
}

impl Payload {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self(bytes.into())
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn read(r: &mut Reader) -> Self {
        Self(r.rest().to_vec())
    }
}

impl Codec for key::Certificate {
    fn encode(&self, bytes: &mut Vec<u8>) {
        codec::u24(self.0.len() as u32).encode(bytes);
        bytes.extend_from_slice(&self.0);
    }

    fn read(r: &mut Reader) -> Option<Self> {
        let len = codec::u24::read(r)?.0 as usize;
        let mut sub = r.sub(len)?;
        let body = sub.rest().to_vec();
        Some(Self(body))
    }
}

/// An arbitrary, unknown-content, u24-length-prefixed payload
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PayloadU24(pub Vec<u8>);

impl PayloadU24 {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl Codec for PayloadU24 {
    fn encode(&self, bytes: &mut Vec<u8>) {
        codec::u24(self.0.len() as u32).encode(bytes);
        bytes.extend_from_slice(&self.0);
    }

    fn read(r: &mut Reader) -> Option<Self> {
        let len = codec::u24::read(r)?.0 as usize;
        let mut sub = r.sub(len)?;
        let body = sub.rest().to_vec();
        Some(Self(body))
    }
}

/// An arbitrary, unknown-content, u16-length-prefixed payload
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PayloadU16(pub Vec<u8>);

impl PayloadU16 {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn encode_slice(slice: &[u8], bytes: &mut Vec<u8>) {
        (slice.len() as u16).encode(bytes);
        bytes.extend_from_slice(slice);
    }
}

impl Codec for PayloadU16 {
    fn encode(&self, bytes: &mut Vec<u8>) {
        Self::encode_slice(&self.0, bytes);
    }

    fn read(r: &mut Reader) -> Option<Self> {
        let len = u16::read(r)? as usize;
        let mut sub = r.sub(len)?;
        let body = sub.rest().to_vec();
        Some(Self(body))
    }
}

/// An arbitrary, unknown-content, u8-length-prefixed payload
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PayloadU8(pub Vec<u8>);

impl PayloadU8 {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

pub struct PayloadU8Len(pub usize);
impl ring::hkdf::KeyType for PayloadU8Len {
    fn len(&self) -> usize {
        self.0
    }
}

impl From<ring::hkdf::Okm<'_, PayloadU8Len>> for PayloadU8 {
    fn from(okm: ring::hkdf::Okm<PayloadU8Len>) -> Self {
        let mut r = vec![0u8; okm.len().0];
        okm.fill(&mut r[..]).unwrap();
        Self::new(r)
    }
}

impl Codec for PayloadU8 {
    fn encode(&self, bytes: &mut Vec<u8>) {
        (self.0.len() as u8).encode(bytes);
        bytes.extend_from_slice(&self.0);
    }

    fn read(r: &mut Reader) -> Option<Self> {
        let len = u8::read(r)? as usize;
        let mut sub = r.sub(len)?;
        let body = sub.rest().to_vec();
        Some(Self(body))
    }
}
