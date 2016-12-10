#include "hana_boilerplate.hpp"

#define let_ const auto

let screen_w = 50_c;
let screen_h = 6_c;

let pixel_off = size_c<0>;
let pixel_on = size_c<1>;

let initial_screen = replicate<tuple_tag>(
    replicate<tuple_tag>(pixel_off, screen_w),
    screen_h
);

struct FillColumn {

    template <class Y, class W, class Screen, class Accum, class X>
    let operator()(Y y, W w, Screen screen, Accum accum, X x) const {
        let_ r = if_(x < w,
            pixel_on,
            at_c<x>(at_c<y>(screen))
        );

        return append(accum, r);
    }

};

let fill_column = FillColumn{};

struct FillRow {

    template <class W, class H, class Screen, class Accum, class Y>
    let operator()(W w, H h, Screen screen, Accum accum, Y y) const {
        let_ row = at_c<y>(screen);

        let_ r = if_(y < h,
            fold_left(
                make_range(size_c<0>, size(row)),
                make_tuple(),
                fill_column | y | w | screen
            ),
            at_c<y>(screen)
        );
        return append(accum, r);
    }

};

let fill_row = FillRow{};

template <unsigned w, unsigned h>
struct Rect {

    template <class Screen>
    let operator()(Screen s) const {
        return fold_left(
            make_range(0_c, size(s)),
            make_tuple(),
            fill_row | w | h | s
        );
    }

};

template <unsigned w, unsigned h>
let rect = Rect<w, h>{};

struct RowRotatorBy {

    template <class Delta, class Row>
    let operator()(Delta delta, Row row) const {
        let_ indexes = unpack(make_range(0_c, size(row)), make_tuple);
        let_ zipped = zip(row, reverse(indexes));
        let_ partitions = span(
            zipped,
            (at & 1_c) >> greater_equal.than(delta)
        );

        return concat(
            transform(second(partitions), at & 0_c),
            transform(first(partitions), at & 0_c)
        );
    }

};

let rotate_row_by = RowRotatorBy{};

struct Rotator {

    template <class TargetY, class DX, class Screen, class Accum, class Y>
    let operator()(TargetY ty, DX dx, Screen screen, Accum accum, Y y) const {
        let_ row = at_c<y>(screen);

        let_ r = if_(
            ty == y,
            rotate_row_by(dx, row),
            row
        );
        return append(accum, r);
    }

};

let rotate = Rotator{};

template <unsigned y, unsigned dx>
struct RotateRow {

    template <class Screen>
    let operator()(Screen s) const {
        return fold_left(
            make_range(0_c, size(s)),
            make_tuple(),
            rotate | uint_c<y> | uint_c<dx> | s
        );
    }

};

template <unsigned y, unsigned dx>
let rotate_row = RotateRow<y, dx>{};

template <unsigned x, unsigned dy>
struct RotateColumn {

    template <class Screen>
    let operator()(Screen s) const {
        let transpose = (unpack & zip);
        return transpose(rotate_row<x, dy>(transpose(s)));
    }

};

template <unsigned x, unsigned dy>
let rotate_column = RotateColumn<x, dy>{};

struct RectDescriptor {

    static let prefix = "rect"_t;

    template <class String>
    let operator()(String s) const {
        let_ remainder = drop_front(s, size(prefix) + size_c<1>); // space
        let not_x = not_equal.to(char_c<'x'>);

        let_ w = to_int(take_while(remainder, not_x));
        let_ h = to_int(drop_front(drop_while(remainder, not_x)));

        return rect<w, h>;
    }

};

struct RotateRowDescriptor {

    static let prefix = "rotate row"_t;

    template <class String>
    let operator()(String s) const {
        let_ remainder = drop_front(s, size(prefix) + size_c<3>); // " y="
        let not_space = not_equal.to(char_c<' '>);

        let_ y = to_int(take_while(remainder, not_space));
        let_ dx = (reverse >> (take_while & not_space) >> reverse >> to_int)(remainder);

        return rotate_row<y, dx>;
    }

};

struct RotateColumnDescriptor {

    static let prefix = "rotate column"_t;

    template <class String>
    let operator()(String s) const {
        let_ remainder = drop_front(s, size(prefix) + size_c<3>); // " x="
        let not_space = not_equal.to(char_c<' '>);

        let_ x = to_int(take_while(remainder, not_space));
        let_ dy = (reverse >> (take_while & not_space) >> reverse >> to_int)(remainder);

        return rotate_column<x, dy>;
    }

};

let instructions = make_tuple(
    RectDescriptor{},
    RotateRowDescriptor{},
    RotateColumnDescriptor{}
);

struct InstructionParser {

    template <class String>
    let operator()(String str) const {
        let_ descriptor = find_if(instructions, [=](auto descriptor) {
            let_ str_prefix = take_front(str, size(descriptor.prefix));

            return str_prefix == descriptor.prefix;
        });

        static_assert(is_just(descriptor), "bad instruction");
        return (*descriptor)(str);
    }

};

let parse_instruction = InstructionParser{};

let apply_instruction = flip(apply);

let day_8_0_impl =
    (transform & parse_instruction) >>
    (fold_left & initial_screen & apply_instruction) >>
    (fold_left & size_c<0> & lockstep(plus)(id, count_if & equal.to(pixel_on)))
    ;

let day_8_0 = variadicly(day_8_0_impl);

int main() {
    static_assert(day_8_0(
        "rect 1x1"_t,
        "rotate row y=0 by 3"_t,
        "rect 2x1"_t,
        "rotate row y=0 by 5"_t,
        "rotate column x=0 by 1"_t,
        "rect 4x1"_t,
        "rotate row y=1 by 12"_t,
        "rotate row y=0 by 10"_t,
        "rotate column x=0 by 1"_t,
        "rect 9x1"_t,
        "rotate column x=7 by 1"_t,
        "rotate row y=1 by 3"_t,
        "rotate row y=0 by 2"_t,
        "rect 1x2"_t,
        "rotate row y=1 by 3"_t,
        "rotate row y=0 by 1"_t,
        "rect 1x3"_t,
        "rotate column x=35 by 1"_t,
        "rotate column x=5 by 2"_t,
        "rotate row y=2 by 5"_t,
        "rotate row y=1 by 5"_t,
        "rotate row y=0 by 2"_t,
        "rect 1x3"_t
    ) == 24u);
}
