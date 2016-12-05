#define let const auto
#define let_ constexpr auto

#define BOOST_HANA_CONFIG_ENABLE_STRING_UDL
#include <boost/hana.hpp>
using namespace boost::hana;
using namespace boost::hana::literals;

template <std::size_t count>
struct Variadic {

    struct Constructor {
        template <class... Ts>
        let_ operator()(Ts...) const {
            static_assert(sizeof...(Ts) == count);
            return tuple<Ts...> { };
        }
    };

    static let_ constructor = Constructor { };

    template <std::size_t i>
    struct Getter {
        template <class... Ts>
        let_ operator()(const tuple<Ts...> t) const { return at_c<i>(t); }
    };

    template <std::size_t i>
    static let_ getter = Getter<i> { };

};

struct Room: Variadic<3> {
    static let_ name = getter<0>;
    static let_ sector_id = getter<1>;
    static let_ given_checksum = getter<2>;
};

constexpr decltype(Room::name) Room::name;
constexpr decltype(Room::sector_id) Room::sector_id;
constexpr decltype(Room::given_checksum) Room::given_checksum;

struct ToDigitConverter {
    template <class C>
    let_ operator()(C c) const { return int_c<c - '0'>; }
};
let_ to_digit = ToDigitConverter { };

let_ to_int = compose(
    reverse_partial(
        fold_left,
        0_c,
        lockstep(plus)(
            partial(
                mult,
                10_c
            ),
            id
        )
    ),
    reverse_partial(
        transform,
        to_digit
    )
);

// The checksum is between square brackets at the end of the description
// => take the (size + 2) last characters and remove the surrounding brackets
let_ checksum_size           = size_c<5>;
let_ bracketed_checksum_size = checksum_size + size_c<2>;

let_ extract_checksum = compose(
    // Remove both ends
    compose(
        drop_front,
        drop_back
    ),
    // Keep the bracketed checksum
    reverse_partial(
        take_back,
        bracketed_checksum_size
    )
);

// From right to left, the sector id is immediately after the bracketed checksum
// up until the first encountered dash character
let_ not_dash = not_equal.to(char_c<'-'>);

let_ extract_room_sector_id = compose(
    // Revert back
    reverse,
    // Keep until first dash
    reverse_partial(
        take_while,
        not_dash
    ),
    // Skip the bracketed checksum
    reverse_partial(
        drop_front,
        bracketed_checksum_size
    ),
    // From right to left
    reverse
);

let_ extract_room_name = compose(
    reverse,
    drop_front,
    reverse_partial(drop_while, not_dash),
    reverse
);

// struct RoomParser {

//     template <class String>
//     let_ operator()(String raw_room_description) const {
//         let_ room_description = unpack(raw_room_description, make<tuple_tag>);

//         let_ room_checksum = extract_checksum(room_description);
//         let_ room_sector_id = extract_room_sector_id(room_description);
//         // The name starts at the beginning and is what's left when you remove the
//         // bracketed checksum and the sector with its separating dash
//         let_ size_to_drop = (
//             bracketed_checksum_size +        // Bracketed checksum size
//             size(room_sector_id) + size_c<1> // Extra separating dash
//         );
//         let_ room_name = drop_back(room_description, size_to_drop);

//         return Room::constructor(room_name, to_int(room_sector_id), room_checksum);
//     }

// };

let_ parse_room = compose(
    reverse_partial(
        unpack,
        Room::constructor
    ),
    partial(
        ap,
        make_tuple(
            extract_room_name,
            compose(to_int, extract_room_sector_id),
            extract_checksum
        )
    ),
    make_tuple,
    reverse_partial(
        unpack,
        make<tuple_tag>
    )
);

// let_ rot = compose(
//     reverse_partial(
//         if_,
//         ,
//         char_c<' '>
//     ),
//     not_dash
// )

struct Rot {

    template <class X, class C>
    constexpr typename std::enable_if<not_dash(C{}), decltype(char_c<((C{}.value - 'a' + X{}) % 26 + 'a')>)>::type
    operator()(X x, C c) const {
        return char_c<((c.value - 'a' + x) % 26 + 'a')>;
    }

    template <class X, class C>
    constexpr typename std::enable_if<!not_dash(C{}), decltype(char_c<' '>)>::type
    operator()(X x, C c) const {
        return char_c<' '>;
    }

};

// let_ deciphered_room_name = compose(
//     reverse_partial(
//         transform,
//         partial(
//             rot,
//             Room::sector_id
//         )
//     ),
//     Room::name
// )

// curry<2>(rot)(room_sector_id(room))

let_ rot = Rot{};
let_ deciphered_room_name = compose(
    reverse_partial(unpack, transform),
    partial(
        ap,
        make_tuple(
            Room::name,
            compose(
                curry<2>(rot),
                Room::sector_id
            )
        )
    ),
    make_tuple
);

// struct RoomDecipherer {
//     template <class R>
//     let_ operator()(R room) const {
//         return transform(
//             Room::name(room),
//             partial(Rot{}, Room::sector_id(room))
//         );
//     }
// };

let_ is_north_pole_object_storage_room = compose(
    equal.to(unpack("northpole object storage"_s, make<tuple_tag>)),
    deciphered_room_name
);

let_ day_4_1 = demux(
    compose(
        partial(ap, just(Room::sector_id)),
        reverse_partial(
            find_if,
            is_north_pole_object_storage_room
        ),
        reverse_partial(
            transform,
            parse_room
        )
    )
)(make_tuple);

int main() {
    // Example

    static_assert(day_4_1(
        "dpmpsgvm-dboez-dpbujoh-dvtupnfs-tfswjdf-831[nzcoy]"_s
        "wlqqp-irsszk-rercpjzj-815[bjyfk]"_s,
        "kyelcrga-aylbw-amyrgle-sqcp-rcqrgle-730[engxw]"_s,
        "ghkmaihex-hucxvm-lmhktzx-501[hmxka]"_s,
        "bgmxkgtmbhgte-cxeeruxtg-ftkdxmbgz-449[gtxbe]"_s,
        "udglrdfwlyh-iorzhu-vklsslqj-751[ldhrs]"_s,
        "fmsledevhsyw-fewoix-asvowlst-282[sewfl]"_s
    ) == just(501));

    // static_assert(lift<integral_constant_tag<int>>(4) == 4_c);

    // static_assert(deciphered_room_name(parse_room("ghkmaihex-hucxvm-lmhktzx-501[hmxka]"_s))
    //     ==
    //     unpack("northpole object storage"_s, make<tuple_tag>));

    // let_ f = lockstep(transform)(Room::name, id);
    // static_assert(f() == );
}
