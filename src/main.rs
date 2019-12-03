fn main() {
    let entry_codes: Vec<usize> = include_str!("./input.txt")
        .trim()
        .split(',')
        .map(|p| p.parse::<usize>().unwrap())
        .collect();
    let mut codes = entry_codes.clone();
    codes[1] = 12;
    codes[2] = 2;
    println!("result 1: {}", codes.exec()[0]);
    for i in 0..=99 {
        for j in 0..=99 {
            let mut codes = entry_codes.clone();
            codes[1] = i;
            codes[2] = j;
            if codes.exec()[0] == 19_690_720 {
                println!("noun: {}\nverb: {}\nResult 2: {}", i, j, 100 * i + j);
                std::process::exit(0);
            }
        }
    }
    unreachable!();
}

impl ExecCode for Vec<usize> {
    fn exec(&mut self) -> &Self {
        let mut head = 0;
        while head < self.len() {
            let result = match self[head] {
                1 => self.get_val(head + 1) + self.get_val(head + 2),
                2 => self.get_val(head + 1) * self.get_val(head + 2),
                99 => return self,
                _ => panic!("Shouldn't happen"),
            };
            let res_loc = self[head + 3];
            self[res_loc] = result;
            head += 4;
        }
        self
    }
    fn get_val(&self, i: usize) -> usize {
        self[self[i]]
    }
}

trait ExecCode {
    fn exec(&mut self) -> &Self;
    fn get_val(&self, i: usize) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let res: Vec<usize> = vec![2, 0, 0, 0, 99];
        assert_eq!(res, *vec![1, 0, 0, 0, 99].exec());
    }
    #[test]
    fn test2() {
        let res: Vec<usize> = vec![2, 3, 0, 6, 99];
        assert_eq!(res, *vec![2, 3, 0, 3, 99].exec());
    }
    #[test]
    fn test3() {
        let res: Vec<usize> = vec![2, 4, 4, 5, 99, 9801];
        assert_eq!(res, *vec![2, 4, 4, 5, 99, 0].exec());
    }
    #[test]
    fn test4() {
        let res: Vec<usize> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(res, *vec![1, 1, 1, 4, 99, 5, 6, 0, 99].exec());
    }
}
