#include "hana_boilerplate.hpp"

struct State: TypeTuple<2> {
    static let input               = at<0>,
               decompressed_length = at<1>;
};

struct Marker: TypeTuple<2> {
    static let length = at<0>,
               times  = at<1>;
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

let parse_marker = [](auto str) {
    let_ rest = drop_front(str);
    let_ length = parse_marker_length(rest);
    let_ times = parse_marker_times(rest);

    return Marker::constructor(size_c<length>, size_c<times>);
};

let decompress_step = [](auto state, auto prefix, auto rest) {
    let_ next_size = State::decompressed_length(state) + size(prefix);

    let_ marker_splitted = span(rest, not_equal.to(char_c<')'>));
    let_ mb_marker = first(marker_splitted);
    let_ marker_size = size(mb_marker) + size_c<1>; // Account for the ')'
    let_ remainder = drop_front(second(marker_splitted)); // Skip the ')'

    // Check if it's a marker and not a dummy paren
    if constexpr (marker_size >= size_c<5> && (char_c<'x'> ^in^ mb_marker)) {
        let_ marker = parse_marker(mb_marker);

        return State::constructor(
            drop_front(remainder, Marker::length(marker)),
            next_size + Marker::length(marker) * Marker::times(marker)
        );
    }
    else
        return State::constructor(remainder, next_size + marker_size);
};

let decompress_state = [](auto state) {
    let_ splitted = span(State::input(state), not_equal.to(char_c<'('>));
    let_ prefix = first(splitted);
    let_ rest = second(splitted);

    let_ next_size = State::decompressed_length(state) + size(prefix);

    if constexpr (size(rest) == size_c<0>)
        return State::constructor(""_t, next_size);
    else
        return decompress_step(state, prefix, rest);
};

let state_input_remains =
    State::input >> size >> greater.than(size_c<0>);

let decompress_all =
    (while_ | state_input_remains) & decompress_state;

let day_9_1 =
    mk_initial_state >>
    decompress_all >>
    State::decompressed_length;

int main() {
    static_assert(day_9_1("ADVENT"_t)            == 6ul);
    static_assert(day_9_1("A(1x5)BC"_t)          == 7ul);
    static_assert(day_9_1("(3x3)XYZ"_t)          == 9ul);
    static_assert(day_9_1("A(2x2)BCD(2x2)EFG"_t) == 11ul);
    static_assert(day_9_1("(6x1)(1x3)A"_t)       == 6ul);
    static_assert(day_9_1("X(8x2)(3x3)ABCY"_t)   == 18ul);
}
