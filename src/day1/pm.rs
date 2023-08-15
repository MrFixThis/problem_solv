pub fn plus_minus(arr: &[i32]) {
    arr.iter()
        .fold(vec![0, 0, 0], |mut acc: Vec<i32>, &v| {
            if v > 0 {
                acc[0] += 1;
            } else if v < 0 {
                acc[1] += 1;
            } else {
                acc[2] += 1;
            }

            acc
        })
        .into_iter()
        .map(|v| v as f64 / arr.len() as f64)
        .for_each(|v| println!("{v:.6}"));
}
