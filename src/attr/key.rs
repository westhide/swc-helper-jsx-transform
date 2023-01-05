use swc_core::ecma::ast::{JSXAttrName, JSXNamespacedName};

use crate::{shared::Transform, utils::is::is_event};

#[derive(Debug)]
pub enum Key<'a> {
    Attr(&'a str),
    Event(&'a str),
    NSAttr {
        ns: &'a str,
        name: &'a str,
    },
    /// dynamic attribute is terrible for compiler optimize,
    /// [Spread Attribute](Key::Spread) not recommend to use
    Spread,
}

impl<'a> Transform<'a, Key<'a>> for JSXAttrName {
    fn transform(&'a self) -> Key<'a> {
        match self {
            JSXAttrName::Ident(ident) => {
                match ident.as_ref() {
                    name if is_event(name) => Key::Event(&name[2..]),
                    name => Key::Attr(name),
                }
            },
            JSXAttrName::JSXNamespacedName(JSXNamespacedName { ns, name }) => {
                let name = name.as_ref();

                match ns.as_ref() {
                    "on" => Key::Event(name),
                    ns => Key::NSAttr { ns, name },
                }
            },
        }
    }
}
