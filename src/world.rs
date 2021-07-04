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

fn get_candidates(cells: &HashSet<Cell>) -> HashSet<Cell> {
  cells
    .into_iter()
    .flat_map(|&cell| get_with_neighbours(cell))
    .collect()
}

fn get_num_neighbours(world_cells: &HashSet<Cell>, cell: Cell) -> usize {
  world_cells
    .clone()
    .intersection(get_neighbour_positions(cell))
    .len()
}

fn get_newborns(world_cells: &HashSet<Cell>) -> HashSet<Cell> {
  let newborn_candidates: HashSet<_> =
    get_candidates(world_cells).relative_complement(world_cells.clone());
  newborn_candidates
    .into_iter()
    .filter(|newborn_candidate| get_num_neighbours(world_cells, *newborn_candidate) == 3)
    .collect()
}

fn get_deceased(world_cells: &HashSet<Cell>) -> HashSet<Cell> {
  world_cells
    .into_iter()
    .filter(|&&cell| {
      let num_neighbours = get_num_neighbours(world_cells, cell);
      num_neighbours < 2 || 3 < num_neighbours
    })
    .map(|&cell| cell)
    .collect()
}

pub fn next_tick(world: World) -> World {
  let World { cells } = world;
  let deceased = get_deceased(&cells);
  let newborns = get_newborns(&cells);
  let next_cells = cells.relative_complement(deceased).union(newborns);
  World { cells: next_cells }
}

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
  fn test_get_num_neighbours_returns_correct_value_for_1_neighbour() {
    let cells = HashSet::from(vec![(0, 0), (0, 1)]);
    let num_neighbours = get_num_neighbours(&cells, (0, 0));

    assert_eq!(num_neighbours, 1);
  }

  #[test]
  fn test_get_num_neighbours_returns_correct_value_for_3_neighbours() {
    let cells = HashSet::from(vec![(3, 0), (3, -1), (2, 0), (2, 1), (12, 45)]);
    let num_neighbours = get_num_neighbours(&cells, (3, 0));

    assert_eq!(num_neighbours, 3);
  }

  #[test]
  fn get_newborns_works_for_oscillator() {
    let initial_cells = HashSet::from(vec![(-1, 0), (0, 0), (1, 0)]);
    let newborns = get_newborns(&initial_cells);

    assert_eq!(newborns, HashSet::from(vec![(0, 1), (0, -1)]));
  }

  #[test]
  fn get_newborns_works_for_block() {
    let initial_cells = HashSet::from(vec![(0, 0), (0, 1), (1, 0)]);
    let newborns = get_newborns(&initial_cells);

    assert_eq!(newborns, HashSet::from(vec![(1, 1)]));
  }

  #[test]
  fn next_tick_returns_correct_new_word_for_oscillator() {
    let initial_cells = vec![(-1, 0), (0, 0), (1, 0)];
    let world = new_world(initial_cells);
    let World { cells: next_cells } = next_tick(world);
    let expected_next_cells: HashSet<Cell> = HashSet::from(vec![(0, 1), (0, 0), (0, -1)]);

    assert_eq!(next_cells, expected_next_cells);
  }
}
