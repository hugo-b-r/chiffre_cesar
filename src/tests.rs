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

