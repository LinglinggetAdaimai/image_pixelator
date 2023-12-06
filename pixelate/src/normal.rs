use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer, Rgba};

struct Dimension {
    width: u32,
    height: u32
}

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn resize (img: &DynamicImage, n :u32) {

    let mut ori_image:Image = img.to_rgba8();

    let old_dimension = Dimension {
        width: ori_image.width(),
        height: ori_image.height()
    };

    let new_dimension: Dimension = Dimension {
        width: (old_dimension.width/n as u32), 
        height: (old_dimension.height/n as u32)
    };
    
    let ratio_w :f32 = old_dimension.width  as f32 / (old_dimension.width/n) as f32;
    let ratio_h :f32 = old_dimension.height  as f32 / (old_dimension.height/n)  as f32;
    
    let mut res_image: Image = ImageBuffer::new(new_dimension.width, new_dimension.height);

    // iterate through the new image and get the pixel from the old image
    (0..new_dimension.width).into_iter().for_each(|x| {
        (0..new_dimension.height).into_iter().for_each(|y| {
            let old_x = (x as f32 * ratio_w ) as u32;
            let old_y = (y as f32 * ratio_h) as u32;
            let old_pixel = ori_image.get_pixel(old_x, old_y);

            res_image.put_pixel(x, y, *old_pixel);
        })
    });

    (0..old_dimension.width).into_iter().for_each(|x| {
        (0..old_dimension.height).into_iter().for_each(|y| {
            let old_x: u32 = (x as f32 / ratio_w) as u32;
            let old_y: u32 = (y as f32 / ratio_h)as u32;
            let old_pixel = res_image.get_pixel(old_x, old_y);

            ori_image.put_pixel(x, y, *old_pixel);
        })
    });

    let pixelate_format = format!("images/norm_pixelate{}.png", n);
    let  _ = ori_image.save(pixelate_format);
}


// fix the new dimension according to n

pub fn norm_pixelate(filename: &String, n: u32) -> Result<(), Box<dyn std::error::Error>> {
    // let image = readImg(filename).unwrap();
    let file_path = format!("images/{}", filename);
    let img = ImageReader::open(file_path)?.decode()?;
    let _ = resize(&img, n );
    Ok(())
}

