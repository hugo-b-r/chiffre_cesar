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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cryptage1() {
        let alphabet = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 ".to_string();
        let crypte = crypter("dream your life in color".to_string(), 23, alphabet);

        assert_eq!(crypte, "1C2x0VJ_FCV9632V6 Vz_9_C".to_string())
    }

    #[test]
    fn cryptage2() {
        let alphabet = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 ".to_string();
        let crypte = crypter("The difference between a dream and a plan is a date.".to_string(), 23, alphabet);

        assert_eq!(crypte, "q52V16332C2 z2Vy2EH22 VxV1C2x0Vx 1VxVA9x V6DVxV1xE2W".to_string())
    }


    #[test]
    fn decryptage1() {
        let alphabet = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 ".to_string();
        let decrypte = decrypter("1C2x0VJ_FCV9632V6 Vz_9_C".to_string(), 23, alphabet);

        assert_eq!(decrypte, "dream your life in color".to_string())
    }

    #[test]
    fn decryptage2() {
        let alphabet = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 ".to_string();
        let decrypte = decrypter("The difference between a dream and a plan is a date.".to_string(), 45, alphabet);

        assert_eq!(decrypte, "m1xRw2yyx x7vxRuxADxx7RtRw xt6Rt7wRtR95t7R2_RtRwtAxS".to_string())
    }
    
}