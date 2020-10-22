extern crate bmp;
extern crate gif;
use bmp::Image;
use bmp::Pixel;
use std::fs::File;

pub fn read_bmp_from_file(path: &str) -> Image {
    let res = bmp::open(path);
    match res {
        Ok(val) => return val,
        _       => panic!("Image file could not be read.")
    }
}

pub fn write_bmp_to_file(img: Image, path:  &str) {
    let _ = img.save(path);
}

pub fn read_gif_from_file(path: &str) -> Vec<Image> {
    let mut decoder_options = gif::DecodeOptions::new();
    //expand image to rgba
    decoder_options.set_color_output(gif::ColorOutput::RGBA);
    let file = File::open(path).unwrap();
    let mut decoder = decoder_options.read_info(file).unwrap();
    let mut output = Vec::new();
    while let Some(frame) = decoder.read_next_frame().unwrap(){
        //process the frame
        let mut to_add = Image::new(frame.width.into(), frame.height.into());
        let buffer = &frame.buffer;
        for (x, y) in to_add.coordinates(){
            let current_pixel_bytes = buffer[(x*y+x) as usize];
            to_add.set_pixel(x, y, Pixel::new(current_pixel_bytes & 0b11000000 >>6,current_pixel_bytes & 0b00110000 >>4,current_pixel_bytes & 0b00001100 >>2));
        }
        output.push(to_add);
    }
    output
}

pub fn write_gif_to_file(images: Vec<Image>, path: &str) {
    let mut frames: Vec<gif::Frame> = Vec::new();
    let gif_width: u16 = images[0].get_width() as u16;
    let gif_height: u16 = images[0].get_height() as u16;
    let pixel_count = gif_width*gif_height;
    for image in images{
        let mut buffer: Vec<u8> = vec![0; pixel_count as usize];
        for (x, y) in image.coordinates(){
            let pixel = image.get_pixel(x, y);
            buffer[(x*y+x) as usize] = (pixel.r << 6 & 0b11000000) | (pixel.g << 4 & 0b00110000) | (pixel.b << 2 & 0b00001100)
        }
        frames.push(gif::Frame::from_rgb(gif_width, gif_height, &mut *buffer))
    }
    let mut image_file = File::create(path).unwrap();
    let mut encoder = gif::Encoder::new(&mut image_file, gif_width, gif_height, &[]).unwrap();
    for frame in frames{
        encoder.write_frame(&frame).unwrap();
    }
}