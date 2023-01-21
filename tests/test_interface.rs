use std::{fs, io::BufWriter};

use marlang::{
    ast::MarRecExpr,
    context::MarContext,
    util::{read_leda, write_leda},
};

#[test]
fn parse_print_symbol_and_string() {
    let x_string: MarRecExpr = "(marlang.value.string x)".parse().unwrap();
    assert_eq!(
        "(marlang.value.string x)",
        x_string.to_string(),
        "String should have quotes!"
    );

    let x_symbol: MarRecExpr = "x".parse().unwrap();
    assert_eq!("x", x_symbol.to_string(), "Symbol should not have quotes!");
}

#[test]
fn leda_write() {
    let mut program = MarContext::new();

    let one = program.mk_int_val(1);
    let zero = program.mk_int_val(0);
    let one_plus_zero = program.mk_int_add(vec![one, zero]);
    let one_plus_zero_gt_0 = program.mk_int_gt(vec![one_plus_zero, zero]);
    program.assert(one_plus_zero_gt_0);

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
