extern crate lib;

use lib::{clip_img::clip_img, thumbnail::thumbnail};
use std::env;

fn main() {
    if cfg!(debug_assertions) {
        return;
    }
    let args: Vec<_> = env::args().collect();
    let action_type: &str = &args[1][..];
    if action_type == "clip_img" {
        if args.len() < 6 || args.len() > 7 {
            args_err_tip();
        }
        let ori_file: &str = &args[2][..];
        let dist_folder: &str = &args[3][..];
        let clip_width = args[4].parse::<u32>().unwrap();
        let clip_height = args[5].parse::<u32>().unwrap();
        let mut prefix = "";
        if args.len() == 7 {
            prefix = &args[6][..];
        }
        clip_img(ori_file, dist_folder, clip_width, clip_height, prefix);
        return;
    } else if action_type == "thumbnail" {
        if args.len() != 6 {
            args_err_tip();
        }
        let ori_path: &str = &args[2][..];
        let dist_path: &str = &args[3][..];
        let end_width = args[4].parse::<u32>().unwrap();
        let end_height = args[5].parse::<u32>().unwrap();
        thumbnail(ori_path, dist_path, end_width, end_height);
        return;
    }
    args_err_tip();
}

fn args_err_tip() {
    panic!("clip_img ori_file dist_folder clipWidth clipHeight | thumbnail ori_path dist_path end_width end_height");
}
