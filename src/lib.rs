pub mod world;
use im::{hashset, HashSet};

use wasm_bindgen::prelude::*;

pub use world::World;

#[wasm_bindgen]
pub fn evolve(alive_cells: &[i64], steps: usize) -> Box<[i64]> {
  let extract_components = |take_xs: bool| {
    let remainder = if take_xs { 1 } else { 0 };
    alive_cells
      .into_iter()
      .enumerate()
      .filter(|(i, _)| i % 2 == remainder)
      .map(|(_, &x)| x)
      .collect::<Vec<i64>>()
  };

  let xs = extract_components(true);
  let ys = extract_components(false);

  let cells: HashSet<(i64, i64)> = xs.into_iter().zip(ys.into_iter()).collect();
  let world = World { cells };
  let World { cells: new_cells } = world.forward(steps);
  new_cells
    .into_iter()
    .fold(Vec::new(), |mut components, (x, y)| {
      components.extend_from_slice(&[x, y]);
      components
    })
    .into_boxed_slice()
}
