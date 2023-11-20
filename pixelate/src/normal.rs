
use image::io::Reader as ImageReader; // image reader that can read from a file
use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba}; 

type Pixel_arr = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let img = ImageReader::open("peppa.jpg")?
        .decode()?;
    println!("Image dimensions: {:?}", img.dimensions());

    Ok(())
}
