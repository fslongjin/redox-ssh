use std::io;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use connection::{Connection, ConnectionType};
use public_key::KeyPair;

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub key: Box<dyn KeyPair>,
}

pub struct Server {
    config: Arc<ServerConfig>,
}

impl Server {
    pub fn with_config(config: ServerConfig) -> Server {
        Server {
            config: Arc::new(config),
        }
    }

    pub fn run(&self) -> io::Result<()> {
        debug!("To listen on {}:{}", self.config.host, self.config.port);
        let listener =
            TcpListener::bind((&*self.config.host, self.config.port))?;

        loop {
            let (mut stream, addr) = listener.accept()?;
            let config = self.config.clone();

            debug!("Incoming connection from {}", addr);

            thread::spawn(move || {
                let mut connection =
                    Connection::new(ConnectionType::Server(config));

                let result = connection.run(&mut stream);

                if let Some(error) = result.err() {
                    println!("sshd: {}", error)
                }
            });
        }
    }
}
