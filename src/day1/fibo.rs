pub struct Fibbonaci {
    curr: usize,
    next: usize,
    count: usize,
    limit: Option<usize>,
}

impl Default for Fibbonaci {
    fn default() -> Self {
        Self {
            curr: 0,
            next: 1,
            count: 0,
            limit: None,
        }
    }
}

impl Fibbonaci {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_limit(limit: usize) -> Self {
        Self {
            curr: 0,
            next: 1,
            count: 0,
            limit: Some(limit),
        }
    }
}

impl Iterator for Fibbonaci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.limit {
            if n == self.count {
                return None;
            }
        }

        self.count += 1;
        let next = self.curr + self.next;
        self.curr = self.next;
        self.next = next;
        Some(self.curr)
    }
}

pub fn fibo_fn(n: usize) -> usize {
    if n <= 0 {
        return n;
    }

    let mut prev = 0;
    let mut curr = 1;

    (2..=n).for_each(|_| {
        println!("{curr}");
        let next = prev + curr;
        prev = curr;
        curr = next;
    });

    curr
}
