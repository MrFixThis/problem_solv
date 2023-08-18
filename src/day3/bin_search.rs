pub fn binary_search<T>(arr: &[T], target: T) -> Option<usize>
where
    T: PartialOrd,
{
    let mut left: usize = 0;
    let mut right: usize = arr.len() - 1;

    while left <= right {
        let mid = (right + left) / 2;

        if target == arr[mid] {
            return Some(mid);
        }

        if mid == 0 {
            return None;
        }

        if target > arr[mid] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

pub fn binary_search_recur<T>(arr: &[T], target: T) -> Option<usize>
where
    T: PartialOrd,
{
    search(arr, target, 0, arr.len())
}

fn search<T>(arr: &[T], target: T, start: usize, end: usize) -> Option<usize>
where
    T: PartialOrd,
{
    if start >= end {
        return None;
    }

    let middle = (start + end) / 2;

    if arr[middle] == target {
        return Some(middle);
    } else if target > arr[middle] {
        search(arr, target, middle, end)
    } else {
        search(arr, target, start, middle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_search_recur() {
        let arr = [1, 2, 4, 5, 6];
        let target = 4;
        println!("{:?}", binary_search_recur(&arr, target));
    }

    #[test]
    fn test_bin_search() {
        let arr = [1, 2, 4, 5, 6];
        let target = -1;
        println!("{:?}", binary_search(&arr, target));
    }
}
