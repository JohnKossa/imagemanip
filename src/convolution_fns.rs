extern crate bmp;
use bmp::{Image, Pixel};

pub fn dummy(_img: &mut Image, _x: u32, _y: u32){
}

pub struct VolatilityGrid{
    pub x: u32,
    pub y: u32,
    pub avg: Pixel,
    pub volatility: f64,
}

pub fn get_volatility_2x2(img: &mut Image, x: u32, y: u32) -> VolatilityGrid{
    let p1 = img.get_pixel(x, y);
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
    }
}