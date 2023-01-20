use std::io::BufWriter;

use marlang::{ast::MarRecExpr, context::MarContext, util::write_leda};

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
fn leda_constant_add() {
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

    let expected = "LEDA.GRAPH
string
string
-1

# Nodes Section
13
|{marlang.meta.nil}|
|{0}|
|{marlang.value.int}|
|{marlang.meta.cons}|
|{1}|
|{marlang.value.int}|
|{marlang.meta.cons}|
|{marlang.operator.int.+}|
|{marlang.meta.cons}|
|{marlang.operator.int.>}|
|{marlang.command.assert}|
|{marlang.meta.cons}|
|{start}|

# Edges Section
14
3 2 0 |{child}|
4 1 0 |{child}|
4 3 0 |{child}|
6 5 0 |{child}|
7 4 0 |{child}|
7 6 0 |{child}|
8 7 0 |{child}|
9 4 0 |{child}|
9 8 0 |{child}|
10 9 0 |{child}|
11 10 0 |{child}|
12 1 0 |{child}|
12 11 0 |{child}|
13 12 0 |{child}|
";

    assert_eq!(expected, output);
}
