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
//          - player reaches goal on the other side from where they started

struct Person {}
struct Piece {}
struct Space {
    middle : bool,
    piece : Option<Piece>,
}
impl Space {
    fn exists(&self) -> bool { true }
    fn has_piece(&self) -> bool {
        match self.piece {
            Some(_) => true,
            None => false,
        }
    }
    fn is_middle(&self) -> bool {
        self.middle
    }
}
struct Row {
    space : Option<Space>,
}
impl Row {
    fn exists(&self) -> bool { true }
    fn has_space(&self) -> bool {
        match self.space {
            Some(_) => true,
            None => false,
        }
    }
}
struct Result {}
struct Goal {
    row : Option<Row>,
}
impl Goal {
    fn has_row(&self) -> bool {
        match self.row {
            Some(_) => true,
            None => false
        }
    }
}
struct Player {
    goal : Option<Goal>,
    rows : Vec<Row>,
}
impl Player {
    fn has_goal(&self) -> bool {
        match self.goal {
            Some(_) => true,
            None => false,
        }
    }
    fn has_row(&self) -> bool {
        match self.rows.get(0) {
            Some(_) => true,
            None => false,
        }
    }
    fn has_start_row(&self) -> bool {
        match self.rows.get(0) {
            Some(_) => true,
            None => false,
        }
    }
}
struct Round { 
    player : Option<Player>,
    result : Option<Result>,
}
impl Round {
    fn has_player(&self) -> bool {
        match self.player {
            Some(_) => true,
            None => false,
        }
    }
    fn has_result(&self) -> bool {
        match self.result {
            Some(_) => true,
            None => false,
        }
    }
}
struct Game {
    pieces : Vec<Piece>,
    round : Option<Round>,
}
impl Game {
    fn has_piece(&self) -> bool {
        match self.pieces.get(0) {
            Some(_) => true,
            None => false
        }
    }
    fn has_round(&self) -> bool {
        match self.round {
            Some(_) => true,
            None => false,
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
        game: Some( Game { 
            pieces : vec![ Piece {} ],
            round : Some( Round {
                player : Some(Player {
                    rows : vec![ Row {
                        space: Some( Space {
                            middle: true,
                            piece: Some( Piece {} ),
                        }),
                    }],
                    goal : Some(Goal {
                        row : Some ( Row {
                            space: Some( Space {
                                middle: true,
                                piece: Some( Piece {} ),
                            }),
                        }),
                    }),
                }),
                result : Some(Result {}),
            }),
        }),
    };
    assert!(table.exists());
    assert!(table.has_person());
    assert!(table.has_another_person());
    assert!(table.has_game());
    let game = table.game.expect("no game for table");
    assert!(game.has_piece());
    assert!(game.has_round());
    let round = game.round.expect("no round for game");
    assert!(round.has_player());
    assert!(round.has_result());
    let player = round.player.expect("no player for round");
    assert!(player.has_row());
    // the player's piece starts in the middle of their first row of spaces
    let start_row = player.rows.get(0).expect("no start row for player");
    assert!(start_row.exists());
    assert!(start_row.has_space());
    let start_space : &Space = start_row.space.as_ref().expect("no space for row");
    assert!(start_space.exists());
    assert!(start_space.is_middle());
    assert!(start_space.has_piece());
    assert!(player.has_start_row());
    assert!(player.has_goal());
    let goal = player.goal.expect("no goal for player");
    assert!(goal.has_row());
    // next line: player reaches goal
    println!("Hello, Bash Dash!");
}