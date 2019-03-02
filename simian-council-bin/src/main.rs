extern crate simian_council;

fn main() {
    let secret = simian_council::ask_for_a_secret(100, 50, 75);
    println!("This is your secret : {}", secret);
}
