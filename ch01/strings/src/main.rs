//strings.rs

fn main() {
    let question = "How are you ?"; //&str type
    let person: String = "Bob".to_string();
    let namaste = String::from("नमत");

    println!("{}! {} {}", namaste, question, person);
}
