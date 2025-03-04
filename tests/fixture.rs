#[path = "../src/lib.rs"]
mod swc_plugin;
use swc_core::ecma::transforms::testing::test_transform;
use swc_ecma_parser::Syntax;
use swc_plugin::TransformVisitor;

fn syntax() -> Syntax {
    Syntax::Es(Default::default())
}

#[test]
fn decimal_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const decimal = 123n;"#,
        r#"const decimal = BigInt(123);"#,
    );
}

#[test]
fn hex_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const hex = 0xffn;"#,
        r#"const hex = BigInt(0xff);"#,
    );
}

#[test]
fn binary_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const binary = 0b1111n;"#,
        r#"const binary = BigInt(0b1111);"#,
    );
}

#[test]
fn octal_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const octal = 0o777n;"#,
        r#"const octal = BigInt(0o777);"#,
    );
}

#[test]
fn large_number_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const large = 9007199254740992n;"#,
        r#"const large = 9007199254740992n;"#,
    );
}

#[test]
fn expression_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const expr = 1n + 2n;"#,
        r#"const expr = BigInt(1) + BigInt(2);"#,
    );
}

#[test]
fn negative_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const negative = -42n;"#,
        r#"const negative = -BigInt(42);"#,
    );
}

#[test]
fn negative_hex_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const negativeHex = -0xffn;"#,
        r#"const negativeHex = -BigInt(0xff);"#,
    );
}

#[test]
fn zero_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const zero = 0n + (-0n);"#,
        r#"const zero = BigInt(0) + -BigInt(0);"#,
    );
}
