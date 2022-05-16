use crate::video_api::VideoApi;
use std::fs::File;

mod handle_file;
mod request_handler;
mod utils;
mod video_api;

#[tokio::main]
async fn main() {
    const TOKEN: &str = "your token here";
    let person_id = " your person id here ".to_owned();
    let base_url = "https://api.linkedin.com/v2/videos/".to_owned();
    let file = File::open("path to your video.mp4").unwrap();

    let res = VideoApi::new(base_url, person_id, TOKEN.to_owned());

    let results = res
        .video_upload_handeler(file, None, None, "FEED_VIDEO".to_owned())
        .await;
    if results.is_ok() {
        println!("result : {:?}", results.unwrap())
    }
}
