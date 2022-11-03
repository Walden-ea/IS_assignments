use ux::u4;

pub fn input_to_u4_arr(input: &str) -> [u4;16] {
    assert!(input.chars().all(|char| (char >= '0' && char <= '9') || (char >= 'A' && char <= 'F')));


    let mut res = [u4::new(0);16];
    for (i,ch) in input.chars().enumerate() {
        res[i] = u4::new(ch.to_digit(16).expect(&format!("parse failed! {}",ch)).try_into().unwrap());
    }
    res
}