use std::collections::HashMap;

fn main() {
    let mut lines: Vec<Line> = include_str!("input.txt")
        .lines()
        .map(|line| line.into())
        .collect();
    let (manhattan, min_steps) = get_result(&mut lines);
    println!("manhattan: {}\nmin_steps: {}", manhattan, min_steps);
}
fn get_first_line(line: &mut Line, board: &mut HashMap<(i64, i64), usize>) {
    let mut cur_pos = (0,0);
    let mut total_steps = 0;
    for mut command in line.commands.iter_mut() {
        while command.steps > 0 {
            command.step(&mut cur_pos, &mut total_steps);
            board.entry(cur_pos).or_insert(total_steps);
            command.steps -= 1;
        }
    }
}
fn get_result(lines: &mut Vec<Line>) -> (i64, usize) {
    let mut board: HashMap<(i64, i64), usize> = HashMap::new();
    get_first_line(&mut lines[0], &mut board);
    let mut cur_pos = (0,0);
    let mut total_steps = 0;
    let (mut manhattan, mut min_steps) = (32_768, 32_768);
    for mut command in lines[1].commands.iter_mut() {
        while command.steps > 0 {
            command.step(&mut cur_pos, &mut total_steps);
            if let Some(x) = board.get(&cur_pos) {
                if manhattan > (cur_pos.0.abs() + cur_pos.1.abs()) {
                    manhattan = cur_pos.0.abs() + cur_pos.1.abs();
                };
                if min_steps > (x + total_steps) {
                    min_steps = x + total_steps;
                };
            };
            command.steps -= 1;
        }
    }
    (manhattan, min_steps)
}
struct Line {
    commands: Vec<Command>,
}
impl From<&str> for Line {
    fn from(line: &str) -> Line {
        let commands: Vec<Command> = line.split(',').map(|x| x.into()).collect();
        Line { commands }
    }
}
struct Command {
    dir: char,
    steps: i64,
}
impl Command {
    fn step(&self, cur_pos: &mut (i64,i64), total_steps: &mut usize) {
        match self.dir {
            'U' => cur_pos.1 += 1,
            'D' => cur_pos.1 -= 1,
            'L' => cur_pos.0 -= 1,
            'R' => cur_pos.0 += 1,
            _ => unreachable!(),
        };
        *total_steps += 1;
    }
}
impl From<&str> for Command {
    fn from(source: &str) -> Command {
        let (first, second) = source.split_at(1);
        let dir = first.chars().next().unwrap();
        let steps = second.parse().unwrap();
        Command { dir, steps }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut source: Vec<Line> = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72".into(),
            "U62,R66,U55,R34,D71,R55,D58,R83".into(),
        ];
        let result = 159;
        assert_eq!(get_result(&mut source).0, result);
    }
    #[test]
    fn test_2() {
        let mut source: Vec<Line> = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".into(),
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".into(),
        ];
        let result = 135;
        assert_eq!(get_result(&mut source).0, result);
    }
    #[test]
    fn test_2_1() {
        let mut source: Vec<Line> = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72".into(),
            "U62,R66,U55,R34,D71,R55,D58,R83".into(),
        ];
        let result = 610;
        assert_eq!(get_result(&mut source).1, result);
    }
    #[test]
    fn test_2_2() {
        let mut source: Vec<Line> = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".into(),
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".into(),
        ];
        let result = 410;
        assert_eq!(get_result(&mut source).1, result);
    }
}
