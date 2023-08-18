mod cookies;
mod digit;
mod bin_search;

pub fn start() {
    // Jesse cookies
    let k = 9;
    let a = [2, 7, 3, 6, 4, 6];
    println!("{}", cookies::mix_cookies(k, &a));

    // superdigit
    let n = "9875";
    let k = 4;
    println!("{}", digit::super_digit(n, k));

    // Binary search
    let arr = [1, 2, 4, 5, 6];
    let target = 4;
    println!("{:?}", bin_search::binary_search(&arr, target));
}
