#[derive(Debug)]
pub enum ParserError {
    QuotationNotClosed,
    BraceNotOpened,
    BraceNotClosed,
    GroupingSyntaxBoxError, // please write \"if\",\"while\" or \"for\" statement head
    OperationError,         // OperationError 見つからない場合
    CommentBlockNotClosed,
    InvalidTypeSyntax,
    // for developers
    UnableToConvertType, // StmtElem をExprElemに変換できないときに発生する
    UnexpectedTypeStmt,
    UnexpectedTypeExpr,
    UnexpectedTypeComma,
    InvalidFuncSyntax,
    DevError,
    Uncategorized,
}
