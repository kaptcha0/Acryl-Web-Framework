use crate::protocols::Response;

pub struct HttpResponse {}

impl Response for HttpResponse {
	fn new() -> Self {
		HttpResponse {}
	}
	
	fn to_string(&self) -> String {
		String::new()
	}
}