use game_of_rife::World;
use im::hashset;

fn main() {
  println!("Hello, world!");

  let initial_cells = hashset![(0, 0), (1, 0), (2, 0), (2, 1), (1, 2)];
  let world0 = World {
    cells: initial_cells,
  };
  println!("{:?}", world0);

  let world1 = world0.forward(1000);
  println!("{:?}", world1);
}
