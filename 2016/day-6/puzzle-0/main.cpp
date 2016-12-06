#include <hana_boilerplate.hpp>

let unpack_to_tuple = unpack & make_tuple;
let unpack_to_string = unpack & make_string;

let most_repeated =
    // Group adjacents
    sort >> group >>
    // Sort group by size descending
    (sort & (greater ^on^ size)) >>
    // extract the first element of the first group
    front >> front
    ;

let day_6_0_impl =
    // Make a sequence out of the strings
    (transform & unpack_to_tuple) >>
    // Transpose the char matrix
    (unpack & zip) >>
    // map rows with the most repeated character
    (transform & most_repeated) >>
    // Make it back a string
    unpack_to_string
    ;

let day_6_0 = variadicly(day_6_0_impl);

int main() {
    static_assert(day_6_0(
        "eedadn"_s,
        "drvtee"_s,
        "eandsr"_s,
        "raavrd"_s,
        "atevrs"_s,
        "tsrnev"_s,
        "sdttsa"_s,
        "rasrtv"_s,
        "nssdts"_s,
        "ntnada"_s,
        "svetve"_s,
        "tesnvt"_s,
        "vntsnd"_s,
        "vrdear"_s,
        "dvrsen"_s,
        "enarar"_s
    ) == "easter"_s);
}
