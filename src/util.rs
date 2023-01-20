use std::io::{self, Write};

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
    writeln!(dest, "\n# Nodes Section\n{}", nodes.len() + 1)?;

    let mut edges: Vec<(MarId, MarId, String)> = vec![];

    let mut count = 0;
    for node in nodes {
        writeln!(dest, "|{{{}}}|", node)?;
        for n in node.children().into_iter().rev() {
            edges.push((count.into(), *n, "child".into()))
        }
        count += 1;
    }
    writeln!(dest, "|{{start}}|")?;
    edges.push((count.into(), (count - 1).into(), "child".into()));

    writeln!(dest, "\n# Edges Section\n{}", edges.len())?;
    for (src, dst, label) in edges {
        let src: usize = src.into();
        let dst: usize = dst.into();
        writeln!(dest, "{} {} 0 |{{{}}}|", src + 1, dst + 1, label)?;
    }

    Ok(())
}
