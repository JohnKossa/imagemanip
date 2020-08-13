extern crate bmp;
use bmp::Image;

pub fn do_on_2x2_grid<T>(func: fn(&mut Image, u32, u32) -> T, img: &mut Image) -> Vec<T>{
    let img_width = img.get_width();
    let img_height = img.get_height();
    let mut ret_val: Vec<T> = Vec::new();
    /*if img_width % 2 != 0 {
        panic!("Selected image is not an even width, cannot partition into 2x2 grid.")
    }
    if img_height % 2 != 0 {
        panic!("Selected image is not an even height, cannot partition into a 2x2 grid.")
    }*/
    for x in 0..(img_width/2){
        for y in 0..(img_height/2){
            ret_val.push(func(img, 2*x, 2*y));
        }
    }
    ret_val
}

pub fn convolve_2x2<T>(func: fn(&mut Image, u32, u32) -> T, img: &mut Image) -> Vec<T>{
    let img_width = img.get_width();
    let img_height = img.get_height();
    let mut ret_val: Vec<T> = Vec::new();
    for x in 0..(img_width-1){
        for y in 0..(img_height-1){
            ret_val.push(func(img, x, y));
        }
    }
    ret_val
}