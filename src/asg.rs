use egg::{define_language, merge_option, Analysis, DidMerge, Id, Language};

use fxhash::FxHashSet as HashSet;
use rug::{Integer, Rational};

pub type MarId = egg::Id;
pub type MarGraph = egg::EGraph<Marlang, MarAnalysis>;
pub type MarSymbol = egg::Symbol;
pub type MarRunner = egg::Runner<Marlang, MarAnalysis>;
pub type MarPattern = egg::Pattern<Marlang>;
pub type MarRecExpr = egg::RecExpr<Marlang>;
pub type MarRewrite = egg::Rewrite<Marlang, MarAnalysis>;
pub type MarPatternAst = egg::PatternAst<Marlang>;
pub type MarExplanation = egg::Explanation<Marlang>;

define_language! {
    pub enum Marlang {
        "var" = Var([MarId; 2]),

        // BEGIN N-ARY
        "+" = Add([MarId; 1]),
        "-" = Sub([MarId; 1]),
        "*" = Mul([MarId; 1]),
        "/" = RealDiv([MarId; 1]),

        "=" = Eq([MarId; 1]),
        ">" = Gt([MarId; 1]),
        ">=" = Ge([MarId; 1]),
        "<" = Lt([MarId; 1]),
        "<=" = Le([MarId; 1]),

        "str.++" = Concat([MarId; 1]),

        "and" = And([MarId; 1]),
        "or" = Or([MarId; 1]),
        "xor" = Xor([MarId; 1]),

        "let" = Let([MarId; 2]), // takes a list of lists (bindings) and a body
        // END N-ARY

        "not" = Not([MarId; 1]),
        "=>" = Implies([MarId; 2]),
        "ite" = Ite([MarId; 3]),

        "set-logic" = SetLogic([MarId; 1]),
        "check-sat" = CheckSat,
        "assert" = Assert([MarId; 1]),
        "declare-const" = DeclareConst([MarId; 2]),
        "declare-fun" = DeclareFun([MarId; 3]),
        "define-fun" = DefineFun([MarId; 4]),

        "CONS" = Cons([MarId; 2]),
        "NIL" = Nil,

        "Bool" = BoolSort,
        "Int" = IntSort,
        "Real" = RealSort,
        "String" = StringSort,

        BoolVal(bool),
        IntVal(Integer),
        RealVal(Rational),
        "str" = StringVal([MarId; 1]), // To avoid clash with symbols since egg can't handle quotes

        Symbol(MarSymbol),
    }
}

#[derive(Default)]
pub struct MarAnalysis;

#[derive(Debug)]
pub struct MarData {
    free: HashSet<MarId>,
    constant: Option<(Marlang, MarPatternAst)>,
}

fn eval(_mgraph: &MarGraph, _mnode: &Marlang) -> Option<(Marlang, MarPatternAst)> {
    // TODO: write eval
    None
}

impl Analysis<Marlang> for MarAnalysis {
    type Data = MarData;
    fn merge(&mut self, to: &mut MarData, from: MarData) -> DidMerge {
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

    fn make(egraph: &MarGraph, enode: &Marlang) -> MarData {
        let mut free = HashSet::default();
        enode.for_each(|c| free.extend(&egraph[c].data.free));
        let constant = eval(egraph, enode);
        MarData { constant, free }
    }

    fn modify(egraph: &mut MarGraph, id: MarId) {
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
