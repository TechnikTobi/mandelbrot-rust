mod mandelbrot;
use mandelbrot::iteration_checker;

fn main() {
	println!("Hello, world!");
	println!("{}", iteration_checker(1.1, 1.2, 100));
}
