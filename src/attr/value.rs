use swc_core::ecma::ast::{Expr, JSXAttrValue, JSXExpr, Lit};

use crate::{element::Element, fragment::Fragment, shared::Transform, utils::is::is_constant_expr};

#[derive(Debug)]
pub enum Value<'a> {
    Lit(&'a Lit),
    /// TODO: [is_constant_expr] Non-strict
    Const(&'a Expr),
    Expr(&'a Expr),
    Element(Box<Element<'a>>),
    Fragment(Box<Fragment<'a>>),
    Empty,
}

impl<'a> Value<'a> {
    fn specialize_expr(expr: &'a Expr) -> Self {
        match expr {
            Expr::Lit(lit) => Self::Lit(lit),
            Expr::JSXElement(box element) => Value::Element(box element.transform()),
            Expr::JSXFragment(fragment) => Value::Fragment(box fragment.transform()),
            expr if is_constant_expr(expr) => Self::Const(expr),
            expr => Self::Expr(expr),
        }
    }

    pub fn is_static(&self) -> bool {
        matches!(self, Self::Lit(_))
    }
}

impl<'a> Transform<'a, Value<'a>> for JSXAttrValue {
    fn transform(&'a self) -> Value<'a> {
        match self {
            JSXAttrValue::Lit(lit) => Value::Lit(lit),
            JSXAttrValue::JSXExprContainer(container) => {
                match &container.expr {
                    JSXExpr::Expr(expr) => Value::specialize_expr(expr),
                    JSXExpr::JSXEmptyExpr(_) => panic!("Forbidden: Empty JSXExprContainer"),
                }
            },
            JSXAttrValue::JSXElement(box element) => Value::Element(box element.transform()),
            JSXAttrValue::JSXFragment(fragment) => Value::Fragment(box fragment.transform()),
        }
    }
}
