use swc_core::ecma::{
    transforms::testing::test,
    visit::as_folder,
};
use swc_core::ecma::parser::{EsConfig, Syntax};

use super::*;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        bigint: true,
        ..Default::default()
    })
}

test!(
    syntax(),
    |_| as_folder(TransformVisitor),
    safe_integer_decimal,
    "const x = 123n;",
    "const x = BigInt(123);"
);

test!(
    syntax(),
    |_| as_folder(TransformVisitor),
    safe_integer_hex,
    "const x = 0xffn;",
    "const x = BigInt(0xff);"
);

test!(
    syntax(),
    |_| as_folder(TransformVisitor),
    safe_integer_binary,
    "const x = 0b1111n;",
    "const x = BigInt(0b1111);"
);

test!(
    syntax(),
    |_| as_folder(TransformVisitor),
    safe_integer_octal,
    "const x = 0o777n;",
    "const x = BigInt(0o777);"
);

test!(
    syntax(),
    |_| as_folder(TransformVisitor),
    large_integer_unchanged,
    "const x = 9007199254740992n;",
    "const x = 9007199254740992n;"
);

test!(
    syntax(),
    |_| as_folder(TransformVisitor),
    negative_safe_integer,
    "const x = -42n;",
    "const x = BigInt(-42);"
);

test!(
    syntax(),
    |_| as_folder(TransformVisitor),
    negative_hex,
    "const x = -0xffn;",
    "const x = BigInt(-0xff);"
);

test!(
    syntax(),
    |_| as_folder(TransformVisitor),
    expression_with_bigint,
    "const x = 1n + 2n;",
    "const x = BigInt(1) + BigInt(2);"
); 