// Each of these functions finds the energy of the lowest energy paths going through 
// each pixel in a given dimension. We'll do this by accumulating the energy of the 
// paths from one side to the other. We can use this to find the paths themselves.

use pixel;
use std;
use vec2d;

// A pixel's energy is the sum in differences in value between it and its neighbors.
fn energy(data: &vec2d::T<pixel::T>) -> vec2d::T<u32> {
  let get = |x, y| {
    if 
      x < 0 || x >= data.width  as isize ||
      y < 0 || y >= data.height as isize ||
      false
    {
      None
    } else {
      Some(*data.get(x as usize, y as usize))
    }
  };

  let mut energy = vec2d::new(data.width, data.height, 0);

  for x in 0..data.width as isize {
  for y in 0..data.height as isize {
    let mut accum = 0;

    let cur = get(x, y).unwrap();
    get(x - 1, y    ).map(|neighbor| accum = accum + pixel::diff(cur, neighbor));
    get(x + 1, y    ).map(|neighbor| accum = accum + pixel::diff(cur, neighbor));
    get(x    , y - 1).map(|neighbor| accum = accum + pixel::diff(cur, neighbor));
    get(x    , y + 1).map(|neighbor| accum = accum + pixel::diff(cur, neighbor));

    *energy.get_mut(x as usize, y as usize) = accum;
  }}

  energy
}

fn y_paths(energy: &vec2d::T<u32>) -> vec2d::T<u32> {
  let mut paths = vec2d::new(energy.width, energy.height, 0);
  for x in 0 .. energy.width {
    *paths.get_mut(x, 0) = *energy.get(x, 0);
  }

  // TODO: turn things into filter-maps

  for y in 1 .. energy.height {
    for x in 0 .. energy.width {
      // Minimum energy path to this cell's "ancestor".
      let mut min_ancestor = *paths.get(x, y - 1);
      if x > 0 {
        min_ancestor = std::cmp::min(min_ancestor, *paths.get(x - 1, y - 1));
      }
      if x < energy.width - 1 {
        min_ancestor = std::cmp::min(min_ancestor, *paths.get(x + 1, y - 1));
      }

      *paths.get_mut(x, y) = min_ancestor + energy.get(x, y);
    }
  }

  paths
}

pub fn decrement_width(data: &vec2d::T<pixel::T>) -> vec2d::T<pixel::T> {
  let energy = energy(&data);

  let paths = y_paths(&energy);

  let mut path = vec2d::new(1, energy.height, 0);
  *path.get_mut(0, energy.height - 1) =
    (0..energy.width)
    .min_by_key(|&x| *paths.get(x, energy.height - 1))
    .unwrap();
  for y in (0 .. energy.height - 2).rev() {
    let ancestor = *path.get(0, y + 1) as isize;
    let (x, _path_energy) =
      [ancestor - 1, ancestor, ancestor + 1].iter()
        .filter_map(|&x| {
          if x < 0 || x >= data.width as isize {
            None
          } else {
            Some((x as usize, paths.get(x as usize, y)))
          }
        })
        .min_by_key(|&(_x, path_energy)| path_energy)
        .unwrap();
    *path.get_mut(0, y) = x;
  }

  let mut new_data = vec2d::new(data.width - 1, data.height, pixel::empty());
  for y in 0 .. data.height {
    for x in 0 .. data.width {
      match x.cmp(&path.get(0, y)) {
        std::cmp::Ordering::Less    => *new_data.get_mut(x, y) = *data.get(x, y),
        std::cmp::Ordering::Greater => *new_data.get_mut(x - 1, y) = *data.get(x, y),
        std::cmp::Ordering::Equal   => {},
      }
    }
  }

  new_data
}

// pub fn decrement_height<pixel::T>(energy: &vec2d::T<pixel::T>) -> vec2d::T<pixel::T> {
//   let mut paths = vec2d::new(energy.width, energy.height, 0);
//   for x in 0..energy.width {
//     *paths.get_mut(x, 0) = energy.get(x, 0);
//   }
// 
//   for y in 1..energy.height {
//     for x in 0..energy.width {
//       // Minimum energy path to this cell's "ancestor".
//       let mut min_to_ancestor = *energy.get(x, y - 1);
//       if x > 0 {
//         min_to_ancestor = std::cmp::min(min_to_ancestor, *energy.get(x - 1, y - 1));
//       }
//       if x < energy.width - 1 {
//         min_to_ancestor = std::cmp::min(min_to_ancestor, *energy.get(x + 1, y - 1));
//       }
// 
//       *paths.get_mut(x, y) = min_to_ancestor + energy.get(x, y);
//     }
//   }
// }

pub fn energy_map(data: &vec2d::T<pixel::T>) -> vec2d::T<pixel::T> {
  let energy = energy(&data);

  let max_energy =
    (0 .. energy.width)
    .flat_map(|x| {
      let energy = &energy;
      (0 .. energy.height)
      .map(move |y| {
        *energy.get(x, y)
      })
    })
    .max()
    .unwrap();

  let mut new_data = vec2d::new(data.width, data.height, pixel::empty());
  for y in 0 .. data.height {
    for x in 0 .. data.width {
      let mut data = *data.get(x, y);
      let energy = std::cmp::min(((*energy.get(x, y) as f32 / max_energy as f32) * 255.0) as u32, 255) as u8;
      data.data[0] = energy;
      data.data[1] = energy;
      data.data[2] = energy;
      *new_data.get_mut(x, y) = data;
    }
  }

  new_data
}

pub fn path_energy_map(data: &vec2d::T<pixel::T>) -> vec2d::T<pixel::T> {
  let energy = energy(&data);

  let paths = y_paths(&energy);

  let max_energy =
    (0 .. paths.width)
    .flat_map(|x| {
      let paths = &paths;
      (0 .. paths.height)
      .map(move |y| {
        *paths.get(x, y)
      })
    })
    .max()
    .unwrap();

  let mut new_data = vec2d::new(data.width, data.height, pixel::empty());
  for x in 0 .. data.width {
  for y in 0 .. data.height {
    let mut data = *data.get(x, y);
    let energy = std::cmp::min(((*paths.get(x, y) as f32 / max_energy as f32) * 255.0) as u32, 255) as u8;
    data.data[0] = energy;
    data.data[1] = energy;
    data.data[2] = energy;
    *new_data.get_mut(x, y) = data;
  }}

  new_data
}

pub fn highlight_y_paths(data: &vec2d::T<pixel::T>) -> vec2d::T<pixel::T> {
  let energy = energy(&data);

  let paths = y_paths(&energy);

  let mut path = vec2d::new(1, energy.height, 0);
  *path.get_mut(0, energy.height - 1) =
    (0..energy.width)
    .min_by_key(|&x| *paths.get(x, energy.height - 1))
    .unwrap();
  for y in (0 .. energy.height - 2).rev() {
    let ancestor = *path.get(0, y + 1) as isize;
    let (x, _path_energy) =
      [ancestor - 1, ancestor, ancestor + 1].iter()
        .filter_map(|&x| {
          if x < 0 || x >= data.width as isize {
            None
          } else {
            Some((x as usize, paths.get(x as usize, y)))
          }
        })
        .min_by_key(|&(_x, path_energy)| path_energy)
        .unwrap();
    *path.get_mut(0, y) = x;
  }

  let mut new_data = vec2d::new(data.width, data.height, pixel::empty());
  for y in 0 .. data.height {
    for x in 0 .. data.width {
      match x.cmp(&path.get(0, y)) {
        std::cmp::Ordering::Equal => {
          let mut data = *data.get(x, y);
          data[0] = 255;
          data[1] = 0;
          data[2] = 0;
          *new_data.get_mut(x, y) = data;
        },
        _ => {
          *new_data.get_mut(x, y) = *data.get(x, y);
        },
      }
    }
  }

  new_data
}
