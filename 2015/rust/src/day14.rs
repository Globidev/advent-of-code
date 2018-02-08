extern crate nom;

type Speed = u32;
type Second = u32;

const RACE_TIME: Second = 2503;

struct Reindeer {
    speed: Speed,
    fly_time: Second,
    rest_time: Second
}

type Points = u32;

enum DeerMovementState { Flying(Second), Resting(Second) }
struct DeerRaceState<'a> {
    deer: &'a Reindeer,
    movement_state: DeerMovementState,
    points: Points,
    distance: u32
}

impl<'a> DeerRaceState<'a> {
    fn new(rd: &'a Reindeer) -> DeerRaceState {
        DeerRaceState {
            deer: rd,
            movement_state: DeerMovementState::Flying(rd.fly_time),
            points: 0,
            distance: 0
        }
    }

    fn tick(&mut self) {
        use self::DeerMovementState::{Flying, Resting};

        self.movement_state = match self.movement_state {
            Flying(1)  => { self.advance(); Resting(self.deer.rest_time) },
            Flying(n)  => { self.advance(); Flying(n - 1) },
            Resting(1) => Flying(self.deer.fly_time),
            Resting(n) => Resting(n-1),
        };
    }

    fn advance(&mut self) {
        self.distance += self.deer.speed;
    }
}

fn parse_reindeer(raw_deer: &str) -> Reindeer {
    use self::nom::*;

    use std::str::FromStr;
    use std::str::from_utf8;

    named!(number<u32>, map!(
        digit,
        |d| FromStr::from_str(from_utf8(d).unwrap()).unwrap())
    );
    named!(deer_name<String>, map!(
        take_while1!(is_alphabetic),
        |s| from_utf8(s).unwrap().to_string()
    ));
    named!(reindeer<Reindeer>, do_parse!(
        deer_name >>
        tag_s!(" can fly ") >>
        speed: number >>
        tag_s!(" km/s for ") >>
        fly_time: number >>
        tag_s!(" seconds, but then must rest for ") >>
        rest_time: number >>
        tag_s!(" seconds.") >>
        (Reindeer { speed: speed, fly_time: fly_time, rest_time: rest_time })
    ));

    match reindeer(raw_deer.as_bytes()) {
        IResult::Done(_, entry) => entry,
        _                       => panic!("Wrong map format")
    }
}

fn travelled_distance(reindeer: &Reindeer, total_time: Second) -> u32 {
    use std::cmp::min;

    let cycle_time = reindeer.fly_time + reindeer.rest_time;

    let total_fly_time = (total_time / cycle_time) * reindeer.fly_time +
                         min(total_time % cycle_time, reindeer.fly_time);

    total_fly_time * reindeer.speed
}

fn race_result(reindeers: &Vec<Reindeer>) -> Points {
    let mut deer_states = reindeers.iter().map(DeerRaceState::new)
                                          .collect::<Vec<_>>();

    for _ in 0..RACE_TIME {
        deer_states.iter_mut().for_each(DeerRaceState::tick);

        let max_distance = deer_states.iter().map(|st| st.distance)
                                             .max().unwrap_or(0);

        deer_states.iter_mut().filter(|st| st.distance == max_distance)
                              .for_each(|st| { st.points += 1; });
    }

    deer_states.iter().map(|st| st.points).max().unwrap_or(0)
}

pub fn p1(input: &str) -> u32 {
    input.trim().split('\n').map(parse_reindeer)
                            .map(|rd| travelled_distance(&rd, RACE_TIME))
                            .max().unwrap_or(0)
}

pub fn p2(input: &str) -> u32 {
    let reindeers = input.trim().split('\n')
                                .map(parse_reindeer)
                                .collect::<Vec<_>>();

    race_result(&reindeers)
}
