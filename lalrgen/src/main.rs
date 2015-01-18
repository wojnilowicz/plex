#![allow(unstable)]
#![feature(quote)]

extern crate lalr;
extern crate syntax;

use lalr::*;
use syntax::codemap::{self, DUMMY_SP};
use syntax::ext::base::ExtCtxt;
use std::collections::BTreeMap;
use syntax::parse::token::str_to_ident;
use lib as lalrgen;

mod lib;

fn N<T>(x: &str) -> Symbol<T, syntax::ast::Ident> {
    Nonterminal(str_to_ident(x))
}

fn T<N>(x: char) -> Symbol<char, N> {
    Terminal(x)
}

macro_rules! map {
    ($($l: expr => $r: expr),*) => ({
        let mut r = BTreeMap::new();
        $(r.insert(str_to_ident($l), $r);)*
        r
    })
}

fn rhs<T, N, A>(syms: Vec<Symbol<T, N>>, act: A) -> Rhs<T, N, A> {
    Rhs {
        syms: syms,
        act: act,
    }
}

fn main() {
    let ps = syntax::parse::new_parse_sess();
    let cx = &mut ExtCtxt::new(&ps, vec![],
        syntax::ext::expand::ExpansionConfig::default("larlgen-test".to_string())
    );
    cx.bt_push(codemap::ExpnInfo {
        call_site: DUMMY_SP,
        callee: codemap::NameAndSpan {
            name: "".to_string(),
            format: codemap::MacroBang,
            span: None,
        }
    });
    let g = Grammar {
        rules: map![
            "S" => vec![
                rhs(vec![N("N")], ()),
            ],
            "N" => vec![
                rhs(vec![N("V"), T('='), N("E")], ()),
                rhs(vec![N("E")], ()),
            ],
            "E" => vec![
                rhs(vec![N("V")], ()),
            ],
            "V" => vec![
                rhs(vec![T('x')], ()),
                rhs(vec![T('*'), N("E")], ()),
            ]
        ],
        start: str_to_ident("S")
    };
    let types = g.rules.keys().map(|k|
        (*k, quote_ty!(cx, ()))
    ).collect();
    let token_ty = quote_ty!(cx, char);
    let x = lalrgen::lr1_machine(cx, g, &types, token_ty, syntax::parse::token::str_to_ident("parse"), syntax::ast::Visibility::Public);
    println!("{}", syntax::print::pprust::item_to_string(&*x));
}
