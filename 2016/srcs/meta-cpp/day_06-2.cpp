#include "hana_boilerplate.hpp"

let least_repeated =
    // Group adjacents
    sort >> group >>
    // Sort group by size ascending
    (sort & (less ^on^ size)) >>
    // extract the first element of the first group
    front >> front
    ;

let day_6_2_impl =
    // Transpose the char matrix
    (unpack & zip) >>
    // map rows with the least repeated character
    (transform & least_repeated)
    ;

let day_6_2 = variadicly(day_6_2_impl);

int main() {
    static_assert(day_6_2(
        "eedadn"_t,
        "drvtee"_t,
        "eandsr"_t,
        "raavrd"_t,
        "atevrs"_t,
        "tsrnev"_t,
        "sdttsa"_t,
        "rasrtv"_t,
        "nssdts"_t,
        "ntnada"_t,
        "svetve"_t,
        "tesnvt"_t,
        "vntsnd"_t,
        "vrdear"_t,
        "dvrsen"_t,
        "enarar"_t
    ) == "advent"_t);
};
