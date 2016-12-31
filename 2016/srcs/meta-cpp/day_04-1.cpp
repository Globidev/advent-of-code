#include "hana_boilerplate.hpp"

struct Room: TypeTuple<3> {
    static let name           = at<0>,
               sector_id      = at<1>,
               given_checksum = at<2>;
};

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

let parse_room = demux(Room::constructor)(
    extract_room_name,
    extract_room_sector_id >> to_int,
    extract_checksum
);

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

let is_room_valid = demux(equal)(
    compute_checksum,
    Room::given_checksum
);

let day_4_1_impl =
    (transform & parse_room) >>
    (filter & is_room_valid) >>
    (fold_left & 0_c & lockstep(plus)(id, Room::sector_id));

let day_4_1 = variadicly(day_4_1_impl);

int main() {
    // Example
    let check_room_description = parse_room >> is_room_valid;
    static_assert(check_room_description("aaaaa-bbb-z-y-x-123[abxyz]"_t) == true);
    static_assert(check_room_description("a-b-c-d-e-f-g-h-987[abcde]"_t) == true);
    static_assert(check_room_description("not-a-real-room-404[oarel]"_t) == true);
    static_assert(check_room_description("totally-real-room-200[decoy]"_t) == false);

    static_assert(day_4_1(
        "aaaaa-bbb-z-y-x-123[abxyz]"_t,
        "a-b-c-d-e-f-g-h-987[abcde]"_t,
        "not-a-real-room-404[oarel]"_t,
        "totally-real-room-200[decoy]"_t,
        "vxupkizork-sgmtkzoi-pkrrehkgt-zxgototm-644[kotgr]"_t
    ) == 123 + 987 + 404 + 644);
}
