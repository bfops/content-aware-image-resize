use std;
use random_access_str;

pub use self::T::*;

pub enum T {
  /// As an absolute amount
  Absolute(u32),
  /// As a fraction
  Relative(f32),
}

impl std::str::FromStr for T {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = random_access_str::of_str(s);
    if s.char_at(s.len() - 1) == '%' {
      let s: String = s.chars().take(s.len() - 1).collect();
      let parsed: f32 = 
        match std::str::FromStr::from_str(&s) {
          Err(err) => return Err(format!("{}", err)),
          Ok(parsed) => parsed,
        };
      Ok(Relative(parsed / 100.0))
    } else {
      let parsed: u32 =
        match std::str::FromStr::from_str(s.as_str()) {
          Err(err) => return Err(format!("{}", err)),
          Ok(parsed) => parsed,
        };
      Ok(Absolute(parsed))
    }
  }
}
