use rayon::prelude::*;

use crate::mandelbrot::iteration_checker;

const DEFAULT_SPAN: f64 = 2.0;

pub fn 
generate_raw_data 
(
    width: u16,
    height: u16,
    x_mid: f64,
    y_mid: f64,
    zoom: f64,
    iterations: u64
) 
-> Vec<u64>
{
    let x_span = DEFAULT_SPAN / zoom;
    let ppi    = x_span       / (width  as f64);
    let y_span = ppi          * (height as f64);

    let x_start = x_mid - x_span/2.0;
    let y_end   = y_mid + y_span/2.0;

    // Reserve & Init memory for the raw data 
    let mut raw_data = vec![0; width as usize * height as usize];

    // Use parallel iterator over this memory to compute the iteration checker
    raw_data.par_iter_mut().enumerate().for_each(|(i, v)| 
        {
            let y = i / width as usize;
            let x = i % width as usize;

            *v = iteration_checker(
                x_start + ppi * x as f64,
                y_end   - ppi * y as f64, 
                iterations
            );
        }
    );

    return raw_data;
}
