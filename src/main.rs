// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html

use std::fs::File;
use std::io::Write;
use std::path::Path;

// Capture fist frame of the video file
fn main() {
    // Green screen rgb: (114, 251, 2)
    let file_path = Path::new("resources/greenscreen.mp4");
    // let mut first_frame = File::create("resources/first_frame.png").unwrap();

    let frame_source = vid2img::FileSource::new(file_path, (500, 500)).unwrap();
    let mut x = 0;
    for frame in frame_source.into_iter() {
        if let Ok(Some(png_img_data)) = frame {
            if x == 1 {
                let mut y = 0;
                for pixel in png_img_data {
                    println!("{}", pixel);
                    y += 1;
                    if y == 10 {break};
                }
                break
            }
            x += 1;
        }
    }
}
