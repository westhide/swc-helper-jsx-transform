use swc_core::ecma::ast::{
    ArrayLit, Expr, ExprOrSpread, Ident, KeyValueProp, ObjectLit, Prop, PropOrSpread, SeqExpr,
};

use crate::{
    constant::{BOOLEAN_ATTRIBUTE, HTML_ELEMENT, SVG_ELEMENT},
    regex,
};

pub fn is_event(text: &str) -> bool {
    regex!("^on[^a-z]").is_match(text)
}

pub fn is_native_tag(tag: &str) -> bool {
    HTML_ELEMENT.contains(tag) || SVG_ELEMENT.contains(tag)
}

pub fn is_bool_attr(name: &str) -> bool {
    BOOLEAN_ATTRIBUTE.contains(name)
}

pub fn is_undefined_ident(ident: &Ident) -> bool {
    ident.as_ref() == "undefined"
}

pub fn is_constant_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Lit(_) => true,
        Expr::Ident(ident) => is_undefined_ident(ident),
        Expr::Array(ArrayLit { elems, .. }) => {
            elems.iter().all(|elem| {
                match elem {
                    None => true,
                    Some(ExprOrSpread {
                        spread: Some(_), ..
                    }) => false,
                    Some(ExprOrSpread { expr, .. }) => is_constant_expr(expr),
                }
            })
        },
        Expr::Object(ObjectLit { props, .. }) => {
            props.iter().all(|prop_or_spread| {
                match prop_or_spread {
                    PropOrSpread::Spread(_) => false,
                    PropOrSpread::Prop(box prop) => {
                        match prop {
                            Prop::Shorthand(ident) => is_undefined_ident(ident),
                            Prop::KeyValue(KeyValueProp { value, .. }) => is_constant_expr(value),
                            _ => false,
                        }
                    },
                }
            })
        },
        Expr::Seq(SeqExpr { exprs, .. }) => exprs.iter().all(|expr| is_constant_expr(expr)),
        _ => false,
    }
}
