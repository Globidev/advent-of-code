#include "hana_boilerplate.hpp"

// Data types
using ID = unsigned;

struct Bot: TypeTuple<3> {
    static let id   = at<0>,
               low  = at<1>,
               high = at<2>;
};

enum OutputType { Bot, Bin };

struct Output: TypeTuple<2> {
    static let type = at<0>,
               id   = at<1>;
};

struct Instruction: TypeTuple<2> {
    static let low  = at<0>,
               high = at<1>;
};

struct State: TypeTuple<3> {
    static let queue        = at<0>,
               bots         = at<1>,
               instructions = at<2>,
               outputs      = at<3>;
};

// Data constructor helpers
template <ID id>                  let id_c = integral_c<ID, id>;

template <ID id, ID low>          let bot_1v = Bot::constructor(id_c<id>, id_c<low>, nothing);
template <ID id, ID low, ID high> let bot_2v = Bot::constructor(id_c<id>, id_c<low>, just(id_c<high>));

template <OutputType ot>          let ot_c = integral_c<OutputType, ot>;

                                  let ot_bot_c = ot_c<Bot>;
                                  let ot_bin_c = ot_c<Bin>;

template <OutputType ot, ID id>   let output_v = Output::constructor(ot_c<ot>, id_c<id>);

template <ID id>                  let o_bot_v = output_v<Bot, id>;
template <ID id>                  let o_bin_v = output_v<Bin, id>;

template <ID id, ID value>        let bin_v = make_pair(id_c<id>, id_c<value>);

// High level state constructor helpers
template <ID id>
let instr = variadicly(
    (unpack & Instruction::constructor) >>
    (make_pair | id_c<id>)
);

let index_bots = variadicly(
    (transform & (demux(make_pair)(Bot::id, id))) >>
    (unpack & make_map)
);

let make_outputs = make_map;

// Problem
let take = [](auto bot, auto value) {
    let bot_id = Bot::id(bot);
    let low    = Bot::low(bot);

    if constexpr (low < value)
        return bot_2v<bot_id, low, value>;
    else
        return bot_2v<bot_id, value, low>;
};

let give = [](auto state, auto value, auto output) {
    let queue     = State::queue(state);
    let bots      = State::bots(state);
    let outputs   = State::outputs(state);
    let output_id = Output::id(output);

    if constexpr (Output::type(output) == ot_bot_c) {
        let mb_bot = find(bots, output_id);

        if constexpr (is_just(mb_bot)) {
            let new_bot = take(*mb_bot, value);
            return State::queue(state, append & new_bot);
        }
        else {
            let new_bot = bot_1v<output_id, value>;
            return State::bots(state, insert & make_pair(output_id, new_bot));
        }
    }
    else
        return State::outputs(state, insert & bin_v<output_id, value>);
};

let apply_instruction = [](auto state, auto bot, auto instruction) {
    let give_low  = give &  Bot::low(bot)  & Instruction::low(instruction);
    let give_high = give & *Bot::high(bot) & Instruction::high(instruction);

    return (give_low >> give_high)(state);
};

let pop_bot = [](auto state) {
    let queue       = State::queue(state);
    let bot         = at_c<0>(queue);
    let instruction = *find(State::instructions(state), Bot::id(bot));

    return apply_instruction(
        State::queue(state, remove_at & 0_c),
        bot,
        instruction
    );
};

let mul_three_first = [](auto outputs) {
    return (
        *find(outputs, id_c<0>) *
        *find(outputs, id_c<1>) *
        *find(outputs, id_c<2>)
    );
};

let queue_remaining = State::queue >> size >> greater.than(size_c<0>);

let day_10_2_impl =
    (unpack & State::constructor) >>
    ((while_ | queue_remaining) & pop_bot) >>
    State::outputs >>
    mul_three_first;

let day_10_2 = variadicly(day_10_2_impl);

int main() {
    /* Example
    value 5 goes to bot 2
    bot 2 gives low to bot 1 and high to bot 0
    value 3 goes to bot 1
    bot 1 gives low to output 1 and high to bot 0
    bot 0 gives low to output 2 and high to output 0
    value 2 goes to bot 2
    */
    let initial_outputs   = make_outputs();
    let one_ms_bots_by_id = index_bots(bot_1v<1, 3>);
    let two_ms_bots       = make_tuple(bot_2v<2, 2, 5>);
    let bot_instructions  = make_map(
        instr<2>(o_bot_v<1>, o_bot_v<0>),
        instr<1>(o_bin_v<1>, o_bot_v<0>),
        instr<0>(o_bin_v<2>, o_bin_v<0>)
    );

    static_assert(
        day_10_2(
            two_ms_bots,
            one_ms_bots_by_id,
            bot_instructions,
            initial_outputs
        )
        ==
        30u
    );
}
