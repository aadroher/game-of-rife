use im::{hashset, HashSet};

type Cell = (i64, i64);

#[derive(Debug)]
pub struct World {
  cells: HashSet<Cell>,
}

impl World {
  fn next_step(self) -> World {
    let World { cells } = self;
    let deceased = get_deceased(&cells);
    let newborns = get_newborns(&cells);
    let next_cells = cells.clone().relative_complement(deceased).union(newborns);
    World { cells: next_cells }
  }

  fn forward(self, n: usize) -> World {
    (0..n).into_iter().fold(self, |world, _| world.next_step())
  }
}

fn get_neighbour_positions(cell: Cell) -> HashSet<Cell> {
  get_with_neighbours(cell).relative_complement(hashset![cell])
}

fn get_with_neighbours(cell: Cell) -> HashSet<Cell> {
  let translations_lengths = [-1, 0, 1];
  translations_lengths
    .iter()
    .flat_map(|d0| {
      translations_lengths
        .iter()
        .map(move |d1| (cell.0 + d0, cell.1 + d1))
    })
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
  get_candidates(world_cells)
    .relative_complement(world_cells.clone())
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_neighbour_positions_retuns_neighbours() {
    let cell: Cell = (5, 17);
    let neighbours = get_neighbour_positions(cell);
    let expected_value = hashset![
      (4, 17),
      (4, 18),
      (5, 18),
      (6, 18),
      (6, 17),
      (6, 16),
      (5, 16),
      (4, 16),
    ];

    assert_eq!(neighbours, expected_value);
  }

  #[test]
  fn test_get_with_neighbours_returns_neighbours_and_cell() {
    let cell: Cell = (5, 17);
    let neighbours = get_with_neighbours(cell);
    let expected_value = hashset![
      (5, 17),
      (4, 17),
      (4, 18),
      (5, 18),
      (6, 18),
      (6, 17),
      (6, 16),
      (5, 16),
      (4, 16),
    ];

    assert_eq!(neighbours, expected_value);
  }

  #[test]
  fn test_get_num_neighbours_returns_right_value_for_0_neighbours() {
    let cells = hashset![(0, 0), (12, 45)];
    let num_neighbours = get_num_neighbours(&cells, (0, 0));

    assert_eq!(num_neighbours, 0);
  }

  #[test]
  fn test_get_num_neighbours_returns_correct_value_for_1_neighbour() {
    let cells = hashset![(0, 0), (0, 1)];
    let num_neighbours = get_num_neighbours(&cells, (0, 0));

    assert_eq!(num_neighbours, 1);
  }

  #[test]
  fn test_get_num_neighbours_returns_correct_value_for_3_neighbours() {
    let cells = hashset![(3, 0), (3, -1), (2, 0), (2, 1), (12, 45)];
    let num_neighbours = get_num_neighbours(&cells, (3, 0));

    assert_eq!(num_neighbours, 3);
  }

  #[test]
  fn test_get_newborns_works_for_oscillator() {
    let initial_cells = hashset![(-1, 0), (0, 0), (1, 0)];
    let newborns = get_newborns(&initial_cells);

    assert_eq!(newborns, hashset![(0, 1), (0, -1)]);
  }

  #[test]
  fn test_get_newborns_works_for_block() {
    let initial_cells = hashset![(0, 0), (0, 1), (1, 0)];
    let newborns = get_newborns(&initial_cells);

    assert_eq!(newborns, hashset![(1, 1)]);
  }

  #[test]
  fn test_next_step_returns_correct_new_word_for_oscillator() {
    let world = World {
      cells: hashset![(-1, 0), (0, 0), (1, 0)],
    };
    let World { cells: next_cells } = world.forward(1);
    let expected_next_cells: HashSet<Cell> = hashset![(0, 1), (0, 0), (0, -1)];

    assert_eq!(next_cells, expected_next_cells);
  }

  #[test]
  fn test_avance_12_steps_returns_correct_word_for_oscillator() {
    let world = World {
      cells: hashset![(-1, 0), (0, 0), (1, 0)],
    };
    let World { cells: next_cells } = world.forward(12);
    let expected_next_cells: HashSet<Cell> = hashset![(-1, 0), (0, 0), (1, 0)];

    assert_eq!(next_cells, expected_next_cells);
  }

  #[test]
  fn test_avance_3_steps_returns_correct_word_for_glider() {
    let world = World {
      cells: hashset![(0, 0), (1, 0), (2, 0), (2, 1), (1, 2)],
    };
    let World { cells: next_cells } = world.forward(3);
    let expected_next_cells: HashSet<Cell> = hashset![(2, 0), (3, 0), (1, 1), (1, -1), (2, -1)];

    assert_eq!(next_cells, expected_next_cells);
  }
}
