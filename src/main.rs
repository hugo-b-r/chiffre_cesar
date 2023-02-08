use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
}

enum Action {
    cryptage,
    decryptage,
}

struct Config {
    action: Action,
    texte: String,
}
