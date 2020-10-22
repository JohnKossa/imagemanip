extern crate bmp;
extern crate num_cpus;
use bmp::{Image, Pixel};
use std::cmp::Ordering;
use std::collections::HashMap;
#[path="convolvers.rs"]
mod convolvers;
#[path="convolution_fns.rs"]
mod convolution_fns;

fn volatility_compare(v1: &convolution_fns::VolatilityGrid, v2: &convolution_fns::VolatilityGrid) -> Ordering{
    if v1.volatility < v2.volatility {
        return Ordering::Less
    } else if v1.volatility > v2.volatility {
        return Ordering::Greater
    }
    return Ordering::Equal
}

fn flatten_2x2(img: &mut Image, x: u32, y: u32, color: Pixel){
    img.set_pixel(x, y, color);
    img.set_pixel(x+1, y, color);
    img.set_pixel(x, y+1, color);
    img.set_pixel(x+1, y+1, color);
}

pub fn cartoonify_v0(img: &mut Image, settings: &HashMap<String, String>){
    let passes = match settings.get("passes"){
        Some(result) => result.parse::<u32>().unwrap(),
        None         => 20
    };
    let cutoff_pct = match settings.get("pct"){
        Some(result) => result.parse::<f64>().unwrap(),
        None         => 0.85
    };
    let mut volatilities: Vec<convolution_fns::VolatilityGrid> = convolvers::do_on_2x2_grid(convolution_fns::get_volatility_2x2, img); //in 2x2 grid, collect volatilities
    volatilities.sort_by(volatility_compare); //sort by volatility
    volatilities = volatilities.into_iter().filter(|v| v.volatility > 0.0).collect();//drop all volatilities of 0
    let mut cutoff_idx = (volatilities.len() as f64 * cutoff_pct) as i32;
    for i in 0..cutoff_idx {
        flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y, volatilities[i as usize].avg); //set all pixels in that grid to the average
    }
    for _ in 0..passes {
        volatilities = convolvers::convolve_2x2(convolution_fns::get_volatility_2x2, img);
        volatilities.sort_by(volatility_compare); //sort by volatility
        volatilities = volatilities.into_iter().filter(|v| v.volatility > 0.0).collect();//drop all volatilities of 0
        cutoff_idx = (volatilities.len() as f64 * cutoff_pct) as i32;
        for i in 0..cutoff_idx {
            flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y, volatilities[i as usize].avg); //set all pixels in that grid to the average
        }
    }
}

pub fn cartoonify_v01(img: &mut Image, settings: &HashMap<String, String>){
    let passes = match settings.get("passes"){
        Some(result) => result.parse::<u32>().unwrap(),
        None         => 20
    };
    let cutoff_pct = match settings.get("pct"){
        Some(result) => result.parse::<f64>().unwrap(),
        None         => 0.85
    };
    let mut volatilities: Vec<convolution_fns::VolatilityGrid> = convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 0, 0, img); //in 2x2 grid, collect volatilities
    volatilities.sort_by(volatility_compare); //sort by volatility
    volatilities = volatilities.into_iter().filter(|v| v.volatility > 0.0).collect();//drop all volatilities of 0
    let mut cutoff_idx = (volatilities.len() as f64 * cutoff_pct) as i32;
    for i in 0..cutoff_idx {
        flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y, volatilities[i as usize].avg); //set all pixels in that grid to the average
    }
    for _ in 0..passes {
        //volatilities = convolvers::convolve_2x2(convolution_fns::get_volatility_2x2, img);
        volatilities = convolvers::readonly_convolve_size_x(convolution_fns::readonly_get_volatility_2x2, 2, img);
        volatilities.sort_by(volatility_compare); //sort by volatility
        volatilities = volatilities.into_iter().filter(|v| v.volatility > 0.0).collect();//drop all volatilities of 0
        cutoff_idx = (volatilities.len() as f64 * cutoff_pct) as i32;
        for i in 0..cutoff_idx {
            flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y, volatilities[i as usize].avg); //set all pixels in that grid to the average
        }
    }
}

pub fn cartoonify_v1(img: &mut Image, settings: &HashMap<String, String>){
    let passes = match settings.get("passes"){
        Some(result) => result.parse::<u32>().unwrap(),
        None         => 20
    };
    let cutoff_pct = match settings.get("pct"){
        Some(result) => result.parse::<f64>().unwrap(),
        None         => 0.85
    };
    let mut volatilities: Vec<convolution_fns::VolatilityGrid> = convolvers::sub_grid_size_x_with_offset(convolution_fns::get_volatility_2x2, 2, 0, 0, img); //in 2x2 grid, collect volatilities
    volatilities.sort_by(volatility_compare); //sort by volatility
    volatilities = volatilities.into_iter().filter(|v| v.volatility > 0.0).collect();//drop all volatilities of 0
    let mut cutoff_idx = (volatilities.len() as f64 * cutoff_pct) as i32;
    for i in 0..cutoff_idx {
        flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y, volatilities[i as usize].avg); //set all pixels in that grid to the average
    }
    for _ in 0..passes {
        //volatilities = convolvers::sub_grid_size_x_with_offset(convolution_fns::get_volatility_2x2, 2, offsets[(n%4) as usize][0], offsets[(n%4) as usize][1], img);
        volatilities = convolvers::sub_grid_size_x_with_offset(convolution_fns::get_volatility_2x2, 2, 0, 0, img);
        volatilities.append(&mut convolvers::sub_grid_size_x_with_offset(convolution_fns::get_volatility_2x2, 2, 0, 1, img));
        volatilities.append(&mut convolvers::sub_grid_size_x_with_offset(convolution_fns::get_volatility_2x2, 2, 1, 1, img));
        volatilities.append(&mut convolvers::sub_grid_size_x_with_offset(convolution_fns::get_volatility_2x2, 2, 1, 0, img));
        volatilities.sort_by(volatility_compare); //sort by volatility
        volatilities = volatilities.into_iter().filter(|v| v.volatility > 0.0).collect();//drop all volatilities of 0
        cutoff_idx = (volatilities.len() as f64 * cutoff_pct) as i32;
        for i in 0..cutoff_idx {
            flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y, volatilities[i as usize].avg); //set all pixels in that grid to the average
        }
    }
}
