use std::path::Path;
use png::{Decoder};
use std::io::Cursor;

// Capture fist frame of the video file
fn main() {
    // Green screen rgb: (114, 251, 2)
    let file_path = Path::new("resources/greenscreen.mp4");
    // let mut first_frame = File::create("resources/first_frame.png").unwrap();
    let mut first_frame;
    let frame_source = vid2img::FileSource::new(file_path, (500, 500)).unwrap();
    let mut x = 0;
    for frame in frame_source.into_iter() {
        if let Ok(Some(png_img_data)) = frame {
            println!("First image:");
            first_frame = Cursor::new(&png_img_data);
            let mut decoder = Decoder::new(first_frame);
            let mut reader = decoder.read_info().unwrap();
            let mut buf = vec![0; reader.output_buffer_size()];
            break;
        }
    }
}
