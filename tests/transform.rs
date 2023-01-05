#![feature(box_syntax)]
#![feature(box_patterns)]

use insta::assert_debug_snapshot;
use swc_core::ecma::{
    ast::Expr,
    parser::{Syntax, TsConfig},
    transforms::testing::Tester,
    visit::{as_folder, VisitMut},
};
use swc_helper_jsx_transform::shared::Transform;

const TSX_SYNTAX: Syntax = Syntax::Typescript(TsConfig {
    tsx: true,
    decorators: false,
    dts: false,
    no_early_errors: false,
});

macro_rules! snapshot {
    ($name:ident, $value:expr) => {
        assert_debug_snapshot!(stringify!($name), $value)
    };
}

macro_rules! test {
    ($name:ident, $src:literal) => {
        #[test]
        fn $name() {
            struct TransformVisitor;

            impl VisitMut for TransformVisitor {
                fn visit_mut_expr(&mut self, expr: &mut Expr) {
                    match expr {
                        Expr::JSXElement(box element) => {
                            snapshot!($name, element.transform())
                        },
                        Expr::JSXFragment(fragment) => {
                            snapshot!($name, fragment.transform())
                        },
                        _ => {},
                    }
                }
            }

            Tester::run(|tester| {
                tester.apply_transform(as_folder(TransformVisitor), "test.tsx", TSX_SYNTAX, $src)
            });
        }
    };
}

test!(tag_native_div, r#"<div></div>"#);
test!(tag_extra_component, r#"<A></A>"#);
test!(tag_extra_custom, r#"<tag-custom></tag-custom>"#);
test!(tag_extra_member, r#"<A.b></A.b>"#);

test!(attr_key_attr, r#"<div class="cls"></div>"#);
test!(attr_key_event_prefix_on, r#"<div onClick="()=>{}"></div>"#);
test!(
    attr_key_event_namespace_on,
    r#"<div on:click="()=>{}"></div>"#
);
test!(attr_key_namespace_attr, r#"<div ns:name="value"></div>"#);

test!(attr_value_lit, r#"<div class="1"></div>"#);
test!(
    attr_value_const_undefined,
    r#"<div class={undefined}></div>"#
);
test!(attr_value_const_array, r#"<div class={[1,2]}></div>"#);
test!(attr_value_const_obj, r#"<div class={{a:1}}></div>"#);
test!(attr_value_expr, r#"<div class={a}></div>"#);
test!(attr_value_element, r#"<div element=<div></div>></div>"#);
test!(
    attr_value_element_container,
    r#"<div element={<div></div>}></div>"#
);
test!(attr_value_fragment, r#"<div fragment=<></>></div>"#);
test!(
    attr_value_fragment_container,
    r#"<div fragment={<></>}></div>"#
);
test!(attr_value_empty, r#"<div class></div>"#);

test!(attr_spread, r#"<div {...spread}></div>"#);
test!(attr_boolean_attribute, r#"<div checked></div>"#);

test!(
    element_is_static,
    r#"
    <div class="cls1" style="background-color: red;">
      <div id="firstChild"></div>
    </div>
    "#
);

test!(
    fragment_with_children,
    r#"
    <>
      <div></div>
      <div></div>
    </>
    "#
);

test!(
    text_clean,
    r#"<div>
            text1
            <br/>
      text2
            </div>
    "#
);
