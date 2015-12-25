#![feature(main)]
#![feature(plugin)]
#![feature(str_char)]

#![plugin(clippy)]
#![allow(len_without_is_empty)]

extern crate argparse;
extern crate content_aware_image_resize;
extern crate env_logger;
extern crate image;
#[macro_use]
extern crate log;

mod main;
mod random_access_str;
mod size;
