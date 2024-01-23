/*
Marco Polo game
If the name is Marco, the program should print Polo!.
Otherwise, the program will respond with "What's your name?".

$ cargo run -- play --name "Marco"

 */


pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo!".to_string()
    } else {
        "What's your name?".to_string()
    }
}
