pub mod asg;

use asg::*;
use fxhash::FxBuildHasher as BuildHasher;
use rug::{Integer, Rational};

use ahash::AHashMap;
use std::{borrow::Borrow, hash::Hash};

type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasher>;

pub struct MarProgram {
    commands: Vec<MId>,
    runner: MRunner,
    rewrites: Vec<MRewrite>,
    symbol_table: SymbolTable<String, MId>,
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

    pub fn mk_declare_fun(&mut self, name: String, params: Vec<MId>, sort: MId) -> MId {
        let params = self.fold(params);
        let f = self.mk_symbol(name);
        self.add(Marlang::DeclareFun([f, params, sort]))
    }

    pub fn mk_define_fun(&mut self, name: String, params: Vec<(String, MId)>, sort: MId, body: MId) -> MId {
        let params: Vec<MId> = params
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

    pub fn mk_cons(&mut self, x: MId, y: MId) -> MId {
        self.add(Marlang::Cons([x, y]))
    }

    pub fn mk_nil(&mut self) -> MId {
        self.add(Marlang::Nil)
    }

    pub fn mk_rest(&mut self) -> MId {
        self.mk_symbol("?MARLANG_REST_PATTERN".into())
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

    pub fn commit(&mut self, command: MId) {
        self.commands.push(command)
    }
}

impl MarProgram {
    pub fn infer_sort(&mut self, term: MId) -> MId {
        let mexpr = self.get_expr(term);
        let last = mexpr.as_ref().len() - 1;
        self.infer_sort_rec(&mexpr, last)
    }

    fn infer_sort_rec(&mut self, expr: &MRecExpr, i: usize) -> MId {
        let node = &expr.as_ref()[i];
        match node {
            Marlang::Var([_, s]) => *s,

            Marlang::Add([args]) => decompose(expr, *args)[0],
            Marlang::Sub([args]) => decompose(expr, *args)[0],
            Marlang::Mul([args]) => decompose(expr, *args)[0],

            Marlang::Eq(_)
            | Marlang::Gt(_)
            | Marlang::Ge(_)
            | Marlang::Lt(_)
            | Marlang::Le(_)
            | Marlang::And(_)
            | Marlang::Or(_)
            | Marlang::Xor(_)
            | Marlang::Not(_)
            | Marlang::Implies(_) => self.mk_bool_sort(),

            Marlang::RealDiv(_) => self.mk_real_sort(),
            Marlang::Concat(_) => self.mk_string_sort(),

            Marlang::Ite([_, y, _]) => self.infer_sort_rec(expr, (*y).into()),

            Marlang::Let([bindings, body]) => {
                let bindings = decompose(expr, (*bindings).into());
                for b in bindings {
                    let pair = decompose(expr, b.into());
                    assert_eq!(pair.len(), 2);
                    let name: usize = pair[0].into();
                    let name = &expr.as_ref()[name];
                    let sort = pair[1];
                    match name {
                        Marlang::Symbol(s) => self.symbol_table.insert(s.to_string(), sort),
                        _ => {
                            panic!("Let-binding must be a name but got {name}!")
                        }
                    }
                }
                self.infer_sort_rec(expr, (*body).into())
            }

            Marlang::Symbol(n) => match self.symbol_table.get(&n.to_string()) {
                Some(s) => *s,
                None => {
                    panic!("Couldn't find {n} in symbol table!")
                }
            },

            Marlang::SetLogic(_)
            | Marlang::CheckSat
            | Marlang::Assert(_)
            | Marlang::DeclareConst(_)
            | Marlang::DeclareFun(_)
            | Marlang::DefineFun(_)
            | Marlang::Cons(_)
            | Marlang::Nil
            | Marlang::BoolSort
            | Marlang::IntSort
            | Marlang::RealSort
            | Marlang::StringSort => unreachable!("Should never check sort of commands!"),

            Marlang::BoolVal(_) => self.mk_bool_sort(),
            Marlang::IntVal(_) => self.mk_int_sort(),
            Marlang::RealVal(_) => self.mk_real_sort(),
            Marlang::StringVal(_) => self.mk_string_sort(),
        }
    }
}

impl MarProgram {
    pub fn new() -> Self {
        let mgraph = MGraph::default().with_explanations_enabled();
        Self {
            commands: vec![],
            rewrites: vec![],
            runner: MRunner::default().with_egraph(mgraph),
            symbol_table: SymbolTable::default(),
        }
    }

    pub fn asg(&mut self) -> MId {
        self.fold(self.commands.clone())
    }

    pub fn extract(&mut self) -> MRecExpr {
        self.runner.egraph.rebuild();
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
        if self.rewrites.len() > 0 {
            let runner: MRunner = MRunner::default()
                .with_egraph(self.runner.egraph)
                .with_iter_limit(iter_limit)
                .run(&self.rewrites);
            Self {
                runner,
                rewrites: self.rewrites,
                commands: self.commands,
                symbol_table: self.symbol_table,
            }
        } else {
            self
        }
    }

    pub fn equiv(&self, left: MRecExpr, right: MRecExpr) -> bool {
        let equivs = self.runner.egraph.equivs(&left, &right);
        equivs.len() > 0
    }

    pub fn explain_equivalence(&mut self, left: MRecExpr, right: MRecExpr) -> MExplanation {
        self.runner.egraph.explain_equivalence(&left, &right)
    }
}

impl MarProgram {
    fn fold(&mut self, args: Vec<MId>) -> MId {
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

    fn add(&mut self, x: Marlang) -> MId {
        let out = self.runner.egraph.add(x);
        out
    }
}

pub fn decompose(mexpr: &MRecExpr, ls: MId) -> Vec<MId> {
    let last = ls.into();
    decompose_rec(mexpr, last)
}

fn decompose_rec(mexpr: &MRecExpr, i: usize) -> Vec<MId> {
    match mexpr.as_ref()[i] {
        Marlang::Cons([x, s]) => {
            let mut x = vec![x];
            let mut rest = decompose_rec(mexpr, s.into());
            x.append(&mut rest);
            x
        }
        Marlang::Nil => vec![],
        _ => unreachable!("Should never decompose a non-list!"),
    }
}

#[derive(Debug)]
pub struct SymbolTable<K, V> {
    scopes: Vec<AHashMap<K, V>>,
}

impl<K, V> SymbolTable<K, V> {
    pub fn new() -> Self {
        Self {
            scopes: vec![AHashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(AHashMap::new());
    }

    pub fn pop_scope(&mut self) {
        match self.scopes.len() {
            0 => unreachable!(),
            1 => {
                panic!("cannot pop last scope in symbol table");
            }
            _ => {
                self.scopes.pop().unwrap();
            }
        }
    }
}

impl<K: Eq + Hash, V> SymbolTable<K, V> {
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.scopes.iter().rev().find_map(|scope| scope.get(key))
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.scopes.last_mut().unwrap().insert(key, value);
    }
}

impl<K, V> Default for SymbolTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
