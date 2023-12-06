mod normal;
mod parallel_pixelate;

use std::env;
use std::time::{Instant, Duration};


fn timed<R, F>(f: F) -> (R, Duration) where F: Fn() -> R {
    let starting_point = Instant::now();
    let res = f();
    (res, starting_point.elapsed())
}


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

    let norm_time = timed(|| normal::norm_pixelate(&file_name, n).unwrap());
    println!("Normal running time: {:?}", norm_time);

    let par_time2 = timed(|| parallel_pixelate::par_pixelate2(&file_name, n).unwrap());
    println!("Parallel2 running time: {:?}", par_time2);

    // no good
    // let par_time3 = timed(|| parallel_pixelate::par_pixelate3(&file_name, n).unwrap());
    // println!("Parallel3 running time: {:?}", par_time3);

    // let par_time4 = timed(|| parallel_pixelate::par_pixelate4(&file_name, n).unwrap());
    // println!("Parallel running time: {:?}", par_time4);


    Ok(())
}