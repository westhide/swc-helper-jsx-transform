use swc_core::ecma::ast::JSXFragment;

use crate::{shared::Transform, vnode::VNode};

#[derive(Debug)]
pub struct Fragment<'a> {
    pub children: Vec<VNode<'a>>,

    pub is_static: bool,
}

impl<'a> Fragment<'a> {
    pub fn static_content(&self) -> String {
        if self.is_static {
            self.children.iter().map(VNode::static_content).collect()
        } else {
            panic!("Forbidden: get non static Fragment content")
        }
    }
}

impl<'a> Transform<'a, Fragment<'a>> for JSXFragment {
    fn transform(&'a self) -> Fragment<'a> {
        let children: Vec<VNode> = self.children.transform();

        let is_static = children.iter().all(VNode::is_static);

        Fragment {
            children,

            is_static,
        }
    }
}
