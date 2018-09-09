extern crate image;

mod elapsed_metrics;

use elapsed_metrics::{ElapsedMetrics};
use std::env;
use std::error::Error;

fn resize(sample_type:&str, img:image::DynamicImage, width:u32, height:u32) -> image::DynamicImage {
    match sample_type {
        "l" => img.resize(width, height, image::imageops::Lanczos3),
        "g" => img.resize(width, height, image::imageops::Gaussian),
        "c" => img.resize(width, height, image::imageops::CatmullRom),
        "n" => img.resize(width, height, image::imageops::Nearest),
        "t" => img.resize(width, height, image::imageops::Triangle),
        _ => img.resize(width, height, image::imageops::Nearest)
    }
}

fn thumbnail(method:&str, img:image::DynamicImage, width:u32, height:u32) -> image::DynamicImage {
    match method {
        "e" => img.thumbnail_exact(width, height),
        "f" => img.thumbnail(width, height),
        _ => img.thumbnail(width, height)
    }
}

fn save_file(mut out_file_name:String, file_location:String, resized:image::DynamicImage) {
    // Create output file name and mime.
    out_file_name.push_str("-image-out.");
    let mime = file_location.char_indices().rev().map(|(i, _)| i).nth(2).unwrap();
    let file_type = &file_location[mime..];
    out_file_name.push_str(file_type);
    
    // Save resized image
    &resized.save(out_file_name.as_str());
}

fn main() -> Result<(), Box<Error>> {
    let mut args = env::args().skip(1);

    // Variables
    let mut metrics = ElapsedMetrics::new();
    let mut out_file_name = String::new();
    let file_location = args.next().unwrap();
    
    metrics.start();
    let img = image::open(&file_location)?;
    metrics.set_load_time();
    
    // Do the job
    metrics.start();
    let width = args.next().unwrap().parse()?;
    let height = args.next().unwrap().parse()?;
    let resized = if args.len() == 1 {
        let sample_type = args.next().unwrap();
        out_file_name.push_str("resize-");
        out_file_name.push_str(&sample_type);
        resize(sample_type.as_str(), img, width, height)
    } else {
        out_file_name.push_str("thumb");
        thumbnail("e", img, width, height)
    };
    metrics.set_resize_time();
    
    // Create output file name and mime.
    metrics.start();
    save_file(out_file_name, file_location, resized);
    metrics.set_save_time();
    
    // All was ok
    println!("{:?}", metrics);
    Ok(())
}
