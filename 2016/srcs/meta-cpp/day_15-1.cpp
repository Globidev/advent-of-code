#include "hana_boilerplate.hpp"

struct Disc: TypeTuple<3> {
    static let positions   = at<0>,
               pos_offset  = at<1>,
               time_offset = at<2>;
};

template <int p, int o>
let disc = Disc::constructor(int_c<p>, int_c<o>, nothing);

let add_time_offset = [](auto indexed_disc) {
    return Disc::time_offset(
        indexed_disc[0_c],
        always(int_c<indexed_disc[1_c]> + 1_c)
    );
};

let disc_offset = demux(_ + _)(
    Disc::pos_offset,
    Disc::time_offset
);

let chinese_x = [](auto B, auto b) {
    let B_ = B % b;

    let r = while_(
        (_ % B_) >> (_ != 0_c),
        1_c,
        _ + b
    );

    return r / B_;
};

let chinese_remainder = [](auto discs) {
    let bs = transform(discs, Disc::positions);
    let cs = transform(discs, disc_offset);

    let B  = fold(bs, (_ * _));
    let Bs = transform(bs, (B / _));

    let Xs = transform(zip(Bs, bs), unpack & chinese_x);

    let product = transform(zip(Bs, Xs, cs), fold & (_ * _));
    let rest = fold(product, (_ + _));

    return B - rest % B;
};

let day_15_1_impl =
    zip_with_index >>
    (transform & add_time_offset) >>
    chinese_remainder;

let day_15_1 = variadicly(day_15_1_impl);

int main() {

    static_assert(
        day_15_1(
            disc<5, 4>,
            disc<2, 1>
        )
        ==
        5
    );

    static_assert(
        day_15_1(
            disc<17, 1>,
            disc<7,  0>,
            disc<19, 2>,
            disc<5,  0>,
            disc<3,  0>,
            disc<13, 5>
        )
        ==
        317371
    );
}

