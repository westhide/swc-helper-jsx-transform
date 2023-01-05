use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{JSXAttr, JSXAttrOrSpread, Lit, SpreadElement, Str},
        atoms::ATOM_JSWORD_ as EMPTY_JS_WORD,
    },
};

use crate::{
    attr::{key::Key, value::Value},
    shared::Transform,
    utils::is::is_bool_attr,
};

pub mod key;
pub mod value;

#[derive(Debug)]
pub struct Attr<'a> {
    pub key: Key<'a>,
    pub value: Value<'a>,
}

impl<'a> Attr<'a> {
    pub fn is_static(&self) -> bool {
        self.value.is_static()
    }
}

const EMPTY_STR_LIT: &Lit = &Lit::Str(Str {
    span: DUMMY_SP,
    value: EMPTY_JS_WORD,
    raw: None,
});

impl<'a> Transform<'a, Attr<'a>> for JSXAttrOrSpread {
    fn transform(&'a self) -> Attr<'a> {
        match self {
            JSXAttrOrSpread::JSXAttr(JSXAttr { name, value, .. }) => {
                let key = name.transform();

                let value = match value.as_ref() {
                    Some(attr_value) => attr_value.transform(),
                    None => {
                        if let Key::Attr(attr) = key && is_bool_attr(attr) {
                            Value::Lit(EMPTY_STR_LIT)
                        } else {
                            Value::Empty
                        }
                    },
                };

                Attr { key, value }
            },
            JSXAttrOrSpread::SpreadElement(SpreadElement { expr, .. }) => {
                Attr {
                    key: Key::Spread,
                    value: Value::Expr(expr),
                }
            },
        }
    }
}
