pub mod protocols;
pub mod application;

use crate::application::{ Application, AppConfig };
use crate::protocols::Protocol;

pub fn acryl<T : Protocol>(config: Option<AppConfig>) -> Application<T> { 
	let conf = match config {
		Some(c) => c,
		None => AppConfig {
			url: "0.0.0.0:80".to_string(),
			is_async: false
		}
	};
	
	Application::<T> {
		config: conf,
		protocol: T::new()
	}
}