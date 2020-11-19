use im::HashSet;

#[derive(Debug)]
type Cell = (i64, i64);

#[derive(Debug)]
pub struct World {
  cells: HashSet<Cell>,
}

pub fn new_world(cells: Cell[]) -> World {
  World {
    cells
  }
}


#[cfg(test)]
mod tests {
  use super::*;
}