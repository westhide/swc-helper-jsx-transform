#![feature(box_syntax)]
#![feature(box_patterns)]

use swc_core::ecma::transforms::testing::test;

mod shared;

#[derive(Debug, Default)]
pub struct Helper {}

test!(
    Default::default(),
    |_| {
        use swc_core::ecma::visit::{as_folder, VisitMut};

        struct TransformVisitor;

        impl VisitMut for TransformVisitor {}

        as_folder(TransformVisitor)
    },
    import_test,
    r#"
    "#,
    r#"
    "#
);
