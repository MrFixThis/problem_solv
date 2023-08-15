mod fibo;
mod median;
mod diag;
mod time;
mod min_max;
mod pm;
mod product_sum;

pub fn start() {
    // plus minus
    let nums: Vec<i32> = vec![1, 1, 0, -1, -1];
    pm::plus_minus(&nums);

    // diagonal difference
    let mtr = [
        [6, 9, 2],
        [4, 1, 7],
        [7, 4, 8],
    ];
    diag::diag_difference(mtr);

    // Min max sums
    let arr = vec![1, 2, 3, 4, 5];
    min_max::min_max_sum(&arr);

    // Time convertion
    let my_time = "07:05:45PM";
    println!("{}", time::time_convertion(my_time));

    //median
    let arr = vec![4, 5, 7, 2, 6, 5];
    println!("{}", median::find_median(&arr));

    // fibbo
    // function
    println!("{}", fibo::fibo_fn(5));

    // iterator
    // for v in fibo::Fibbonaci::with_limit(30) {
    //     println!("{v}")
    // }

    //product_sum
    let arr = vec![1, 4, 2, 4, 5, 1];
    let pres = 6;
    product_sum::product_sum(&arr, pres);
}
