extern crate glutin;
extern crate gl;

fn show (pic: Vec<Vec<u32>>) {
	let window = glutin::Window::new().unwrap();
	unsafe { window.make_current(); }
	gl::load_with(|symbol| window.get_proc_address(symbol));
	
}
