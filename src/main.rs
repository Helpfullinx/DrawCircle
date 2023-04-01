#![feature(array_chunks)]

use std::borrow::Borrow;
use itertools::Itertools;
use image;
use image::{ColorType, DynamicImage, EncodableLayout, GenericImage, GenericImageView, Pixel, Rgb, Rgba, RgbaImage};

fn main() {
    let mut img = DynamicImage::new_rgba8(2048, 2048).into_rgba8();

    img.fill(0);

    draw_circle(255,&mut img,255,0,0, 100.0,100.0);
    draw_circle(255,&mut img,0,255,0, -100.0,100.0);
    draw_circle(255,&mut img,0,0,255, 0.0,-100.0);

    img.save("circle.png").unwrap();
}

pub fn draw_circle(radius: i32, img: &mut RgbaImage, r: u8, g: u8, b: u8, offset_x: f32,  offset_y: f32){
    let center = (((img.width() as f32 / 2.0) + offset_x) as i32, ((img.height() as f32 / 2.0) + offset_y) as i32);
    let mut point_vec:Vec<(i32,i32)> = vec![];
    let mut fill_vec:Vec<(i32, i32)> = vec![];

    for x in (center.0 - radius)..(center.0 + radius){
        let a = x - center.0;
        let z = ((radius.pow(2) as i32) - a.pow(2)) as i32;
        let f = (z as f64).sqrt();

        let mut coord:(i32,i32) = (x, center.1 + (f as i32));
        point_vec.push(coord);
        coord = (x, center.1 - (f as i32));
        point_vec.push(coord);
    }

    for (prev,next) in point_vec.into_iter().tuples() {
        let mut i:i32 = prev.1;
        while i > next.1 {
            fill_vec.push((prev.0, i));
            i -= 1;
        }
    }

    let width = img.width();
    let mut pixels= img.pixels_mut().collect_vec();

    for x in fill_vec {
        let location = (x.1 * width as i32)+ x.0;
        let mut pixel = &mut pixels[location as usize];

        let red_clamp = ((pixel.0[0] as u32) + r as u32 ).clamp(0,255);
        let green_clamp = ((pixel.0[1] as u32) + g as u32 ).clamp(0,255);
        let blue_clamp = ((pixel.0[2] as u32) + b as u32 ).clamp(0,255);

        pixel.0[0] = red_clamp as u8;
        pixel.0[1] = green_clamp as u8;
        pixel.0[2] = blue_clamp as u8;
        pixel.0[3] = 255;
    }
}