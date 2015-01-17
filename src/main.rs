mod bitmap;

fn main() {
	let bimp = bitmap::Bmp { bmp: bitmap::new("test.txt", 18, 18) };
    bimp.write("out.bmp", 18, 18);
}
