extern crate image;

use std::env;
use std::error::Error;
use std::time::SystemTime;

fn main() -> Result<(), Box<Error>> {
    // Variables
    let resized:image::DynamicImage;
    let mut out_file_name = String::new();
    
    // Reading args
    let mut args = env::args().skip(1);
    let file_location = args.next().unwrap();
    let width = args.next().unwrap().parse()?;
    let height = args.next().unwrap().parse()?;
    let thumbnail:bool = if args.len() == 1 {
        false
    } else {
        true
    };
    
    // Load the file
    let img = image::open(&file_location)?;
    
    // Do the job
    let now = SystemTime::now();
    if thumbnail {
        println!("Resizing as thumbnail");
        out_file_name.push_str("thumb");
        resized = img.thumbnail_exact(width, height);
    } else {
        let sample_type = args.next().unwrap();
        out_file_name.push_str("resize-");
        out_file_name.push_str(&sample_type);
        resized = match sample_type.as_str() {
            "l" => img.resize(width, height, image::imageops::Lanczos3),
            "g" => img.resize(width, height, image::imageops::Gaussian),
            "c" => img.resize(width, height, image::imageops::CatmullRom),
            "n" => img.resize(width, height, image::imageops::Nearest),
            "t" => img.resize(width, height, image::imageops::Triangle),
            _ => image::DynamicImage::new_rgba8(0, 0)
        };
    }
    if let Ok(elapsed) = now.elapsed() {
        println!(
            "resizing: {}.{:03}",
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    }
    
    // Create output file name and mime.
    out_file_name.push_str("-image-out.");
    let mime = file_location.char_indices().rev().map(|(i, _)| i).nth(2).unwrap();
    let file_type = &file_location[mime..];
    out_file_name.push_str(file_type);
    
    // Save resized image
    &resized.save(out_file_name.as_str());
    
    // All was ok
    Ok(())
}
