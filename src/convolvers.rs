extern crate bmp;
extern crate num_cpus;
use bmp::Image;
use rayon::prelude::*;

pub fn do_on_2x2_grid<T>(func: fn(&mut Image, u32, u32) -> T, img: &mut Image) -> Vec<T>{
    sub_grid_size_x(func, 2, img)
}

pub fn sub_grid_size_x<T>(func: fn(&mut Image, u32, u32) -> T, size: u32, img: &mut Image) -> Vec<T>{
    sub_grid_size_x_with_offset(func, size, 0, 0, img)
}

pub fn sub_grid_size_x_with_offset<T>(func: fn(&mut Image, u32, u32) -> T, size: u32, offset_x: u32, offset_y: u32, img: &mut Image) -> Vec<T>{
    let img_width = img.get_width();
    let img_height = img.get_height();
    let mut ret_val: Vec<T> = Vec::new();
    for x in 0..((img_width-offset_x)/size){
        if (size*x)+offset_x > img_width {
            continue;
        }
        for y in 0..((img_height-offset_y)/size){
            if (size*y)+offset_y > img_height {
                continue;
            }
            ret_val.push(func(img, (size*x)+offset_x, (size*y)+offset_y));
        }
    }
    ret_val
}

pub fn readonly_sub_grid_size_x_with_offset<T>(func: fn(&Image, u32, u32) -> T, size: u32, offset_x: u32, offset_y: u32, img: &Image) -> Vec<T>{
    let img_width = img.get_width();
    let img_height = img.get_height();
    let mut ret_val: Vec<T> = Vec::new();
    for x in 0..((img_width-offset_x)/size){
        if (size*x)+offset_x > img_width {
            continue;
        }
        for y in 0..((img_height-offset_y)/size){
            if (size*y)+offset_y > img_height {
                continue;
            }
            ret_val.push(func(img, (size*x)+offset_x, (size*y)+offset_y));
        }
    }
    ret_val
}

pub fn readonly_sub_grid_size_x_with_offset_multi<T: Send>(func: fn(&Image, u32, u32) -> T, size: u32, offset_x: u32, offset_y: u32, img: &Image) -> Vec<T>{
    let img_width = img.get_width();
    let img_height = img.get_height();
    let mut ret_val: Vec<T> = Vec::new();
    let mut coords: Vec<(u32,u32)> = Vec::new();
    for x in 0..((img_width-offset_x)/size){
        if (size*x)+offset_x > img_width {
            continue;
        }
        for y in 0..((img_height-offset_y)/size){
            if (size*y)+offset_y > img_height {
                continue;
            }
            coords.push(((size*x)+offset_x, (size*y)+offset_y));
        }
    }
    coords.into_par_iter().map(|pair: (u32, u32)| -> T {
        let (x,y) = pair;
        func(img, x, y)
    }).collect_into_vec(&mut ret_val);
    ret_val
}

pub fn convolve_2x2<T>(func: fn(&mut Image, u32, u32) -> T, img: &mut Image) -> Vec<T>{
    convolve_size_x(func, 2, img)
}

pub fn convolve_size_x<T>(func: fn(&mut Image, u32, u32) -> T, mut size: u32, img: &mut Image) -> Vec<T>{
    let img_width = img.get_width();
    let img_height = img.get_height();
    let mut ret_val: Vec<T> = Vec::new();
    if size < 1 {
        size = 1;
    }
    for x in 0..(img_width-(size-1)){
        for y in 0..(img_height-(size-1)){
            ret_val.push(func(img, x, y));
        }
    }
    ret_val
}

pub fn readonly_convolve_size_x<T>(func: fn(&Image, u32, u32) -> T, size: u32, img: &Image) -> Vec<T>{
    let img_width = img.get_width();
    let img_height = img.get_height();
    let mut ret_val: Vec<T> = Vec::new();
    for x in 0..(img_width-(size-1)){
        for y in 0..(img_height-(size-1)){
            ret_val.push(func(img, x, y));
        }
    }
    ret_val
}

pub fn readonly_convolve_size_x_multi<T: Send>(func: fn(&Image, u32, u32) -> T, size: u32, img: &Image) -> Vec<T>{
    let img_width = img.get_width();
    let img_height = img.get_height();
    let mut ret_val: Vec<T> = Vec::new();
    let mut coords: Vec<(u32,u32)> = Vec::new();
    for x in 0..(img_width-(size-1)){
        for y in 0..(img_height-(size-1)){
            coords.push((x,y));
        }
    }
    coords.into_par_iter().map(|pair: (u32, u32)| -> T {
        let (x,y) = pair;
        func(img, x, y)
    }).collect_into_vec(&mut ret_val);
    ret_val
}