extern crate bmp;
use bmp::Image;
use std::env;
mod file_io;
mod algorithms;
mod config;

fn main() {
    let configs = config::collect_configs();
    let input_image_path = configs.get("in").unwrap().as_str();
    let output_image_path = configs.get("out").unwrap().as_str();
    let passes = configs.get("passes").unwrap().parse::<u32>().unwrap();
    let pct = configs.get("pct").unwrap().parse::<f64>().unwrap();
    let mut img: Image = file_io::read_from_file(input_image_path);
    algorithms::cartoonify_v0(&mut img, pct, passes);
    file_io::write_to_file(img, output_image_path);
    println!("Hello, world!");
}
