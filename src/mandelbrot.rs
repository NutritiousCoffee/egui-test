use num_complex::Complex;
use std::os::raw::c_int;

pub fn in_mandelbrot(a: f64,b: f64, iterations: usize) ->bool{
	let c = Complex::new(a, b);
	let mut z = Complex::new(0.0, 0.0);
	for n in 1..iterations{
		let z_new = z*z+c;
		z = z_new;
		if z.re*z.re > 4.0 || z.im * z.im > 4.0{
			return false;
		}

	}
	dbg!(a, b);
	return true;
}