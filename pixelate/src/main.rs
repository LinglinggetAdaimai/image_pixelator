mod normal;
mod parallel_pixelate;


use image::io::Reader as ImageReader; // image reader that can read from a file
use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba, RgbImage, GenericImage};
use core::time;
use std::env;
use std::time::{Instant, Duration};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // if have time I wanna make it run with bash script 
    // > pixelate <filename>
    let args: Vec<String> = env::args().collect();

    // Ensure at least two arguments are provided (executable name and file name)
    if args.len() != 3  {
        println!("Usage: cargo run <file_name>");
        println!("Please provide a file name and size of pixel as an argument.");
        return Ok(())
    }
    let file_name = &args[1]; // The second argument is the file name
    let n = args[2].parse::<u32>().unwrap()  ;

    let n10 = 10;
    let n50 = 50;
    let n2 = 2;

    // let start = Instant::now();
    // normal::norm_pixelate(&file_name, n).unwrap();
    // let duration = start.elapsed();
    // println!("Normal running time: {:?}", duration);

    // let start = Instant::now();
    parallel_pixelate::par_pixelate(&file_name, n).unwrap();
    // let duration = start.elapsed();
    // println!("Parallel running time: {:?}", duration);


    // let normal_running_time_n10 = timeit(&|| {normal::norm_pixelate(&file_name, n10).unwrap();});
    // let normal_running_time_n50 = timeit(&|| {normal::norm_pixelate(&file_name, n50).unwrap();});
    // let normal_running_time_n2 = timeit(&|| {normal::norm_pixelate(&file_name, n2).unwrap();});

    // println!("Normal running time n10: {:?}", normal_running_time_n10);
    // println!("Normal running time n50: {:?}", normal_running_time_n50);
    // println!("Normal running time n2: {:?}", normal_running_time_n2);


    Ok(())
}