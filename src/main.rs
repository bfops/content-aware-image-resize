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
use image;
use image::ImageDecoder;

#[main]
fn main() {
  let input = std::io::stdin();
  let mut img_decoder = image::jpeg::JPEGDecoder::new(input);

  let (w, h) = img_decoder.dimensions().unwrap();
  println!("Image is {} by {}", w, h);
}
