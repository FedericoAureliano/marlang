use marlang::context::MarContext;

#[test]
fn add_zero() {
    let mut program = MarContext::new();

    let zero = program.mk_int_val(0);
    let int_sort = program.mk_int_sort();
    let y_def = program.declare_const("y", int_sort);
    let empty = program.mk_nil();
    let y = program.mk_call(y_def, empty);
    let one_plus_zero = program.mk_int_add(vec![y, zero]);
    let one_plus_zero_gt_0 = program.mk_int_gt(vec![one_plus_zero, zero]);
    program.assert(one_plus_zero_gt_0);

    assert_eq!(
        program.extract().to_string(),
        "(marlang.meta.cons (marlang.command.declare-fun y marlang.meta.nil marlang.sort.int) (marlang.meta.cons (marlang.command.assert (marlang.operator.int.> (marlang.meta.cons (marlang.operator.int.+ (marlang.meta.cons (marlang.function.call (marlang.command.declare-fun y marlang.meta.nil marlang.sort.int) marlang.meta.nil) (marlang.meta.cons (marlang.value.int 0) marlang.meta.nil))) (marlang.meta.cons (marlang.value.int 0) marlang.meta.nil)))) marlang.meta.nil))"
    );

    let x = program.mk_symbol("x");
    let x_plus_zero = program.mk_int_add(vec![x, zero]);
    let left = program.get_pattern(x_plus_zero, vec![x]);
    let right = program.get_pattern(x, vec![x]);
    program.add_rewrite("add-zero".into(), left, right);

    let mut program = program.simplify(1);

    assert_eq!(
        program.extract().to_string(),
        "(marlang.meta.cons (marlang.command.declare-fun y marlang.meta.nil marlang.sort.int) (marlang.meta.cons (marlang.command.assert (marlang.operator.int.> (marlang.meta.cons (marlang.function.call (marlang.command.declare-fun y marlang.meta.nil marlang.sort.int) marlang.meta.nil) (marlang.meta.cons (marlang.value.int 0) marlang.meta.nil)))) marlang.meta.nil))"
    )
}
