use std;

pub struct T<'a> {
  indices: Vec<usize>,
  str: &'a str,
}

impl<'a> T<'a> {
  pub fn char_at(&self, idx: usize) -> char {
    self.str.char_at(self.indices[idx])
  }

  pub fn len(&self) -> usize {
    self.indices.len()
  }

  pub fn chars(&self) -> std::str::Chars {
    self.str.chars()
  }

  pub fn as_str(&self) -> &str {
    self.str
  }
}

pub fn of_str(str: &str) -> T {
  T {
    indices: str.char_indices().map(|(idx, _c)| idx).collect(),
    str: str,
  }
}
