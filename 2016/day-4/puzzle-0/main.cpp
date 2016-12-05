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
    let_ operator()(C c) const { return int_c<c.value - '0'>; }
};
let_ to_digit = ToDigitConverter { };

struct Multiplier {
    template <class A, class B>
    let_ operator()(A a, B b) const { return a * b; }
};
let_ multiply = Multiplier { };

let_ to_int = compose(
    reverse_partial(
        fold_left,
        0_c,
        lockstep(plus)(
            partial(
                multiply,
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

// The checksum is the 5 most common letters present in the ciphered name:
//   1) remove the dashes
//   2) sort letters (to break ties)
//   3) group letters by packs
//   4) sort the groups by length
//   5) return the letters in the 5 first groups
let_ compute_checksum = compose(
    // Return the first 5 letters (5)
    reverse_partial(
        take_front,
        checksum_size
    ),
    // Map each group to its corresponding letter (taking the first element)
    reverse_partial(
        transform,
        reverse_partial(
            at,
            0_c
        )
    ),
    // Sort groups by lengths descending (4)
    reverse_partial(
        sort,
        greater ^on^ size
    ),
    // Group letters (3)
    group,
    // Sort letters (2)
    sort,
    // Remove the dashes (1)
    reverse_partial(
        filter,
        not_dash
    ),
    // Use the room name
    Room::name
);

let_ is_room_valid = demux(equal)(compute_checksum, Room::given_checksum);

let_ day_4_0 = demux(
    compose(
        reverse_partial(
            fold_left,
            0_c,
            lockstep(plus)(
                id,
                Room::sector_id
            )
        ),
        reverse_partial(
            filter,
            is_room_valid
        ),
        reverse_partial(
            transform,
            parse_room
        )
    )
)(make_tuple);

int main() {
    // Example
    let_ check_room = compose(is_room_valid, parse_room);
    static_assert(check_room("aaaaa-bbb-z-y-x-123[abxyz]"_s) == true);
    static_assert(check_room("a-b-c-d-e-f-g-h-987[abcde]"_s) == true);
    static_assert(check_room("not-a-real-room-404[oarel]"_s) == true);
    static_assert(check_room("totally-real-room-200[decoy]"_s) == false);

    static_assert(day_4_0(
        "aaaaa-bbb-z-y-x-123[abxyz]"_s,
        "a-b-c-d-e-f-g-h-987[abcde]"_s,
        "not-a-real-room-404[oarel]"_s,
        "totally-real-room-200[decoy]"_s
    ) == 123 + 987 + 404);
}
