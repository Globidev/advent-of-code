#include <boost/hana.hpp>

#define let const auto
#define let_ constexpr auto

// Data types
enum Direction { North, South, East, West };

template <int x_, int y_>
struct Position {
    constexpr Position() = default;

    static let_ x = x_,
                y = y_;
};

template <int dx_, int dy_>
struct Vector2D {
    constexpr Vector2D() = default;

    static let_ dx = dx_,
                dy = dy_;
};

template <int bc>
struct Movement { static let_ block_count = bc; };

template <int bc>
struct Right: Movement<bc> {
    constexpr Right() = default;

    static let_ turn(Direction d) {
        switch(d) {
            case North: return East;
            case South: return West;
            case East:  return South;
            case West:  return North;
        }
    }

};

template <int bc>
struct Left: Movement<bc> {
    constexpr Left() = default;

    static let_ turn(Direction d) {
        switch(d) {
            case North: return East;
            case South: return West;
            case East:  return South;
            case West:  return North;
        }
    }

};

template <int bc>
struct Ahead: Movement<bc> {
    constexpr Ahead() = default;

    static let_ turn(Direction d) {
        return d;
    }

};

template <int bc> constexpr Right<bc> right;
template <int bc> constexpr Left<bc>  left;
template <int bc> constexpr Ahead<bc> ahead;

template <int x, int y, Direction d>
struct State { };

// Problem
let_ v_north = Vector2D<0,  1> { };
let_ v_south = Vector2D<0, -1> { };
let_ v_east  = Vector2D<1,  0> { };
let_ v_west  = Vector2D<-1, 0> { };

template <Direction d>
constexpr typename std::enable_if<d == North, decltype(v_north)>::type
direction_vector() { return v_north; }

template <Direction d>
constexpr typename std::enable_if<d == South, decltype(v_south)>::type
direction_vector() { return v_south; }

template <Direction d>
constexpr typename std::enable_if<d == East, decltype(v_east)>::type
direction_vector() { return v_east; }

template <Direction d>
constexpr typename std::enable_if<d == West, decltype(v_west)>::type
direction_vector() { return v_west; }

struct MovementResolver {

    template <class Movement, int x, int y, Direction d>
    let_ operator()(State<x, y, d> s, Movement) const {
        let_ new_direction = Movement::turn(d);
        let_ vector        = direction_vector<new_direction>();
        let_ nx = x + vector.dx * Movement::block_count,
             ny = y + vector.dy * Movement::block_count;

        return State<nx, ny, new_direction> { };
    }

};

struct MovementFlattener {

    template <int count, template <int> class Movement>
    let_ operator()(Movement<count>) const {
        using namespace boost::hana;

        return concat(
            make_tuple(Movement<1>{ }),
            cycle(make_tuple(ahead<1>), size_c<count - 1>)
        );
    }

};

template <class T>
let_ const_abs(const T x) { return x >= 0 ? x : -x; }

template <int x1, int x2, int y1, int y2>
let_ taxicab_distance(const Position<x1, y1>, const Position<x2, y2>) {
    return const_abs(x2 - x1) +
           const_abs(y2 - y1);
}

struct PositionGetter {

    template <int x, int y, Direction d>
    let_ operator()(State<x, y, d> s) const {
        return Position<x, y> {};
    }

};

struct PositionEqual {

    template <int x1, int x2, int y1, int y2>
    constexpr typename std::enable_if<(x1 == x2 && y1 == y2), decltype(boost::hana::true_c)>::type
    operator()(const Position<x1, y1>, const Position<x2, y2>) const {
        return boost::hana::true_c;
    }

    template <int x1, int x2, int y1, int y2>
    constexpr typename std::enable_if<(x1 != x2 || y1 != y2), decltype(boost::hana::false_c)>::type
    operator()(const Position<x1, y1>, const Position<x2, y2>) const {
        return boost::hana::false_c;
    }

};


template <class Sequence>
let_ find_first_repeating(const Sequence s) {
    using namespace boost::hana;
    using namespace boost::hana::literals;

    let count_by_equal_position = compose(partial(count_if, s), curry<2>(PositionEqual{}));
    let counts = transform(s, count_by_equal_position);

    let count_is_at_least_2 = compose(partial(flip(greater_equal), ulong_c<2>), partial(flip(at), 1_c));
    let mb_pos_count_pair = find_if(zip(s, counts), count_is_at_least_2);
    let default_value = make_tuple(at_c<0>(s), 0);

    return at_c<0>(mb_pos_count_pair.value_or(default_value));
}

template <class... Instructions>
let_ day1_1(const Instructions... ins) {
    using namespace boost::hana;

    let instructions = make_tuple(ins...);
    let flattened_instructions = fold_left(
        transform(instructions, MovementFlattener{}),
        concat
    );

    let initial_state = State<0, 0, North> { };
    let states = scan_left(
        flattened_instructions,
        initial_state,
        MovementResolver{}
    );

    let positions = transform(states, PositionGetter{});
    let first_repeated_position = find_first_repeating(positions);

    return taxicab_distance(
        Position<0, 0> { },
        first_repeated_position
    );
}

int main(int, char **) {
    // Example
    static_assert(day1_1(right<8>, right<4>, right<4>, right<8>) == 4);

    // My input
    // static_assert(day1_1(left<5>, right<1>, right<3>, left<4>, right<3>, right<1>, left<3>, left<2>, right<3>, left<5>, left<1>, left<2>, right<5>, left<1>, right<5>, right<1>, left<4>, right<1>, right<3>, left<4>, left<1>, right<2>, right<5>, right<3>, right<1>, right<1>, left<1>, right<1>, left<1>, left<2>, left<1>, right<2>, left<5>, left<188>, left<4>, right<1>, right<4>, left<3>, right<47>, right<1>, left<1>, right<77>, right<5>, left<2>, right<1>, left<2>, right<4>, left<5>, left<1>, right<3>, right<187>, left<4>, left<3>, left<3>, right<2>, left<3>, left<5>, left<4>, left<4>, right<1>, right<5>, left<4>, left<3>, left<3>, left<3>, left<2>, left<5>, right<1>, left<2>, right<5>, left<3>, left<4>, right<4>, left<5>, right<3>, right<4>, left<2>, left<1>, left<4>, right<1>, left<3>, right<1>, right<3>, left<2>, right<1>, right<4>, right<5>, left<3>, right<5>, right<3>, left<3>, right<4>, left<2>, left<5>, left<1>, left<1>, right<3>, right<1>, left<4>, right<3>, right<3>, left<2>, right<5>, right<4>, right<1>, right<3>, left<4>, right<3>, right<3>, left<2>, left<4>, left<5>, right<1>, left<4>, left<5>, right<4>, left<2>, left<1>, left<3>, left<3>, left<5>, right<3>, left<4>, left<3>, right<5>, right<4>, right<2>, left<4>, right<2>, right<3>, left<3>, right<4>, left<1>, left<3>, right<2>, right<1>, right<5>, left<4>, left<5>, left<5>, right<4>, left<5>, left<2>, left<4>, right<4>, right<4>, right<1>, left<3>, left<2>, left<4>, right<3>) == 115);

    return 0;
}
