use std::collections::VecDeque;

pub fn time_convertion(s: &str) -> String {
    let (mut time, spec) = {
        let parts = s.split_at(s.len() - 2);
        (
            parts.0.split(':').collect::<VecDeque<_>>(),
            parts.1
        )
    };

    let change = match (time.pop_front().unwrap(), spec) {
        ("12", "AM") => "00",
        (front, _) => match front {
            "01" => "13",
            "02" => "14",
            "03" => "15",
            "04" => "16",
            "05" => "17",
            "06" => "18",
            "07" => "19",
            "08" => "20",
            "09" => "21",
            "10" => "22",
            "11" => "23",
            _ => front,
        },
    };

    time.push_front(change);
    time
        .iter()
        .enumerate()
        .fold(String::with_capacity(time.len() + 2), |mut acc, (i, &s)| {
            acc.push_str(s);
            if i != time.len() - 1 {
                acc.push_str(":")
            }
            acc
        })
}
