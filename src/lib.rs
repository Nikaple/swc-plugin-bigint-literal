use swc_core::{
    common::{Span, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::*,
        atoms::JsWord,
        visit::{VisitMut, VisitMutWith, as_folder},
    },
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

const MAX_SAFE_INTEGER: i64 = 9007199254740991; // 2^53 - 1
const MIN_SAFE_INTEGER: i64 = -9007199254740991; // -(2^53 - 1)

pub struct TransformVisitor;

impl TransformVisitor {
    fn is_in_safe_integer_range(value: i64) -> bool {
        value >= MIN_SAFE_INTEGER && value <= MAX_SAFE_INTEGER
    }

    fn parse_numeric_literal(value_str: &str) -> Option<i64> {
        if value_str.starts_with("0x") {
            // 十六进制
            i64::from_str_radix(&value_str[2..], 16).ok()
        } else if value_str.starts_with("0b") {
            // 二进制
            i64::from_str_radix(&value_str[2..], 2).ok()
        } else if value_str.starts_with("0o") {
            // 八进制
            i64::from_str_radix(&value_str[2..], 8).ok()
        } else {
            // 十进制
            value_str.parse::<i64>().ok()
        }
    }

    fn create_bigint_call(&self, span: Span, value: String) -> Expr {
        // 创建 BigInt 标识符
        let bigint_ident = Ident::new(
            JsWord::from("BigInt"),
            span,
            SyntaxContext::default(),
        );
        
        // 创建参数 - 数值字符串
        let arg = Lit::Num(Number {
            span,
            value: if value.starts_with("0") {
                // 对于非十进制数，保持原始字符串形式
                0.0 // 这个值不重要，因为我们会使用 raw
            } else {
                value.parse().unwrap_or(0.0)
            },
            raw: Some(value.into()),
        });

        // 创建函数调用
        Expr::Call(CallExpr {
            span,
            callee: Callee::Expr(Box::new(Expr::Ident(bigint_ident))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Lit(arg)),
            }],
            type_args: None,
            ctxt: SyntaxContext::default(),
        })
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        if let Expr::Lit(Lit::BigInt(big_int)) = expr {
            // 获取原始值字符串，移除 'n' 后缀
            let value_str = big_int.raw.as_ref()
                .map(|s| s.trim_end_matches('n').to_string())
                .unwrap_or_else(|| big_int.value.to_string());

            // 尝试解析数值
            if let Some(value) = Self::parse_numeric_literal(&value_str) {
                // 检查是否在安全整数范围内
                if Self::is_in_safe_integer_range(value) {
                    // 转换为 BigInt() 调用，保持原始格式
                    *expr = self.create_bigint_call(
                        big_int.span,
                        value_str
                    );
                }
            }
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
} 