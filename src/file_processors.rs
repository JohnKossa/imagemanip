use bmp::Image;
use std::collections::HashMap;

pub fn process_bmp(func: fn(&mut Image, &HashMap<String, String>), img: &mut Image, settings: &HashMap<String, String>){
    func(img,settings)
}

pub fn process_gif(func: fn(&mut Image, &HashMap<String, String>), gif_frames: Vec<Image>, settings: &HashMap<String, String>){
    //for each gif frame, invoke func on it
    //reassemble the gif, with timings, etc
    //call write?
}