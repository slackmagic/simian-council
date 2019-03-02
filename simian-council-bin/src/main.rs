extern crate simian_council;

fn main() {
    let secret = simian_council::ask_for_a_secret(10, 1, 100);
    println!("This is your secret : {}", secret);
}
