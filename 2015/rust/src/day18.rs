const GRID_W: usize = 100;
const GRID_H: usize = 100;

#[derive(PartialEq, Clone)]
enum LightState { On, Off }
type LightGrid = Vec<LightState>;

const NEIGHBOR_DELTAS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

const ALWAYS_ON_INDEXES: [usize; 4] = [
    0,
    GRID_W - 1,
    (GRID_H - 1) * GRID_W,
    GRID_W * GRID_H - 1
];

fn state_from_char(c: char) -> LightState {
    match c {
        '#' => LightState::On,
        _   => LightState::Off,
    }
}

fn next_light_state(state: LightState, neighbors: Vec<LightState>) -> LightState {
    use self::LightState::{On, Off};

    match state {
        On  => match neighbors.iter().filter(|&s| *s == On).count() {
            2 | 3 => On,
            _     => Off,
        }
        Off => match neighbors.iter().filter(|&s| *s == On).count() {
            3 => On,
            _ => Off,
        }
    }
}

fn neighbors(grid: &LightGrid, x: usize, y: usize) -> Vec<LightState> {
    NEIGHBOR_DELTAS.iter().map(|&(dx, dy)| {
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx < 0 || nx >= GRID_W as i32 || ny < 0 || ny >= GRID_H as i32 {
            LightState::Off
        }
        else {
            let idx = (ny * GRID_H as i32) + nx;
            unsafe { grid.get_unchecked(idx as usize) }.clone()
        }
    }).collect()
}

fn next_grid_state_p1(grid: &LightGrid) -> LightGrid {
    let mut new_grid = vec![LightState::Off; GRID_H * GRID_W];

    (0..GRID_W * GRID_H).for_each(|idx| {
        let light_state = unsafe { grid.get_unchecked(idx as usize) };
        let (x, y) = (idx % GRID_W, idx / GRID_H);
        let neighbors = neighbors(grid, x, y);

        new_grid.insert(
            idx as usize,
            next_light_state(light_state.clone(), neighbors)
        );
    });

    new_grid
}

fn next_grid_state_p2(grid: &LightGrid) -> LightGrid {
    let mut new_grid = vec![LightState::Off; GRID_H * GRID_W];

    (0..GRID_W * GRID_H).for_each(|idx| {
        let new_light_state = {
            if ALWAYS_ON_INDEXES.contains(&idx) {
                LightState::On
            }
            else {
                let (x, y) = (idx % GRID_W, idx / GRID_H);
                let neighbors = neighbors(grid, x, y);
                let light_state = unsafe { grid.get_unchecked(idx as usize) };
                next_light_state(light_state.clone(), neighbors)
            }
        };

        new_grid.insert(idx as usize, new_light_state);
    });

    new_grid
}

pub fn p1(input: &str) -> usize {
    let mut grid = input.trim().split('\n')
                               .flat_map(|l| l.chars().map(state_from_char))
                               .collect::<LightGrid>();

    for _ in 0..100 {
        grid = next_grid_state_p1(&grid);
    }

    grid.iter().filter(|&s| *s == LightState::On).count()
}

pub fn p2(input: &str) -> usize {
    let mut grid = input.trim().split('\n')
                               .flat_map(|l| l.chars().map(state_from_char))
                               .collect::<LightGrid>();

    ALWAYS_ON_INDEXES.iter().for_each(|idx| { grid[*idx] = LightState::On; });

    for _ in 0..100 {
        grid = next_grid_state_p2(&grid);
    }

    grid.iter().filter(|&s| *s == LightState::On).count()
}
