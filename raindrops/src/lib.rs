pub fn raindrops(n: u32) -> String {
    let by_three = n % 3 == 0;
    let by_five = n % 5 == 0;
    let by_seven = n % 7 == 0;
    if by_three || by_five || by_seven {
        let mut sound = String::new();
        if by_three {
            sound.push_str("Pling");
        }
        if by_five {
            sound.push_str("Plang");
        }
        if by_seven {
            sound.push_str("Plong");
        }
        sound
    } else {
        n.to_string()
    }
}
