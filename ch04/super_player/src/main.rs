fn main() {
    struct Audio(String);
    struct Video(String);

    impl Playable for Audio {
        fn play(&self) {
            println!("Now Playing: {}", self.0);
        }
    }
    fn main() {}
}
