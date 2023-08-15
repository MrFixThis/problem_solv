// unsorted and general -> avg
pub fn find_median(arr: &[i32]) -> i32 {
    let len = arr.len();
    if arr.len() & 1 == 0 {
        (arr.get(len / 2).unwrap() + arr.get((len / 2) - 1).unwrap()) / len as i32
    } else {
        *arr.get(len / 2).unwrap()
    }
}
