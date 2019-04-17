extern crate image;

use image::{open, GenericImageView, ImageBuffer};

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
            img_buf.save(&path).unwrap();
            println!("{}:>{}%", dist_file, n * 100 / all_n);
        }
    }
}

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

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

#[test]
fn test_thumbnail() {
    thumbnail("./test/src/bg.jpg", "./test/dist/thumbnail.jpg", 1228, 636)
}
