#![feature(plugin)]

#![plugin(clippy)]
#![allow(len_without_is_empty)]

#[macro_use]
extern crate log;

pub mod pixel;
pub mod resize;
pub mod vec2d;
