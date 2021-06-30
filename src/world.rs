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

fn get_neighbour_positions(cell: Cell) -> HashSet<Cell> {
  let (x0, y0) = cell;
  let with_neighbours = get_with_neighbours(cell);
  with_neighbours
    .into_iter()
    .filter(|(x1, y1)| x0 != *x1 || y0 != *y1)
    .collect()
}

fn get_with_neighbours(cell: Cell) -> HashSet<Cell> {
  let translations = vec![-1, 0, 1];
  let (x, y) = cell;
  translations
    .iter()
    .flat_map(|d0| translations.iter().map(move |d1| (x + d0, y + d1)))
    .collect()
}

// fn get_candidates(cells: &HashSet<Cell>) -> HashSet<Cell> {
//   let with_repetition: Vec<Cell> = cells
//     .into_iter()å
//     .flat_map(|cell| get_with_neighbours(cell))
//     .collect();
//   HashSet::from(with_repetition)
// }

fn get_num_neighbours(world_cells: &HashSet<Cell>, cell: Cell) -> usize {
  let neighbour_positions = get_neighbour_positions(cell);
  // println!("{:?}", cell);
  // println!("{:?}", neighbour_positions);
  neighbour_positions
    .into_iter()
    .filter(|cell| world_cells.contains(cell))
    .count()
}

// pub fn next_tick(world: World) -> World {
//   let World { cells } = world;
//   let candidates = get_candidates(&cells);
//   println!("{:?}", candidates);

//   let survivors: Vec<_> = candidates
//     .into_iter()
//     .map(|cell| (cell, get_num_neighbours(&cells, &cell)))
//     .collect();
//   println!("{:?}", survivors);
//   World { cells: cells }
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_neighbour_positions_retuns_neighbours() {
    let cell: Cell = (5, 17);
    let neighbours = get_neighbour_positions(cell);
    let expected_value = HashSet::from(vec![
      (4, 17),
      (4, 18),
      (5, 18),
      (6, 18),
      (6, 17),
      (6, 16),
      (5, 16),
      (4, 16),
    ]);

    assert_eq!(neighbours, expected_value);
  }

  #[test]
  fn test_get_with_neighbours_returns_neighbours_and_cell() {
    let cell: Cell = (5, 17);
    let neighbours = get_with_neighbours(cell);
    let expected_value = HashSet::from(vec![
      (5, 17),
      (4, 17),
      (4, 18),
      (5, 18),
      (6, 18),
      (6, 17),
      (6, 16),
      (5, 16),
      (4, 16),
    ]);

    assert_eq!(neighbours, expected_value);
  }

  #[test]
  fn test_get_num_neighbours_returns_right_value_for_0_neighbours() {
    let cells = HashSet::from(vec![(0, 0), (12, 45)]);
    let num_neighbours = get_num_neighbours(&cells, (0, 0));

    assert_eq!(num_neighbours, 0);
  }

  #[test]
  fn test_get_num_neighbours_returns_right_value_for_1_neighbour() {
    let cells = HashSet::from(vec![(0, 0), (0, 1)]);
    let num_neighbours = get_num_neighbours(&cells, (0, 0));

    assert_eq!(num_neighbours, 1);
  }

  #[test]
  fn test_get_num_neighbours_returns_right_value_for_3_neighbour() {
    let cells = HashSet::from(vec![(3, 0), (3, -1), (2, 0), (2, 1), (12, 45)]);
    let num_neighbours = get_num_neighbours(&cells, (3, 0));

    assert_eq!(num_neighbours, 3);
  }

  // #[test]
  // fn it_works() {
  //   let initial_cells = vec![(-1, 0), (0, 0), (0, 1)];
  //   let world = new_world(initial_cells);
  //   let World { cells: next_cells } = next_tick(world);
  //   let expected_next_cells: HashSet<Cell> = HashSet::from(vec![(0, 1), (0, 0), (0, -1)]);

  //   println!("{:?}", next_cells);

  //   assert_eq!(next_cells, expected_next_cells);
  // }
}
