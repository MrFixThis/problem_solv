pub fn diag_difference<const N: usize>(mtr: [[i32; N]; N]) -> i32 {
    let mtrx_iter = mtr.iter().enumerate();
    let fd: i32 = mtrx_iter.clone().filter_map(|(i, v)| v.get(i)).sum();
    let sd: i32 = mtrx_iter
        .filter_map(|(i, v)| v.iter().rev().skip(i).next())
        .sum();
    fd - sd
}
