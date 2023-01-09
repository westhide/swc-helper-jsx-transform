use swc_core::ecma::ast::{Ident, JSXElementName, JSXMemberExpr};

use crate::{shared::Transform, utils::is::is_native_tag};

#[derive(Debug)]
pub enum Tag<'a> {
    Native(&'a str),
    /// Component or Custom element
    Extra(&'a Ident),
    Member(&'a JSXMemberExpr),
}

impl<'a> Tag<'a> {
    pub fn is_native(&self) -> bool {
        matches!(self, Self::Native(_))
    }
}

impl<'a> Transform<'a, Tag<'a>> for JSXElementName {
    fn transform(&'a self) -> Tag<'a> {
        match self {
            JSXElementName::Ident(ident) if is_native_tag(&ident.sym) => Tag::Native(&ident.sym),
            JSXElementName::Ident(ident) => Tag::Extra(ident),
            JSXElementName::JSXMemberExpr(member) => Tag::Member(member),
            JSXElementName::JSXNamespacedName(_) => {
                panic!("Non support: JSXNamespacedName Element")
            },
        }
    }
}
