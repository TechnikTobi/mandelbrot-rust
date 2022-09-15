use num::complex::Complex;

const SQR_THRESHOLD: f64 = 4.0;

pub fn
iteration_checker
(
	x: f64,
	y: f64,
	iterations: u64
)
-> u64
{

	let mut z = Complex::new(0.0 as f64, 0.0 as f64);
	let c     = Complex::new(x  , y  );

	let mut iteration_counter: u64 = 0;

	while
		z.re * z.re + z.im * z.im <= SQR_THRESHOLD &&
		iteration_counter < iterations
	{
		z = z*z + c;
		iteration_counter += 1;
	}

	return iteration_counter;

}

