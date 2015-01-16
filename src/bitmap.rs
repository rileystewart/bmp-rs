use std::io::File;
use std::string::String;


pub struct Bmp {
pub bmp: Vec<Vec<i32>>
}
impl Bmp { 
	fn new(&self, fname: String, width: i32, height: i32) -> Vec<Vec<i32>> {
	self.bmp = Vec::new();
	let contents = File::open(&Path::new(fname)).read_to_string().unwrap();
	let line = 0;
	let pos = 0;
	let c: char;
	for c in contents.as_slice().iter() {
		self.bmp[line].push(c.parse());
		pos += 1;
		if pos > width {
			pos = 0;
			line += 1;
		}
	}
	self.bmp
}
}

			
