use fxhash::FxBuildHasher as BuildHasher;

use std::fmt;

use crate::ast::{
    MarExplanation, MarGraph, MarId, MarPattern, MarRecExpr, MarRewrite, MarRunner, Marlang,
};

type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasher>;

pub struct MarContext {
    runner: MarRunner,
    commands: Vec<MarId>,
    rewrites: Vec<MarRewrite>,
}

impl MarContext {
    pub fn mk_call(&mut self, def: MarId, args: MarId) -> MarId {
        self.add(Marlang::Call([def, args]))
    }

    pub fn mk_real_add(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::RealAdd([folded]))
    }

    pub fn mk_real_sub(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::RealSub([folded]))
    }

    pub fn mk_real_mul(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::RealMul([folded]))
    }

    pub fn mk_real_div(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::RealDiv([folded]))
    }

    pub fn mk_real_gt(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::RealGt([folded]))
    }

    pub fn mk_real_ge(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::RealGe([folded]))
    }

    pub fn mk_real_lt(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::RealLt([folded]))
    }

    pub fn mk_real_le(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::RealLe([folded]))
    }

    pub fn mk_int_add(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::IntAdd([folded]))
    }

    pub fn mk_int_sub(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::IntSub([folded]))
    }

    pub fn mk_int_mul(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::IntMul([folded]))
    }

    pub fn mk_int_gt(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::IntGt([folded]))
    }

    pub fn mk_int_ge(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::IntGe([folded]))
    }

    pub fn mk_int_lt(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::IntLt([folded]))
    }

    pub fn mk_int_le(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::IntLe([folded]))
    }

    pub fn mk_eq(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::Eq([folded]))
    }

    pub fn mk_concat(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::Concat([folded]))
    }

    pub fn mk_and(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::And([folded]))
    }

    pub fn mk_or(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::Or([folded]))
    }

    pub fn mk_xor(&mut self, args: Vec<MarId>) -> MarId {
        let folded = self.fold(args);
        self.add(Marlang::Xor([folded]))
    }

    pub fn mk_let(&mut self, bindings: Vec<(String, MarId)>, body: MarId) -> MarId {
        let bindings: Vec<MarId> = bindings
            .into_iter()
            .map(|(name, value)| {
                let name = self.mk_symbol(name);
                self.fold(vec![name, value])
            })
            .collect();
        let bindings = self.fold(bindings);
        self.add(Marlang::Let([bindings, body]))
    }

    pub fn mk_not(&mut self, arg: MarId) -> MarId {
        self.add(Marlang::Not([arg]))
    }

    pub fn mk_implies(&mut self, x: MarId, y: MarId) -> MarId {
        self.add(Marlang::Implies([x, y]))
    }

    pub fn mk_ite(&mut self, x: MarId, y: MarId, z: MarId) -> MarId {
        self.add(Marlang::Ite([x, y, z]))
    }

    pub fn mk_set_logic(&mut self, logic: String) -> MarId {
        let logic = self.mk_symbol(logic);
        self.add(Marlang::SetLogic([logic]))
    }

    pub fn mk_check_sat(&mut self) -> MarId {
        self.add(Marlang::CheckSat)
    }

    pub fn mk_assert(&mut self, expr: MarId) -> MarId {
        self.add(Marlang::Assert([expr]))
    }

    pub fn mk_declare_const<T: ToString>(&mut self, name: T, sort: MarId) -> MarId {
        let x = self.mk_symbol(name);
        let empty = self.mk_nil();
        self.add(Marlang::DeclareFun([x, empty, sort]))
    }

    pub fn mk_declare_fun<T: ToString>(
        &mut self,
        name: T,
        params: Vec<MarId>,
        sort: MarId,
    ) -> MarId {
        let params = self.fold(params);
        let f = self.mk_symbol(name);
        self.add(Marlang::DeclareFun([f, params, sort]))
    }

    pub fn mk_define_fun<T: ToString>(
        &mut self,
        name: T,
        params: Vec<(T, MarId)>,
        sort: MarId,
        body: MarId,
    ) -> MarId {
        let params: Vec<MarId> = params
            .into_iter()
            .map(|(name, value)| {
                let name = self.mk_symbol(name);
                self.fold(vec![name, value])
            })
            .collect();
        let params = self.fold(params);
        let f = self.mk_symbol(name);
        self.add(Marlang::DefineFun([f, params, sort, body]))
    }

    pub fn mk_bool_sort(&mut self) -> MarId {
        self.add(Marlang::BoolSort)
    }

    pub fn mk_int_sort(&mut self) -> MarId {
        self.add(Marlang::IntSort)
    }

    pub fn mk_real_sort(&mut self) -> MarId {
        self.add(Marlang::RealSort)
    }

    pub fn mk_string_sort(&mut self) -> MarId {
        self.add(Marlang::StringSort)
    }

    pub fn mk_bool_val(&mut self, i: bool) -> MarId {
        let i = self.mk_symbol(i.to_string());
        self.add(Marlang::BoolVal([i]))
    }

    pub fn mk_int_val<T: ToString>(&mut self, i: T) -> MarId {
        let i = self.mk_symbol(i.to_string());
        self.add(Marlang::IntVal([i]))
    }

    pub fn mk_real_val<T: ToString>(&mut self, i: T) -> MarId {
        let i = self.mk_symbol(i.to_string());
        self.add(Marlang::RealVal([i]))
    }

    pub fn mk_string_val(&mut self, i: String) -> MarId {
        let s = self.mk_symbol(i);
        self.add(Marlang::StringVal([s]))
    }

    pub fn mk_symbol<T: ToString>(&mut self, name: T) -> MarId {
        self.add(Marlang::Symbol(name.to_string()))
    }

    pub fn mk_cons(&mut self, x: MarId, y: MarId) -> MarId {
        self.add(Marlang::Cons([x, y]))
    }

    pub fn mk_nil(&mut self) -> MarId {
        self.add(Marlang::Nil)
    }

    pub fn mk_rest(&mut self) -> MarId {
        self.mk_symbol("?MARLANG_REST_PATTERN")
    }
}

impl MarContext {
    pub fn set_logic(&mut self, logic: String) -> MarId {
        let c = self.mk_set_logic(logic);
        self.commands.push(c);
        c
    }

    pub fn check_sat(&mut self) -> MarId {
        let c = self.mk_check_sat();
        self.commands.push(c);
        c
    }

    pub fn assert(&mut self, expr: MarId) -> MarId {
        let c = self.mk_assert(expr);
        self.commands.push(c);
        c
    }

    pub fn declare_const<T: ToString>(&mut self, name: T, sort: MarId) -> MarId {
        let c = self.mk_declare_const(name, sort);
        self.commands.push(c);
        c
    }

    pub fn commit(&mut self, command: MarId) {
        self.commands.push(command)
    }
}

impl MarContext {
    pub fn new() -> Self {
        let mgraph = MarGraph::default().with_explanations_enabled();
        Self {
            runner: MarRunner::default().with_egraph(mgraph),
            commands: vec![],
            rewrites: vec![],
        }
    }

    pub fn asg(&mut self) -> MarId {
        self.fold(self.commands.clone())
    }

    pub fn extract_best(&mut self) -> MarRecExpr {
        let asg = self.asg();
        self.runner.egraph.rebuild();
        let extractor = egg::Extractor::new(&self.runner.egraph, egg::AstSize);
        let (_, best_expr) = extractor.find_best(asg);
        best_expr
    }

    pub fn extract_any(&mut self) -> MarRecExpr {
        let asg = self.asg();
        self.runner.egraph.rebuild();
        self.get_expr(asg)
    }

    pub fn get_expr(&self, expr: MarId) -> MarRecExpr {
        self.runner.egraph.id_to_expr(expr)
    }

    pub fn get_pattern(&self, expr: MarId, subs: Vec<MarId>) -> MarPattern {
        let mut subs_map = HashMap::default();
        for x in subs.into_iter() {
            subs_map.insert(x, x);
        }
        let (p, _) = self.runner.egraph.id_to_pattern(expr, &subs_map);
        p
    }

    pub fn add_rewrite(&mut self, name: String, left: MarPattern, right: MarPattern) {
        self.rewrites.push(egg::rewrite!(name; left => right))
    }

    pub fn simplify(mut self, iter_limit: usize) -> Self {
        self.runner.egraph.rebuild();
        if self.rewrites.len() > 0 {
            let runner: MarRunner = MarRunner::default()
                .with_egraph(self.runner.egraph)
                .with_iter_limit(iter_limit)
                .run(&self.rewrites);
            Self {
                runner,
                commands: self.commands,
                rewrites: self.rewrites,
            }
        } else {
            self
        }
    }

    pub fn equiv(&self, left: MarRecExpr, right: MarRecExpr) -> bool {
        let equivs = self.runner.egraph.equivs(&left, &right);
        equivs.len() > 0
    }

    pub fn explain_equivalence(&mut self, left: MarRecExpr, right: MarRecExpr) -> MarExplanation {
        self.runner.egraph.explain_equivalence(&left, &right)
    }
}

impl MarContext {
    fn fold(&mut self, args: Vec<MarId>) -> MarId {
        let (start, to_skip) = if args.last() == Some(&self.mk_rest()) {
            (self.mk_rest(), 1)
        } else {
            (self.mk_nil(), 0)
        };
        args.iter()
            .rev()
            .skip(to_skip)
            .fold(start, |acc, x| self.mk_cons(*x, acc))
    }

    fn add(&mut self, x: Marlang) -> MarId {
        let out = self.runner.egraph.add(x);
        out
    }

    pub fn add_recexpr(&mut self, x: MarRecExpr) -> MarId {
        let out = self.runner.egraph.add_expr(&x);
        out
    }

    pub fn graph(&self) -> &MarGraph {
        &self.runner.egraph
    }
}

impl fmt::Debug for MarContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "MarGraph:\n{:?}", self.runner.egraph.dump())?;
        writeln!(f, "Commands: {:?}", self.commands)?;
        writeln!(f, "Rewrites: {:?}", self.rewrites)
    }
}
