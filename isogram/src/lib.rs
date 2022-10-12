pub fn check(candidate: &str) -> bool {
    for (idx, char) in candidate.to_lowercase().chars().enumerate() {
        if (char != '-' && char != ' ') && candidate[idx + 1..].contains(char) {
            return false;
        }
    }

    true
}
