use image::io::Reader as ImageReader; // image reader that can read from a file
use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba, RgbImage, GenericImage, Pixel, SubImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator, IndexedParallelIterator, self};
use rayon::slice::{ParallelSlice, ParallelSliceMut}; 
use std::env;
use std::os::macos::raw::stat;
use rayon::iter::*;
use std::time::{Instant, Duration};

const IMAGE_THRES: usize = 1024*16;

fn timeit (f: &dyn Fn()) -> Duration {
    let start = Instant::now();
    f();
    let end = Instant::now();
    end.duration_since(start)
}

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

// recursion + sequencially downscaling
fn downscaling(ori_img: &Image, sub_img: &mut [&mut [u8]] , new_w: u32, ratio_w: f32, ratio_h: f32, start:usize) {
    let len = sub_img.len();

    if len <= IMAGE_THRES {
        sub_img.into_iter().enumerate()
            .for_each(|(index, pixel)| {
            let y = (index+start) / new_w as usize;
            let x = (index+start) % new_w as usize;

            let ori_x = (x as f32 * ratio_w) as u32;
            let ori_y = (y as f32 * ratio_h) as u32;

            let old_pixels = ori_img.get_pixel(ori_x, ori_y).channels();

            pixel.iter_mut().zip(old_pixels).for_each(|(new_pix, &old_pix)| {
                *new_pix = old_pix;
            });
        });
    }
    
    else {
        let mid = len/2;
        let (left, right) = sub_img.split_at_mut(mid);
        rayon::join(|| downscaling(ori_img,left, new_w, ratio_w, ratio_h,start),
                    || downscaling(ori_img, right, new_w, ratio_w, ratio_h,start+mid));
    }
    
}
fn upscaling(ori_img: &Image, sub_img: &mut [&mut [u8]] , width: u32, ratio_w: f32, ratio_h: f32, start:usize) {
    let len = sub_img.len();

    if len <= IMAGE_THRES {
        sub_img.into_iter().enumerate()
            .for_each(|(index, pixel)| {
            let y = (index+start) / width as usize;
            let x = (index+start) % width as usize;

            let ori_x = (x as f32 / ratio_w) as u32;
            let ori_y = (y as f32 / ratio_h) as u32;

            let old_pixels = ori_img.get_pixel(ori_x, ori_y).channels();

            pixel.iter_mut().zip(old_pixels).for_each(|(new_pix, &old_pix)| {
                *new_pix = old_pix;
            });
        });
    }

    else {
        let mid = len/2;
        let (left, right) = sub_img.split_at_mut(mid);
        rayon::join(|| upscaling(ori_img,left, width, ratio_w, ratio_h, start),
                    || upscaling(ori_img, right, width, ratio_w, ratio_h,start+mid));
    }
    
}
fn pixelate_down(ori_img: &Image, sub_img: &mut [&mut [u8]] , width: u32, ratio_w: f32, ratio_h: f32, start:usize) {
    sub_img.into_iter().enumerate()
        .for_each(|(index, pixel)| {
        let y = (index+start) / width as usize;
        let x = (index+start) % width as usize;

        let ori_x = (x as f32 * ratio_w) as u32;
        let ori_y = (y as f32 * ratio_h) as u32;

        let old_pixels = ori_img.get_pixel(ori_x, ori_y).channels();

        pixel.iter_mut().zip(old_pixels).for_each(|(new_pix, &old_pix)| {
            *new_pix = old_pix;
        });
    });
}
fn pixelate_up (ori_img: &Image, sub_img: &mut [&mut [u8]] , width: u32, ratio_w: f32, ratio_h: f32, start:usize) {

    sub_img.into_iter().enumerate()
        .for_each(|(index, pixel)| {
        let y = (index+start) / width as usize;
        let x = (index+start) % width as usize;

        let ori_x = (x as f32 / ratio_w) as u32;
        let ori_y = (y as f32 / ratio_h) as u32;

        let old_pixels = ori_img.get_pixel(ori_x, ori_y).channels();

        pixel.iter_mut().zip(old_pixels).for_each(|(new_pix, &old_pix)| {
            *new_pix = old_pix;
        });
    });
}
// recursion
fn resize2 (img: &DynamicImage, n :u32) {

    let mut ori_image:Image = img.to_rgba8();

    let old_width =  ori_image.width();
    let old_height = ori_image.height();

    let new_width = old_width/ n;
    let new_height = old_height/ n;

    let ratio_w :f32 = old_width  as f32 / (old_width/n) as f32;
    let ratio_h :f32 = old_height  as f32 / (old_height/n)  as f32;

    let mut res_image: Image = ImageBuffer::new(new_width, new_height);

    let mut result_pixels: Vec<_> = res_image.par_chunks_exact_mut(4 as usize).collect();

    let mid = result_pixels.len()/2;

    let (l,r) = result_pixels.split_at_mut(mid);

    rayon::join(|| downscaling(&ori_image,l, new_width, ratio_w, ratio_h,0),
                 || downscaling(&ori_image, r, new_width, ratio_w, ratio_h,mid));

    let mut upscaling_pixels: Vec<_> = ori_image.par_chunks_exact_mut(4 as usize).collect();

    let (l,r) = upscaling_pixels.split_at_mut(mid);

    rayon::join(|| upscaling(&res_image,l, old_width, ratio_w, ratio_h, 0),
                 || upscaling(&res_image, r, old_width, ratio_w, ratio_h, mid));

    let pixelate_format = format!("images/par_pixelated2new_{}.png", n);
    let  _ = ori_image.save(pixelate_format);


}
fn resize3 (img: &DynamicImage, n :u32) {

    let mut ori_image:Image = img.to_rgba8();

    let old_width =  ori_image.width();
    let old_height = ori_image.height();

    let new_width = old_width/ n;
    let new_height = old_height/ n;

    let ratio_w :f32 = old_width  as f32 / (old_width/n) as f32;
    let ratio_h :f32 = old_height  as f32 / (old_height/n)  as f32;

    let mut res_image: Image = ImageBuffer::new(new_width, new_height);

    let mut result_pixels: Vec<_> = res_image.par_chunks_exact_mut(4 as usize).collect();

    let mid = result_pixels.len()/2;

    let (l,r) = result_pixels.split_at_mut(mid);

    rayon::join(|| pixelate_down(&ori_image, l, new_width, ratio_w, ratio_h,0),
                 || pixelate_down(&ori_image, r, new_width, ratio_w, ratio_h,mid));

    let mut upscaling_pixels: Vec<_> = ori_image.par_chunks_exact_mut(4 as usize).collect();


    let (l,r) = upscaling_pixels.split_at_mut(mid);

    rayon::join(|| pixelate_up(&res_image,l, old_width, ratio_w, ratio_h, 0),
                 || pixelate_up(&res_image, r, old_width, ratio_w, ratio_h, mid));
    

    let pixelate_format = format!("images/par_pixelated3new_{}.png", n);
    let  _ = ori_image.save(pixelate_format);
}
//not done, wanna do 
    // create a new picture with the old dimensiuon and then concurrently change it no need to downscaling

// pixelate by half par - seq
fn resize4 (img: &DynamicImage, n :u32) {

    let mut ori_image:Image = img.to_rgba8();

    let old_width =  ori_image.width();
    let old_height = ori_image.height();

    let new_width = old_width/ n;
    let new_height = old_height/ n;

    let ratio_w :f32 = old_width  as f32 / (old_width/n) as f32;
    let ratio_h :f32 = old_height  as f32 / (old_height/n)  as f32;

    let mut res_image: Image = ImageBuffer::new(new_width, new_height);

    let mut result_pixels: Vec<_> = res_image.par_chunks_exact_mut(4 as usize).collect();

    let len = result_pixels.len();
    let mid = len/2;
    let (l,r) = result_pixels.split_at_mut(len/2);
    rayon::join(|| pixelate_down(&ori_image,l, new_width, ratio_w, ratio_h,0),
                 || pixelate_down(&ori_image, r, new_width, ratio_w, ratio_h,mid));

    let mut upscaling_pixels: Vec<_> = ori_image.par_chunks_exact_mut(4 as usize).collect();


    let (l,r) = upscaling_pixels.split_at_mut(mid);

    rayon::join(|| pixelate_up(&res_image,l, old_width, ratio_w, ratio_h, 0),
                 || pixelate_up(&res_image, r, old_width, ratio_w, ratio_h, mid));

    let pixelate_format = format!("images/par_seqnew_{}.png", n);
    let  _ = ori_image.save(pixelate_format);

}

pub fn par_pixelate2(filename: &String, n: u32) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = format!("images/{}", filename);
    let img = ImageReader::open(file_path)?.decode()?;
    let _ = resize2(&img, n);
    Ok(())
}

pub fn par_pixelate3(filename: &String, n: u32) -> Result<(), Box<dyn std::error::Error>> {

    let file_path = format!("images/{}", filename);
    let img = ImageReader::open(file_path)?.decode()?;

    let _ = resize3(&img, n);

    Ok(())
}
pub fn par_pixelate4(filename: &String, n: u32) -> Result<(), Box<dyn std::error::Error>> {

    let file_path = format!("images/{}", filename);
    let img = ImageReader::open(file_path)?.decode()?;
    let _ = resize4(&img, n);

    Ok(())
}


// Normal running time: ((), 421.595084ms)
// 1st parallel pixelate time: 450.271916ms
// 2nd parallel pixelate time: 433.154458ms
// 3rd parallel pixelate time: 314.355375ms
// 5th parallel pixelate time: 312.147459ms

// Normal running time: ((), 26.530666ms)
// 1st parallel pixelate time: 10.849ms
// 2nd parallel pixelate time: 10.917ms
// 3rd parallel pixelate time: 5.13825ms
// 5th parallel pixelate time: 5.530167ms