extern crate simian_council;

fn main() {
    //println!("============================================================");
    //let secret = simian_council::ask_for_a_secret(10, 1, 100);
    //println!("This is your secret : {}", secret);

    println!("============================================================");
    let pokemon = simian_council::ask_for_a_pokemon();
    println!("This is your pokemon : The {}", pokemon);
    println!("============================================================");
}
