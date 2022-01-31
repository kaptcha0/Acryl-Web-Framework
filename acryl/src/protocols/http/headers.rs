use std::collections::HashMap;

pub struct Headers {
	headers: HashMap<String, String>
}

impl Headers {
	pub fn new(raw_headers: Vec<String>) -> Headers {
		let mut headers = HashMap::new();
		
		for line in raw_headers {
			let mut split = line.split(":");

			let h = split.collect::<Vec<&str>>();

			headers.insert(h[0].trim().to_string(), h[1].trim().to_string());
		}

		Headers {
			headers
		}
	}
}

impl std::ops::Index<Headers> for Headers {
	type Output = &str;

	fn index(&self, )
}