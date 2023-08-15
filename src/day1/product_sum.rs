pub fn product_sum(arr: &[usize], target: usize) {
    let mut comb: Vec<Vec<usize>> = Vec::new();
    let mut skiped_idxs: Vec<usize> = Vec::new();
    arr.iter().enumerate().for_each(|(i, &v)| {
        skiped_idxs.push(i);
        backtrack(arr, target, v, &mut comb, &mut skiped_idxs);
        skiped_idxs.pop();
    });
    println!("{comb:?}")
}

fn backtrack(
    arr: &[usize],
    target: usize,
    curr_val: usize,
    comb: &mut Vec<Vec<usize>>,
    skiped_idxs: &mut Vec<usize>,
) {
    let mut pos: Vec<usize> = Vec::from_iter(skiped_idxs.iter().cloned());
    for (i, &v) in arr.iter().enumerate() {
        if skiped_idxs.contains(&i) {
            continue;
        }

        let sum = curr_val + v;
        if sum == target {
            pos.push(i);
            comb.push(pos.clone());
            pos.pop();
        } else if sum < target {
            skiped_idxs.push(i);
            backtrack(arr, target, sum, comb, skiped_idxs);
            skiped_idxs.pop();
        }
    }
}
