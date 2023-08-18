pub fn product_sum(arr: &[usize], target: usize) {
    let mut combs: Vec<Vec<usize>> = Vec::new();
    let mut hasht: Vec<usize> = Vec::new();

    arr.iter().enumerate().for_each(|(i, &v)| {
        hasht.push(i);
        backtrack(arr, target, v, &mut combs, &mut hasht);
        hasht.pop();
    });
    println!("{combs:?}")
}

fn backtrack(
    arr: &[usize],
    target: usize,
    curr_val: usize,
    combs: &mut Vec<Vec<usize>>,
    hasht: &mut Vec<usize>,
) {
    for (i, &v) in arr.iter().enumerate() {
        if hasht.contains(&i) {
            continue;
        }

        let sum = curr_val + v;
        if sum == target {
            hasht.push(i);
            combs.push(hasht.clone());
            hasht.pop();
        } else if sum < target {
            hasht.push(i);
            backtrack(arr, target, sum, combs, hasht);
            hasht.pop();
        }
    }
}
