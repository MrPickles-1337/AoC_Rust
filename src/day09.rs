#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let mut dict: Vec<String> = Vec::new();
    let mut result = Vec::new();
    for line in input.lines() {
        let mut split = line.split(' ');
        let source = split.next().unwrap().to_string();

        let source_n = if let Some(n) = dict.iter().position(|e| *e == source) {
            n
        } else {
            dict.push(source);
            dict.len() as isize - 1
        };

        split.next();
        let destination = split.next().unwrap().to_string();
        let dest_n = if let Some(n) = dict.iter().position(|e| *e == destination) {
            n
        } else {
            dict.push(destination);
            dict.len() as isize - 1
        };
        split.next();
        let distance = split.next().unwrap().parse::<u32>().unwrap();
        if source_n > (result.len() as isize - 1) as usize {
            result.push(vec![u32::MAX; source_n]);
        }
        println!("{source_n} {dest_n}");
        dbg!(&result);
        result[source_n][dest_n] = distance;
    }
    result
}

#[aoc(day9, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    dbg!(input);
    1
}
