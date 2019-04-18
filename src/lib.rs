pub mod clip_img;
pub mod thumbnail;
pub mod utils;

use clip_img::clip_img;
use std::time::Instant;
use thumbnail::thumbnail;

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
