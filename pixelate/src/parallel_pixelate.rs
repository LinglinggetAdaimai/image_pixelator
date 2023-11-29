use image::io::Reader as ImageReader; // image reader that can read from a file
use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba, RgbImage, GenericImage, Pixel};
use rayon::iter::{IntoParallelIterator, ParallelIterator, IndexedParallelIterator, self};
use rayon::slice::{ParallelSlice, ParallelSliceMut}; 
use std::env;
use rayon::iter::*;
use std::time::{Instant, Duration};

fn timeit (f: &dyn Fn()) -> Duration {
    let start = Instant::now();
    f();
    let end = Instant::now();
    end.duration_since(start)
}

struct Dimension {
    width: u32,
    height: u32
}

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn resize (img: &DynamicImage, n :u32) {

    let ori_image:Image = img.to_rgba8();

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
    
    println!("ori dimension: {}, {}", old_dimension.width, old_dimension.height);
    println!("new dimension: {}, {}", new_dimension.width, new_dimension.height);
    
    // println!("ratio_w: {}, ratio_h: {}", ratio_w, ratio_h);
    // let mut res_image: Image = ImageBuffer::new(old_dimension.width, old_dimension.height);
    let mut res_image: Image = ImageBuffer::new(new_dimension.width, new_dimension.height);
    
    // group of pixel [n*m]
    let result_pixels = res_image.par_chunks_exact_mut(4 as usize);
    // let result_pixels = res_image.par_chunks_exact_mut(4*new_dimension.height as usize);
    println!("result_pixels: {}", result_pixels.len());
    
    result_pixels.enumerate()
        .for_each(|(index, new_pixels)| {
            let x = index / new_dimension.width as usize;
            let y = index % new_dimension.width as usize;
            let ori_x = (x as f32 * ratio_w) as u32;
            let ori_y = (y as f32 * ratio_h) as u32;

            // print!("x: {}, y: {}, ori_x: {}, ori_y: {}", x, y, ori_x, ori_y);

            let old_pixels = ori_image.get_pixel(ori_x, ori_y).channels();

            new_pixels.par_iter_mut().zip(old_pixels)
                .enumerate()
                .for_each(|(index, (new_pix, &old_pix))| {
                // println!("index: {}", index);
                // println!("new_pix: {}", new_pix);
                // println!("old_pix: {}", old_pix);
                *new_pix = old_pix;

                println!("new_pix after: {}", new_pix);
                });
        });
        
    let pixelate_format = format!("images/par_pixelated_{}.png", n);
    let  _ = res_image.save(pixelate_format);


    // go through new image by row
    // for each pixel in the row, find the corresponding pixel in the old image
    // by also going through the ori image row by ratio
    
    // /**
    //  * new idea, pariter -> Enumerate, like L19 through the new img with the old img dimension and then adjust the ratio
    // */

    // as an array of pixels
    // x = i/width
    // y = i%width

    // let start = Instant::now();
    // let built_in_groups = ori_image.par_chunks(4*old_dimension.width as usize).count();
    // let done: Instant = Instant::now();
    // println!("time built_in_groups: {:?}", done.duration_since(start));
    // println!("built_in_groups: {}", built_in_groups);


}

// fn test_resize(img: DynamicImage, n: u32) {
//     // can we loop through the image, then get the pixel and then aplly to every pixel in the image
//     let mut ori_image:Image = img.to_rgba8();

//     let mut pixelated:Image = ImageBuffer::new(ori_image.width(), ori_image.height());

//     // pixelated.par_chunks_exact_mut((n * 4) as usize).for_each(|x| {
//     //     x.chunks_exact_mut(4).for_each(|y| {
//     //         y[0] = 255;
//     //         y[1] = 0;
//     //         y[2] = 0;
//     //         y[3] = 255;
//     //     })
//     // });
//     let _ = pixelated.par_chunks_exact_mut(4).zip(ori_image.into_par_iter().skip(n as usize).chunks(4))
//         .into_par_iter().for_each(|(res_pix, ori_pix)| {
//         res_pix[0] = *ori_pix[0];
//         res_pix[1] = *ori_pix[1];
//         res_pix[2] = *ori_pix[2];
//         // res_pix[3] = *ori_pix[3];
//     });

    // pixelated
// }

pub fn par_pixelate(filename: &String, n: u32) -> Result<(), Box<dyn std::error::Error>> {

    // break the image into resonable size
    // pass to the resize function
    // save the image

    let file_path = format!("images/{}", filename);
    let img = ImageReader::open(file_path)?.decode()?;
    let _ = resize(&img, n);

    Ok(())
}

