struct Review<'a> {
    which_game: &'a Game,
    reviewer: &'a Player,
    status: Status,
    rate: u8,
}

impl<'a> Review<'a> {
    fn new(player_index: usize, game_index: usize, games_vec: &'a Vec<Game>, players_vec: &'a Vec<Player>, status: Status, rate: u8) -> Option<Self> {
        if let Some(player) = players_vec.get(player_index) {
            if let Some(game) = games_vec.get(game_index) {
                Some(Review {
                    which_game: game,
                    reviewer: player,
                    status,
                    rate,
                })
            } else {
                println!("Game index out of range.");
                None
            }
        } else {
            println!("Player index out of range.");
            None
        }
    }
}


#[derive(Debug)]
enum Status {
    Has,
    Had,
    NeverHad,
}

struct Player {
    id: u32,
    name: String,
    surname: String,
    age: u8,
}

struct Game {
    id: u32,
    name: String,
    category: Category,
}

enum Category {
    Casual,
    Party,
    Coop,
    Logical,
    Strategic,
}

fn main() {
    let player_id = 1;
    let game_id = 1;
    let rating = 9;


}
