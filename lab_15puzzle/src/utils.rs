// use ux::u4;

pub fn str_to_u8_arr(input: &str) -> [u8;16] {
    assert!(input.chars().all(|char| (char >= '0' && char <= '9') || (char >= 'A' && char <= 'F')));


    let mut res = [0;16];
    for (i,ch) in input.chars().enumerate() {
        res[i] = ch.to_digit(16).expect(&format!("parse failed! {}",ch)).try_into().unwrap();
    }
    res
}
