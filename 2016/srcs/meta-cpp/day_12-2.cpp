#include "hana_boilerplate.hpp"

// Data types
enum Register { A, B, C, D };

struct Process: TypeTuple<2> {
    static let instructions = at<0>,
               registers    = at<1>,
               pc           = at<2>;
};

// Data constructor helpers
template <Register r> let reg_c = integral_c<unsigned, r>;
                      let reg_a_c = reg_c<A>;
                      let reg_b_c = reg_c<B>;
                      let reg_c_c = reg_c<C>;
                      let reg_d_c = reg_c<D>;

// Problem
let initial_pc = int_c<0>;
let initial_registers = make_map(
    make_pair(reg_a_c, int_c<0>),
    make_pair(reg_b_c, int_c<0>),
    make_pair(reg_c_c, int_c<1>),
    make_pair(reg_d_c, int_c<0>)
);

let clone = Process::constructor & initial_registers & initial_pc;

let reg_value = demux(*_)(
    lockstep(find)(
        Process::registers,
        id
    )
);

// Instructions
let cpy_val_i = [](auto src, auto dst, auto process) {
    return Process::registers(process, map_replace & dst & src);
};

let cpy_reg_i = [](auto src, auto dst, auto process) {
    return cpy_val_i(reg_value(process, src), dst, process);
};

let inc_i = [](auto reg, auto process) {
    return Process::registers(process, map_update & reg & (_ + 1_c));
};

let dec_i = [](auto reg, auto process) {
    return Process::registers(process, map_update & reg & (_ - 1_c));
};

let jnz_val_i = [](auto val, auto di, auto process) {
    if constexpr (val != 0_c)
        return Process::pc(process, _ + (di - 1_c));
    else
        return process;
};

let jnz_reg_i = [](auto reg, auto di, auto process) {
    return jnz_val_i(reg_value(process, reg), di, process);
};

// Instruction constructor helpers
template <int src, Register dst>       let cpyv = cpy_val_i | int_c<src> | reg_c<dst>;
template <Register src, Register dst>  let cpyr = cpy_reg_i | reg_c<src> | reg_c<dst>;
template <Register r>                  let inc  = inc_i | reg_c<r>;
template <Register r>                  let dec  = dec_i | reg_c<r>;
template <int v, int di>               let jnzv = jnz_val_i | int_c<v> | int_c<di>;
template <Register r, int di>          let jnzr = jnz_reg_i | reg_c<r> | int_c<di>;

let current_instruction = demux(at)(
    Process::instructions,
    Process::pc
);

let exec_instruction = [](auto process) {
    let instruction = current_instruction(process);
    let next_state  = apply(instruction, process);

    return Process::pc(next_state, _ + 1_c);
};

let process_exited = demux(_ >= _)(
    Process::pc >> to_integral<std::size_t>,
    Process::instructions >> size
);

let run_process = (while_ | (not_ << process_exited)) & exec_instruction;

let day_12_2_impl =
    clone >> run_process >>
    Process::registers >>
    (find & reg_a_c) >> *_;

let day_12_2 = variadicly(day_12_2_impl);

int main() {
    // Example
    static_assert(
        day_12_2(
            cpyv<41, A>,
            inc<A>,
            inc<A>,
            dec<A>,
            jnzr<A, 2>,
            dec<A>
        )
        ==
        42
    );

    // My input (exceeds default template instantiation depth on gcc)
    // static_assert(
    //     day_12_2(
    //         cpyv<1, A>,
    //         cpyv<1, B>,
    //         cpyv<26, D>,
    //         jnzr<C, 2>,
    //         jnzv<1, 5>,
    //         cpyv<7, C>,
    //         inc<D>,
    //         dec<C>,
    //         jnzr<C, -2>,
    //         cpyr<A, C>,
    //         inc<A>,
    //         dec<B>,
    //         jnzr<B, -2>,
    //         cpyr<C, B>,
    //         dec<D>,
    //         jnzr<D, -6>,
    //         cpyv<14, C>,
    //         cpyv<14, D>,
    //         inc<A>,
    //         dec<D>,
    //         jnzr<D, -2>,
    //         dec<C>,
    //         jnzr<C, -5>
    //     )
    //     ==
    //     9227661
    // );

    return 0;
}
