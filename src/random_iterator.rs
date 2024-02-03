///
/// Generates a random iterator that will iterate
/// over all numbers in the range [0, max) in a random order.
///
pub struct RandomIterator {
    max: usize,
    offset: usize,
    step_size: usize,
    actual: usize,
    i: usize,
}

impl RandomIterator {
    pub fn new(max: usize) -> Self {
        Self {
            max: max,
            offset: 0,
            actual: 0,
            step_size: max / 2,
            i: 0,
        }
    }
    pub fn reset(&mut self) {
        self.i = 0;
        self.offset = 0;
        self.actual = 0;
        self.step_size = self.max / 2;
    }
    pub fn i(&self) -> usize {
        self.i
    }
}

impl Iterator for RandomIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.i >= self.max {
            return None;
        }
        self.i += 1;
        let out = self.actual;
        self.actual += self.step_size;
        if self.actual >= self.max {
            self.offset += 1;
            self.actual = self.offset;
        }
        Some(out)
    }
}

#[cfg(test)]
mod randomiterator_test {
    use crate::random_iterator::RandomIterator;

    fn test(max: usize) {
        let mut nums = vec![];
        let mut generator = RandomIterator::new(max);
        for _ in 0..max {
            nums.push(generator.next().unwrap());
        }

        for i in 0..max {
            if nums.iter().find(|&x| *x == i).is_none() {
                panic!("{} not found", i);
            }
        }
        if generator.next().is_some() {
            panic!("generator did not end");
        }
    }

    #[test]
    fn test_cases() {
        test(0);
        test(1);
        test(2);
        test(3);
        test(1000);
        test(1001);
        test(1002);
    }
}
