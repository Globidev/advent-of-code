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
                    Marker::length(parse_marker(marker)) *
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

let day_9_0 =
    mk_initial_state >>
    decompress_all >>
    State::decompressed_length;

int main() {
    static_assert(day_9_0("ADVENT"_t)            == 6ul);
    static_assert(day_9_0("A(1x5)BC"_t)          == 7ul);
    static_assert(day_9_0("(3x3)XYZ"_t)          == 9ul);
    static_assert(day_9_0("A(2x2)BCD(2x2)EFG"_t) == 11ul);
    static_assert(day_9_0("(6x1)(1x3)A"_t)       == 6ul);
    static_assert(day_9_0("X(8x2)(3x3)ABCY"_t)   == 18ul);
}