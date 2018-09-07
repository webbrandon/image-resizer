extern crate image;

use std::env;
use std::error::Error;
use std::time::SystemTime;

fn main() -> Result<(), Box<Error>> {
    // Args arrangement
    let mut args = env::args().skip(1);
    //assert_eq!(args.len(), 3, "Arguments must be: file_location width height");
    
    // Reading args
    let file_location = args.next().unwrap();
    let width = args.next().unwrap().parse()?;
    let height = args.next().unwrap().parse()?;
    let thumbnail:bool = if args.len() == 1 {
        false
    } else {
        true
    };
    // Do the job
    let img = image::open(&file_location)?;
    let mut resized:image::DynamicImage = image::DynamicImage::new_rgba8(0, 0);
    let now = SystemTime::now();
    if thumbnail {
        println!("Resizing as thumbnail");
        resized = img.thumbnail_exact(width, height);
    } else {
        let sample_type = args.next().unwrap();
        match sample_type.as_str() {
            "l" => {
                println!("Resizing with Lanczos3");
                resized = img.resize(width, height, image::imageops::Lanczos3)
            },
            "g" => {
                println!("Resizing with Gaussian");
                resized = img.resize(width, height, image::imageops::Gaussian)
            },
            "c" => {
                println!("Resizing with CatmullRom");
                resized = img.resize(width, height, image::imageops::CatmullRom)
            },
            "n" => {
                println!("Resizing with Nearest");
                resized = img.resize(width, height, image::imageops::Nearest)
            },
            "t" => {
                println!("Resizing with Triangle");
                resized = img.resize(width, height, image::imageops::Triangle)
            },
            _ => ()
        }
    }
    if let Ok(elapsed) = now.elapsed() {
        println!(
            "resizing: {}.{:03}",
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    }
    
    // Create output file name and mime.
    let mut out_file_name = "out.".to_string();
    let mime = file_location.char_indices().rev().map(|(i, _)| i).nth(2).unwrap();
    let file_type = &file_location[mime..];
    out_file_name.push_str(file_type);
    
    // Save resized image
    &resized.save(out_file_name.as_str());
    
    // All was ok
    Ok(())
}
