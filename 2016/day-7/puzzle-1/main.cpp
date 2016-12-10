#include "hana_boilerplate.hpp"

#define let_ const auto

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

struct SupernetParser {

    template <class State>
    let operator()(State state) const {
        let_ input = ParserState::input(state);
        let_ sequence = take_while(input, not_equal.to(char_c<'['>));

        return ParserState::constructor(
            append(ParserState::supernet_sequences(state), sequence),
            ParserState::hypernet_sequences(state),
            drop_front(input, size(sequence) + size_c<1>),
            hypernet_flag
        );
    };

};

let parse_supernet = SupernetParser{};

struct HypernetParser {

    template <class State>
    let operator()(State state) const {
        let_ input = ParserState::input(state);
        let_ sequence = take_while(input, not_equal.to(char_c<']'>));

        return ParserState::constructor(
            ParserState::supernet_sequences(state),
            append(ParserState::hypernet_sequences(state), sequence),
            drop_front(input, size(sequence) + size_c<1>),
            supernet_flag
        );
    }

};

let parse_hypernet = HypernetParser{};

struct IPParser {

    template <class IPDescriptor>
    let operator()(IPDescriptor ip_descriptor) const {
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
    }

};

let parse_ip = IPParser{};

// Logic
let is_aba = demux(and_)(
    demux(equal)    (at & 0_c, at & 2_c),
    demux(not_equal)(at & 0_c, at & 1_c)
);

let bab = demux(make_tuple)(
    at & 1_c,
    at & 0_c,
    at & 1_c
);

let sequences_of_3 = demux(zip_shortest)(
    drop_front & 0_c,
    drop_front & 1_c,
    drop_front & 2_c
);

let abas = sequences_of_3 >> (filter & is_aba);
let all_abas = fold_left & make_tuple() & lockstep(concat)(id, abas);
let all_babs = all_abas >> (transform & bab);

let supports_ssl =
    demux(intersection)(
        IP::supernet_sequences >> all_babs >> to_set,
        IP::hypernet_sequences >> all_abas >> to_set
    ) >>
    size >> greater.than(size_c<0>);

let day_7_1_impl =
    (transform & parse_ip) >>
    (count_if & supports_ssl);

let day_7_1 = variadicly(day_7_1_impl);

int main() {
    static_assert(day_7_1(
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
    ) == 3ul);
}
