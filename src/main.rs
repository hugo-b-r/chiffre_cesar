use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("probleme rencontre lors de l'interpretation des arguments
            : {}", err);
        process::exit(1);
    });
    
}

enum Action {
    Cryptage,
    Decryptage,
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
        let action: Action;
        if args[1].clone() == "cryptage" {
            action = Action::Cryptage;
        } else if args[1].clone() == "decryptage" {
            action = Action::Decryptage;
        } else {
            return Err("Premier argument n'est ni cryptage ni decryptage");
        }
        let texte = args[2].clone();


        Ok(Config {
            action,
            texte,
        })
    }
}