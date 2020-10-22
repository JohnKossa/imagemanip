extern crate bmp;
use bmp::{Image, Pixel};

pub fn _dummy(_img: &mut Image, _x: u32, _y: u32){
}

pub struct VolatilityGrid{
    pub x: u32,
    pub y: u32,
    pub avg: Pixel,
    pub volatility: f64,
}

pub fn get_volatility_2x2(img: &mut Image, x: u32, y: u32) -> VolatilityGrid{
    /*let p1 = img.get_pixel(x, y);
    let p2 = img.get_pixel(x+1, y);
    let p3 = img.get_pixel(x, y+1);
    let p4 = img.get_pixel(x+1, y+1);
    let avg_r = ((p1.r as u16 + p2.r as u16 + p3.r as u16 + p4.r as u16)/4) as u8;
    let avg_g = ((p1.g as u16 + p2.g as u16 + p3.g as u16 + p4.g as u16)/4) as u8;
    let avg_b = ((p1.b as u16 + p2.b as u16 + p3.b as u16 + p4.b as u16)/4) as u8;
    let v1 = (((avg_r as i32 - p1.r as i32).pow(2) + (avg_g as i32 - p1.g as i32).pow(2) + (avg_b as i32 - p1.b as i32).pow(2)) as f64).sqrt();
    let v2 = (((avg_r as i32 - p2.r as i32).pow(2) + (avg_g as i32 - p2.g as i32).pow(2) + (avg_b as i32 - p2.b as i32).pow(2)) as f64).sqrt();
    let v3 = (((avg_r as i32 - p3.r as i32).pow(2) + (avg_g as i32 - p3.g as i32).pow(2) + (avg_b as i32 - p3.b as i32).pow(2)) as f64).sqrt();
    let v4 = (((avg_r as i32 - p4.r as i32).pow(2) + (avg_g as i32 - p4.g as i32).pow(2) + (avg_b as i32 - p4.b as i32).pow(2)) as f64).sqrt();
    VolatilityGrid {
        x: x,
        y: y,
        avg: Pixel{ r: avg_r, g: avg_g, b: avg_b },
        volatility: v1+v2+v3+v4,
    }*/
    get_volatility(img, 2, x, y)
}

pub fn get_volatility(img: &mut Image, size: u32, x: u32, y: u32) -> VolatilityGrid{
    let mut pixels: Vec<Pixel> = Vec::new();
    let width = img.get_width();
    let height = img.get_height();
    for grid_x in 0..size {
        if (x+grid_x) > width{
            println!("Overran size x {}", x+grid_x);
            continue;
        }
        for grid_y in 0..size {
            if (y+grid_y) > height{
                println!("Overran size y {}", y+grid_y);
                continue;
            }
            pixels.push(img.get_pixel(x+grid_x, y+grid_y));
        }
    }
    let pixel_count: u16 = (size*size) as u16;
    let avg_r: u8 = (pixels.iter().fold(0, |acc, elem| acc+elem.r as u16)/pixel_count) as u8;
    let avg_g: u8 = (pixels.iter().fold(0, |acc, elem| acc+elem.g as u16)/pixel_count) as u8;
    let avg_b: u8 = (pixels.iter().fold(0, |acc, elem| acc+elem.b as u16)/pixel_count) as u8;
    let volatility: f64 = pixels.iter().fold(0.0, |acc, elem| acc + (((avg_r as i32 - elem.r as i32).pow(2) + (avg_g as i32 - elem.g as i32).pow(2) + (avg_b as i32 - elem.b as i32).pow(2)) as f64).sqrt());
    VolatilityGrid {
        x: x,
        y: y,
        avg: Pixel{ r: avg_r, g: avg_g, b: avg_b },
        volatility: volatility,
    }
}

pub fn readonly_get_volatility_2x2(img: &Image, x: u32, y: u32) -> VolatilityGrid {
    readonly_get_volatility(img, 2, x, y)
}

pub fn readonly_get_volatility(img: &Image,size: u32, x: u32, y: u32) -> VolatilityGrid {
    let mut pixels: Vec<Pixel> = Vec::new();
    let width = img.get_width();
    let height = img.get_height();
    for grid_x in 0..size {
        if (x+grid_x) > width{
            println!("Overran size x {}", x+grid_x);
            continue;
        }
        for grid_y in 0..size {
            if (y+grid_y) > height{
                println!("Overran size y {}", y+grid_y);
                continue;
            }
            pixels.push(img.get_pixel(x+grid_x, y+grid_y));
        }
    }
    let pixel_count: u16 = (size*size) as u16;
    let avg_r: u8 = (pixels.iter().fold(0, |acc, elem| acc+elem.r as u16)/pixel_count) as u8;
    let avg_g: u8 = (pixels.iter().fold(0, |acc, elem| acc+elem.g as u16)/pixel_count) as u8;
    let avg_b: u8 = (pixels.iter().fold(0, |acc, elem| acc+elem.b as u16)/pixel_count) as u8;
    let volatility: f64 = pixels.iter().fold(0.0, |acc, elem| acc + (((avg_r as i32 - elem.r as i32).pow(2) + (avg_g as i32 - elem.g as i32).pow(2) + (avg_b as i32 - elem.b as i32).pow(2)) as f64).sqrt());
    VolatilityGrid {
        x: x,
        y: y,
        avg: Pixel{ r: avg_r, g: avg_g, b: avg_b },
        volatility: volatility,
    }
}