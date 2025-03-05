use swc_core::{
    common::{Span, SyntaxContext},
    ecma::{
        ast::Pass,
        ast::*,
        atoms::Atom,
        visit::{Fold, FoldWith, VisitMut, VisitMutWith},
    },
};

const MAX_SAFE_INTEGER: i64 = 9007199254740991;

pub struct TransformVisitor;

impl TransformVisitor {
    pub fn new() -> Self {
        TransformVisitor
    }

    fn parse_numeric_literal(value: &str) -> Option<i64> {
        if value.starts_with("0x") || value.starts_with("-0x") {
            i64::from_str_radix(value.trim_start_matches("-0x").trim_start_matches("0x"), 16).ok()
        } else if value.starts_with("0b") || value.starts_with("-0b") {
            i64::from_str_radix(value.trim_start_matches("-0b").trim_start_matches("0b"), 2).ok()
        } else if value.starts_with("0o") || value.starts_with("-0o") {
            i64::from_str_radix(value.trim_start_matches("-0o").trim_start_matches("0o"), 8).ok()
        } else {
            value.parse().ok()
        }
    }

    fn is_in_safe_integer_range(value: i64) -> bool {
        value.abs() <= MAX_SAFE_INTEGER
    }

    fn create_bigint_call(&self, span: Span, value: String) -> Expr {
        let bigint_ident = Ident::new(Atom::from("BigInt"), span, SyntaxContext::empty());

        let arg = Lit::Num(Number {
            span,
            value: if value.starts_with("0") {
                0.0
            } else {
                value.parse().unwrap_or(0.0)
            },
            raw: Some(value.into()),
        });

        Expr::Call(CallExpr {
            span,
            callee: Callee::Expr(Box::new(Expr::Ident(bigint_ident))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Lit(arg)),
            }],
            type_args: None,
            ctxt: SyntaxContext::empty(),
        })
    }
}

impl Pass for TransformVisitor {
    fn process(&mut self, program: &mut Program) {
        program.visit_mut_with(self);
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        if let Expr::Lit(Lit::BigInt(big_int)) = expr {
            let value_str = big_int
                .raw
                .as_ref()
                .map(|s| s.trim_end_matches('n').to_string())
                .unwrap_or_else(|| big_int.value.to_string());

            if let Some(value) = Self::parse_numeric_literal(&value_str) {
                if Self::is_in_safe_integer_range(value) {
                    *expr = self.create_bigint_call(big_int.span, value_str);
                }
            }
        }
    }
}

impl Fold for TransformVisitor {
    fn fold_module(&mut self, module: Module) -> Module {
        module.fold_children_with(self)
    }
}
