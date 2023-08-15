pub fn min_max_sum(arr: &[i32]) {
    // let mut cl: Vec<_> = Vec::from(arr);
    // cl.sort_by(|a, b| a.cmp(b));
    //
    // let min: i64 = cl.iter().take(4).sum();
    // let max: i64 = cl.iter().rev().take(4).sum(); 
    //
    // println!("{min} {max}");

    // Or... (more cumbersome)
    let iter = (0..arr.len()).map(|i| {
        arr.iter()
            .enumerate()
            .filter_map(|(j, &b)| {
                if i == j {
                    return None;
                }
                Some(b)
            })
            .sum::<i32>()
    });

    println!("{} {}", iter.clone().min().unwrap(), iter.max().unwrap());
}
