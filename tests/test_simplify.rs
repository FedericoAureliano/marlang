use marlang::*;

#[test]
fn add_zero() {
    let mut program = MarContext::new();

    let one = program.mk_int_val(1.into());
    let zero = program.mk_int_val(0.into());
    let x = program.mk_symbol("x".into());
    let one_plus_zero = program.mk_add(vec![one, zero]);
    let one_plus_zero_gt_0 = program.mk_gt(vec![one_plus_zero, zero]);
    program.assert(one_plus_zero_gt_0);

    assert_eq!(
        program.extract().to_string(),
        "(CONS (assert (> (CONS (+ (CONS 1 (CONS 0 NIL))) (CONS 0 NIL)))) NIL)"
    );

    let x_plus_zero = program.mk_add(vec![x, zero]);
    let left = program.get_pattern(x_plus_zero, vec![x]);
    let right = program.get_pattern(x, vec![x]);
    program.add_rewrite("add-zero".into(), left, right);

    let mut program = program.simplify(1);

    assert_eq!(
        program.extract().to_string(),
        "(CONS (assert (> (CONS 1 (CONS 0 NIL)))) NIL)"
    )
}
