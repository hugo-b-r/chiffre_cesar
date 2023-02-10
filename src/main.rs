use std::env;
use std::process;
use chiffre_cesar::{decrypter, crypter, Config, Action};

fn main() {
    let alphabet = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 ".to_string();
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("probleme rencontre lors de l'interpretation des arguments
            : {}", err);
        process::exit(1);
    });
    let resultat: String;
    match config.action { 
        Action::Decryptage => {
            resultat = decrypter(config.texte, config.cle, alphabet);
        },
        Action::Cryptage => {
            resultat = crypter(config.texte, config.cle, alphabet);
        },
    };

    println!("{}", resultat);
    
}

#[cfg(test)]
mod tests;

