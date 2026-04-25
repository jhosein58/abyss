use abyss_ir::ir::{IrExpr, IrStmt};

#[derive(Clone)]
pub enum AstNode {
    Expr(IrExpr),
    Stmt(IrStmt),
}

impl AstNode {
    pub fn unwrap_expr(self) -> IrExpr {
        match self {
            AstNode::Expr(e) => e,
            AstNode::Stmt(_) => panic!("Expected expression, found statement"),
        }
    }

    pub fn unwrap_stmt(self) -> IrStmt {
        match self {
            AstNode::Stmt(s) => s,
            AstNode::Expr(e) => IrStmt::Expr(e),
        }
    }
}
