struct PresentDimension {
    length: u32,
    width: u32,
    height: u32,
}

fn parse_present_dimension(raw_dimensions: &str) -> PresentDimension {
    let dimensions: Vec<u32> = raw_dimensions.split('x')
                                             .map(|s| s.parse().unwrap())
                                             .collect();

    assert_eq!(dimensions.len(), 3);

    PresentDimension {
        length: dimensions.get(0).unwrap().clone(),
        width: dimensions.get(1).unwrap().clone(),
        height: dimensions.get(2).unwrap().clone(),
    }
}

fn wrapping_paper_size(dim: PresentDimension) -> u32 {
    let sides = [
        dim.length * dim.width,
        dim.width * dim.height,
        dim.height * dim.length,
    ];

    let base_area: u32 = sides.iter().map(|x| x * 2).sum();
    let extra_area = sides.iter().min().unwrap();

    base_area + extra_area
}

fn ribbon_size(dim: PresentDimension) -> u32 {
    let mut sides = [dim.length, dim.width, dim.height];
    sides.sort();

    let smallest_perimeter = 2 * (sides[0] + sides[1]);
    let bow_size = sides.iter().fold(1, |t, x| t * x);

    smallest_perimeter + bow_size
}

pub fn p1(input: &str) -> u32 {
    input.trim().split('\n')
         .map(parse_present_dimension)
         .map(wrapping_paper_size)
         .sum()
}

pub fn p2(input: &str) -> u32 {
    input.trim().split('\n')
         .map(parse_present_dimension)
         .map(ribbon_size)
         .sum()
}
