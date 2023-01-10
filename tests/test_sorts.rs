use marlang::*;

#[test]
fn add_expr_sorts_int() {
    let mut program = MarProgram::new();
    let int_sort = program.mk_int_sort();

    let one = program.mk_int(1.into());
    assert_eq!(program.infer_sort(one), int_sort, "1 must be an integer!");

    let x = program.mk_var("x".into(), int_sort);
    assert_eq!(program.infer_sort(x), int_sort, "x must be an integer!");

    let add = program.mk_add(vec![one, x]);
    assert_eq!(
        program.infer_sort(add),
        int_sort,
        "(+ 1 x) must be an integer!"
    );
}

#[test]
fn mult_expr_sorts_real() {
    let mut program = MarProgram::new();
    let real_sort = program.mk_real_sort();

    let one = program.mk_real(1.into());
    assert_eq!(
        program.infer_sort(one),
        real_sort,
        "1 must be a real number!"
    );

    let x = program.mk_var("x".into(), real_sort);
    assert_eq!(program.infer_sort(x), real_sort, "x must be a real number!");

    let add = program.mk_mul(vec![one, x]);
    assert_eq!(
        program.infer_sort(add),
        real_sort,
        "(+ 1 x) must be a real number!"
    );
}

#[test]
fn div_is_real() {
    let mut program = MarProgram::new();
    let real_sort = program.mk_real_sort();

    let one = program.mk_int(1.into());
    let two = program.mk_int(2.into());

    let div = program.mk_div(vec![two, one]);
    assert_eq!(
        program.infer_sort(div),
        real_sort,
        "div must be a real number!"
    );
}

#[test]
fn let_sort() {
    let mut program = MarProgram::new();

    let bool_sort = program.mk_bool_sort();
    let int_sort = program.mk_int_sort();

    let x = program.mk_symbol("x".into());
    let one = program.mk_int(1.into());
    let two = program.mk_int(2.into());
    let one_plus_x = program.mk_add(vec![one, x]);
    let let_x = program.mk_let(vec![("x".into(), two)], one_plus_x);
    assert_eq!(
        program.infer_sort(let_x),
        int_sort,
        "Let expr should be an int!"
    );

    let gt_one = program.mk_gt(vec![let_x, one]);
    assert_eq!(
        program.infer_sort(gt_one),
        bool_sort,
        "Gt expr should be a bool!"
    );
}
