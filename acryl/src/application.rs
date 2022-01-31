use crate::protocols::Protocol;

use std::net::{ TcpStream, TcpListener };
use std::io;
use std::io::Result;
use std::io::Read;

pub struct Application<T : Protocol> {
	pub(crate) config: AppConfig,
	pub(crate) protocol: T
}

impl<T : Protocol> Application<T> {
	fn setup(&self) -> Result<TcpListener> {
		let listener = TcpListener::bind(self.config.url.as_str())?;
		
		if self.config.is_async {
			listener.set_nonblocking(true)?;
		}
		
		Ok(listener)
	}

	fn handle_connection(&self, stream: &mut TcpStream) {
		let mut data = String::new();
		stream.read_to_string(&mut data).unwrap();

		let req = self.protocol.parse(&data);
	}
	
	pub fn start(&self) {
		let socket = self.setup().unwrap();

		println!("Server started on {}", self.config.url);

		for stream in socket.incoming() {
			match stream {
		        Ok(mut s) => {
		            // do something with the TcpStream
		            self.handle_connection(&mut s);
		        }
		        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
		            // wait until network socket is ready, typically implemented
		            // via platform-specific APIs such as epoll or IOCP
		            // wait_for_fd();
		            
					// println!("waiting...");
					continue;
		        }
		        Err(e) => panic!("encountered IO error: {}", e),
		    }	
		}
	}
}

pub struct AppConfig {
	pub url: String,
	pub is_async: bool
}