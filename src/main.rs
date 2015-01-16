mod bitmap;

fn main() {
	let bimp = bitmap::Bmp { bmp: Vec::new() };
	let veccy = bimp.new("test.txt".to_string(), 20, 20);
    println!("{}", veccy[0][0]);
}
