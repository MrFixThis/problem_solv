use std::{collections::BinaryHeap, cmp::Reverse};

pub fn mix_cookies(k: i32, a: &[i32]) -> i32 {
    let mut a: BinaryHeap<_> = a.iter().map(|&v| Reverse(v)).collect();
    let mut count = 0;
    while a.iter().any(|&l| l.0 < k) {
        if a.len() == 1 {
            return -1;
        }

        // SAFETY: at this point, there are at least 2 values in the heap
        let (f, s) = (a.pop().unwrap().0, a.pop().unwrap().0);
        a.push(Reverse(f + 2 * s));
        count += 1;
    }
    count
}
