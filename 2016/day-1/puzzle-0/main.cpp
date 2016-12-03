#include <boost/hana.hpp>

#define let const auto
#define let_ constexpr auto

// Data types
enum Direction { North, South, East, West };
enum Move      { Right, Left };

struct Position { int x, y; };
struct Vector2D { int dx, dy; };

struct Instruction {
    Move move;
    int block_count;
};

struct State {
    Position position;
    Direction direction;
};

// Problem
constexpr Vector2D
    v_north { 0, 1  },
    v_south { 0, -1 },
    v_east  { 1, 0  },
    v_west  { -1, 0 }
;

let_ turn_left(const Direction d) {
    switch(d) {
        case North: return West;
        case South: return East;
        case East:  return North;
        case West:  return South;
    }
}

let_ turn_right(const Direction d) {
    switch(d) {
        case North: return East;
        case South: return West;
        case East:  return South;
        case West:  return North;
    }
}

let_ direction_vector(const Direction d) {
    switch(d) {
        case North: return v_north;
        case South: return v_south;
        case East:  return v_east;
        case West:  return v_west;
    }
}

let_ move(const State s, const Instruction i) {
    let turn          = (i.move == Left ? turn_left : turn_right);
    let new_direction = turn(s.direction);
    let vector        = direction_vector(new_direction);
    let new_position  = Position {
        s.position.x + vector.dx * i.block_count,
        s.position.y + vector.dy * i.block_count,
    };

    return State { new_position, new_direction };
}

template <class T>
let_ const_abs(const T x) { return x >= 0 ? x : -x; }

let_ taxicab_distance(const Position p1, const Position p2) {
    return const_abs(p2.x - p1.x) +
           const_abs(p2.y - p1.y);
}

let_ day1_0() { return 0; }

template <class... Instructions>
let_ day1_0(const Instruction in, const Instructions... ins) {
    using namespace boost::hana;

    let instructions = make_tuple(in, ins...);

    let initial_state = State { Position { 0, 0 }, North };
    let final_state = fold_left(instructions, initial_state, move);

    return taxicab_distance(
        initial_state.position,
        final_state.position
    );
}

// Syntatic sugar
template <class... Params>
let_ day1_0(const Move m, const int i, const Params... params) {
    return day1_0(params..., Instruction { m, i });
}

int main(int, char **) {
    // Examples
    static_assert(day1_0(Right, 2, Left,  3)                     == 5 );
    static_assert(day1_0(Right, 2, Right, 2, Right, 2)           == 2 );
    static_assert(day1_0(Right, 5, Left,  5, Right, 5, Right, 3) == 12);

    // My input
    static_assert(day1_0(Left, 5, Right, 1, Right, 3, Left, 4, Right, 3, Right, 1, Left, 3, Left, 2, Right, 3, Left, 5, Left, 1, Left, 2, Right, 5, Left, 1, Right, 5, Right, 1, Left, 4, Right, 1, Right, 3, Left, 4, Left, 1, Right, 2, Right, 5, Right, 3, Right, 1, Right, 1, Left, 1, Right, 1, Left, 1, Left, 2, Left, 1, Right, 2, Left, 5, Left, 188, Left, 4, Right, 1, Right, 4, Left, 3, Right, 47, Right, 1, Left, 1, Right, 77, Right, 5, Left, 2, Right, 1, Left, 2, Right, 4, Left, 5, Left, 1, Right, 3, Right, 187, Left, 4, Left, 3, Left, 3, Right, 2, Left, 3, Left, 5, Left, 4, Left, 4, Right, 1, Right, 5, Left, 4, Left, 3, Left, 3, Left, 3, Left, 2, Left, 5, Right, 1, Left, 2, Right, 5, Left, 3, Left, 4, Right, 4, Left, 5, Right, 3, Right, 4, Left, 2, Left, 1, Left, 4, Right, 1, Left, 3, Right, 1, Right, 3, Left, 2, Right, 1, Right, 4, Right, 5, Left, 3, Right, 5, Right, 3, Left, 3, Right, 4, Left, 2, Left, 5, Left, 1, Left, 1, Right, 3, Right, 1, Left, 4, Right, 3, Right, 3, Left, 2, Right, 5, Right, 4, Right, 1, Right, 3, Left, 4, Right, 3, Right, 3, Left, 2, Left, 4, Left, 5, Right, 1, Left, 4, Left, 5, Right, 4, Left, 2, Left, 1, Left, 3, Left, 3, Left, 5, Right, 3, Left, 4, Left, 3, Right, 5, Right, 4, Right, 2, Left, 4, Right, 2, Right, 3, Left, 3, Right, 4, Left, 1, Left, 3, Right, 2, Right, 1, Right, 5, Left, 4, Left, 5, Left, 5, Right, 4, Left, 5, Left, 2, Left, 4, Right, 4, Right, 4, Right, 1, Left, 3, Left, 2, Left, 4, Right, 3) == 273);

    return 0;
}
