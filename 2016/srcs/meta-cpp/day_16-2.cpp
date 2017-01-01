#include "hana_boilerplate.hpp"

let c0 = char_c<'0'>;
let c1 = char_c<'1'>;

let invert_bit = (_ == c0) >> (if_ & c1 & c0);

let generate_more_data = demux(concat)(
    append & c0,
    reverse >> (transform & invert_bit)
);

let fill_disk = [](auto input, auto length) {
    return while_(
        size >> (_ < length),
        input,
        generate_more_data
    );
};

let pair_match = demux(_ == _)(_[0_c], _[1_c]) >> (if_ & c1 & c0);
let compute_checksum = chunks_of<2> >> (transform & pair_match);

let day_16 = [](auto input, auto length) {
    let data = fill_disk(input, size_c<length>);

    return while_(
        size >> to_integral<int> >> (_ % 2_c) >> (_ == 0_c),
        take_front(data, length),
        compute_checksum
    );
};

let day_16_2 = day_16 & 35651584_c;

int main() {
    // Example
    static_assert(
        day_16("10000"_t, 20_c)
        ==
        "01100"_t
    );

    // My input (takes way too long)
    // static_assert(
    //     day_16_2("10111100110001111"_t)
    //     ==
    //     "10111100110001111"_t
    // );
}
