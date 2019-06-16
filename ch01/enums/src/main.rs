//enums.rs
#[derive(Debug)]
enum Direction {
    N, S, E, W
}

enum PlayerAction {
    Move {
          direction: Direction, 
          speed: u8,
    },
    Wait,
    Attack(Direction),
}

fn print_player_action(action: PlayerAction){
    match action {
        PlayerAction::Wait => {
            println!("Player wants to wait.")
        },
        PlayerAction::Move{direction, speed} => {
            println!("Player wants to move in direction {:?} with speed {}.", direction, speed)
        },
        PlayerAction::Attack(direction) => {
            println!("Player wants to attack in the {:?} direction.", direction)
        },
    };
}

fn main() {
    let simulated_player_action_1 = PlayerAction::Wait;
    let simulated_player_action_2 = PlayerAction::Move{
        direction: Direction::N,
        speed: 2,
    };
    let simulated_player_action_3 = PlayerAction::Attack(Direction::W);
    
    print_player_action(simulated_player_action_1);
    print_player_action(simulated_player_action_2);
    print_player_action(simulated_player_action_3);
}
