use std::{
    fmt::Debug,
    io::{self, Read, Write},
};

// mod rsa;
mod ed25519;

// pub use self::rsa::RSA;

pub use self::ed25519::ED25519;

pub trait KeyPair: Sync + Send + Debug {
    fn system(&self) -> &'static CryptoSystem;

    fn has_private(&self) -> bool;

    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, ()>;
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, ()>;

    fn write_public(&self, w: &mut dyn Write) -> io::Result<()>;
    fn export(&self, w: &mut dyn Write) -> io::Result<()>;
}

pub struct CryptoSystem {
    pub id: &'static str,
    pub generate_key_pair: fn(bits: Option<u32>) -> Box<dyn KeyPair>,
    pub import: fn(r: &mut dyn Read) -> io::Result<Box<dyn KeyPair>>,
    pub read_public: fn(r: &mut dyn Read) -> io::Result<Box<dyn KeyPair>>,
}
