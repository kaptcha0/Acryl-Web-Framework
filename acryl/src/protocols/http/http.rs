use crate::protocols::{ Protocol, Request, Response };
use crate::protocols::http::{ HttpRequest, HttpResponse };

pub enum HttpVersion {
	V1_0,
	V1_1
}

pub struct Http {
}

impl Protocol for Http {
	fn parse(&self, buffer: &String) -> Box<dyn Request> {
		Box::new(HttpRequest::new(buffer))
	}

	fn handle_request(&self) -> Box<dyn Response> {
		Box::new(HttpResponse::new())
	}
	
	fn new() -> Self {
		Http {}
	}
}