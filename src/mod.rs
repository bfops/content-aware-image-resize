#![feature(main)]
#![feature(plugin)]
#![feature(str_char)]

#![plugin(clippy)]
#![allow(len_without_is_empty)]

extern crate argparse;
extern crate env_logger;
extern crate image;
#[macro_use]
extern crate log;
extern crate num;

mod main;
mod pixel;
mod random_access_str;
mod resize;
mod size;
mod vec2d;
