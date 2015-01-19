extern crate time;

use time::precise_time_ns;

mod bitmap;

fn main() {
	let i = precise_time_ns();
	let width = 800;
	let height = 800;
	//let bimp = bitmap::Bmp { bmp: bitmap::new("test.txt", 18, 18) };
	let bimp = bitmap::Bmp { bmp: bitmap::new_rand(width, height) };
    bimp.write_24("out.bmp", width, height);
    println!("{}", (precise_time_ns() - i)/1000000);
}
