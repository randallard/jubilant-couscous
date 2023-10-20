use std::any::type_name;

struct Space {}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

struct Grid {
    spaces : Vec<Space>,
}

impl Grid {
    fn has_spaces(&self) -> bool {
        type_of(self.get_spaces()) == "&alloc::vec::Vec<run_game::Space>"
    }
    fn get_spaces(&self) -> &Vec<Space> {
        &self.spaces
    }
}
struct Board {
    grid : Grid,
}

impl Board {
    fn has_grid(&self) -> bool {
        type_of(&self.get_grid()) == "&&run_game::Grid"
    }
    fn get_grid(&self) -> &Grid {
        &self.grid
    }
}

struct Person {}

struct Table {
    people : Vec<Person>,
}

impl Table {
    fn exists(&self) -> bool { true }
    fn has_person(&self) -> bool {
         match self.people.get(0) {
            Some(_) => true,
            None => false,
         }
    }
}

fn main() {
    let table = Table { people : vec![ Person { } ]};
    assert!(table.exists());
    assert!(table.has_person());
    //assert!(table.has_game());
    //assert!(game.has_a_player());
    //assert!(game.has_a_piece());
    //assert!(game.each_player_is_a_person());
    //assert!(game.each_player_has_piece());
    //assert!(game.has_board());
    let board : Board = Board { grid: Grid { spaces : vec![ Space { } ]  }};
    assert!(board.has_grid());
    assert!(board.get_grid().has_spaces());    
    //assert!(game.each_piece_is_on_the_board());
    println!("Hello, Bash Dash!");
}