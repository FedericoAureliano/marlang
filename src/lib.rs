pub mod asg;

use asg::*;
use fxhash::FxBuildHasher as BuildHasher;
use rug::{Integer, Rational};

type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasher>;

pub struct MarProgram {
    commands: Vec<MId>,
    runner: MRunner,
    rewrites: Vec<MRewrite>,
}

impl MarProgram {
    pub fn mk_var(&mut self, name: String, sort: MId) -> MId {
        let x = self.mk_symbol(name);
        self.add(Marlang::Var([x, sort]))
    }

    pub fn mk_add(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::Add([folded]))
    }

    pub fn mk_sub(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::Sub([folded]))
    }

    pub fn mk_mul(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::Mul([folded]))
    }

    pub fn mk_div(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::RealDiv([folded]))
    }

    pub fn mk_eq(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::Eq([folded]))
    }

    pub fn mk_gt(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::Gt([folded]))
    }

    pub fn mk_ge(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::Ge([folded]))
    }

    pub fn mk_lt(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::Lt([folded]))
    }

    pub fn mk_le(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::Le([folded]))
    }

    pub fn mk_concat(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::Concat([folded]))
    }

    pub fn mk_and(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::And([folded]))
    }

    pub fn mk_or(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::Or([folded]))
    }

    pub fn mk_xor(&mut self, args: Vec<MId>) -> MId {
        let folded = self.fold(args);
        self.add(Marlang::Xor([folded]))
    }

    pub fn mk_let(&mut self, bindings: Vec<(String, MId)>, body: MId) -> MId {
        let bindings: Vec<MId> = bindings
            .into_iter()
            .map(|(name, value)| {
                let name = self.mk_symbol(name);
                self.fold(vec![name, value])
            })
            .collect();
        let bindings = self.fold(bindings);
        self.add(Marlang::Let([bindings, body]))
    }

    pub fn mk_not(&mut self, arg: MId) -> MId {
        self.add(Marlang::Not([arg]))
    }

    pub fn mk_implies(&mut self, x: MId, y: MId) -> MId {
        self.add(Marlang::Implies([x, y]))
    }

    pub fn mk_ite(&mut self, x: MId, y: MId, z: MId) -> MId {
        self.add(Marlang::Ite([x, y, z]))
    }

    pub fn mk_set_logic(&mut self, logic: String) -> MId {
        let logic = self.mk_symbol(logic);
        self.add(Marlang::SetLogic([logic]))
    }

    pub fn mk_check_sat(&mut self) -> MId {
        self.add(Marlang::CheckSat)
    }

    pub fn mk_assert(&mut self, expr: MId) -> MId {
        self.add(Marlang::Assert([expr]))
    }

    pub fn mk_declare_const(&mut self, name: String, sort: MId) -> MId {
        let x = self.mk_symbol(name);
        self.add(Marlang::DeclareConst([x, sort]))
    }

    pub fn mk_bool_sort(&mut self) -> MId {
        self.add(Marlang::BoolSort)
    }

    pub fn mk_int_sort(&mut self) -> MId {
        self.add(Marlang::IntSort)
    }

    pub fn mk_real_sort(&mut self) -> MId {
        self.add(Marlang::RealSort)
    }

    pub fn mk_string_sort(&mut self) -> MId {
        self.add(Marlang::StringSort)
    }

    pub fn mk_bool(&mut self, i: bool) -> MId {
        self.add(Marlang::BoolVal(i))
    }

    pub fn mk_int(&mut self, i: Integer) -> MId {
        self.add(Marlang::IntVal(i))
    }

    pub fn mk_real(&mut self, i: Rational) -> MId {
        self.add(Marlang::RealVal(i))
    }

    pub fn mk_string(&mut self, i: String) -> MId {
        self.add(Marlang::StringVal(i))
    }

    pub fn mk_symbol(&mut self, name: String) -> MId {
        self.add(Marlang::Symbol(name.into()))
    }
}

impl MarProgram {
    pub fn set_logic(&mut self, logic: String) {
        let c = self.mk_set_logic(logic);
        self.commands.push(c)
    }

    pub fn check_sat(&mut self) {
        let c = self.mk_check_sat();
        self.commands.push(c)
    }

    pub fn assert(&mut self, expr: MId) {
        let c = self.mk_assert(expr);
        self.commands.push(c)
    }

    pub fn declare_const(&mut self, name: String, sort: MId) {
        let c = self.mk_declare_const(name, sort);
        self.commands.push(c)
    }
}

impl MarProgram {
    pub fn new() -> Self {
        let mgraph = MGraph::default().with_explanations_enabled();
        Self {
            commands: vec![],
            rewrites: vec![],
            runner: MRunner::default().with_egraph(mgraph),
        }
    }

    pub fn asg(&mut self) -> MId {
        self.fold(self.commands.clone())
    }

    pub fn extract(&mut self) -> MRecExpr {
        let asg = self.asg();
        let extractor = egg::Extractor::new(&self.runner.egraph, egg::AstSize);
        let (_, best_expr) = extractor.find_best(asg);
        best_expr
    }

    pub fn get_expr(&self, expr: MId) -> MRecExpr {
        self.runner.egraph.id_to_expr(expr)
    }

    pub fn get_pattern(&self, expr: MId, subs: Vec<MId>) -> MPattern {
        let mut subs_map = HashMap::default();
        for x in subs.into_iter() {
            subs_map.insert(x, x);
        }
        let (p, _) = self.runner.egraph.id_to_pattern(expr, &subs_map);
        p
    }

    pub fn add_rewrite(&mut self, name: String, left: MPattern, right: MPattern) {
        self.rewrites.push(egg::rewrite!(name; left => right))
    }

    pub fn simplify(self, iter_limit: usize) -> Self {
        let runner: MRunner = MRunner::default()
            .with_egraph(self.runner.egraph)
            .with_iter_limit(iter_limit)
            .run(&self.rewrites);
        Self {
            runner,
            rewrites: self.rewrites,
            commands: self.commands,
        }
    }

    pub fn equiv(&self, left: MRecExpr, right: MRecExpr) -> bool {
        let equivs = self.runner.egraph.equivs(&left, &right);
        equivs.len() > 0
    }

    pub fn explain_equivalence(&mut self, left: MRecExpr, right: MRecExpr) -> MExplanation {
        self.runner.egraph.explain_equivalence(&left, &right)
    }

    fn fold(&mut self, args: Vec<MId>) -> MId {
        let start = self.mk_nil();
        args.iter()
            .rev()
            .fold(start, |acc, x| self.mk_cons(*x, acc))
    }

    pub fn mk_cons(&mut self, x: MId, y: MId) -> MId {
        self.add(Marlang::Cons([x, y]))
    }

    pub fn mk_nil(&mut self) -> MId {
        self.add(Marlang::Nil)
    }

    fn add(&mut self, x: Marlang) -> MId {
        let out = self.runner.egraph.add(x);
        self.runner.egraph.rebuild();
        out
    }
}
