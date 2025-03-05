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
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }

    fn parse_numeric_literal(value: &str) -> Option<i64> {
        match value.get(..2) {
            Some("0x") => i64::from_str_radix(&value[2..], 16),
            Some("0b") => i64::from_str_radix(&value[2..], 2),
            Some("0o") => i64::from_str_radix(&value[2..], 8),
            _ => value.parse(),
        }
        .ok()
    }

    fn create_bigint_call(&self, span: Span, value: &str) -> Expr {
        let bigint_ident = Ident::new(Atom::from("BigInt"), span, SyntaxContext::empty());
        
        Expr::Call(CallExpr {
            span,
            callee: Callee::Expr(Box::new(Expr::Ident(bigint_ident))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Lit(Lit::Num(Number {
                    span,
                    value: value.parse().unwrap_or(0.0),
                    raw: Some(value.into()),
                }))),
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
            let owned_str = big_int.value.to_string();
            let value_str = big_int
                .raw
                .as_deref()
                .map(|s| s.trim_end_matches('n'))
                .unwrap_or(&owned_str);

            if let Some(value) = Self::parse_numeric_literal(value_str) {
                if value.abs() <= MAX_SAFE_INTEGER {
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
