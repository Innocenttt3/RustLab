
struct Review<'a> {
    which_game: &'a Game,
    reviewer: &'a Player,
    status: Status,
    rate: u8,
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
    let category = Category::Casual;

    let game = Game {
        id: 1,
        name: String::from("Example Game"),
        category,
    };

    let player = Player {
        id: 1,
        name: String::from("John"),
        surname: String::from("Doe"),
        age: 25,
    };

    let review = Review {
        which_game: &game,
        reviewer: &player,
        status: Status::Has,
        rate: 4,
    };

    println!("Review of '{}' by {} {}: {:?}", game.name, player.name, player.surname, review.status);
    println!("Rating: {}", review.rate);
}
