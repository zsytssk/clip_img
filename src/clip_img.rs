extern crate image;

use image::{open, GenericImageView, ImageBuffer};
use std::thread;

pub fn clip_img(ori_path: &str, end_folder: &str, clip_width: u32, clip_height: u32, prefix: &str) {
    let img = open(ori_path).unwrap();
    let (width, height) = img.dimensions();

    let wn = (width as f32 / clip_width as f32).ceil() as u32;
    let hn = (height as f32 / clip_height as f32).ceil() as u32;

    let mut n = 0;
    let all_n = wn * hn;
    println!(
        "{}:>w:{}, h:{}, split to {} files",
        ori_path, width, height, all_n
    );
    let mut handles = vec![];
    for j in 0..hn {
        for i in 0..wn {
            n = n + 1;
            let start_x = i * clip_width;
            let start_y = j * clip_height;
            let mut img_buf = ImageBuffer::new(clip_width, clip_height);
            for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
                if x + start_x >= width {
                    continue;
                }
                if y + start_y >= height {
                    continue;
                }
                *pixel = img.get_pixel(x + start_x, y + start_y);
            }
            let dist_file = format!("{}_{}.jpg", prefix, n);
            let path = format!("{}/{}", end_folder, dist_file);

            let handle = thread::spawn(move || {
                img_buf.save(&path).unwrap();
                println!("{}:>{}%", dist_file, n * 100 / all_n);
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
