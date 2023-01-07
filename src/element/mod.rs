use swc_core::ecma::ast::JSXElement;

use crate::{
    attr::Attr, element::tag::Tag, shared::Transform, utils::emit_code::emit_code, vnode::VNode,
};

pub mod tag;

#[derive(Debug)]
pub struct Element<'a> {
    pub tag: Tag<'a>,
    pub attrs: Vec<Attr<'a>>,
    pub children: Vec<VNode<'a>>,

    pub raw: &'a JSXElement,
    pub is_static: bool,
}

impl<'a> Element<'a> {
    pub fn static_content(&self) -> String {
        if self.is_static {
            emit_code(self.raw)
        } else {
            panic!("Forbidden: get non static  content")
        }
    }
}

impl<'a> Transform<'a, Element<'a>> for JSXElement {
    fn transform(&'a self) -> Element<'a> {
        let Self {
            opening, children, ..
        } = self;

        let tag = opening.name.transform();

        let attrs = opening.attrs.transform();

        let children: Vec<VNode> = children.transform();

        let is_static = tag.is_native()
            && attrs.iter().all(Attr::is_static)
            && children.iter().all(VNode::is_static);

        Element {
            tag,
            attrs,
            children,

            raw: self,
            is_static,
        }
    }
}
