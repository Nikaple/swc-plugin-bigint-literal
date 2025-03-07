use num_bigint::BigUint;
use num_traits::{Num, ToPrimitive};
use swc_core::{
    common::{Span, SyntaxContext},
    ecma::{
        ast::Pass,
        ast::*,
        atoms::Atom,
        visit::{Fold, FoldWith, VisitMut, VisitMutWith},
    },
};

const MAX_SAFE_INTEGER: u64 = 9007199254740991;

pub struct TransformVisitor;

impl TransformVisitor {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }

    fn parse_numeric_literal(value: &str) -> Option<BigUint> {
        match value.get(..2) {
            Some("0x") => BigUint::from_str_radix(&value[2..], 16),
            Some("0b") => BigUint::from_str_radix(&value[2..], 2),
            Some("0o") => BigUint::from_str_radix(&value[2..], 8),
            _ => BigUint::from_str_radix(value, 10),
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

    fn create_binary_expr(&self, span: Span, left: Expr, op: BinaryOp, right: Expr) -> Expr {
        Expr::Bin(BinExpr {
            span,
            op,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn decompose_large_number(&self, span: Span, mut value: BigUint) -> Expr {
        let max_safe = BigUint::from(MAX_SAFE_INTEGER);
        if value <= max_safe {
            return self.create_bigint_call(span, &value.to_string());
        }

        // 计算需要多少个 MAX_SAFE_INTEGER 的幂
        let mut coefficients = Vec::new();
        while value > BigUint::from(0u64) {
            let remainder = &value % &max_safe;
            coefficients.push(remainder.to_u64().unwrap());
            value /= &max_safe;
        }

        // 构建表达式
        let mut result = if coefficients[0] != 0 {
            self.create_bigint_call(span, &coefficients[0].to_string())
        } else {
            self.create_bigint_call(span, "0")
        };

        let base = self.create_bigint_call(span, &MAX_SAFE_INTEGER.to_string());
        let mut current_power = base.clone();

        for &coef in coefficients.iter().skip(1) {
            if coef != 0 {
                let term = if coef == 1 {
                    current_power.clone()
                } else {
                    let coef_expr = self.create_bigint_call(span, &coef.to_string());
                    self.create_binary_expr(span, coef_expr, BinaryOp::Mul, current_power.clone())
                };
                result = self.create_binary_expr(span, result, BinaryOp::Add, term);
            }
            current_power = self.create_binary_expr(span, current_power, BinaryOp::Mul, base.clone());
        }

        result
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
                *expr = self.decompose_large_number(big_int.span, value);
            }
        }
    }
}

impl Fold for TransformVisitor {
    fn fold_module(&mut self, module: Module) -> Module {
        module.fold_children_with(self)
    }
}
