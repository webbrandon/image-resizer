extern crate raster; // In your main rust file

use std::env;
use raster::{editor, ResizeMode};



fn main() {
    let mut args = env::args();
    args.next();
    let file_location = args.next().unwrap();
    let width = args.next().unwrap().parse().unwrap();
    let height = args.next().unwrap().parse().unwrap();
    
    println!("Resizing {:?} to {:?} x {:?} pixels.....",file_location, width, height);
    // Create an image from file
    let mut image = raster::open(file_location.as_str()).unwrap();
    
    // Resize it
    editor::resize(&mut image, width, height, ResizeMode::Fill).unwrap();
        
    let mut out_file_name = "out.".to_string();
    println!("Completed resizing.");
    
    let last_two_at = file_location.char_indices().rev().map(|(i, _)| i).nth(2).unwrap();
    let file_type = &file_location[last_two_at..];

    out_file_name.push_str(file_type);
    // Save opened image
    raster::save(&image, out_file_name.as_str()).unwrap();
}
