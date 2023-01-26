use marlang::ast::MarRecExpr;

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
