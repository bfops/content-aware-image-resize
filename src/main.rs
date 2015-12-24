// mod args {
//   use argparse;
// 
//   pub struct T {
//     pub target_width: u32,
//     pub target_height: u32,
//   }
// 
//   /// Parse the command-line arguments
//   pub fn parse_or_exit() -> T {
//     let mut t =
//       T {
//         target_width:  0,
//         target_height: 0,
//       };
// 
//     {
//       let mut args = argparse::ArgumentParser::new();
//       args.set_description("Content-aware image resizer");
//       args.refer(&mut t.target_width)
//         .add_option(
//           &["-w", "-width"],
//           argparse::Store,
//           "Target width of the image",
//         );
//       args.refer(&mut t.target_height)
//         .add_option(
//           &["-h", "-height"],
//           argparse::Store,
//           "Target height of the image",
//         );
// 
//       args.parse_args_or_exit();
//     }
// 
//     t
//   }
// }

use std;
use env_logger;
use image;
use image::{GenericImage, ImageDecoder};

use pixel;
use resize;
use vec2d;

fn resize(data: &vec2d::T<pixel::T>) -> vec2d::T<pixel::T> {
  let mut data = data.clone();

  let shrink_amount = 1;
  for i in 1 .. shrink_amount + 1 {
    data = resize::decrement_width(&data);
    debug!("Decremented width {}", i);
  }

  data
}

fn load_input() -> image::ImageResult<vec2d::T<pixel::T>> {
  let input = std::io::stdin();
  let decoder = image::jpeg::JPEGDecoder::new(input);
  let image = try!(image::decoder_to_image(decoder));

  let (w, h) = image.dimensions();
  info!("Image is {} by {}", w, h);

  let mut data = vec2d::new(w as usize, h as usize, pixel::empty());

  for x in 0..w {
  for y in 0..h {
    *data.get_mut(x as usize, y as usize) = image.get_pixel(x, y);
  }}

  Ok(data)
}

fn output(data: &vec2d::T<pixel::T>) -> std::io::Result<()> {
  let mut image = image::DynamicImage::new_rgba8(data.width as u32, data.height as u32);

  for x in 0 .. data.width {
  for y in 0 .. data.height {
    image.put_pixel(x as u32, y as u32, *data.get(x, y));
  }}

  let as_vec = image.raw_pixels();

  let mut output = std::io::stdout();
  let mut img_encoder = image::jpeg::JPEGEncoder::new(&mut output);
  try!(img_encoder.encode(&as_vec, data.width as u32, data.height as u32, image.color()));

  Ok(())
}

#[main]
fn main() {
  env_logger::init().unwrap();

  let data = load_input().unwrap();
  let data = resize(&data);
  output(&data).unwrap();
}
