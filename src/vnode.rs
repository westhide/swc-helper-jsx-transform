use swc_core::ecma::ast::{Expr, JSXElementChild, JSXExpr, JSXExprContainer, JSXSpreadChild};

use crate::{element::Element, fragment::Fragment, shared::Transform, text::Text};

#[derive(Debug)]
pub enum VNode<'a> {
    Text(Box<Text<'a>>),
    Element(Box<Element<'a>>),
    Expr(&'a Expr),
    Spread(&'a Expr),
    Fragment(Box<Fragment<'a>>),
}

impl<'a> VNode<'a> {
    pub fn is_static(&self) -> bool {
        match self {
            Self::Text(_) => true,
            Self::Element(element) => element.is_static,
            Self::Fragment(fragment) => fragment.is_static,
            Self::Expr(_) | Self::Spread(_) => false,
        }
    }

    pub fn static_content(&self) -> String {
        match self {
            Self::Text(text) => text.content.clone(),
            Self::Element(element) => element.static_content(),
            Self::Fragment(fragment) => fragment.static_content(),
            _ => panic!("Forbidden: get non static VNode content"),
        }
    }
}

impl<'a> From<Text<'a>> for VNode<'a> {
    fn from(text: Text<'a>) -> Self {
        Self::Text(box text)
    }
}

impl<'a> Transform<'a, VNode<'a>> for JSXExprContainer {
    fn transform(&'a self) -> VNode<'a> {
        match &self.expr {
            JSXExpr::JSXEmptyExpr(_) => panic!("Forbidden: Empty JSXExprContainer"),
            JSXExpr::Expr(expr) => VNode::Expr(expr),
        }
    }
}

impl<'a> Transform<'a, VNode<'a>> for JSXSpreadChild {
    fn transform(&'a self) -> VNode<'a> {
        VNode::Spread(&self.expr)
    }
}

impl<'a> Transform<'a, Option<VNode<'a>>> for JSXElementChild {
    fn transform(&'a self) -> Option<VNode<'a>> {
        match self {
            JSXElementChild::JSXText(jsx_text) => jsx_text.transform().map(VNode::from),
            JSXElementChild::JSXExprContainer(container) => Some(container.transform()),
            JSXElementChild::JSXSpreadChild(spread_child) => Some(spread_child.transform()),
            JSXElementChild::JSXElement(box element) => {
                Some(VNode::Element(box element.transform()))
            },
            JSXElementChild::JSXFragment(_) => panic!("Forbidden: JSXElementChild Fragment"),
        }
    }
}

impl<'a> Transform<'a, Vec<VNode<'a>>> for [JSXElementChild] {
    fn transform(&'a self) -> Vec<VNode<'a>> {
        self.iter().filter_map(Transform::transform).collect()
    }
}
