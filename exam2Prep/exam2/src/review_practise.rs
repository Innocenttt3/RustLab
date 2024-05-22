#[derive(Debug)]
struct Game{
    id: u8,
    name: String,
    category: Category,
}
#[derive(Debug)]
enum Category {
    Casual,
    Multi,
    Single
}

#[derive(Debug)]
enum Status {
    Unreleased,
    Released,
    Popular,
    Niche
}
#[derive(Debug)]
struct Player {
    id: u32,
    name: String,
    surname: String,
    age: u8
}
#[derive(Debug)]
struct Review<'a> {
    game: &'a Game,
    player: &'a Player,
    status_info: Status,
    rating: u8
}

impl<'a> Review<'a>{
    fn new(pla)
}
fn main(){
    let example_game = Game {id: 1, name: String::from("CSGO"), category: Category::Casual};
    println!("Game ID: {:?}", example_game);
    let example_player = Player {id: 2115, name: String::from("John"), surname: String::from("Terry"), age: 15};
    println!("Player: {:?}", example_player);
    let example_review = Review {game: &example_game, player: &example_player, status_info: Status::Popular, rating: 7};
    println!("Review: {:?}", example_review);

}