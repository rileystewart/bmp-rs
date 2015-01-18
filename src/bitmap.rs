use std::io::File;
use std::rand;

fn from_u32(u: u32) -> char {
	let v: u8 = (u%256) as u8;
	v as char
}

pub struct Bmp {
	pub bmp: Vec<Vec<u32>>,
}
impl Bmp { 
	pub fn write_8(&self, oname: &str, w: u32, h: u32) {
		let mut f = File::create(&Path::new(oname));
		let mut pad: String = String::new();
		f.write_u8(0x42);
		f.write_u8(0x4D);
		f.write_le_u32(26 + (w+w%4)*h);
		f.write_le_u16(2);
		f.write_le_u16(2);
		f.write_le_u32(26);
		f.write_le_u32(12);
		f.write_le_u16((w + w%4) as u16);
		f.write_le_u16(h as u16);
		f.write_le_u16(1);
		f.write_le_u16(8);
		for i in range (0, self.bmp.len()) {
			for j in range (0, self.bmp[i].len()) {
				f.write_u8(self.bmp[i][j] as u8);
			}
			if w%4 != 0 {
				for k in range (0, w%4) {
					f.write_u8(0);
				}
			}
		}
	}
	pub fn write_24(&self, oname: &str, w: u32, h: u32) {
		let mut f = File::create(&Path::new(oname));
		let mut pad: String = String::new();
		f.write_u8(0x42);
		f.write_u8(0x4D);
		f.write_le_u32(26 + 3*(w+w%4)*h);
		f.write_le_u16(2);
		f.write_le_u16(2);
		f.write_le_u32(26);
		f.write_le_u32(12);
		f.write_le_u16((w + w%4) as u16);
		f.write_le_u16(h as u16);
		f.write_le_u16(1);
		f.write_le_u16(24);
		for i in range (0, self.bmp.len()) {
			for j in range (0, self.bmp[i].len()) {
				f.write_u8((self.bmp[i][j]/65536) as u8);
				f.write_u8((self.bmp[i][j]/256) as u8);
				f.write_u8(self.bmp[i][j] as u8);
			}
			if w%4 != 0 {
				for k in range (0, w%4) {
					f.write_u8(0);
					f.write_u8(0);
					f.write_u8(0);
				}
			}
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

pub fn new_rand(width: u32, height: u32) -> Vec<Vec<u32>> {
	let mut rbmp: Vec<Vec<u32>> = Vec::new();
	for i in range(0, height) {
		rbmp.push(Vec::new());
		for j in range (0, width) {
			let red = rand::random::<u32>()%20 + 120;
			let green = rand::random::<u32>()%20 + 90;
			let blue = rand::random::<u32>()%20 + 120;
			let color: u32 = red*65536 + green*256 + blue;
			println!("color: {}", color);
			rbmp[i as uint].push(color);
		}
	}
	rbmp
}
		
