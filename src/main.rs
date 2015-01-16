mod bitmap;

fn main() {
	let bimp = bitmap::Bmp {bmp: bitmap::new("test.txt", 20, 20),
							head: bitmap::getheader(20u32, 20u32) };
    println!("{}", bimp.bmp[5][5]);
}
