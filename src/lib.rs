extern crate byteorder;
extern crate crypto;
extern crate num_bigint;
extern crate rand;
#[macro_use]
extern crate log;
#[cfg(not(target_os = "redox"))]
extern crate libc;
#[cfg(target_os = "redox")]
extern crate syscall;

mod algorithm;
mod channel;
mod connection;
mod encryption;
mod error;
mod key_exchange;
mod mac;
mod message;
mod packet;

pub mod public_key;
pub mod server;

#[cfg(target_os = "redox")]
#[path = "sys/redox.rs"]
pub mod sys;

#[cfg(not(target_os = "redox"))]
#[path = "sys/unix.rs"]
pub mod sys;

pub use self::server::{Server, ServerConfig};
