// struct_methods.rs

struct Player {
    name: String,
    iq: u8,
    friends: u8,
}

impl Player {
    fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            iq: 10,
            friends: 100,
        }
    }
    fn get_friends(&self) -> u8 {
        self.friends
    }
    fn set_friends(&mut self, count: u8) {
        self.friends = count;
    }
}

fn main() {
    let mut player = Player::with_name("Randy");
    player.set_friends(2);
    println!("{}'s friends count: {}", player.name, player.get_friends());
    //another way to call instance methods
    let _ = Player::get_friends(&player);
}
