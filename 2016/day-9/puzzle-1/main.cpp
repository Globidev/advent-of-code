#include "hana_boilerplate.hpp"

#define let_ const auto

struct State: Variadic<2> {
    static let input = getter<0>;
    static let decompressed_length = getter<1>;
};

struct Marker: Variadic<2> {
    static let length = getter<0>;
    static let times = getter<1>;
};

struct Day9_1Forward;

let mk_initial_state = State::constructor & size_c<0>;

let marker_separator = char_c<'x'>;
let parse_marker_length =
    (take_while & not_equal.to(marker_separator)) >>
    to_int;

let parse_marker_times =
    reverse >>
    (take_while & not_equal.to(marker_separator)) >>
    reverse >>
    to_int;

struct MarkerParser {

    template <class String>
    let operator()(String str) const {
        let_ rest = drop_front(str);
        let_ length = parse_marker_length(rest);
        let_ times = parse_marker_times(rest);

        return Marker::constructor(size_c<length>, size_c<times>);
    }

};

let parse_marker = MarkerParser{};

struct StepDecompressor {

    template <class SomeState, class Prefix, class Rest>
    let operator()(SomeState state, Prefix prefix, Rest rest) const {
        let_ next_size = State::decompressed_length(state) + size(prefix);

        let_ marker_splitted = span(rest, not_equal.to(char_c<')'>));
        let_ marker = first(marker_splitted);
        let_ marker_size = size(marker) + size_c<1>; // Account for the ')'
        let_ remainder = drop_front(second(marker_splitted)); // Skip the ')'

        // Check if it's a marker and not a dummy paren
        return if_(
            marker_size >= size_c<5> && (char_c<'x'> ^in^ marker),
            State::constructor(
                drop_front(remainder, Marker::length(parse_marker(marker))),
                next_size +
                    Day9_1Forward{}(take_front(remainder, Marker::length(parse_marker(marker)))) *
                    Marker::times(parse_marker(marker))
            ),
            State::constructor(remainder, next_size + marker_size)
        );
    }

};

let decompress_step = StepDecompressor{};

struct StateDecompressor {
    template <class SomeState>
    let operator()(SomeState state) const {
        let_ splitted = span(State::input(state), not_equal.to(char_c<'('>));
        let_ prefix = first(splitted);
        let_ rest = second(splitted);

        let_ next_size = State::decompressed_length(state) + size(prefix);

        return if_(
            size(rest) == size_c<0>,
            State::constructor(""_t, next_size),
            decompress_step(state, prefix, rest)
        );
    }
};

let decompress_state = StateDecompressor{};

let state_input_remains =
    State::input >> size >> greater.than(size_c<0>);

let decompress_all =
    (while_ | state_input_remains) & decompress_state;

let day_9_1 =
    mk_initial_state >>
    decompress_all >>
    State::decompressed_length;

// We need to call day_9_1 recursively above
// We have to wrap it in a functor struct to forward declare it
struct Day9_1Forward {
    template <class T>
    let operator()(T t) const {
        return day_9_1(t);
    }
};

int main() {
    static_assert(day_9_1("(3x3)XYZ"_t)                                                 == 9ul);
    static_assert(day_9_1("X(8x2)(3x3)ABCY"_t)                                          == 20ul);
    static_assert(day_9_1("(27x12)(20x12)(13x14)(7x10)(1x12)A"_t)                       == 241920ul);
    static_assert(day_9_1("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"_t) == 445ul);
}
