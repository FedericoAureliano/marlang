use egg::{define_language, Analysis, DidMerge, Id, Language};

use fxhash::FxHashSet as HashSet;

use crate::util::decompose_using_graph;

pub type MarId = egg::Id;
pub type MarVar = egg::Var;
pub type MarGraph = egg::EGraph<Marlang, MarAnalysis>;
pub type MarRunner = egg::Runner<Marlang, MarAnalysis>;
pub type MarPattern = egg::Pattern<Marlang>;
pub type MarRecExpr = egg::RecExpr<Marlang>;
pub type MarRewrite = egg::Rewrite<Marlang, MarAnalysis>;
pub type MarPatternAst = egg::PatternAst<Marlang>;
pub type MarExplanation = egg::Explanation<Marlang>;

define_language! {
    pub enum Marlang {
        "marlang.function.call" = Call([MarId; 2]), // point to def/dec/let and then args

        // BEGIN N-ARY
        "marlang.operator.int.+" = IntAdd([MarId; 1]),
        "marlang.operator.int.-" = IntSub([MarId; 1]),
        "marlang.operator.int.*" = IntMul([MarId; 1]),
        "marlang.operator.int.>" = IntGt([MarId; 1]),
        "marlang.operator.int.>=" = IntGe([MarId; 1]),
        "marlang.operator.int.<" = IntLt([MarId; 1]),
        "marlang.operator.int.<=" = IntLe([MarId; 1]),

        "marlang.operator.real.+" = RealAdd([MarId; 1]),
        "marlang.operator.real.-" = RealSub([MarId; 1]),
        "marlang.operator.real.*" = RealMul([MarId; 1]),
        "marlang.operator.real./" = RealDiv([MarId; 1]),
        "marlang.operator.real.>" = RealGt([MarId; 1]),
        "marlang.operator.real.>=" = RealGe([MarId; 1]),
        "marlang.operator.real.<" = RealLt([MarId; 1]),
        "marlang.operator.real.<=" = RealLe([MarId; 1]),

        "marlang.operator.str.++" = Concat([MarId; 1]),

        "marlang.operator.core.and" = And([MarId; 1]),
        "marlang.operator.core.or" = Or([MarId; 1]),
        "marlang.operator.core.xor" = Xor([MarId; 1]),

        // takes a list of lists (bindings) and a body
        "marlang.operator.core.let" = Let([MarId; 2]),
        "marlang.operator.core.=" = Eq([MarId; 1]),
        // END N-ARY

        "marlang.operator.core.not" = Not([MarId; 1]),
        "marlang.operator.core.=>" = Implies([MarId; 2]),
        "marlang.operator.core.ite" = Ite([MarId; 3]),

        "marlang.command.set-logic" = SetLogic([MarId; 1]),
        "marlang.command.check-sat" = CheckSat,
        "marlang.command.assert" = Assert([MarId; 1]),
        "marlang.command.declare-fun" = DeclareFun([MarId; 3]),
        "marlang.command.define-fun" = DefineFun([MarId; 4]),

        "marlang.meta.cons" = Cons([MarId; 2]),
        "marlang.meta.nil" = Nil,

        "marlang.sort.bool" = BoolSort,
        "marlang.sort.int" = IntSort,
        "marlang.sort.real" = RealSort,
        "marlang.sort.string" = StringSort,

        "marlang.value.bool" = BoolVal([MarId; 1]),
        "marlang.value.int" = IntVal([MarId; 1]),
        "marlang.value.real" = RealVal([MarId; 1]),
        "marlang.value.string" = StringVal([MarId; 1]),

        Symbol(String),
    }
}

#[derive(Default)]
pub struct MarAnalysis;

#[derive(Debug)]
pub struct MarData {
    free: HashSet<Id>,
}

impl Analysis<Marlang> for MarAnalysis {
    type Data = MarData;
    fn merge(&mut self, to: &mut MarData, from: MarData) -> DidMerge {
        let before_len = to.free.len();
        to.free.retain(|i| from.free.contains(i));
        DidMerge(
            before_len != to.free.len(),
            to.free.len() != from.free.len(),
        )
    }

    fn make(egraph: &MarGraph, enode: &Marlang) -> MarData {
        let f = |i: &Id| egraph[*i].data.free.iter().cloned();
        let mut free = HashSet::default();
        match enode {
            Marlang::Call([n, _]) => {
                free.insert(*n);
            }
            Marlang::Let([bindings, body]) => {
                free.extend(f(body));

                let bindings = decompose_using_graph(egraph, *bindings);
                let bindings = decompose_using_graph(egraph, bindings[0]);
                for pair in bindings {
                    let pair = decompose_using_graph(egraph, pair);
                    if pair.len() == 0 {
                        continue;
                    } else if pair.len() == 2 {
                        panic!("malformed let binding");
                    }
                    free.remove(&pair[0]);
                    free.extend(f(&pair[1]));
                }
            }
            _ => enode.for_each(|c| free.extend(&egraph[c].data.free)),
        }
        MarData { free }
    }
}
