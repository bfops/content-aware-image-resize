#[derive(Debug, Clone)]
pub struct T<X> {
  pub as_vec: Vec<X>,
  pub width: usize,
  pub height: usize,
}

fn index<X>(this: &T<X>, x: usize, y: usize) -> usize {
  y * this.width + x
}

impl<X> T<X> {
  pub fn get(&self, x: usize, y: usize) -> &X {
    assert!(x < self.width);
    assert!(y < self.height);

    let index = index(self, x, y);
    &self.as_vec[index]
  }

  pub fn get_mut(&mut self, x: usize, y: usize) -> &mut X {
    assert!(x < self.width);
    assert!(y < self.height);

    let index = index(self, x, y);
    &mut self.as_vec[index]
  }
}

pub fn new<X: Clone>(width: usize, height: usize, default: X) -> T<X> {
  let mut as_vec = Vec::new();
  for _ in 0.. width * height {
    as_vec.push(default.clone());
  }

  T {
    as_vec: as_vec,
    width: width,
    height: height,
  }
}
