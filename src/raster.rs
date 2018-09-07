extern crate raster; 

use raster::{editor, ResizeMode};
use std::env;
use std::error::Error;
use std::time::SystemTime;

fn main() -> Result<(), Box<Error>> {
    // Args arrangement
    let mut args = env::args().skip(1);
    assert_eq!(args.len(), 3, "Arguments must be: file_location width height");

    // Reading args
    let file_location = args.next().unwrap();
    let width = args.next().unwrap().parse()?;
    let height = args.next().unwrap().parse()?;

    // Do the job
    let mut image = raster::open(file_location.as_str()).unwrap();
    let now = SystemTime::now();
    editor::resize(&mut image, width, height, ResizeMode::Fill).unwrap();
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
    raster::save(&image, out_file_name.as_str()).unwrap();
    
    // All was ok
    Ok(())
}
