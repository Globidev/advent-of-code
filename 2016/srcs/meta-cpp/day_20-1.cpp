#include <iostream>
#include "hana_boilerplate.hpp"

let range_start = uint_c<0>;
let range_end = uint_c<4294967295>;

struct IpRange: TypeTuple<2> {
    static let low  = at<0>,
               high = at<1>;
};

struct State: TypeTuple<2> {
    static let index = at<0>,
               high  = at<1>;
};

template <unsigned low, unsigned high>
let bl = IpRange::constructor(uint_c<low>, uint_c<high>);

let update_high = [](auto map, auto range) {
    let low = IpRange::low(range);
    let high = IpRange::high(range);
    let mb_high = find(map, low);

    if constexpr (is_just(mb_high))
        return map_replace(map, low, max(high, *mb_high));
    else
        return insert(map, make_pair(low, high));
};

let day_20_1_impl = [](auto ranges) {
    let high_by_lows = fold(ranges, make_map(), update_high);
    let lows = sort(keys(high_by_lows));

    let current_low = State::index >> (at | lows);
    let next_high = current_low >> (at_key | high_by_lows) >> (_ + uint_c<1>);

    let no_gap = demux(_ >= _)(
        State::high,
        current_low
    );

    let advance = demux(State::constructor)(
        State::index >> (_ + 1_c),
        demux(max)(State::high, next_high)
    );

    let state = while_(
        no_gap,
        State::constructor(0_c, range_start),
        advance
    );

    return State::high(state);
};

let day_20_1 = variadicly(day_20_1_impl);

int main() {
    // Example
    static_assert(
        day_20_1(
            bl<5, 8>,
            bl<0, 2>,
            bl<4, 7>
        )
        ==
        3u
    );

    // Modified example
    static_assert(
        day_20_1(
            bl<5, 8>,
            bl<21, 42>,
            bl<0, 3>,
            bl<12, 13>,
            bl<7, 15>,
            bl<14, 18>,
            bl<4, 7>
        )
        ==
        19u
    );

    // My input take too much time / memory
}
