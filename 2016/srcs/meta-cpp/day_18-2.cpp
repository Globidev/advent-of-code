#include "hana_boilerplate.hpp"

struct State: TypeTuple<2> {
    static let trap_count  = at<0>,
               current_row = at<1>;
};

let safe_c = char_c<'.'>;
let trap_c = char_c<'^'>;

let trap_patterns = make_set(
    "^^."_t,
    ".^^"_t,
    "^.."_t,
    "..^"_t
);

// Walls are safe tiles
let extend_row = (prepend & safe_c) >> (append & safe_c);

let interleaved_triplets = demux(zip_shortest)(
    drop_front & 0_c,
    drop_front & 1_c,
    drop_front & 2_c
);

let tile_type_from_triplet =
    (in & trap_patterns) >>
    (if_ & trap_c & safe_c);

let compute_next_row =
    extend_row >>
    interleaved_triplets >>
    (transform & tile_type_from_triplet);

let count_safe_tiles = count_if & (_ == safe_c);

let compute_next_state = demux(State::constructor)(
    demux(_ + _)(
        State::trap_count,
        State::current_row >> count_safe_tiles
    ),
    State::current_row >> compute_next_row
);

template <unsigned count>
let day_18 =
    (State::constructor | size_c<0>) >>
    (iterate<count> | compute_next_state) >>
    State::trap_count;

let day_18_2 = day_18<400'000u>;

int main() {
    // Example
    static_assert(
        day_18<10>(".^^.^.^^^^"_t)
        ==
        38ul
    );

    // My input (Takes way too much time/memory)
    // static_assert(
    //     day_18_2(".^.^..^......^^^^^...^^^...^...^....^^.^...^.^^^^....^...^^.^^^...^^^^.^^.^.^^..^.^^^..^^^^^^.^^^..^"_t)
    //     ==
    //     19984714ul
    // );
}
