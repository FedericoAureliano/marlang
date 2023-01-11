use egg::{define_language, merge_option, Analysis, DidMerge, Id, Language};

use fxhash::FxHashSet as HashSet;
use rug::{Integer, Rational};

use crate::decompose_using_graph;

pub type MarId = egg::Id;
pub type MarVar = egg::Var;
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
        "int.+" = IntAdd([MarId; 1]),
        "int.-" = IntSub([MarId; 1]),
        "int.*" = IntMul([MarId; 1]),
        "int./" = IntDiv([MarId; 1]),
        "int.>" = IntGt([MarId; 1]),
        "int.>=" = IntGe([MarId; 1]),
        "int.<" = IntLt([MarId; 1]),
        "int.<=" = IntLe([MarId; 1]),

        "real.+" = RealAdd([MarId; 1]),
        "real.-" = RealSub([MarId; 1]),
        "real.*" = RealMul([MarId; 1]),
        "real./" = RealDiv([MarId; 1]),
        "real.>" = RealGt([MarId; 1]),
        "real.>=" = RealGe([MarId; 1]),
        "real.<" = RealLt([MarId; 1]),
        "real.<=" = RealLe([MarId; 1]),

        "str.++" = Concat([MarId; 1]),

        "and" = And([MarId; 1]),
        "or" = Or([MarId; 1]),
        "xor" = Xor([MarId; 1]),

        "let" = Let([MarId; 2]), // takes a list of lists (bindings) and a body
        "=" = Eq([MarId; 1]),
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

impl Marlang {
    fn int(&self) -> Option<Integer> {
        match self {
            Marlang::IntVal(n) => Some(n.into()),
            _ => None,
        }
    }
    fn real(&self) -> Option<Rational> {
        match self {
            Marlang::RealVal(n) => Some(n.into()),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct MarAnalysis;

#[derive(Debug)]
pub struct MarData {
    free: HashSet<Id>,
    constant: Option<(Marlang, MarPatternAst)>,
}

fn eval(egraph: &MarGraph, enode: &Marlang) -> Option<(Marlang, MarPatternAst)> {
    let x = |i: &Id| egraph[*i].data.constant.as_ref().map(|c| &c.0);
    match enode {
        Marlang::IntVal(n) => Some((enode.clone(), format!("{}", n).parse().unwrap())),
        Marlang::BoolVal(b) => Some((enode.clone(), format!("{}", b).parse().unwrap())),
        Marlang::IntAdd([args]) => {
            let decomposed = decompose_using_graph(egraph, *args);
            let mut int_val: Integer = 0.into();
            for a in decomposed {
                let a = x(&a)?.int()?;
                int_val = int_val + a;
            }
            let int_val = Marlang::IntVal(int_val);
            let args = egraph.id_to_expr(*args);
            let pattern: MarPatternAst = format!("(int.+ {})", args).parse().unwrap();
            Some((int_val, pattern))
        }
        Marlang::IntSub([args]) => {
            let decomposed = decompose_using_graph(egraph, *args);
            let mut int_val: Integer = 0.into();
            for a in decomposed {
                let a = x(&a)?.int()?;
                int_val = int_val - a;
            }
            let int_val = Marlang::IntVal(int_val);
            let args = egraph.id_to_expr(*args);
            let pattern: MarPatternAst = format!("(int.- {})", args).parse().unwrap();
            Some((int_val, pattern))
        }
        Marlang::IntMul([args]) => {
            let decomposed = decompose_using_graph(egraph, *args);
            let mut int_val: Integer = 0.into();
            for a in decomposed {
                let a = x(&a)?.int()?;
                int_val = int_val * a;
            }
            let int_val = Marlang::IntVal(int_val);
            let args = egraph.id_to_expr(*args);
            let pattern: MarPatternAst = format!("(int.* {})", args).parse().unwrap();
            Some((int_val, pattern))
        }
        Marlang::RealAdd([args]) => {
            let decomposed = decompose_using_graph(egraph, *args);
            let mut real_val: Rational = 0.into();
            for a in decomposed {
                let a = x(&a)?.real()?;
                real_val = real_val + a;
            }
            let real_val = Marlang::RealVal(real_val);
            let args = egraph.id_to_expr(*args);
            let pattern: MarPatternAst = format!("(real.+ {})", args).parse().unwrap();
            Some((real_val, pattern))
        }
        Marlang::RealSub([args]) => {
            let decomposed = decompose_using_graph(egraph, *args);
            let mut real_val: Rational = 0.into();
            for a in decomposed {
                let a = x(&a)?.real()?;
                real_val = real_val - a;
            }
            let real_val = Marlang::RealVal(real_val);
            let args = egraph.id_to_expr(*args);
            let pattern: MarPatternAst = format!("(real.- {})", args).parse().unwrap();
            Some((real_val, pattern))
        }
        Marlang::RealMul([args]) => {
            let decomposed = decompose_using_graph(egraph, *args);
            let mut real_val: Rational = 0.into();
            for a in decomposed {
                let a = x(&a)?.real()?;
                real_val = real_val * a;
            }
            let real_val = Marlang::RealVal(real_val);
            let args = egraph.id_to_expr(*args);
            let pattern: MarPatternAst = format!("(real.* {})", args).parse().unwrap();
            Some((real_val, pattern))
        }
        _ => None,
    }
}

impl Analysis<Marlang> for MarAnalysis {
    type Data = MarData;
    fn merge(&mut self, to: &mut MarData, from: MarData) -> DidMerge {
        let before_len = to.free.len();
        to.free.retain(|i| from.free.contains(i));
        DidMerge(
            before_len != to.free.len(),
            to.free.len() != from.free.len(),
        ) | merge_option(&mut to.constant, from.constant, |a, b| {
            assert_eq!(a.0, b.0, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }

    fn make(egraph: &MarGraph, enode: &Marlang) -> MarData {
        let f = |i: &Id| egraph[*i].data.free.iter().cloned();
        let mut free = HashSet::default();
        match enode {
            Marlang::Var([n, _]) => {
                free.insert(*n);
            }
            Marlang::Let([bindings, body]) => {
                free.extend(f(body));

                let bindings = decompose_using_graph(egraph, *bindings);
                for pair in bindings {
                    let pair = decompose_using_graph(egraph, pair);
                    free.remove(&pair[0]);
                    free.extend(f(&pair[1]));
                }
            }
            _ => enode.for_each(|c| free.extend(&egraph[c].data.free)),
        }
        let constant = eval(egraph, enode);
        MarData { constant, free }
    }

    fn modify(egraph: &mut MarGraph, id: Id) {
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
