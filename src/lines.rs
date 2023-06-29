#[derive(Debug)]
pub struct Lines {
    numbers: Vec<i64>,
    lengths: Vec<usize>,
}

impl Lines {
    pub fn new() -> Self {
        Lines {
            numbers: Vec::with_capacity(8),
            lengths: Vec::with_capacity(8),
        }
    }

    pub fn add(&mut self, line: i64) -> &mut Self {
        match self.numbers.last() {
            None => {
                self.numbers.push(line);
                self.lengths.push(1);
            }
            Some(line_) => {
                if *line_ == line {
                    let length = self.lengths.pop().unwrap();
                    self.lengths.push(length + 1);
                } else {
                    self.numbers.push(line_ + 1);
                    let length = self.lengths.last().unwrap();
                    self.lengths.push(length + 1);
                }
            }
        }

        return self;
    }

    pub fn get(&self, offset: usize) -> Option<i64> {
        self.position(offset).map(|i| self.numbers[i])
    }

    fn position(&self, offset: usize) -> Option<usize> {
        if self.lengths.is_empty() {
            return None;
        }

        let offset = offset + 1;

        if offset <= self.lengths[0] {
            return Some(0);
        }

        for i in 0..(self.lengths.len() - 1) {
            if self.lengths[i] < offset && offset <= self.lengths[i + 1] {
                return Some(i + 1);
            }
        }

        return (offset <= *self.lengths.last().unwrap()).then(|| self.lengths.len());
    }

    // #[allow(dead_code)]
    // fn to_vector(&self) -> Vec<u64> {
    //     let mut vec = Vec::new();
    //     for (i, length) in self.lengths.iter().enumerate() {
    //         let line = self.numbers[i];
    //         for _ in 0..*length {
    //             vec.push(line);
    //         }
    //     }
    //     vec
    // }
}

#[cfg(test)]
mod test {
    #[test]
    fn lines_test() {
        use super::Lines;
        let mut lines = Lines::new();

        lines
            .add(1)
            .add(1)
            .add(2)
            .add(2)
            .add(2)
            .add(2)
            .add(3)
            .add(3)
            .add(3)
            .add(4)
            .add(4)
            .add(4)
            .add(4)
            .add(4)
            .add(5);

        println!("{:?}", lines);

        for i in 0..15 {
            println!(
                "{} -> pos={:?}, line={:?}",
                i,
                lines.position(i),
                lines.get(i)
            );
        }
    }
}
