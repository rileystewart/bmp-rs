use std::io::File;

pub fn getheader(w: u32, ht: u32) -> String {
	let filesize = 54 + 3*w*ht;
	let mut h: String = String::new();
	h.push('B');
	h.push('M');
	h.push(from_u32(filesize));
	h.push(from_u32(filesize >> 8));
	h.push(from_u32(filesize >> 16));
	h.push(from_u32(filesize >> 24));
	h.push(from_u32(0));
	h.push(from_u32(0));
	h.push(from_u32(0));
	h.push(from_u32(0));
	h.push(from_u32(54));
	h.push(from_u32(0));
	h.push(from_u32(0));
	h.push(from_u32(0));
	h.push(from_u32(40));
	h.push(from_u32(0));
	h.push(from_u32(0));
	h.push(from_u32(0));
	h.push(from_u32(w));
	h.push(from_u32(w>>8));
	h.push(from_u32(w>>16));
	h.push(from_u32(w>>24));
	h.push(from_u32(ht));
	h.push(from_u32(ht>>8));
	h.push(from_u32(ht>>16));
	h.push(from_u32(ht>>24));
	h
}

fn from_u32(u: u32) -> char {
	(u as u8) as char
}

pub struct Bmp {
	pub bmp: Vec<Vec<u32>>,
	pub head: String
}
impl Bmp { 
	pub fn write(&self, oname: &str) {
		let f = File::create(&Path::new("out.bmp"));
		f.write(self.head.to_slice());
		for i in range (self.bmp.len(), 0) {
			for j in range (self.bmp[i].len(), 0) {
				
	}
}

pub fn new(fname: &str, width: u32, height: u32) -> Vec<Vec<u32>> {
	let mut nbmp: Vec<Vec<u32>> = Vec::new();
	let contents = File::open(&Path::new(fname)).read_to_string().unwrap();
	let bytes = contents.as_bytes();
	let mut line = 0;
	let mut pos = 0;
	for j in range (0, height) {
		nbmp.push(Vec::new());
	}
	for i in range(0, bytes.len()) {
		nbmp[line].push(bytes[i] as u32 - 48);
		pos += 1;
		if pos >= width {
			pos = 0;
			line += 1;
			if line == 20 {
				break;
			}
		}
	}
	nbmp
}
		
