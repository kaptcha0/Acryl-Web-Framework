use acryl::{ acryl, protocols::http::Http };

fn main() {
	acryl::<Http>(None).start();
}
