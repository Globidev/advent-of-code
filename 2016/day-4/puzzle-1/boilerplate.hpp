#define let const auto
#define let_ constexpr auto

#define BOOST_HANA_CONFIG_ENABLE_STRING_UDL
#include <boost/hana.hpp>
using namespace boost::hana;
using namespace boost::hana::literals;

// Some glue operators
template <class F1, class F2>
let_ operator<<(F1 f1, F2 f2) { return compose(f1, f2); }

template <class F1, class A>
let_ operator|(F1 f1, A a) { return curry<2>(f1)(a); }

template <class F1, class A>
let_ operator&(F1 f1, A a) { return flip(f1) | a; }

template <class F1, class F2>
let_ operator>>(F1 f1, F2 f2) { return compose(f2, f1); }

// Can be used to make variadic named tuples
template <std::size_t count>
struct Variadic {
    static let_ constructor = make_tuple;

    template <std::size_t i>
    static let_ getter = (at & size_c<i>);
};

// Forward the variadic pack to the caller
let_ variadicly = (apply & make_tuple) << demux;
