# visual-remix-rs WIP
visual-remix-rs is a command tool written in Rust that allows you to slice a video and match it to an audio file, creating a music-video-like effect.
## How?
- Slice the video into 1-second clips, calculate how much the pixels change in each clip, and order them from the highest pixel change to the lowest.
- Slice the audio into 1-second clips, add the amplitude in the samples in 1 second, and order them from the highest amplitude to the lowest.
- Match them, and render! 

## Why?
It's fun, and I want to get better at Rust. 

 ## TO DO:
 1. Get MP3 amplitude per sample.
 2. Get RGB values per frame from an MP4. (use Gstreamer Rust bindings? because it's a pain to implement from scratch)
 3. ...
 4. Compile Rust to WebAssembly
