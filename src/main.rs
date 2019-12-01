fn main() -> Result<(), std::io::Error> {
    let content = std::fs::read_to_string("input.txt")?;
    let fuel: i64 = content.lines().map(|x| get_fuel(x.parse().unwrap())).sum();
    println!("Fuel: {}", fuel);
    let total_fuel: i64 = content.lines().map(|x| get_total_fuel(x.parse().unwrap())).sum();
    println!("Total Fuel: {}", total_fuel);
    Ok(())
}

#[inline]
fn get_fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

#[inline]
fn get_total_fuel(mut mass: i64) -> i64 {
    let mut sum = 0;
    loop {
        mass = get_fuel(mass);
        if mass <= 0 {
            return sum
        };
        sum += mass;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_12() {
        assert_eq!(2, get_fuel(12));
    }
    #[test]
    fn test_14() {
        assert_eq!(2, get_fuel(14));
        assert_eq!(2, get_total_fuel(14))
    }
    #[test]
    fn test_1969() {
        assert_eq!(654, get_fuel(1969));
        assert_eq!(966, get_total_fuel(1969));
    }
    #[test]
    fn test_100756() {
        assert_eq!(33_583, get_fuel(100_756));
        assert_eq!(50_346, get_total_fuel(100_756));
    }
}
