use std::ops::Add;
use std::fmt::Display;

fn add_thing<T: Add> (fst: T, snd: T) {
    let _ = fst + snd;
}

fn show_me<T: Display> (val: T) {
    println!("{}", val);
}

fn main() {
    add_thing(2, 2);
    show_me(1234);
}
