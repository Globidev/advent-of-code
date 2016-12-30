#include "hana_boilerplate.hpp"

let most_repeated =
    // Group adjacents
    sort >> group >>
    // Sort group by size descending
    (sort & (greater ^on^ size)) >>
    // extract the first element of the first group
    front >> front;

let day_6_1_impl =
    // Transpose the char matrix
    (unpack & zip) >>
    // map rows with the most repeated character
    (transform & most_repeated);

let day_6_1 = variadicly(day_6_1_impl);

int main() {
    static_assert(day_6_1(
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
    ) == "easter"_t);
}
