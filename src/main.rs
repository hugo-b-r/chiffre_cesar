use std::env;
use std::process;

fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890";
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("probleme rencontre lors de l'interpretation des arguments
            : {}", err);
        process::exit(1);
    //match config.action { \
    //    Action::Decryptage => {}, 
    //    Action::Cryptage => {},
    //}
    });
    
}

enum Action {
    Cryptage,
    Decryptage,
}

struct Config {
    action: Action,
    cle: u8,
    texte: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
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
        let cle = args[2].clone().parse::<u8>().unwrap();
        let texte = args[3].clone();


        Ok(Config {
            action,
            cle,
            texte,
        })
    }
}