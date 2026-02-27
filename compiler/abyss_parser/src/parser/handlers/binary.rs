use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{BinaryOp, Expr, ExprKind},
    parser::{engine::PrattEngine, precedence::Precedence, rules::get_rule},
};

pub fn parse_binary(eng: &mut PrattEngine, left: Expr) -> Result<Expr, ()> {
    let op_tk = eng.current_token();
    let rule = get_rule(op_tk.kind);

    // :=
    if op_tk.kind == Tk::ColonEq {
        eng.advance();

        let right = eng.parse_expression_bp(Precedence::AssignmentRhs)?;

        // _
        let wildcard_expr = eng.new_expr(ExprKind::Wildcard);

        // left : _
        let key_value_expr = eng.new_expr(ExprKind::Binary(
            Box::new(left),
            BinaryOp::KeyValue,
            Box::new(wildcard_expr),
        ));

        // (left : _) = right
        return Ok(eng.new_expr(ExprKind::Binary(
            Box::new(key_value_expr),
            BinaryOp::Assign,
            Box::new(right),
        )));
    }

    let op = match op_tk.kind {
        // :
        Tk::Colon => BinaryOp::KeyValue,
        // ::
        Tk::ColonColon => BinaryOp::ConstDef,

        // +
        Tk::Plus => BinaryOp::Add,
        // -
        Tk::Minus => BinaryOp::Sub,
        // *
        Tk::Star => BinaryOp::Mul,
        // /
        Tk::Slash => BinaryOp::Div,
        // %
        Tk::Percent => BinaryOp::Mod,

        // ==
        Tk::EqEq => BinaryOp::Eq,
        // !=
        Tk::BangEq => BinaryOp::Neq,
        // <
        Tk::Lt => BinaryOp::Lt,
        // <=
        Tk::LtEq => BinaryOp::Lte,
        // >
        Tk::Gt => BinaryOp::Gt,
        // >=
        Tk::GtEq => BinaryOp::Gte,

        // &&
        Tk::And => BinaryOp::And,
        // ||
        Tk::Or => BinaryOp::Or,

        // &
        Tk::Amp => BinaryOp::BitAnd,
        // |
        Tk::Pipe => BinaryOp::Pipe,
        // ^
        Tk::Caret => BinaryOp::BitXor,
        // <<
        Tk::LeftShift => BinaryOp::Shl,
        // >>
        Tk::RightShift => BinaryOp::Shr,

        // =
        Tk::Assign => BinaryOp::Assign,
        // +=
        Tk::PlusAssign => BinaryOp::AssignAdd,
        // -=
        Tk::MinusAssign => BinaryOp::AssignSub,
        // *=
        Tk::StarAssign => BinaryOp::AssignMul,
        // /=
        Tk::SlashAssign => BinaryOp::AssignDiv,
        // %=
        Tk::PercentAssign => BinaryOp::AssignMod,
        // &=
        Tk::AmpAssign => BinaryOp::AssignBitAnd,
        // |=
        Tk::PipeAssign => BinaryOp::AssignBitOr,
        // ^=
        Tk::CaretAssign => BinaryOp::AssignBitXor,
        // <<=
        Tk::LeftShiftAssign => BinaryOp::AssignShl,
        // >>=
        Tk::RightShiftAssign => BinaryOp::AssignShr,

        // fail
        _ => {
            eng.report_error(
                op_tk.span(eng.file_id),
                format!("Unknown binary operator: {:?}", op_tk.kind),
            );
            return Err(());
        }
    };

    eng.advance();

    // right assoc rhs
    let right_precedence = if rule.precedence == Precedence::Assignment {
        Precedence::AssignmentRhs
    } else {
        rule.precedence
    };

    let right = eng.parse_expression_bp(right_precedence)?;

    Ok(eng.new_expr(ExprKind::Binary(Box::new(left), op, Box::new(right))))
}
