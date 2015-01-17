use std::io::File;

pub fn getheader(w: u32, ht: u32) -> String {
	let filesize = 26 + w*ht;
	println!("filesize {}", filesize);
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
	h.push(from_u32(26));
	h.push(from_u32(0));
	h.push(from_u32(0));
	h.push(from_u32(0));
	h.push(from_u32(12));
	h.push(from_u32(0));
	h.push(from_u32(0));
	h.push(from_u32(0));
	h.push(from_u32(w));
	h.push(from_u32(0));
	h.push(from_u32(ht));
	h.push(from_u32(0));
	h.push(from_u32(1));
	h.push(from_u32(0));
	h.push(from_u32(8));
	h.push(from_u32(0));
	h
}

fn from_u32(u: u32) -> char {
	let v: u8 = (u%256) as u8;
	v as char
}

pub struct Bmp {
	pub bmp: Vec<Vec<u32>>,
	pub head: String
}
impl Bmp { 
	pub fn write(&self, oname: &str, w: u32) {
		let mut f = File::create(&Path::new(oname));
		let mut pad: String = String::new();
		if w%4 != 0 {
			for i in range (0, w%4) {
				pad.push(0 as char);
			}
		}
		f.write_str(self.head.as_slice());
		for i in range (0, self.bmp.len()) {
			for j in range (0, self.bmp[i].len()) {
				f.write_u8(self.bmp[i][j] as u8);
			}
			f.write_str(pad.as_slice());
		}
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
			if line==height as usize {
				break;
			}
		}
	}
	nbmp
}
		
