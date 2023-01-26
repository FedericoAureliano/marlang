use std::{fs, io::BufWriter};

use marlang::{
    context::MarContext,
    util::{read_leda, sample, write_leda},
};
use rand::{rngs::StdRng, SeedableRng};

#[test]
fn leda_write() {
    let mut program = MarContext::new();

    let int_sort = program.mk_int_sort();

    let x_def = program.declare_const("x", int_sort);
    let empty = program.mk_nil();
    let x_use = program.mk_call(x_def, empty);
    let zero = program.mk_int_val(0);
    let x_plus_zero = program.mk_int_add(vec![x_use, zero]);
    let x_plus_zero_gt_0 = program.mk_int_gt(vec![x_plus_zero, zero]);
    program.assert(x_plus_zero_gt_0);

    let mut buffer = BufWriter::new(Vec::new());
    let program = program.extract();

    write_leda(&mut buffer, &program).expect("Must be able to write program to buffer");

    let output = std::str::from_utf8(buffer.buffer()).unwrap().to_string();
    let expected =
        fs::read_to_string("tests/simple.leda").expect("File must exist and be readable");
    assert_eq!(expected, output);
}

#[test]
fn leda_read() {
    let input = fs::read_to_string("tests/simple.leda").expect("File must exist and be readable");
    let expected =
        fs::read_to_string("tests/simple.marlang").expect("File must exist and be readable");
    let parsed = read_leda(&mut input.as_bytes()).expect("Must be able to parse program");
    assert_eq!(expected, parsed.to_string());
}

#[test]
fn test_subgraph() {
    let input = fs::read_to_string("tests/simple.leda").expect("File must exist and be readable");
    let parsed = read_leda(&mut input.as_bytes()).expect("Must be able to parse program");

    let mut rng = StdRng::seed_from_u64(0);
    let subgraph = sample(&mut rng, &parsed, 2);

    let expected = "(marlang.operator.int.> (marlang.meta.cons (marlang.operator.int.+ (marlang.meta.cons ?marlang.fresh.uaqovgdtbf (marlang.meta.cons ?marlang.fresh.ttpsnyvyhd marlang.meta.nil))) (marlang.meta.cons (marlang.value.int 0) marlang.meta.nil)))";
    assert_eq!(expected, subgraph.to_string());
}
