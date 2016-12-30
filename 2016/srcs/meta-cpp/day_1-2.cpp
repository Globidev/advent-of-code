#include "hana_boilerplate.hpp"

// Data types
enum Direction { North, South, East, West };
enum Move      { Right, Left, Ahead };

template <Direction d>
let dir_c = integral_constant<Direction, d>{};
template <Move m>
let move_c = integral_constant<Move, m>{};

struct Position: TypeTuple<2> {
    static let x = at<0>,
               y = at<1>;
};

struct Vector2D: TypeTuple<2> {
    static let dx = at<0>,
               dy = at<1>;
};

struct Instruction: TypeTuple<2> {
    static let movement    = at<0>,
               block_count = at<1>;
};

struct State: TypeTuple<2> {
    static let position  = at<0>,
               direction = at<1>;
};

let north_c = dir_c<North>;
let south_c = dir_c<South>;
let east_c  = dir_c<East>;
let west_c  = dir_c<West>;

let right_c = move_c<Right>;
let left_c  = move_c<Left>;
let ahead_c = move_c<Ahead>;

let v_north = Vector2D::constructor(int_c<0>,  int_c<1> );
let v_south = Vector2D::constructor(int_c<0>,  int_c<-1>);
let v_east  = Vector2D::constructor(int_c<1>,  int_c<0> );
let v_west  = Vector2D::constructor(int_c<-1>, int_c<0> );

template <unsigned bc> let right = Instruction::constructor(right_c, uint_c<bc>);
template <unsigned bc> let left  = Instruction::constructor(left_c,  uint_c<bc>);
template <unsigned bc> let ahead = Instruction::constructor(ahead_c, uint_c<bc>);

// Problem
let turn_right = switch_ &
    case_(north_c, east_c) &
    case_(south_c, west_c) &
    case_(east_c,  south_c) &
    case_(west_c,  north_c);

let turn_left = switch_ &
    case_(north_c, west_c) &
    case_(south_c, east_c) &
    case_(east_c,  north_c) &
    case_(west_c,  south_c);

let turn_logic = switch_ &
    case_(right_c, turn_right) &
    case_(left_c,  turn_left) &
    case_(ahead_c, id);

let direction_vector = switch_ &
    case_(north_c, v_north) &
    case_(south_c, v_south) &
    case_(east_c,  v_east) &
    case_(west_c,  v_west);

let move = [](auto state, auto instruction) {
    let turn          = turn_logic(Instruction::movement(instruction));
    let new_direction = turn(State::direction(state));
    let vector        = direction_vector(new_direction);
    let block_count   = Instruction::block_count(instruction);
    let current_pos   = State::position(state);

    let nx = Position::x(current_pos) + Vector2D::dx(vector) * int_c<block_count>,
        ny = Position::y(current_pos) + Vector2D::dy(vector) * int_c<block_count>;

    return State::constructor(Position::constructor(nx, ny), new_direction);
};

let const_abs = [](auto x) { return x >= 0 ? x : -x; };

let taxicab_distance = [](auto p1, auto p2) {
    return const_abs(Position::x(p2) - Position::x(p1)) +
           const_abs(Position::y(p2) - Position::y(p1));
};

let position_equal = [](auto p1, auto p2) {
    return Position::x(p1) == Position::x(p2) &&
           Position::y(p1) == Position::y(p2);
};

let find_first_repeating = [](auto s) {
    let count_by_equal_position = curry<2>(position_equal) >> (count_if | s);
    let counts                  = transform(s, count_by_equal_position);

    let count_is_at_least_2 = (at & 1_c) >> greater_equal.than(size_c<2>);
    let mb_pos_count_pair   = find_if(zip(s, counts), count_is_at_least_2);

    return at_c<0>(*mb_pos_count_pair);
};

let flatten_instruction = [](auto instruction) {
    let movement = Instruction::movement(instruction);

    return concat(
        make_tuple(Instruction::constructor(movement, uint_c<1>)),
        cycle(
            make_tuple(ahead<1>),
            size_c<Instruction::block_count(instruction) - uint_c<1>>
        )
    );
};

let initial_position = Position::constructor(0_c, 0_c);
let initial_state = State::constructor(initial_position, north_c);

let day1_1_impl =
    (transform & flatten_instruction) >> flatten >>
    (scan_left & initial_state & move) >>
    (transform & State::position) >> find_first_repeating >>
    (taxicab_distance | initial_position)
    ;

let day1_1 = variadicly(day1_1_impl);

int main(int, char **) {
    // Example
    static_assert(day1_1(right<8>, right<4>, right<4>, right<8>) == 4);

    // My input (Doesn't compile in decent time/memory)
    // static_assert(day1_1(
    //     left<5>, right<1>, right<3>, left<4>, right<3>, right<1>, left<3>,
    //     left<2>, right<3>, left<5>, left<1>, left<2>, right<5>, left<1>,
    //     right<5>, right<1>, left<4>, right<1>, right<3>, left<4>, left<1>,
    //     right<2>, right<5>, right<3>, right<1>, right<1>, left<1>, right<1>,
    //     left<1>, left<2>, left<1>, right<2>, left<5>, left<188>, left<4>,
    //     right<1>, right<4>, left<3>, right<47>, right<1>, left<1>, right<77>,
    //     right<5>, left<2>, right<1>, left<2>, right<4>, left<5>, left<1>,
    //     right<3>, right<187>, left<4>, left<3>, left<3>, right<2>, left<3>,
    //     left<5>, left<4>, left<4>, right<1>, right<5>, left<4>, left<3>,
    //     left<3>, left<3>, left<2>, left<5>, right<1>, left<2>, right<5>,
    //     left<3>, left<4>, right<4>, left<5>, right<3>, right<4>, left<2>,
    //     left<1>, left<4>, right<1>, left<3>, right<1>, right<3>, left<2>,
    //     right<1>, right<4>, right<5>, left<3>, right<5>, right<3>, left<3>,
    //     right<4>, left<2>, left<5>, left<1>, left<1>, right<3>, right<1>,
    //     left<4>, right<3>, right<3>, left<2>, right<5>, right<4>, right<1>,
    //     right<3>, left<4>, right<3>, right<3>, left<2>, left<4>, left<5>,
    //     right<1>, left<4>, left<5>, right<4>, left<2>, left<1>, left<3>,
    //     left<3>, left<5>, right<3>, left<4>, left<3>, right<5>, right<4>,
    //     right<2>, left<4>, right<2>, right<3>, left<3>, right<4>, left<1>,
    //     left<3>, right<2>, right<1>, right<5>, left<4>, left<5>, left<5>,
    //     right<4>, left<5>, left<2>, left<4>, right<4>, right<4>, right<1>,
    //     left<3>, left<2>, left<4>, right<3>
    // ) == 115);

    return 0;
}
