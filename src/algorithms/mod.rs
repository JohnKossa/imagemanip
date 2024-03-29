use bmp::{Image, Pixel};
use std::cmp::Ordering;
use std::collections::HashMap;

mod convolvers;
mod convolution_fns;

fn volatility_compare(v1: &convolution_fns::VolatilityGrid, v2: &convolution_fns::VolatilityGrid) -> Ordering{
    match v1.volatility.partial_cmp(&v2.volatility) {
        Some(ordering) => ordering,
        None => {
            v1.volatility.to_bits().cmp(&v2.volatility.to_bits())
        }
    }
}

fn flatten_2x2(img: &mut Image, x: u32, y: u32, color: Pixel){
    img.set_pixel(x, y, color);
    img.set_pixel(x+1, y, color);
    img.set_pixel(x, y+1, color);
    img.set_pixel(x+1, y+1, color);
}

fn modal_or_median_flatten_2x2(img: &mut Image, x: u32, y: u32, color: Pixel){
    //check all pixels, if any two have the exact same color, flatten to that color instead of provided one
    //flatten to the provided mean otherwise
    let pixels = vec![img.get_pixel(x, y), img.get_pixel(x+1, y), img.get_pixel(x, y+1), img.get_pixel(x+1, y+1)];
    let mut counts = vec![1,1,1,1];
    for i in 0..pixels.len(){
        for idx in (i+1)..pixels.len(){
            if pixels.get(i) == pixels.get(idx){
                counts[i]+=1;
            }
        }
    }
    let flatten_value = match counts.iter().max(){
        None          => panic!("Attempted to get max of an empty Vec"),
        Some(1)       => color,
        Some(max_val) => match counts.iter().filter(|x| x == &max_val).count(){
            0 => panic!("This shouldn't be able to happen"),
            1 => *pixels.get(counts.iter().position(|r| r == max_val).unwrap()).unwrap(),
            _ => color
        }
    };
    img.set_pixel(x, y, flatten_value);
    img.set_pixel(x+1, y, flatten_value);
    img.set_pixel(x, y+1, flatten_value);
    img.set_pixel(x+1, y+1, flatten_value);
}

fn modal_flatten_2x2(img: &mut Image, x: u32, y: u32, color: Pixel){
    //check all pixels, if any two have the exact same color, flatten to that color instead of provided one
    //flatten to the provided mean otherwise
    let pixels = vec![img.get_pixel(x, y), img.get_pixel(x+1, y), img.get_pixel(x, y+1), img.get_pixel(x+1, y+1)];
    let mut counts = vec![1,1,1,1];
    for i in 0..pixels.len(){
        for idx in (i+1)..pixels.len(){
            if pixels.get(i) == pixels.get(idx){
                counts[i]+=1;
            }
        }
    }
    let flatten_value = match counts.iter().max(){
        None          => panic!("Attempted to get max of an empty Vec"),
        Some(1)       => Some(color),
        Some(max_val) => match counts.iter().filter(|x| x == &max_val).count(){
            0 => panic!("This shouldn't be able to happen"),
            1 => Some(*pixels.get(counts.iter().position(|r| r == max_val).unwrap()).unwrap()),
            _ => None
        }
    };
    match flatten_value{
        Some(val) => {
            img.set_pixel(x, y, val);
            img.set_pixel(x+1, y, val);
            img.set_pixel(x, y+1, val);
            img.set_pixel(x+1, y+1, val);
        },
        None => {}
    }
}

fn median_flatten_2x2(img: &mut Image, x: u32, y: u32){
    //check all pixels, if any two have the exact same color, flatten to that color instead of provided one
    //flatten to the provided mean otherwise
    let pixels = vec![img.get_pixel(x, y), img.get_pixel(x+1, y), img.get_pixel(x, y+1), img.get_pixel(x+1, y+1)];
    let avg_r: u8 = ((pixels[1].r as u16 + pixels[2].r as u16 )/2) as u8;
    let avg_g: u8 = ((pixels[1].g as u16 + pixels[2].g as u16 )/2) as u8;
    let avg_b: u8 = ((pixels[1].b as u16 + pixels[2].b as u16 )/2) as u8;
    let flatten_value = Pixel {r: avg_r, g: avg_g, b: avg_b};
    img.set_pixel(x, y, flatten_value);
    img.set_pixel(x+1, y, flatten_value);
    img.set_pixel(x, y+1, flatten_value);
    img.set_pixel(x+1, y+1, flatten_value);
}

fn geomean_flatten_2x2(img: &mut Image, x: u32, y: u32){
    //check all pixels, if any two have the exact same color, flatten to that color instead of provided one
    //flatten to the provided mean otherwise
    let pixels = vec![img.get_pixel(x, y), img.get_pixel(x+1, y), img.get_pixel(x, y+1), img.get_pixel(x+1, y+1)];
    let avg_r: u8 = (pixels[0].r as f64 * pixels[1].r as f64 * pixels[2].r as f64 * pixels[3].r as f64).sqrt().sqrt() as u8;
    let avg_g: u8 = (pixels[0].g as f64 * pixels[1].g as f64 * pixels[2].g as f64 * pixels[3].g as f64).sqrt().sqrt() as u8;
    let avg_b: u8 = (pixels[0].b as f64 * pixels[1].b as f64 * pixels[2].b as f64 * pixels[3].b as f64).sqrt().sqrt() as u8;
    let flatten_value = Pixel {r: avg_r, g: avg_g, b: avg_b};
    img.set_pixel(x, y, flatten_value);
    img.set_pixel(x+1, y, flatten_value);
    img.set_pixel(x, y+1, flatten_value);
    img.set_pixel(x+1, y+1, flatten_value);
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
    let mut volatilities: Vec<convolution_fns::VolatilityGrid> = convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 0, 0, img); //in 2x2 grid, collect volatilities
    volatilities.sort_by(volatility_compare); //sort by volatility
    volatilities = volatilities.into_iter().filter(|v| v.volatility > 0.0).collect();//drop all volatilities of 0
    let mut cutoff_idx = (volatilities.len() as f64 * cutoff_pct) as i32;
    for i in 0..cutoff_idx {
        flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y, volatilities[i as usize].avg); //set all pixels in that grid to the average
    }
    for _ in 0..passes {
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
        Some(result) => result.parse::<usize>().unwrap(),
        None         => 20
    };
    let cutoff_pct = match settings.get("pct"){
        Some(result) => result.parse::<f64>().unwrap(),
        None         => 0.85
    };
    let mut volatilities: Vec<convolution_fns::VolatilityGrid> = convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 0, 0, img); //in 2x2 grid, collect volatilities
    volatilities.sort_by(volatility_compare); //sort by volatility
    volatilities = volatilities.into_iter().filter(|v| v.volatility > 0.0).collect();//drop all volatilities of 0
    let mut cutoff_idx = (volatilities.len() as f64 * cutoff_pct) as usize;
    for i in 0..cutoff_idx {
        flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y, volatilities[i as usize].avg); //set all pixels in that grid to the average
    }
    for i in 0..passes {
        volatilities = match i % 4 {
            0 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 0, 0, img),
            1 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 1, 1, img),
            2 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 1, 0, img),
            3 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 0, 1, img),
            _ => panic!()
        };
        volatilities.sort_by(volatility_compare); //sort by volatility
        volatilities = volatilities.into_iter().filter(|v| v.volatility > 0.0).collect();//drop all volatilities of 0
        cutoff_idx = (volatilities.len() as f64 * cutoff_pct) as usize;
        for i in 0..cutoff_idx {
            flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y, volatilities[i as usize].avg); //set all pixels in that grid to the average
        }
    }
}

pub fn cartoonify_v1_mode(img: &mut Image, settings: &HashMap<String, String>){
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
        modal_flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y, volatilities[i as usize].avg); //set all pixels in that grid to the average
    }
    for i in 0..passes {
        volatilities = match i % 4 {
            0 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 0, 0, img),
            1 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 1, 1, img),
            2 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 1, 0, img),
            3 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 0, 1, img),
            _ => unreachable!()
        };
        volatilities.sort_by(volatility_compare); //sort by volatility
        volatilities = volatilities.into_iter().filter(|v| v.volatility > 0.0).collect();//drop all volatilities of 0
        cutoff_idx = (volatilities.len() as f64 * cutoff_pct) as i32;
        for i in 0..cutoff_idx {
            modal_flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y, volatilities[i as usize].avg); //set all pixels in that grid to the average
        }
    }
}

pub fn cartoonify_v1_median(img: &mut Image, settings: &HashMap<String, String>){
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
        median_flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y); //set all pixels in that grid to the average
    }
    for i in 0..passes {
        volatilities = match i % 4 {
            0 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 0, 0, img),
            1 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 1, 1, img),
            2 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 1, 0, img),
            3 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 0, 1, img),
            _ => panic!()
        };
        volatilities.sort_by(volatility_compare); //sort by volatility
        volatilities = volatilities.into_iter().filter(|v| v.volatility > 0.0).collect();//drop all volatilities of 0
        cutoff_idx = (volatilities.len() as f64 * cutoff_pct) as i32;
        for i in 0..cutoff_idx {
            median_flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y); //set all pixels in that grid to the average
        }
    }
}

pub fn cartoonify_v1_geomean(img: &mut Image, settings: &HashMap<String, String>){
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
        geomean_flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y); //set all pixels in that grid to the average
    }
    for i in 0..passes {
        volatilities = match i % 4 {
            0 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 0, 0, img),
            1 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 1, 1, img),
            2 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 1, 0, img),
            3 => convolvers::readonly_sub_grid_size_x_with_offset(convolution_fns::readonly_get_volatility_2x2, 2, 0, 1, img),
            _ => panic!()
        };
        volatilities.sort_by(volatility_compare); //sort by volatility
        volatilities = volatilities.into_iter().filter(|v| v.volatility > 0.0).collect();//drop all volatilities of 0
        cutoff_idx = (volatilities.len() as f64 * cutoff_pct) as i32;
        for i in 0..cutoff_idx {
            geomean_flatten_2x2(img, volatilities[i as usize].x, volatilities[i as usize].y); //set all pixels in that grid to the average
        }
    }
}

pub fn res_drop(img: &mut Image, x: u32, y: u32, cuts: u8){
    let color = img.get_pixel(x,y);
    let mask: u8 = 0b11111111 << cuts;
    img.set_pixel(x, y, Pixel::new(color.r & mask, color.g & mask, color.b & mask));
}

pub fn color_res_drop(img: &mut Image, settings: &HashMap<String, String>){
    let cuts = match settings.get("passes"){
        Some(result) => result.parse::<u8>().unwrap(),
        None         => 6
    };
    for (x,y) in img.coordinates(){
        res_drop(img, x, y, cuts);
    }
}