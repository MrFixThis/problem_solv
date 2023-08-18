pub fn super_digit(n: &str, k: i32) -> i32 {
    // Dynamic programming, baby!
    let mut n = n.repeat(k as usize);
    while n.len() != 1 {
        n = n
            .chars()
            .filter_map(|c| c.to_digit(10))
            .sum::<u32>()
            .to_string();
    }
    n.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_super_digit() {
        let n = "9875";
        let k = 4;
        println!("{}", super_digit(n, k))
    }
}
