mod args {
  use argparse;
  use size;

  pub struct T {
    pub target_width: size::T,
    pub target_height: size::T,
  }

  /// Parse the command-line arguments
  pub fn parse_or_exit() -> T {
    let mut t =
      T {
        target_width:  size::Absolute(0),
        target_height:  size::Absolute(0),
      };

    {
      let mut args = argparse::ArgumentParser::new();
      args.set_description("Content-aware image resizer");
      args.refer(&mut t.target_width)
        .required()
        .add_option(
          &["-w", "--width"],
          argparse::Store,
          "Target width of the image",
        )
        .required();
      args.refer(&mut t.target_height)
        .required()
        .add_option(
          &["-h", "--height"],
          argparse::Store,
          "Target height of the image",
        )
        .required();

      args.parse_args_or_exit();
    }

    t
  }
}

use std;
use env_logger;
use image;
use image::{GenericImage, ImageDecoder};

use content_aware_image_resize::pixel;
use content_aware_image_resize::resize;
use content_aware_image_resize::vec2d;

use size;

fn load_input() -> image::ImageResult<vec2d::T<pixel::T>> {
  let input = std::io::stdin();
  let image = try!(image::load_jpeg(input));

  let (w, h) = image.dimensions();
  info!("Image is {} by {}", w, h);

  let mut data = vec2d::new(w as usize, h as usize, pixel::empty());

  for x in 0..w {
  for y in 0..h {
    data.get_mut(x as usize, y as usize).data = image.get_pixel(x, y).data;
  }}

  Ok(data)
}

fn output(data: &vec2d::T<pixel::T>) -> std::io::Result<()> {
  let mut image = image::DynamicImage::new_rgba8(data.width as u32, data.height as u32);

  for x in 0 .. data.width {
  for y in 0 .. data.height {
    image.get_pixel_mut(x as u32, y as u32).data = data.get(x, y).data;
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

  let args = args::parse_or_exit();

  info!("Loading input..");
  let data = load_input().unwrap();

  info!("Performing operations..");
  let mut data = data.clone();

  {
    let shrink_amount =
      match args.target_width {
        size::Absolute(w) => data.width as u32 - w,
        size::Relative(ratio) => (data.width as f32 * (1.0 - ratio)) as u32,
      };
    let shrink_amount = std::cmp::max(shrink_amount, 0);
    let shrink_amount = std::cmp::min(shrink_amount, data.width as u32);
    for i in 1 .. shrink_amount + 1 {
      data = resize::decrement_width(&data);
      debug!("Width iteration {}/{} done", i, shrink_amount);
    }
  }

  {
    let shrink_amount =
      match args.target_height {
        size::Absolute(w) => data.height as u32 - w,
        size::Relative(ratio) => (data.height as f32 * (1.0 - ratio)) as u32,
      };
    let shrink_amount = std::cmp::max(shrink_amount, 0);
    let shrink_amount = std::cmp::min(shrink_amount, data.height as u32);
    for i in 1 .. shrink_amount + 1 {
      data = resize::decrement_height(&data);
      debug!("Height iteration {}/{} done", i, shrink_amount);
    }
  }

  info!("Writing output..");
  output(&data).unwrap();
}
