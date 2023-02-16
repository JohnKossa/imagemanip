use bmp::{Image, Pixel};
use std::collections::HashMap;

pub fn color_histogram(img: &Image){
    let histogram = HashMap::new();
    for (x,y) in img.coordinates(){
        let color = img.get_pixel(x, y);
        let count = histogram.entry(color).or_insert(0);
        *count += 1;
    }
    histogram
}
