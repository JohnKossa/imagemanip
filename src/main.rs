use bmp::Image;
use std::time::Instant;
mod algorithms;
mod config;
mod file_io;
mod file_processors;


fn main() {
    let configs = config::collect_configs();
    let input_image_path = configs.get("in").unwrap().as_str();
    let output_image_path = configs.get("out").unwrap().as_str();
    let algorithm = match configs.get("alg") {
        Some(val) => match val.as_str() {
            "v0" => algorithms::cartoonify_v0,
            "v1" => algorithms::cartoonify_v1,
            _    => panic!("Specified algorithm is not supported")
        },
        None      => algorithms::cartoonify_v0
    };
    let now = Instant::now();
    if input_image_path.ends_with(".bmp"){
        let mut img: Image = file_io::read_bmp_from_file(input_image_path);
        file_processors::process_bmp(algorithm, &mut img, &configs);
        file_io::write_bmp_to_file(img, output_image_path);
    }else if input_image_path.ends_with(".gif"){
        let mut gif_data = file_io::read_gif_from_file(input_image_path);
        file_processors::process_gif(algorithm, &mut gif_data, &configs);
        //file_io::write_gif_to_file(gif_data, output_image_path);
    }else{
        println!("Input file is not one of the supported types.");
    }
    println!("{} seconds", now.elapsed().as_secs());
    println!("Conversion Complete");
}
