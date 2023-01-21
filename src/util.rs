use std::{
    io::{self, Read, Write},
    usize,
};

use egg::Language;

use crate::ast::{MarGraph, MarId, MarRecExpr, Marlang};

pub fn decompose_using_graph(mgraph: &MarGraph, ls: MarId) -> Vec<MarId> {
    let mexpr = mgraph.id_to_expr(ls);
    let last = mexpr.as_ref().len() - 1;
    decompose_using_expr_rec(&mexpr, last)
}

pub fn decompose_using_expr(mexpr: &MarRecExpr, ls: MarId) -> Vec<MarId> {
    let last = ls.into();
    decompose_using_expr_rec(mexpr, last)
}

fn decompose_using_expr_rec(mexpr: &MarRecExpr, i: usize) -> Vec<MarId> {
    match mexpr.as_ref()[i] {
        Marlang::Cons([x, s]) => {
            let mut x = vec![x];
            let mut rest = decompose_using_expr_rec(mexpr, s.into());
            x.append(&mut rest);
            x
        }
        Marlang::Nil => vec![],
        Marlang::Symbol(_) => vec![i.into()], // rest pattern is sometimes in the last place
        _ => unreachable!("Should never decompose a non-list!"),
    }
}

pub fn write_leda<T: Write>(dest: &mut T, mexpr: &MarRecExpr) -> io::Result<()> {
    writeln!(dest, "LEDA.GRAPH\nstring\nstring\n-1")?;
    let nodes = mexpr.as_ref().to_owned();
    writeln!(dest, "\n# Nodes Section\n{}", nodes.len())?;

    let mut edges: Vec<(MarId, MarId, String)> = vec![];

    let mut count = 0;
    for node in nodes {
        writeln!(dest, "|{{{}}}|", node)?;
        for n in node.children().into_iter().rev() {
            edges.push((count.into(), *n, "child".into()))
        }
        count += 1;
    }

    writeln!(dest, "\n# Edges Section\n{}", edges.len())?;
    for (src, dst, label) in edges {
        let src: usize = src.into();
        let dst: usize = dst.into();
        writeln!(dest, "{} {} 0 |{{{}}}|", src + 1, dst + 1, label)?;
    }

    Ok(())
}

pub fn read_leda<T: Read>(source: &mut T) -> io::Result<MarRecExpr> {
    let parse_edge = |line: &str| -> (usize, usize) {
        let line: Vec<&str> = line.split(" ").collect();
        let src = line[0].parse::<usize>().unwrap() - 1;
        let dst = line[1].parse::<usize>().unwrap() - 1;
        (src, dst)
    };

    let parse_node = |line: &str| -> String {
        let line = line.to_string();
        let line = line
            .strip_prefix("|{")
            .expect("node must start with |{")
            .to_string();
        let line = line
            .strip_suffix("}|")
            .expect("node must end with }|")
            .to_string();
        line.to_string()
    };

    let mut buffer = String::new();
    source.read_to_string(&mut buffer)?;

    let mut count = 0;
    let mut state = "expect_leda";
    let mut expect_nodes_number = 0;
    let mut expect_edges_number = 0;
    let mut nodes: Vec<String> = vec![];
    let mut edges: Vec<(usize, usize)> = vec![];
    for line in buffer.lines() {
        if line.starts_with("#") {
            continue;
        } else if line.is_empty() {
            continue;
        } else {
            count += 1;
            if state == "expect_leda" && line == "LEDA.GRAPH" {
                state = "expect_string";
            } else if state == "expect_string" && line == "string" && count == 2 {
                state = "expect_string";
            } else if state == "expect_string" && line == "string" && count == 3 {
                state = "expect_dash1";
            } else if state == "expect_dash1" {
                state = "expect_nodes_number";
            } else if state == "expect_nodes_number" {
                state = "expect_nodes";
                expect_nodes_number = line.parse::<usize>().unwrap();
                count = 0;
            } else if state == "expect_nodes" && count < expect_nodes_number {
                nodes.push(parse_node(line));
            } else if state == "expect_nodes" && count == expect_nodes_number {
                state = "expect_edges_number";
                nodes.push(parse_node(line));
            } else if state == "expect_edges_number" {
                state = "expect_edges";
                expect_edges_number = line.parse::<usize>().unwrap();
                count = 0;
            } else if state == "expect_edges" && count < expect_edges_number {
                edges.push(parse_edge(line));
            } else if state == "expect_edges" && count == expect_edges_number {
                state = "done";
                edges.push(parse_edge(line));
            } else {
                panic!("Unexpected line: {}", line);
            }
        }
    }

    assert_eq!(state, "done", "state: {}", state);

    let mut mexpr = MarRecExpr::default();

    for (i, node) in nodes.iter().enumerate() {
        let children = edges
            .iter()
            .filter(|(src, _dst)| *src == i)
            .map(|(_src, dst)| dst)
            .map(|dst| (*dst).into())
            .rev()
            .collect::<Vec<MarId>>();

        match node.as_str() {
            "marlang.function" => mexpr.add(Marlang::Function([children[0], children[1]])),
            "marlang.operator.int.+" => mexpr.add(Marlang::IntAdd([children[0]])),
            "marlang.operator.int.-" => mexpr.add(Marlang::IntSub([children[0]])),
            "marlang.operator.int.*" => mexpr.add(Marlang::IntMul([children[0]])),
            "marlang.operator.int.>" => mexpr.add(Marlang::IntGt([children[0]])),
            "marlang.operator.int.>=" => mexpr.add(Marlang::IntGe([children[0]])),
            "marlang.operator.int.<" => mexpr.add(Marlang::IntLt([children[0]])),
            "marlang.operator.int.<=" => mexpr.add(Marlang::IntLe([children[0]])),
            "marlang.operator.real.+" => mexpr.add(Marlang::RealAdd([children[0]])),
            "marlang.operator.real.-" => mexpr.add(Marlang::RealSub([children[0]])),
            "marlang.operator.real.*" => mexpr.add(Marlang::RealMul([children[0]])),
            "marlang.operator.real./" => mexpr.add(Marlang::RealDiv([children[0]])),
            "marlang.operator.real.>" => mexpr.add(Marlang::RealGt([children[0]])),
            "marlang.operator.real.>=" => mexpr.add(Marlang::RealGe([children[0]])),
            "marlang.operator.real.<" => mexpr.add(Marlang::RealLt([children[0]])),
            "marlang.operator.real.<=" => mexpr.add(Marlang::RealLe([children[0]])),
            "marlang.operator.str.++" => mexpr.add(Marlang::Concat([children[0]])),
            "marlang.operator.core.and" => mexpr.add(Marlang::And([children[0]])),
            "marlang.operator.core.or" => mexpr.add(Marlang::Or([children[0]])),
            "marlang.operator.core.xor" => mexpr.add(Marlang::Xor([children[0]])),
            "marlang.operator.core.let" => mexpr.add(Marlang::Let([children[0], children[1]])),
            "marlang.operator.core.=" => mexpr.add(Marlang::Eq([children[0]])),
            "marlang.operator.core.not" => mexpr.add(Marlang::Not([children[0]])),
            "marlang.operator.core.=>" => mexpr.add(Marlang::Implies([children[0], children[1]])),
            "marlang.operator.core.ite" => {
                mexpr.add(Marlang::Ite([children[0], children[1], children[2]]))
            }
            "marlang.command.set-logic" => mexpr.add(Marlang::SetLogic([children[0]])),
            "marlang.command.check-sat" => mexpr.add(Marlang::CheckSat),
            "marlang.command.assert" => mexpr.add(Marlang::Assert([children[0]])),
            "marlang.command.declare-const" => {
                mexpr.add(Marlang::DeclareConst([children[0], children[1]]))
            }
            "marlang.command.declare-fun" => {
                mexpr.add(Marlang::DeclareFun([children[0], children[1], children[2]]))
            }
            "marlang.command.define-fun" => mexpr.add(Marlang::DefineFun([
                children[0],
                children[1],
                children[2],
                children[3],
            ])),
            "marlang.meta.cons" => mexpr.add(Marlang::Cons([children[0], children[1]])),
            "marlang.meta.nil" => mexpr.add(Marlang::Nil),
            "marlang.sort.bool" => mexpr.add(Marlang::BoolSort),
            "marlang.sort.int" => mexpr.add(Marlang::IntSort),
            "marlang.sort.real" => mexpr.add(Marlang::RealSort),
            "marlang.sort.string" => mexpr.add(Marlang::StringSort),
            "marlang.value.bool" => mexpr.add(Marlang::BoolVal([children[0]])),
            "marlang.value.int" => mexpr.add(Marlang::IntVal([children[0]])),
            "marlang.value.real" => mexpr.add(Marlang::RealVal([children[0]])),
            "marlang.value.string" => mexpr.add(Marlang::StringVal([children[0]])),
            s => mexpr.add(Marlang::Symbol(s.into())),
        };
    }

    Ok(mexpr)
}
