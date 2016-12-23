#include "hana_boilerplate.hpp"

// Parsing
let supernet_flag = 0_c;
let hypernet_flag = 1_c;

struct ParserState: Variadic<4> {
    static let supernet_sequences = getter<0>;
    static let hypernet_sequences = getter<1>;
    static let input = getter<2>;
    static let net_flag = getter<3>;
};

struct IP: Variadic<2> {
    static let supernet_sequences = getter<0>;
    static let hypernet_sequences = getter<1>;
};

let parse_supernet = [](auto state) {
    let_ input = ParserState::input(state);
    let_ sequence = take_while(input, not_equal.to(char_c<'['>));

    return ParserState::constructor(
        append(ParserState::supernet_sequences(state), sequence),
        ParserState::hypernet_sequences(state),
        drop_front(input, size(sequence) + size_c<1>),
        hypernet_flag
    );
};

let parse_hypernet = [](auto state) {
    let_ input = ParserState::input(state);
    let_ sequence = take_while(input, not_equal.to(char_c<']'>));

    return ParserState::constructor(
        ParserState::supernet_sequences(state),
        append(ParserState::hypernet_sequences(state), sequence),
        drop_front(input, size(sequence) + size_c<1>),
        supernet_flag
    );
};

let parse_ip = [](auto ip_descriptor) {
    let_ initial_state = ParserState::constructor(
        make_tuple(), // Supernet sequences
        make_tuple(), // Hypernet sequences
        ip_descriptor,
        supernet_flag
    );

    let parser = demux(if_)(
        ParserState::net_flag >> equal.to(supernet_flag),
        parse_supernet,
        parse_hypernet
    );

    let_ final_state = while_(
        ParserState::input >> size >> not_equal.to(size_c<0>),
        initial_state,
        parser
    );

    return IP::constructor(
        ParserState::supernet_sequences(final_state),
        ParserState::hypernet_sequences(final_state)
    );
};

// Logic
let is_abba = demux(and_)(
    demux(equal)    (at & 0_c, at & 3_c),
    demux(equal)    (at & 1_c, at & 2_c),
    demux(not_equal)(at & 0_c, at & 1_c)
);

let sequences_of_4 = demux(zip_shortest)(
    drop_front & 0_c,
    drop_front & 1_c,
    drop_front & 2_c,
    drop_front & 3_c
);

let has_abbas = sequences_of_4 >> (any_of & is_abba);

let supports_tls = demux(and_)(
    IP::supernet_sequences >> (any_of  & has_abbas),
    IP::hypernet_sequences >> (none_of & has_abbas)
);

let day_7_0_impl =
    (transform & parse_ip) >>
    (count_if & supports_tls);

let day_7_0 = variadicly(day_7_0_impl);

int main() {
    static_assert(day_7_0(
        "dnwtsgywerfamfv[gwrhdujbiowtcirq]bjbhmuxdcasenlctwgh"_t,
        "rnqfzoisbqxbdlkgfh[lwlybvcsiupwnsyiljz]kmbgyaptjcsvwcltrdx[ntrpwgkrfeljpye]jxjdlgtntpljxaojufe"_t,
        "jgltdnjfjsbrffzwbv[nclpjchuobdjfrpavcq]sbzanvbimpahadkk[yyoasqmddrzunoyyk]knfdltzlirrbypa"_t,
        "vvrchszuidkhtwx[ebqaetowcthddea]cxgxbffcoudllbtxsa"_t,
        "olgvwasskryjoqpfyvr[hawojecuuzobgyinfi]iywikscwfnlhsgqon"_t,
        "jlzynnkpwqyjvqcmcbz[fdjxnwkoqiquvbvo]bgkxfhztgjyyrcquoiv[xetgnqvwtdiuyiyv]zyfprefpmvxzauur"_t,
        "vjqhodfzrrqjshbhx[lezezbbswydnjnz]ejcflwytgzvyigz[hjdilpgdyzfkloa]mxtkrysovvotkuyekba"_t,
        "xjmkkppyuxybkmzya[jbmofazcbdwzameos]skmpycixjqsagnzwmy"_t,
        "zeebynirxqrjbdqzjav[cawghcfvfeefkmx]xqcdkvawumyayfnq[qhhwzlwjvjpvyavtm]sbnvwssglfpyacfbua[wpbknuubmsjjbekkfy]icimffaoqghdpvsbx"_t,
        "enupgggxsmwvfdljoaj[qlfmrciiyljngimjh]qkjawvmtnvkidcclfay[bllphejvluylyfzyvli]heboydfsgafkqoi"_t,
        "ottpscfbgoiyfri[iwzhojzrpzuinumuwd]orfroqlcemumqbqqrea"_t,
        "zhrhvyfxxcsdpris[xdqecoqujrnqbgla]bpwibmrkcfbzigf[rlqtqykdltcpusvc]ybtsglkxrhucxwv"_t,
        "msaebhhuxyaevahov[skkhuecthcqtrvtunw]bzlvljpsapsezchptjs[lbcxoczqbyysmha]zdqlfydjdctfnuetghr[owwhfhnjmpekukafw]qqitepzwooogqifl"_t,
        "jhdfwesnofrkpse[mkruficpgplktbmoo]mnrjpuvsauanolvzhym"_t,
        "ucibfxxivatgxlupp[rxlbgrqostcioowo]faiimhdhgpockadenua[teomupxzwrernokhyud]ohsfljkyjvkfzwus"_t,
        "gzxcgjqdbyvfndfpw[ypfsapvecfqihnpuszq]mvwxgfkniekgqzqid"_t,
        "fipkggpfwvgrqiwosi[itadifxotejgzkt]szwurlcbvffhgse"_t,
        "ketltdpowbxcusrcua[oonjssgqvcgwvlz]otjxgpizqfpcriuco[mgtgmwcjecomtdkxdev]dnrecyeyhqcpausqzsw"_t,
        "lcototgbpkkoxhsg[erticxnxcjwypnunco]notoouvtmgqcfdupe[hubcmesmprktstzyae]unuquevgbpxqnrib[egalxegqwowylkdjkdg]spqmkzfjnzwcwgutl"_t,
        "nesmourutitzqtolwd[rurfefjvljejcufm]jagkqdwpkefkjdz[cctohikipqxxbdjxsg]badmffkslhmgsxqscf"_t
    ) == 1ul);

    // static_assert(!has_abbas("abcdef"_t));
    // static_assert(has_abbas("ioxxoj"_t));

    // static_assert(sequences_of_4("abcd"_t) == make_tuple("abcd"_t));
    // static_assert(sequences_of_4("abcdef"_t) == make_tuple("abcd"_t, "bcde"_t, "cdef"_t));

    // static_assert(is_abba("abba"_t));
    // static_assert(!is_abba("abcd"_t));
    // static_assert(!is_abba("abca"_t));
    // static_assert(!is_abba("abbd"_t));
    // static_assert(!is_abba("aaaa"_t));

    // let_ ip = parse_ip("rnqfzoisbqxbdlkgfh[lwlybvcsiupwnsyiljz]kmbgyaptjcsvwcltrdx[ntrpwgkrfeljpye]jxjdlgtntpljxaojufe"_t);

    // static_assert(IP::supernet_sequences(ip) == make_tuple(
    //     "rnqfzoisbqxbdlkgfh"_t,
    //     "kmbgyaptjcsvwcltrdx"_t,
    //     "jxjdlgtntpljxaojufe"_t
    // ));

    // static_assert(IP::hypernet_sequences(ip) == make_tuple(
    //     "lwlybvcsiupwnsyiljz"_t,
    //     "ntrpwgkrfeljpye"_t
    // ));
}
