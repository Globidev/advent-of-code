#include "hana_boilerplate.hpp"

struct Room: Variadic<3> {
    static let name = getter<0>;
    static let sector_id = getter<1>;
    static let given_checksum = getter<2>;
};

let unpack_to_tuple = unpack & make_tuple;

let checksum_size           = size_c<5>;
let bracketed_checksum_size = checksum_size + size_c<2>;

let extract_checksum =
    (take_back & bracketed_checksum_size) >>
    drop_back >> drop_front;

let not_dash = not_equal.to(char_c<'-'>);

let extract_room_sector_id =
    reverse >>
    (drop_front & bracketed_checksum_size) >>
    (take_while & not_dash) >>
    reverse;

let extract_room_name =
    reverse >>
    (drop_while & not_dash) >>
    drop_front >>
    reverse;

let to_digit = minus & char_c<'0'>;
let to_int =
    (transform & to_digit) >>
    (fold_left & 0_c & lockstep(plus)(mult | 10_c, id));

let parse_room =
    unpack_to_tuple >>
    make_tuple >>
    (ap | make_tuple(
        extract_room_name,
        extract_room_sector_id >> to_int,
        extract_checksum
    )) >>
    (unpack & Room::constructor);

// The checksum is the 5 most common letters present in the ciphered name
let compute_checksum =
    Room::name >>
    // Remove the dashes
    (filter & not_dash) >>
    // Sort letters (to break ties)
    sort >>
    // Group letters by packs
    group >>
    // Sort the groups by length
    (sort & (greater ^on^ size)) >>
    // Return the common letter in the 5 first groups
    (transform & (at & 0_c)) >>
    (take_front & checksum_size);

let is_room_valid = demux(equal)(compute_checksum, Room::given_checksum);

let day_4_0_impl =
    (transform & parse_room) >>
    (filter & is_room_valid) >>
    (fold_left & 0_c & lockstep(plus)(id, Room::sector_id));

let day_4_0 = variadicly(day_4_0_impl);

int main() {
    // Example
    let check_room_description = parse_room >> is_room_valid;
    static_assert(check_room_description("aaaaa-bbb-z-y-x-123[abxyz]"_s) == true);
    static_assert(check_room_description("a-b-c-d-e-f-g-h-987[abcde]"_s) == true);
    static_assert(check_room_description("not-a-real-room-404[oarel]"_s) == true);
    static_assert(check_room_description("totally-real-room-200[decoy]"_s) == false);

    static_assert(day_4_0(
        "aaaaa-bbb-z-y-x-123[abxyz]"_s,
        "a-b-c-d-e-f-g-h-987[abcde]"_s,
        "not-a-real-room-404[oarel]"_s,
        "totally-real-room-200[decoy]"_s,
        "vxupkizork-sgmtkzoi-pkrrehkgt-zxgototm-644[kotgr]"_s
    ) == 123 + 987 + 404 + 644);
}
