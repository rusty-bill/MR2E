mod media;
use media::Playable;

struct Audio(String, String);
struct Video(String, String);

impl Playable for Audio {
    fn play(&self) {
        println!("Now Playing: {} by {}", self.0, self.1);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Now Playing: {} by {}", self.0, self.1);
    }
}

fn main() {
    println!("Super Player!");
    let audio = Audio("ambient_music.mp3".to_string(), "falala guy".to_string());
    let video = Video("ufo_documentary.mkv".to_string(), "aliens_guy".to_string());
    audio.play();
    video.play();
}
