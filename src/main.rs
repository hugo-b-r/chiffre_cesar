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

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("il n'y a pas assez d'arguments");
        }
        match (args[1].clone()) {
            "cryptage" => let action = Action::cryptage;
            "decryptage" => let action = Action::decryptage;
        }
        let texte = args[2].clone();


        Ok(Config {
            recherche,
            texte,
        })
    }
}