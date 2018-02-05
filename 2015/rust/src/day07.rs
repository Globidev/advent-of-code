extern crate nom;

use std::collections::HashMap;

type WireID = String;
type Signal = u16;

enum SignalProvider {
    Wire(WireID),
    Value(Signal),
}

enum Gate {
    Assign(SignalProvider),
    And(SignalProvider, SignalProvider),
    Or(SignalProvider, SignalProvider),
    Lshift(SignalProvider, SignalProvider),
    Rshift(SignalProvider, SignalProvider),
    Not(SignalProvider),
}
type Instruction = (Gate, WireID);
type Circuit = Vec<Instruction>;
type Wires = HashMap<WireID, Signal>;

fn is_gate_resolvable(gate: &Gate, wires: &Wires) -> bool {
    use self::Gate::*;
    use self::SignalProvider::*;

    let resolvable = |sp| {
        match sp {
            &Value(_) => true,
            &Wire(ref id) => wires.contains_key(id),
        }
    };

    match gate {
        &Assign(ref s)          => resolvable(s),
        &And(ref s1, ref s2)    => resolvable(s1) && resolvable(s2),
        &Or(ref s1, ref s2)     => resolvable(s1) && resolvable(s2),
        &Lshift(ref s1, ref s2) => resolvable(s1) && resolvable(s2),
        &Rshift(ref s1, ref s2) => resolvable(s1) && resolvable(s2),
        &Not(ref s)             => resolvable(s),
    }
}

fn compute_gate(gate: &Gate, wires: &Wires) -> Signal {
    use self::Gate::*;
    use self::SignalProvider::*;

    let get_value = |sp| {
        match sp {
            &Value(x) => x,
            &Wire(ref id) => wires.get(id).unwrap().clone(),
        }
    };

    match gate {
        &Assign(ref s)          => get_value(s),
        &And(ref s1, ref s2)    => get_value(s1) &  get_value(s2),
        &Or(ref s1, ref s2)     => get_value(s1) |  get_value(s2),
        &Lshift(ref s1, ref s2) => get_value(s1) << get_value(s2),
        &Rshift(ref s1, ref s2) => get_value(s1) >> get_value(s2),
        &Not(ref s)             => !get_value(s),
    }
}

fn run_circuit(circuit: &Circuit, initial_wires: Wires) -> Vec<(WireID, Signal)> {
    let mut wires = initial_wires;

    loop {
        let mut altered = false;

        for &(ref gate, ref wid) in circuit {
            if !wires.contains_key(wid) && is_gate_resolvable(&gate, &wires) {
                let output_signal = compute_gate(&gate, &wires);
                wires.insert(wid.clone(), output_signal);
                altered = true;
            }
        }

        if !altered { break };
    }

    wires.iter().map(|(wid, sig)| (wid.clone(), *sig)).collect()
}

fn parse_instruction(raw_instr: &str) -> Instruction {
    use self::Gate::*;
    use self::SignalProvider::*;
    use self::nom::*;

    use std::str::FromStr;
    use std::str::from_utf8;

    named!(wire_id<WireID>, map!(
        take_while1!(is_alphabetic),
        |s| from_utf8(s).unwrap().to_string())
    );
    named!(signal<Signal>, map!(
        digit,
        |d| FromStr::from_str(from_utf8(d).unwrap()).unwrap()
    ));
    named!(signal_provider<SignalProvider>, alt!(
        map!(wire_id, Wire) |
        map!(signal, Value)
    ));

    named!(assign<Gate>, map!(signal_provider, Assign));
    named!(and<Gate>, do_parse!(
        s1: signal_provider >>
        tag_s!(" AND ")     >>
        s2: signal_provider >>
        (And(s1, s2))
    ));
    named!(or<Gate>, do_parse!(
        s1: signal_provider >>
        tag_s!(" OR ")     >>
        s2: signal_provider >>
        (Or(s1, s2))
    ));
    named!(lshift<Gate>, do_parse!(
        s1: signal_provider >>
        tag_s!(" LSHIFT ")     >>
        s2: signal_provider >>
        (Lshift(s1, s2))
    ));
    named!(rshift<Gate>, do_parse!(
        s1: signal_provider >>
        tag_s!(" RSHIFT ")     >>
        s2: signal_provider >>
        (Rshift(s1, s2))
    ));
    named!(not<Gate>, do_parse!(
        tag_s!("NOT ")     >>
        s: signal_provider >>
        (Not(s))
    ));

    named!(gate<Gate>, alt!(and | or | lshift | rshift | not | assign));

    named!(instruction_parser<Instruction>, do_parse!(
        gate: gate     >>
        tag_s!(" -> ") >>
        wid:  wire_id  >>
        ((gate, wid))
    ));

    match instruction_parser(raw_instr.as_bytes()) {
        IResult::Done(_, instr) => instr,
        _                       => panic!("Wrong instruction format")
    }
}

pub fn p1(input: &str) -> Signal {
    let circuit: Circuit = input.trim().split('\n')
                                       .map(parse_instruction)
                                       .collect();

    let final_values = run_circuit(&circuit, Wires::new());

    final_values.iter().find(|&&(ref wid, _)| wid == "a").unwrap().1
}

pub fn p2(input: &str) -> Signal {
    let circuit: Circuit = input.trim().split('\n')
                                       .map(parse_instruction)
                                       .collect();

    let mut wires = Wires::new();
    wires.insert("b".to_string(), p1(&input));
    let final_values = run_circuit(&circuit, wires);

    final_values.iter().find(|&&(ref wid, _)| wid == "a").unwrap().1
}
