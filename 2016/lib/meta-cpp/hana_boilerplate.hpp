#define let constexpr auto
#define let_ const auto

#define BOOST_HANA_CONFIG_ENABLE_STRING_UDL

#include <boost/hana.hpp>

using namespace boost::hana;
using namespace boost::hana::literals;

// Some glue operators
template <class F1, class F2>
let operator<<(F1 f1, F2 f2) { return compose(f1, f2); }

template <class F1, class A>
let operator|(F1 f1, A a) { return partial(f1, a); }

template <class F1, class A>
let operator&(F1 f1, A a) { return flip(f1) | a; }

template <class F1, class F2>
let operator>>(F1 f1, F2 f2) { return compose(f2, f1); }

// More transformers
let zip_with_index = demux(zip)(
    id,
    size >> (make_range | size_c<0>) >> to_tuple
);

template <std::size_t i>
let chunks_of =
    zip_with_index >>
    (group & comparing([](auto p) { return at(p, 1_c) / size_c<i>; })) >>
    (transform & (transform & (at & 0_c)));

template <std::size_t i>
let replace_at = [](auto s, auto v) {
    let zipped = zip_with_index(s);
    let replaced = replace_if(
        zipped,
        [](auto p) { return at(p, 1_c) == size_c<i>; },
        make_tuple(v, size_c<i>)
    );

    return transform(replaced, at & 0_c);
};

template <std::size_t count>
struct TypeTuple {
    static let constructor = make_tuple,
               build       = make_tuple,
               make        = make_tuple;

    template <std::size_t i>
    struct GetSet {
        template <class T>
        let operator()(T t) const { return at_c<i>(t); }

        template <class T, class V>
        let operator()(T t, V v) const { return replace_at<i>(t, v); }
    };

    template <std::size_t i>
    static let at = GetSet<i>{};
};

// Literal suffix to make a string like tuple
template <typename CharT, CharT ...cs>
let operator"" _t() {
    return tuple_c<CharT, cs...>;
}

// transform a char sequence to an integral constant
let to_digit = minus & char_c<'0'>;
let to_int =
    (transform & to_digit) >>
    (fold_left & 0_c & ((mult | 10_c) >> plus));

// Forward the variadic pack to the caller
let variadicly = (apply & make_tuple) << demux;

// Compile time switch
let case_ = make_pair;

// struct default_t{};
// auto default_ = case_ | default_t{};

let switch_ = [](auto val, auto... cases_) {
    let_ cases = make_tuple(cases_...);

    let_ match = find_if(cases, first >> equal.to(val));

    static_assert(match != nothing, "Missing case in switch");

    return second(*match);
};
