use std::usize;

struct GeneratePrimeValue {
    limit: usize
}

fn process_prime(limit: usize) -> Vec<bool> {
    let mut refine = vec![true; limit];
    let mut m = 2;
    while m * m < limit {
        if refine[m] {
            for i in (m * 2..limit).step_by(m) {
                refine[i] = false;
            }
        }
        m += 1;
    }
    refine
}

impl GeneratePrimeValue {
    fn iter(&self) -> Primeiterator {
        Primeiterator {
            index: 2,
            processed: process_prime(self.limit)
        }
    }

    fn new(limit: usize) -> GeneratePrimeValue {
        GeneratePrimeValue { limit }
    }
}

struct Primeiterator {
    index: usize,
    processed: Vec<bool>
}

impl Iterator for Primeiterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.index += 1;
            if self.index > self.processed.len() - 1 {
                return None;
            } else if self.processed[self.index] {
                return Some(self.index);
            } else {
                continue
            }
        }
    }
}

fn main() {
    let primes = GeneratePrimeValue::new(100);
    for i in primes.iter() {
        println!("{}, ", i);
    }
}

