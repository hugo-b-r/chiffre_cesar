pub enum Action {
    Cryptage,
    Decryptage,
}

pub struct Config {
    pub action: Action,
    pub cle: u8,
    pub texte: String,
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

pub fn crypter(texte: String, cle: u8, alphabet: String) -> String {
    let mut resultat  = "".to_string();
    for lettre in texte.chars() {
        let mut index = alphabet.find(lettre).unwrap_or(0);
        index += usize::from(cle);
        index = index % alphabet.len();
        resultat.push(alphabet.chars().nth(index).unwrap());
    }
    resultat
}

pub fn decrypter(texte: String, cle: u8, alphabet: String) -> String {
    let mut resultat = "".to_string();
    for lettre in texte.chars() {
        let mut index = alphabet.find(lettre).unwrap_or(0);
        if index <= alphabet.len() {
            index += alphabet.len();
        }
        index -= usize::from(cle) % alphabet.len();
        index = index % alphabet.len();
        resultat.push(alphabet.chars().nth(index).unwrap());
    }
    resultat
}

