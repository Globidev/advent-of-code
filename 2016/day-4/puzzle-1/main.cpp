#include "boilerplate.hpp"

// Data type
struct Room: Variadic<3> {
    static let_ name = getter<0>;
    static let_ sector_id = getter<1>;
    static let_ given_checksum = getter<2>;
};

let_ unpack_to_tuple = unpack & make_tuple;

let_ checksum_size           = size_c<5>;
let_ bracketed_checksum_size = checksum_size + size_c<2>;

let_ extract_checksum =
    (take_back & bracketed_checksum_size) >>
    drop_back >> drop_front;

let_ not_dash = not_equal.to(char_c<'-'>);

let_ extract_room_sector_id =
    reverse >>
    (drop_front & bracketed_checksum_size) >>
    (take_while & not_dash) >>
    reverse;

let_ extract_room_name =
    reverse >>
    (drop_while & not_dash) >>
    drop_front >>
    reverse;

let_ to_digit = minus & char_c<'0'>;
let_ to_int =
    (transform & to_digit) >>
    (reverse_partial(
        fold_left, 0_c,
        lockstep(plus)(mult | 10_c, id)
    ));

let_ parse_room =
    unpack_to_tuple >>
    make_tuple >>
    (ap | make_tuple(
        extract_room_name,
        extract_room_sector_id >> to_int,
        extract_checksum
    )) >>
    (unpack & Room::constructor);

let_ a_ = char_c<'a'>;

// This one is painfull to write in point free...
struct NRotator {
    template <class X, class C>
    let_ operator()(X x, C c) const {
        return if_(
            not_dash(c),
            (c - a_ + x) % 26_c + a_,
            char_c<' '>
        );
    }
};
let_ rotn = NRotator{};

let_ deciphered_room_name =
    make_tuple >>
    (ap | make_tuple(
        Room::name,
        (Room::sector_id >> curry<2>(rotn))
    )) >>
    (unpack & transform);

let_ northpole_room_name = unpack_to_tuple("northpole object storage"_s);

let_ is_north_pole_object_storage_room =
    deciphered_room_name >>
    equal.to(northpole_room_name);

let_ day_4_1_impl =
    (transform & parse_room) >>
    (find_if   & is_north_pole_object_storage_room) >>
    (ap        | just(Room::sector_id));

let_ day_4_1 = variadicly(day_4_1_impl);

int main() {
    static_assert(day_4_1(
        "dpmpsgvm-dboez-dpbujoh-dvtupnfs-tfswjdf-831[nzcoy]"_s
        "wlqqp-irsszk-rercpjzj-815[bjyfk]"_s,
        "kyelcrga-aylbw-amyrgle-sqcp-rcqrgle-730[engxw]"_s,
        "ghkmaihex-hucxvm-lmhktzx-501[hmxka]"_s,
        "bgmxkgtmbhgte-cxeeruxtg-ftkdxmbgz-449[gtxbe]"_s,
        "udglrdfwlyh-iorzhu-vklsslqj-751[ldhrs]"_s,
        "fmsledevhsyw-fewoix-asvowlst-282[sewfl]"_s
    ) == just(501));
}
