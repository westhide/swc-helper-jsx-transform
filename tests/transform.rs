#![feature(box_syntax)]
#![feature(box_patterns)]

use insta::assert_debug_snapshot;
use swc_core::ecma::{
    ast::{JSXElement, JSXFragment},
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
        assert_debug_snapshot!(stringify!($name), $value.transform())
    };
}

macro_rules! test {
    ($name:ident, $src:literal) => {
        #[test]
        fn $name() {
            struct TransformVisitor;

            impl VisitMut for TransformVisitor {
                fn visit_mut_jsx_element(&mut self, element: &mut JSXElement) {
                    snapshot!($name, element)
                }

                fn visit_mut_jsx_fragment(&mut self, fragment: &mut JSXFragment) {
                    snapshot!($name, fragment)
                }
            }

            Tester::run(|tester| {
                tester.apply_transform(as_folder(TransformVisitor), "test.tsx", TSX_SYNTAX, $src)
            });
        }
    };

    ($($name:ident : $src:literal),+ $(,)?) => {
        $(test!($name,$src);)+
    };

    ($($mod:ident: {
        $($name:ident : $src:literal),+ $(,)?
    }),+ $(,)?) => {
        $(
        #[allow(non_snake_case)]
        mod $mod {
            use super::*;
            $(test!($name,$src);)+
        }
        )+
    };
}

test!(

    Tag:{
         native_div: r#"<div></div>"#,
         extra_component: r#"<A></A>"#,
         extra_custom: r#"<tag-custom></tag-custom>"#,
         extra_member: r#"<A.b></A.b>"#,
    },
    AttrKey:{
        attr: r#"<div class="cls"></div>"#,
        event_prefix: r#"<div onClick="()=>{}"></div>"#,
        event_namespace: r#"<div on:click="()=>{}"></div>"#,
        namespace_attr: r#"<div ns:name="value"></div>"#,
        spread: r#"<div {...spread}></div>"#,
    },
    AttrValue:{
        lit: r#"<div class="1"></div>"#,
        const_undefined: r#"<div class={undefined}></div>"#,
        const_array: r#"<div class={[1,2]}></div>"#,
        const_obj: r#"<div class={{a:1}}></div>"#,
        expr: r#"<div class={a}></div>"#,
        element: r#"<div element=<div></div>></div>"#,
        element_container: r#"<div element={<div></div>}></div>"#,
        fragment: r#"<div fragment=<></>></div>"#,
        fragment_container: r#"<div fragment={<></>}></div>"#,
        empty: r#"<div class></div>"#,
        boolean_attribute: r#"<div checked></div>"#,
    },

    Element:{
        is_static: r#"
          <div class="cls1" style="background-color: red;">
            <div id="firstChild"></div>
          </div>
        "#,
    },
    Fragment:{
        children: r#"
          <>
            <div></div>
            <div></div>
          </>
        "#,
    },
    Text:{
        clean_text:r#"
            <div>
             text1
             <br/>
       text2
             </div>
        "#,
    },
);
