//  People playing a game 
//  People in the same room
//      Talking happens in real life
//  Each person plays on their own device
//  Bash Dash - get to the other side of the board 
//      - set your play 
//          - you can move left, right or up
//          - you can set a blocker either left or right of your current space
//
//      - points awarded for 
//          - spaces moved towards the other side
//          - opponent running into your blocker
//          - leaving the board on the other side
//
//      - points taken away for 
//          - setting a blocker
//          - running into your oponnent's blocker 
//
//      - round ends when
//          - opponnents collide
//          - player moves into a space that contains an oponnent's blocker
//          - player leaves the board on the other side

struct Person {}
struct Player {}
struct Piece {}
struct Space {}
struct Row {
    space : Option<Space>,
}
impl Row {
    fn has_space(&self) -> bool {
        match self.space {
            Some(_) => true,
            None => false,
        }
    }
}
struct Board {
    row : Option<Row>,
}
impl Board {
    fn has_row(&self) -> bool {
        match self.row {
            Some(_) => true,
            None => false,
        }
    }
}
struct Game {
    players : Vec<Player>,
    pieces : Vec<Piece>,
    board : Option<Board>,
}
impl Game {
    fn has_player(&self) -> bool {
        match self.players.get(0) {
            Some(_) => true,
            None => false,
        }
    }
    fn has_piece(&self) -> bool {
        match self.pieces.get(0) {
            Some(_) => true,
            None => false
        }
    }
    fn has_board(&self) -> bool {
        match self.board {
            Some(_) => true,
            None => false
        }
    }
}

struct Table {
    people : Vec<Person>,
    game : Option<Game>,
}

impl Table {
    fn exists(&self) -> bool { true }
    fn has_game(&self) -> bool {
        match self.game {
            Some(_) => true,
            None => false,
        }
    }
    fn has_person(&self) -> bool {
         match self.people.get(0) {
            Some(_) => true,
            None => false,
         }
    }
    fn has_another_person(&self) -> bool {
         match self.people.get(1) {
            Some(_) => true,
            None => false,
         }
    }
}

fn main() {
    let table = Table { 
        people : vec![ Person{}, Person{} ],
        game: Some(Game{ 
            players : vec![ Player {} ],
            pieces : vec![ Piece {} ],
            board : Some(Board { 
                row: Some( Row {
                    space: Some( Space {} ),
                } ), 
            }),
        }),
    };
    assert!(table.exists());
    assert!(table.has_person());
    assert!(table.has_another_person());
    assert!(table.has_game());
    let game = table.game.expect("table has no game");
    assert!(game.has_player());
    assert!(game.has_piece());
    assert!(game.has_board());
    // the players piece starts in the middle of their first row of spaces
    let board = game.board.expect("game has no board");
    assert!(board.has_row());
    let row = board.row.expect("board has no row");
    assert!(row.has_space());
    println!("Hello, Bash Dash!");
}