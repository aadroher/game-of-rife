use im::HashSet;

type Cell = (i64, i64);

#[derive(Debug)]
pub struct World {
  cells: HashSet<Cell>,
}

pub fn new_world(cells: Vec<Cell>) -> World {
  World {
    cells: HashSet::from(cells),
  }
}

fn with_neighbours(cell: &Cell) -> HashSet<Cell> {
  let (x, y) = cell;
  HashSet::from(vec![
    (x - 1, y + 1),
    (x + 0, y + 1),
    (x + 1, y + 1),
    (x - 1, y + 0),
    (x + 0, y + 0),
    (x + 1, y + 0),
    (x - 1, y - 1),
    (x + 0, y - 1),
    (x + 1, y - 1),
  ])
}

fn get_candidates(cells: &HashSet<Cell>) -> HashSet<Cell> {
  let with_repetition: Vec<Cell> = cells
    .into_iter()
    .flat_map(|cell| with_neighbours(cell))
    .collect();
  HashSet::from(with_repetition)
}

fn get_num_neighbours(world_cells: &HashSet<Cell>, cell: &Cell) -> usize {
  let (x, y) = cell;
  let neighbour_positions = vec![
    (x - 1, y + 1),
    (x + 0, y + 1),
    (x + 1, y + 1),
    (x - 1, y + 0),
    (x + 1, y + 0),
    (x - 1, y - 1),
    (x + 0, y - 1),
    (x + 1, y - 1),
  ];
  neighbour_positions
    .into_iter()
    .filter(|cell| world_cells.contains(cell))
    .count()
}

pub fn next_tick(world: World) -> World {
  let World { cells } = world;
  let candidates = get_candidates(&cells);
  let survivors: HashSet<Cell> = candidates
    .into_iter()
    .filter(|cell| get_num_neighbours(&cells, &cell) == 3)
    .collect();
  World { cells: survivors }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let initial_cells = vec![(-1, 0), (0, 0), (0, 1)];
    let world = new_world(initial_cells);
    let World { cells: next_cells } = next_tick(world);
    let expected_next_cells: HashSet<Cell> = HashSet::from(vec![(1, 0), (0, 0), (0, 1)]);

    println!("{:?}", next_cells);

    assert_eq!(next_cells, expected_next_cells);
  }
}
