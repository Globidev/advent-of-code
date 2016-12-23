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

// Can be used to make variadic named tuples
template <std::size_t count>
struct Variadic {
    static let constructor = make_tuple;

    template <std::size_t i>
    static let getter = (at & size_c<i>);
};

// Make a string like tuple
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
