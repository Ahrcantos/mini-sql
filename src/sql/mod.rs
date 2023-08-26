pub mod tokenizer;
pub mod parser;

pub enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Delete(DeleteStatement),
    Update(UpdateStatement),
}

pub struct SelectStatement {
    selections: Vec<Selection>,
    table: String,
    r#where: Option<WhereExpression>,
    pagination: Option<Pagination>,
}

pub struct InsertStatement {
    pub table: String,
    pub columns: Option<Vec<String>>,
    pub values: Vec<Value>,
}

pub struct DeleteStatement {
    table: String,
    r#where: Option<WhereExpression>,
}

pub struct UpdateStatement {
    assignments: Vec<UpdateAssignment>,
    table: String,
    r#where: Option<WhereExpression>,
}

pub struct UpdateAssignment {
    field: String,
    value: Value,
}

pub struct Pagination {
    limit: Option<usize>,
    offset: Option<usize>,
}

pub enum WhereExpression {
    And(Vec<Self>),
    Or(Vec<Self>),
    Not(Box<Self>),
    Condition(WhereCondition),
}

pub struct WhereCondition {
    field: String,
    operator: Operator,
    value: Value,
}

pub enum Operator {
    GreaterThan,
    GreaterThanEquals,
    Equals,
    SmallerThanEquals,
    SmallerThan,
    Contains,
}

pub enum Value {
    String(String),
    Int(u32),
}

pub struct Selection {
    column: String,
    alias: Option<String>,
}