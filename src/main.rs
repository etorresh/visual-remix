use std::fs::File;
use std::io::Write;
use std::path::Path;

// Capture fist frame of the video file
fn main() {
    let file_path = Path::new("resources/video.mp4");
    let mut first_frame = File::create("resources/first_frame.png").unwrap();

    let frame_source = vid2img::FileSource::new(file_path, (500, 1000)).unwrap();
    for frame in frame_source.into_iter() {
        if let Ok(Some(png_img_data)) = frame {
            println!("{}", png_img_data.len());
            first_frame.write_all(&png_img_data).unwrap();
            break;
        }
    }
}
