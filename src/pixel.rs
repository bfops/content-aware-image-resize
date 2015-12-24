use image;
use num;

pub type T = image::Rgba<u8>;

pub fn empty() -> T {
  image::Rgba { data: [0; 4] }
}

#[test]
fn assert_mem_layout() {
  assert!(std::mem::size_of::<T>() == std::mem::size_of::<u8>());
}

pub fn diff(t1: T, t2: T) -> u32 {
  assert!(t1.data[3] == t2.data[3]);
  let r =
    num::abs(t1.data[0] as i32 - t2.data[0] as i32) +
    num::abs(t1.data[1] as i32 - t2.data[1] as i32) +
    num::abs(t1.data[2] as i32 - t2.data[2] as i32);
  r as u32
}
