use crate::protocols::Request;
use crate::protocols::http::{ HttpVersion, Headers };

pub struct HttpRequest {
	version: HttpVersion,
	headers: Headers
}

impl HttpRequest {
	fn parse(raw: &str) -> HttpRequest {
		let mut lines = raw.lines();

		let head = lines.next();

		let mut headers = Vec::<String>::new();
		
		for (i, line) in lines.enumerate() {
			if line.to_string().trim() == "".to_string() {
				break;
			}

			headers.push(line.to_string());
		}
		
		HttpRequest {
			version: HttpVersion::V1_1,
			headers: Headers::new(headers)
		}
	}
}

impl Request for HttpRequest {
	fn new(raw_request: &String) -> Self {
		HttpRequest::parse(&raw_request)
	}
}