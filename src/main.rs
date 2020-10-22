extern crate bmp;
use bmp::Image;
mod algorithms;
mod config;
mod file_io;
mod file_processors;

fn main() {
    let configs = config::collect_configs();
    let input_image_path = configs.get("in").unwrap().as_str();
    let output_image_path = configs.get("out").unwrap().as_str();
    if input_image_path.ends_with(".bmp"){
        let mut img: Image = file_io::read_bmp_from_file(input_image_path);
        file_processors::process_bmp(algorithms::cartoonify_v0, &mut img, &configs);
        file_io::write_bmp_to_file(img, output_image_path);
    }else if input_image_path.ends_with(".gif"){
        let mut gif_data = file_io::read_gif_from_file(input_image_path);
        file_processors::process_gif(algorithms::cartoonify_v0, gif_data, &configs);
        //file_io::write_gif_to_file(gif_data, output_image_path);
    }else{
        println!("Input file is not one of the supported types.");
    }
    println!("Conversion Complete");
}
