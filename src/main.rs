extern crate time;

use time::precise_time_ns;

mod bitmap;
mod display;

fn main() {
	let i = precise_time_ns();
	let width = 1000;
	let height = 1000;
	//let bimp = bitmap::Bmp { bmp: bitmap::new("test.txt", 18, 18) };
	let bimp = bitmap::Bmp { bmp: bitmap::new_rand(width, height) };
    bimp.write_24("out.bmp", width, height);
    println!("elapsed time: {}", (precise_time_ns() - i)/1000000);
    display::show(width, height);
}
