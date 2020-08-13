extern crate bmp;
use bmp::Image;

pub fn read_from_file(path: &str) -> Image {
    let res = bmp::open(path);
    match res {
        Ok(val) => return val,
        _       => panic!("Image file could not be read.")
    }
}

pub fn write_to_file(img: Image, path:  &str) {
    let _ = img.save(path);
}
