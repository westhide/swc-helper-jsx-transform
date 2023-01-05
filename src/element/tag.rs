use swc_core::ecma::ast::{JSXElementName, JSXMemberExpr};

use crate::{shared::Transform, utils::is::is_native_tag};

#[derive(Debug)]
pub enum Tag<'a> {
    Native(&'a str),
    /// Component or Custom element
    Extra(&'a str),
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
            JSXElementName::Ident(ident) => {
                match ident.as_ref() {
                    name if is_native_tag(name) => Tag::Native(name),
                    name => Tag::Extra(name),
                }
            },
            JSXElementName::JSXMemberExpr(member) => Tag::Member(member),
            JSXElementName::JSXNamespacedName(_) => {
                panic!("Non support: JSXNamespacedName Element")
            },
        }
    }
}
