pub mod clip_img;
pub mod thumbnail;
pub mod utils;

use clip_img::clip_img;
use std::env;
use std::time::Instant;
use thumbnail::thumbnail;

fn main() {
    let now = Instant::now();
    if cfg!(debug_assertions) {
        return;
    }
    let args: Vec<_> = env::args().skip(1).collect();
    let action_type: &str = &args[0];
    let params: Vec<String> = args[1..].to_vec();

    if action_type != "clip_img" && action_type != "thumbnail" {
        args_err_tip();
    }

    if action_type == "clip_img" {
        if params.len() < 4 || params.len() > 5 {
            args_err_tip();
        }
        let ori_file: &str = &params[0];
        let dist_folder: &str = &params[1];
        let clip_width = params[2].parse::<u32>().unwrap();
        let clip_height = params[3].parse::<u32>().unwrap();
        let mut prefix = "";
        if params.len() == 5 {
            prefix = &params[4];
        }
        clip_img(ori_file, dist_folder, clip_width, clip_height, prefix);
        println!("{} complete:> {}", action_type, now.elapsed().as_millis());
        return;
    }
    if params.len() != 4 {
        args_err_tip();
    }
    let ori_path: &str = &params[0];
    let dist_path: &str = &params[1];
    let end_width = params[2].parse::<u32>().unwrap();
    let end_height = params[3].parse::<u32>().unwrap();
    thumbnail(ori_path, dist_path, end_width, end_height);
    println!("{} complete:> {}", action_type, now.elapsed().as_millis());
    return;
}

fn args_err_tip() {
    panic!("clip_img ori_file dist_folder clipWidth clipHeight | thumbnail ori_path dist_path end_width end_height");
}

#[test]
fn test_clip_img() {
    let now = Instant::now();
    clip_img("./test/src/bg.jpg", "./test/dist", 256, 256, "test");
    println!("3:> {}", now.elapsed().as_millis());
}

#[test]
fn test_thumbnail() {
    thumbnail("./test/src/bg.jpg", "./test/dist/thumbnail.jpg", 1228, 636)
}
