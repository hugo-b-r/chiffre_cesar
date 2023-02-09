use std::env;
use std::process;

fn main() {
    let alphabet = "_abcdefghijklmnopqrstuvwxyz1234567890".to_string();
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("probleme rencontre lors de l'interpretation des arguments
            : {}", err);
        process::exit(1);
    });
    let resultat: String;
    match config.action { 
        Action::Decryptage => {resultat = decrypter(config.texte, config.cle, alphabet);},
        Action::Cryptage => {resultat = crypter(config.texte, config.cle, alphabet);},
    };

    println!("{}", resultat);
    
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

fn crypter(texte: String, cle: u8, alphabet: String) -> String {
    let mut resultat  = "".to_string();
    for lettre in texte.chars() {
        let index = alphabet.find(lettre).unwrap_or(0);
        resultat.push(alphabet.chars().nth(index + usize::from(cle)).unwrap());
    }
    resultat
}

fn decrypter(texte: String, cle: u8, alphabet: String) -> String {
    let mut resultat = "".to_string();
    for lettre in texte.chars() {
        let index = alphabet.find(lettre).unwrap_or(0);
        resultat.push(alphabet.chars().nth(index - usize::from(cle)).unwrap());
    }
    resultat
}