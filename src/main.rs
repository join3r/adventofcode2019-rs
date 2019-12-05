const PASS_FROM: u64 = 123_257;
const PASS_TO: u64 = 647_015;

fn main() {
    let mut num_pass = 0;
    let mut num_pass2 = 0;
    for pass in PASS_FROM..=PASS_TO {
        let digits: Vec<u32> = pass.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        if get_result(&digits) {
            num_pass += 1;
            if get_result2(&digits) {
                num_pass2 += 1;
            }
        };
    }
    println!("result1 = {}\nresult2 = {}", num_pass, num_pass2);
}

fn get_result(digits: &[u32]) -> bool {
    digits.windows(2).any(|x| x[0] == x[1]) && digits.windows(2).all(|x| x[0] <= x[1])
}
fn get_result2(digits: &[u32]) -> bool {
    for i in digits {
        if digits.iter().filter(|x| **x == *i).count() == 2 {
            return true;
        };
    }
    false
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1_1() {
        let pass = 111111;
        let digits: Vec<u32> = pass.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        let result = true;
        assert_eq!(get_result(&digits), result);
    }
    #[test]
    fn test_1_2() {
        let pass = 223450;
        let digits: Vec<u32> = pass.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        let result = false;
        assert_eq!(get_result(&digits), result);
    }
    #[test]
    fn test_1_3() {
        let pass = 123789;
        let digits: Vec<u32> = pass.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        let result = false;
        assert_eq!(get_result(&digits), result);
    }
    #[test]
    fn test_2_1() {
        let pass = 112233;
        let digits: Vec<u32> = pass.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        let result = true;
        assert_eq!(get_result(&digits) && get_result2(&digits), result);
    }
    #[test]
    fn test_2_2() {
        let pass = 123444;
        let digits: Vec<u32> = pass.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        let result = false;
        assert_eq!(get_result(&digits) && get_result2(&digits), result);
    }
    #[test]
    fn test_2_3() {
        let pass = 111122;
        let digits: Vec<u32> = pass.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        let result = true;
        assert_eq!(get_result(&digits) && get_result2(&digits), result);
    }
}
