pub trait Request {
	fn new(raw_request: &String) -> Self where Self : Sized;
}

pub trait Response {
	fn to_string(&self) -> String;
	fn new() -> Self where Self : Sized;
}

pub trait Protocol {
	fn parse(&self, buffer: &String) -> Box<dyn Request>;
	fn handle_request(&self) -> Box<dyn Response>;
	fn new() -> Self where Self : Sized;
}