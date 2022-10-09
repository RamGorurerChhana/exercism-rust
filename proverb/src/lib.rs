pub fn build_proverb(list: &[&str]) -> String {
    // Returns an iterator over all contiguous windows of length 2
    let proverb = list
        .windows(2)
        .map(|s| format!("For want of a {} the {} was lost.\n", s[0], s[1]))
        .collect::<String>();
    if list.len() > 0 {
        format!("{}And all for the want of a {}.", proverb, list[0])
    } else {
        proverb
    }
}
