#include "hana_boilerplate.hpp"

// Data type
struct Room: Variadic<3> {
    static let name = getter<0>;
    static let sector_id = getter<1>;
    static let given_checksum = getter<2>;
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

let a_ = char_c<'a'>;

// This one is painful to write in point free...
struct NRotator {

    template <class X, class C>
    let operator()(X x, C c) const {
        return if_(
            not_dash(c),
            (c - a_ + x) % 26_c + a_,
            char_c<' '>
        );
    }

};

let rotn = NRotator{};

let deciphered_room_name = demux(transform)(
    Room::name,
    Room::sector_id >> curry<2>(rotn)
);

let northpole_room_name = "northpole object storage"_t;

let is_north_pole_object_storage_room =
    deciphered_room_name >>
    equal.to(northpole_room_name);

let day_4_1_impl =
    (transform & parse_room) >>
    (find_if   & is_north_pole_object_storage_room) >>
    (ap        | just(Room::sector_id));

let day_4_1 = variadicly(day_4_1_impl);

int main() {
    static_assert(day_4_1(
        "dpmpsgvm-dboez-dpbujoh-dvtupnfs-tfswjdf-831[nzcoy]"_t,
        "wlqqp-irsszk-rercpjzj-815[bjyfk]"_t,
        "kyelcrga-aylbw-amyrgle-sqcp-rcqrgle-730[engxw]"_t,
        "ghkmaihex-hucxvm-lmhktzx-501[hmxka]"_t,
        "bgmxkgtmbhgte-cxeeruxtg-ftkdxmbgz-449[gtxbe]"_t,
        "udglrdfwlyh-iorzhu-vklsslqj-751[ldhrs]"_t,
        "fmsledevhsyw-fewoix-asvowlst-282[sewfl]"_t
    ) == just(501));
}
