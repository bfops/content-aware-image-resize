#![feature(main)]
#![feature(plugin)]

#![plugin(clippy)]

extern crate argparse;
extern crate env_logger;
extern crate image;
#[macro_use]
extern crate log;
extern crate num;

mod main;
mod pixel;
mod resize;
mod vec2d;
