// tuples.rs

fn main() {
    let num_and_str: (u8, &str) = (3, "beers");
    println!("{:?}", num_and_str );
    let (num, string) = num_and_str;
    print!("From tuple: Number: {}, String: {}", num, string);
}
