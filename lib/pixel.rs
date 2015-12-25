#[derive(Debug, Clone, Copy)]
pub struct T {
  pub data: [u8; 4],
}

pub fn empty() -> T {
  T { data: [0; 4] }
}

fn abs(x: i32) -> i32 {
  if x >= 0 { x } else { -x }
}

pub fn diff(t1: T, t2: T) -> u32 {
  assert!(t1.data[3] == t2.data[3]);
  let r =
    abs(t1.data[0] as i32 - t2.data[0] as i32) +
    abs(t1.data[1] as i32 - t2.data[1] as i32) +
    abs(t1.data[2] as i32 - t2.data[2] as i32);
  r as u32
}
