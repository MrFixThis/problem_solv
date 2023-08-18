pub fn no_prefix(words: &[String]) {
    for (i, a) in words.iter().enumerate() {
        for (_, b) in words
            .iter()
            .enumerate()
            .filter(|&(j, b)| i != j || b.len() > a.len())
        {
            if &b[..a.len()] == a {
                return println!("BAD SET\n{b}");
            }
        }
    }
    println!("GOOD SET");
}
