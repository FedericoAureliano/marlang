use egg::*;

use fxhash::FxHashSet as HashSet;
use rug::{Integer, Rational};

pub type MId = egg::Id;
pub type MGraph = egg::EGraph<Marlang, MarAnalysis>;
pub type MSymbol = egg::Symbol;
pub type MRunner = egg::Runner<Marlang, MarAnalysis>;
pub type MPattern = egg::Pattern<Marlang>;
pub type MRecExpr = egg::RecExpr<Marlang>;
pub type MRewrite = egg::Rewrite<Marlang, MarAnalysis>;
pub type MPatternAst = egg::PatternAst<Marlang>;
pub type MExplanation = egg::Explanation<Marlang>;

define_language! {
    pub enum Marlang {
        "var" = Var([Id; 2]),

        // BEGIN N-ARY
        "+" = Add([Id; 1]),
        "-" = Sub([Id; 1]),
        "*" = Mul([Id; 1]),
        "/" = RealDiv([Id; 1]),

        "=" = Eq([Id; 1]),
        ">" = Gt([Id; 1]),
        ">=" = Ge([Id; 1]),
        "<" = Lt([Id; 1]),
        "<=" = Le([Id; 1]),

        "str.++" = Concat([Id; 1]),

        "and" = And([Id; 1]),
        "or" = Or([Id; 1]),
        "xor" = Xor([Id; 1]),
        // END N-ARY

        "not" = Not([Id; 1]),
        "=>" = Implies([Id; 2]),
        "ite" = Ite([Id; 3]),

        "set-logic" = SetLogic([Id; 1]),
        "check-sat" = CheckSat,
        "assert" = Assert([Id; 1]),
        "declare-const" = DeclareConst([Id; 2]),

        "CONS" = Cons([Id; 2]),
        "NIL" = Nil,

        "Bool" = BoolSort,
        "Int" = IntSort,
        "Real" = RealSort,
        "String" = StringSort,

        BoolVal(bool),
        IntVal(Integer),
        RealVal(Rational),
        StringVal(String),

        Symbol(Symbol),
    }
}

#[derive(Default)]
pub struct MarAnalysis;

#[derive(Debug)]
pub struct MData {
    free: HashSet<Id>,
    constant: Option<(Marlang, MPatternAst)>,
}

fn eval(_mgraph: &MGraph, _mnode: &Marlang) -> Option<(Marlang, MPatternAst)> {
    // TODO: write eval
    None
}

impl Analysis<Marlang> for MarAnalysis {
    type Data = MData;
    fn merge(&mut self, to: &mut MData, from: MData) -> DidMerge {
        let before_len = to.free.len();
        // to.free.extend(from.free);
        to.free.retain(|i| from.free.contains(i));
        // compare lengths to see if I changed to or from
        DidMerge(
            before_len != to.free.len(),
            to.free.len() != from.free.len(),
        ) | merge_option(&mut to.constant, from.constant, |a, b| {
            assert_eq!(a.0, b.0, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }

    fn make(egraph: &MGraph, enode: &Marlang) -> MData {
        let mut free = HashSet::default();
        enode.for_each(|c| free.extend(&egraph[c].data.free));
        let constant = eval(egraph, enode);
        MData { constant, free }
    }

    fn modify(egraph: &mut MGraph, id: Id) {
        if let Some(c) = egraph[id].data.constant.clone() {
            if egraph.are_explanations_enabled() {
                egraph.union_instantiations(
                    &c.0.to_string().parse().unwrap(),
                    &c.1,
                    &Default::default(),
                    "analysis".to_string(),
                );
            } else {
                let const_id = egraph.add(c.0);
                egraph.union(id, const_id);
            }
        }
    }
}
