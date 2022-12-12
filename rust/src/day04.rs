use std::fs;

pub fn solve() {
    let content = fs::read_to_string("../_inputs/day04.txt").unwrap();

    let pairs = content
        .trim()
        .lines()
        .map(|line| {
            let pairs = line
                .split(',')
                .map(|range| {
                    let range = range
                        .split('-')
                        .map(|section| section.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();

                    (range[0], range[1])
                })
                .collect::<Vec<_>>();

            let first = pairs[0];
            let second = pairs[1];

            (first, second)
        })
        .collect::<Vec<_>>();

    let includes = pairs
        .iter()
        .map(|(first, second)| {
            let first_includes_second = first.0 <= second.0 && first.1 >= second.1;
            let second_includes_first = second.0 <= first.0 && second.1 >= first.1;

            usize::from(first_includes_second || second_includes_first)
        })
        .sum::<usize>();

    let overlaps = pairs
        .iter()
        .map(|(first, second)| {
            let overlaps = first.0 <= second.1 && second.0 <= first.1;

            usize::from(overlaps)
        })
        .sum::<usize>();

    println!("[day04.p1] includes: {:?}", includes);
    println!("[day04.p2] overlaps: {:?}", overlaps);
}
