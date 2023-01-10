use marlang::asg::MarRecExpr;

#[test]
fn parse_print_symbol_and_string() {
    let x_string: MarRecExpr = "(str x)".parse().unwrap();
    assert_eq!(
        "(str x)",
        x_string.to_string(),
        "String should have quotes!"
    );

    let x_symbol: MarRecExpr = "x".parse().unwrap();
    assert_eq!("x", x_symbol.to_string(), "Symbol should not have quotes!");
}
