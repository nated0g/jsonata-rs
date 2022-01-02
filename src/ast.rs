use super::position::Position;

// Object constructor, represented by tuples of (key, value)
pub type Object = Vec<(Node, Node)>;

// Sort terms, representend by expresions and a bool indicating descending/ascending
pub type SortTerms = Vec<(Node, bool)>;

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus(Box<Node>),
    ArrayConstructor(Vec<Node>),
    ObjectConstructor(Object),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Concat,
    And,
    Or,
    In,
    Map,
    Range,
    ContextBind,
    PositionalBind,
    Predicate,
    Apply,
    Bind,
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
            BinaryOp::Modulus => "%",
            BinaryOp::Equal => "=",
            BinaryOp::NotEqual => "!=",
            BinaryOp::LessThan => "<",
            BinaryOp::GreaterThan => ">",
            BinaryOp::LessThanEqual => "<=",
            BinaryOp::GreaterThanEqual => ">=",
            BinaryOp::Concat => "&",
            BinaryOp::And => "and",
            BinaryOp::Or => "or",
            BinaryOp::In => "in",
            BinaryOp::Map => ".",
            BinaryOp::Range => "..",
            BinaryOp::ContextBind => "@",
            BinaryOp::PositionalBind => "#",
            BinaryOp::Predicate => "[]",
            BinaryOp::Apply => "~>",
            BinaryOp::Bind => ":=",
        })
    }
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Empty,
    Null,
    Bool(bool),
    String(String),
    Number(f64),
    Name(String),
    Var(String),
    Unary(UnaryOp),
    Binary(BinaryOp, Box<Node>, Box<Node>),
    GroupBy(Box<Node>, Object),
    OrderBy(Box<Node>, Vec<(Node, bool)>),
    Block(Vec<Node>),
    Wildcard,
    Descendent,
    Parent,
    Function {
        proc: Box<Node>,
        args: Vec<Node>,
        is_partial: bool,
    },
    PartialArg,
    Lambda {
        args: Vec<Node>,
        body: Box<Node>,
    },
    Ternary {
        cond: Box<Node>,
        truthy: Box<Node>,
        falsy: Option<Box<Node>>,
    },
    Transform {
        pattern: Box<Node>,
        update: Box<Node>,
        delete: Option<Box<Node>>,
    },

    // Generated by AST post-processing
    Path(Vec<Node>),
    Filter(Box<Node>),
    Sort(SortTerms),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub position: Position,

    pub keep_array: bool,
    pub cons_array: bool,
    pub keep_singleton_array: bool,

    /// An optional group by expression, represented as an object.
    pub group_by: Option<(Position, Object)>,

    /// An optional list of predicates.
    pub predicates: Option<Vec<Node>>,

    /// An optional list of evaluation stages, for example this specifies the filtering and
    /// indexing for various expressions.
    pub stages: Option<Vec<Node>>,
}

impl Default for Node {
    fn default() -> Node {
        Node::new(NodeKind::Empty, Default::default())
    }
}

impl Node {
    pub(crate) fn new(kind: NodeKind, position: Position) -> Self {
        Self {
            kind,
            position,
            keep_array: false,
            cons_array: false,
            keep_singleton_array: false,
            group_by: None,
            predicates: None,
            stages: None,
        }
    }
}
