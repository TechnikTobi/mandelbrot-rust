# mandelbrot-rust
Implementing a Mandelbrot Image Generator (In Rust) (Agai... wait, this is my first time using Rust!)

![example workflow](https://github.com/TechnikTobi/mandelbrot-rust/actions/workflows/rust.yml/badge.svg)

## Examples
```mandelbrot-rust -w 4000 -h 4000 --x_mid=-1.25737518304050244765 --y_mid=0.378759463618701690545 -z 6800000000 -i 2040 -c 13```

## Structure
- ```main.rs``` contains the main function called upon startup (duh)
- ```cli.rs``` gets the values for the different parameters to use during computations (and providing default values if non are given) 
- ```generator.rs``` creates the "raw" mandelbrot data, computing the iterations for every point of interest on the gaussian plane, using...
- ... ```mandelbrot.rs```, which provides the function to check for a given complex value how many iterations it takes to exceed the threshold of 2^2 (=4)
- ```color_map.rs``` contains different color modes to map the raw mandelbrot data to RGB-values in the form of a vector in row-major order, with every triple of values forming the RGB values of a single pixel
- ```png_writer.rs``` then takes this vector and writes it to a PNG image file

## Used crates / libraries
- ```num``` for handling complex numbers
- ```image``` for writing image data as PNGs
- ```clap``` for handling CLI option inputs
- ```little_exif``` for writing parameters as EXIF metadata

## ToDo / Wishes
- Functionality to also generate images of julia sets
- More color modes / better CLI (currently using numbers - at least provide information about which color mode is which)
- Video generation capability
- Custom complex expression evaluation for mandelbrot sets (currently only fixed z = z^2 + c) 


