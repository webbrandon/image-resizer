extern crate raster; 
mod elapsed_metrics;

use elapsed_metrics::ElapsedMetrics;
use raster::{editor, ResizeMode};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    // Args arrangement
    let mut args = env::args().skip(1);
    assert_eq!(args.len(), 3, "Arguments must be: file_location width height");
    let mut metrics = ElapsedMetrics::new();

    // Reading args
    let file_location = args.next().unwrap();
    let width = args.next().unwrap().parse()?;
    let height = args.next().unwrap().parse()?;

    // Do the job
    metrics.start();
    let mut image = raster::open(file_location.as_str()).unwrap();
    metrics.set_load_time();
    
    metrics.start();
    editor::resize(&mut image, width, height, ResizeMode::Fill).unwrap();
    metrics.set_resize_time();
    
    metrics.start();
    // Create output file name and mime.
    let mut out_file_name = "raster-out.".to_string();
    let mime = file_location.char_indices().rev().map(|(i, _)| i).nth(2).unwrap();
    let file_type = &file_location[mime..];
    out_file_name.push_str(file_type);
    // Save resized image
    raster::save(&image, out_file_name.as_str()).unwrap();
    metrics.set_save_time();
    
    // All was ok
    println!("{:?}", metrics);
    Ok(())
}
