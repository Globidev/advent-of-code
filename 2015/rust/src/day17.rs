type Container = u32;

fn combinations(sum: u32, containers: Vec<Container>, so_far: Vec<Container>) -> Vec<Vec<Container>> {
    use std::cmp::Ordering::{Less, Equal};

    let mut all = Vec::new();

    let new_combination = |container| {
        let mut combination = so_far.clone();
        combination.push(container);
        combination
    };

    for (container, i) in containers.iter().zip(0..) {
        match container.cmp(&sum) {
            Equal => all.push(new_combination(*container)),
            Less => {
                let combination = new_combination(*container);
                let remaining = sum - container;
                let new_containers = containers.iter().skip(i + 1)
                                               .map(|c| *c)
                                               .collect::<Vec<_>>();
                all.append(&mut combinations(
                    remaining, new_containers, combination
                ));
            },
            _ => ()
        }
    }

    all
}

pub fn p1(input: &str) -> usize {
    use std::str::FromStr;

    let containers = input.trim().split('\n').map(|s|
        FromStr::from_str(s).unwrap()
    ).collect::<Vec<_>>();

    combinations(150, containers, Vec::new()).len()
}

pub fn p2(input: &str) -> usize {
    use std::str::FromStr;

    let containers = input.trim().split('\n').map(|s|
        FromStr::from_str(s).unwrap()
    ).collect::<Vec<_>>();

    let combinations = combinations(150, containers, Vec::new());
    let smallest = combinations.iter().map(Vec::len).min().unwrap_or(0);
    combinations.iter().filter(|c| c.len() == smallest).count()
}
