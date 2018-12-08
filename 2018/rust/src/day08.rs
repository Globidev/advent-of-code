const RAW_INPUT_STR: &str = include_str!("../../inputs/day08.txt");

pub fn day08() -> (u32, u32) {
    let data: Vec<_> = parse_input(RAW_INPUT_STR).collect();

    (part1(&data), part2(&data))
}

pub fn part1(data: &[u32]) -> u32 {
    let graph = build_graph(data);

    fn sum_meta(Node { children, meta_data }: &Node) -> u32 {
        let self_sum: u32 = meta_data.iter().sum();
        let children_sum: u32 = children.iter()
            .map(sum_meta)
            .sum();

        self_sum + children_sum
    }

    sum_meta(&graph)
}

pub fn part2(data: &[u32]) -> u32 {
    let graph = build_graph(data);

    fn sum_values(Node { children, meta_data }: &Node) -> u32 {
        if children.is_empty() {
            meta_data.iter().sum()
        } else {
            meta_data.iter()
                .filter(|&&m| m > 0 && m - 1 < children.len() as u32)
                .map(|&m| sum_values(&children[m as usize - 1]))
                .sum()
        }
    }

    sum_values(&graph)
}

pub fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.split(' ')
        .map(|x| x.parse().expect("Badly formatted number"))
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    meta_data: Vec<u32>
}

fn build_graph(data: &[u32]) -> Node {
    parse_nodes(&mut data.iter().cloned(), 1)
        .remove(0)
}

fn parse_nodes(data: &mut impl Iterator<Item = u32>, count: u32) -> Vec<Node> {
    (0..count)
        .map(|_| {
            let (entry_len, meta_len) = (data.next().unwrap(), data.next().unwrap());
            let children = parse_nodes(data, entry_len);
            let meta_data = data.take(meta_len as usize).collect();
            Node { children, meta_data }
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let data: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part1(&data), 44338);
    }

    #[test]
    fn p2() {
        let data: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part2(&data), 37560);
    }
}
