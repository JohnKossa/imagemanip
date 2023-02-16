use bmp::{Image, Pixel};

pub fn _dummy(_img: &mut Image, _x: u32, _y: u32){
}

pub struct VolatilityGrid{
    pub x: u32,
    pub y: u32,
    pub avg: Pixel,
    pub volatility: f64,
}

unsafe impl Send for VolatilityGrid {}

pub fn get_volatility_2x2(img: &mut Image, x: u32, y: u32) -> VolatilityGrid{
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
        x,
        y,
        avg: Pixel{ r: avg_r, g: avg_g, b: avg_b },
        volatility,
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
        x,
        y,
        avg: Pixel{ r: avg_r, g: avg_g, b: avg_b },
        volatility,
    }
}