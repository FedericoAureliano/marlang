use rand::Rng;

use std::{
    collections::HashMap,
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
        _ => vec![i.into()],
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
            "marlang.function.call" => mexpr.add(Marlang::Call([children[0], children[1]])),
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
                let empty = mexpr.add(Marlang::Nil);
                mexpr.add(Marlang::DeclareFun([children[0], empty, children[1]]))
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

pub fn sample<R: Rng>(rng: &mut R, og: &MarRecExpr, max_depth: usize) -> MarRecExpr {
    let nodes = og.as_ref().to_owned();
    let position = rng.gen_range(0..nodes.len());

    let node = og.as_ref().to_owned()[position].clone();

    match node {
        Marlang::Cons(_) | Marlang::Nil | Marlang::Symbol(_) => {
            // don't ever sample just a meta operator or symbol at the top
            return sample(rng, og, max_depth.checked_sub(1).unwrap_or(0));
        }
        _ => (),
    };

    let mut out = MarRecExpr::default();

    sample_helper(
        rng,
        &mut out,
        og,
        max_depth,
        &(position.into()),
        &mut HashMap::new(),
        &mut HashMap::new(),
    );

    out
}

fn add_helper(out: &mut MarRecExpr, ids: &mut HashMap<Marlang, MarId>, expr: Marlang) -> MarId {
    if ids.contains_key(&expr) {
        return ids[&expr];
    } else {
        let id = out.add(expr.clone());
        ids.insert(expr, id);
        id
    }
}

fn sample_helper<R: Rng>(
    rng: &mut R,
    out: &mut MarRecExpr,
    og: &MarRecExpr,
    max_depth: usize,
    position: &MarId,
    pts: &mut HashMap<Marlang, Marlang>,
    ids: &mut HashMap<Marlang, MarId>,
) -> MarId {
    let position: usize = (*position).into();

    let mut node = match &og.as_ref().to_owned()[position] {
        // The first three shouldn't count against depth
        Marlang::Cons([a, b]) => {
            let cons = Marlang::Cons([
                sample_helper(rng, out, og, max_depth, &a, pts, ids),
                sample_helper(rng, out, og, max_depth, &b, pts, ids),
            ]);
            return add_helper(out, ids, cons);
        }
        Marlang::DeclareFun([n, ps, s]) => {
            let declare_fun = Marlang::DeclareFun([
                sample_helper(rng, out, og, max_depth, &n, pts, ids),
                sample_helper(rng, out, og, max_depth, &ps, pts, ids),
                sample_helper(rng, out, og, max_depth, &s, pts, ids),
            ]);
            return add_helper(out, ids, declare_fun);
        }
        Marlang::SetLogic([s]) => {
            let set_logic =
                Marlang::SetLogic([sample_helper(rng, out, og, max_depth, &s, pts, ids)]);
            return add_helper(out, ids, set_logic);
        }
        node if node.children().len() == 0 => node.clone(),
        node if max_depth == 0 && pts.contains_key(&node) => pts[&node].clone(),
        node if max_depth == 0 => {
            let pattern = Marlang::Symbol(random_pattern_variable(rng).into());
            pts.insert(node.clone(), pattern.clone());
            pattern
        }
        node => node.clone(),
    };

    for i in 0..node.children().len() {
        let child = node.children()[i];
        let new_child_position = sample_helper(
            rng,
            out,
            og,
            max_depth.checked_sub(1).unwrap_or(0),
            &(child.into()),
            pts,
            ids,
        );
        node.children_mut()[i] = new_child_position;
    }

    add_helper(out, ids, node)
}

fn random_pattern_variable<R: Rng>(rng: &mut R) -> String {
    let mut s = String::new();
    s.insert_str(0, "?marlang.fresh.");
    for _ in 0..10 {
        s.push(rng.gen_range('a'..'z'));
    }
    s
}
