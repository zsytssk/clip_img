extern crate image;

use super::utils::min;
use image::{open, GenericImageView};

pub fn thumbnail(ori_path: &str, end_path: &str, mut end_width: u32, mut end_height: u32) {
    let img = open(ori_path).unwrap();
    let (width, height) = img.dimensions();

    let ratio: f32 = min(
        (width as f32) / (end_width as f32),
        (end_height as f32) / (height as f32),
    );

    end_width = ((width as f32) * ratio).ceil() as u32;
    end_height = ((height as f32) * ratio).ceil() as u32;

    let thumb_nail = img.thumbnail(end_width, end_height);
    thumb_nail.save(end_path).unwrap();
}
