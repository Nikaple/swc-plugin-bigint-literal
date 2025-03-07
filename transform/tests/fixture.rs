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
        r#"const hex = BigInt(255);"#,
    );
}

#[test]
fn binary_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const binary = 0b1111n;"#,
        r#"const binary = BigInt(15);"#,
    );
}

#[test]
fn octal_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const octal = 0o777n;"#,
        r#"const octal = BigInt(511);"#,
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
        r#"const negativeHex = -BigInt(255);"#,
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

#[test]
fn huge_int_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const huge = 730750818665451215712927172538123444058715062272n;"#,
        r#"const huge = BigInt(1) + BigInt(9007199254740991) * BigInt(9007199254740991) * BigInt(9007199254740991);"#,
    );
}

#[test]
fn not_safe_int_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const huge = 9007199254740992n;"#,
        r#"const huge = BigInt(1) + BigInt(9007199254740991)"#,
    );
}

#[test]
fn another_huge_int_test() {
    test_transform(
        syntax(),
        Some(false),
        |_| TransformVisitor,
        r#"const huge = 13164036458569646875738116129555399058346566568785520866804115739n;"#,
        r#"const huge = BigInt(4357431240991) + BigInt(6) * (BigInt(9007199254740991) * BigInt(9007199254740991) * BigInt(9007199254740991)) + BigInt(2) * (BigInt(9007199254740991) * BigInt(9007199254740991) * BigInt(9007199254740991) * BigInt(9007199254740991));"#,
    );
}
