use memchr::{memchr, memchr_iter};
use nom::{
    branch::{alt},
    character::complete::{alpha1, multispace0},
    combinator::{map, map_opt, recognize, opt, not, peek},
    error::*,
    multi::{fold_many1, fold_many0, many0_count, separated_list0, separated_list1, many0, many1},
    sequence::{delimited, pair, preceded, tuple, terminated},
};

pub type IResult<'a, U> = nom::IResult<&'a str, U, nom::error::VerboseError<&'a str>>;

fn build_err<'a, U>(s: &'a str, msg: &'static str) -> IResult<'a, U> {
    Err(nom::Err::Error(
        VerboseError {
            errors: vec![(s, VerboseErrorKind::Context(msg))]
        }))
}

/// Returns a parser which runs `p` then consumes all whitespace
fn ws<'a, U, F>(p: F) -> impl FnMut(&'a str) -> IResult<'a, U>
    where F: FnMut(&'a str) -> IResult<'a, U>
{
    terminated(p, multispace0)
}

/// Overloaded version of nom's `char` that eats trailing whitespace
fn char<'a>(c: char) -> impl FnMut(&'a str) -> IResult<'a, char> {
    ws(nom::character::complete::char(c))
}

/// Overloaded version of nom's `tag` that eats trailing whitespace
fn tag<'a>(s: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str> {
    ws(nom::bytes::complete::tag(s))
}

/// Matches a specific keyword, which ensuring that it's not followed by
/// a letter.  This avoids cases like `generic_expression` being parsed as
/// `generic`, `_expression`.
fn kw<'a>(s: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str> {
    ws(terminated(nom::bytes::complete::tag(s),
                  not(alt((letter, digit, char('_'))))))
}

/// Returns a parser which recognizes '(' p ')' with optional whitespace
fn parens<'a, U, F>(p: F) -> impl FnMut(&'a str) -> IResult<'a, U>
    where F: FnMut(&'a str) -> IResult<'a, U>
{
    delimited(char('('), ws(p), char(')'))
}

/// Returns a parser for zero or more items p, delimited by c with whitespace
fn list0<'a, U, F>(c: char, p: F) -> impl FnMut(&'a str) -> IResult<'a, Vec<U>>
    where F: FnMut(&'a str) -> IResult<'a, U>
{
    separated_list0(char(c), ws(p))
}

/// Returns a parser for zero or more items p, delimited by c with whitespace
fn list1<'a, U, F>(c: char, p: F) -> impl FnMut(&'a str) -> IResult<'a, Vec<U>>
    where F: FnMut(&'a str) -> IResult<'a, U>
{
    separated_list1(char(c), ws(p))
}

/// Some rules are simple wrappers around other rules.  The `alias` macro
/// lets you define them without as much boilerplate, with or without a
/// separate parser function.
macro_rules! alias {
    ($a:ident $(< $lt:lifetime >)?, $b:ident) => {
        #[derive(Debug)]
        pub struct $a $(< $lt >)?(pub $b $(< $lt >)?);
        impl $(< $lt >)? $a $(< $lt >)?  {
            fn parse(s: &$( $lt )? str) -> IResult<Self> {
                map($b::parse, Self)(s)
            }
        }
    };
    ($a:ident $(< $lt:lifetime >)?, $b:ident, $parse_a:ident) => {
        alias!($a$(< $lt >)?, $b);
        fn $parse_a(s: &str) -> IResult<$a> {
            $a::parse(s)
        }
    };
}

/// `id_type` lets you easy construct a type which wraps a `&str` and uses the
/// parse rules for [`SimpleId`].  This is useful for semantic types types which
/// are equivalent to `simple_id`; using this macro means we don't have to
/// deference `a.0.0.0` to get from a `TypeRef` to a `TypeId` to a `SimpleId`
/// to the inner `&str`.
macro_rules! id_type {
    ($a:ident, $parse_a:ident) => {
        id_type!($a);
        fn $parse_a(s: &str) -> IResult<$a> {
            let (s, r) = SimpleId::parse(s)?;
            Ok((s, $a(r.0)))
        }
    };
    ($a:ident) => {
        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        pub struct $a<'a>(pub &'a str);
    }
}


/// Remove comments from an EXPRESS file and converts to lower-case.  This
/// should be run before any parsers.
pub fn strip_comments_and_lower(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        match data[i] {
            // Block comments
            b'(' if i + 1 < data.len() && data[i + 1] == b'*' => {
                for j in memchr_iter(b')', &data[i + 2..]) {
                    if data[i + j + 1] == b'*' {
                        i += j + 2;
                        break;
                    }
                }
            },
            // Single-line comments
            b'-' if i + 1 < data.len() && data[i + 1] == b'-' => {
                let newline = memchr(b'\n', &data[i + 2..]);
                i += newline.unwrap_or(0) + 2;
            },
            c => out.push(c.to_ascii_lowercase() as char)
        }
        i += 1;
    }
    out
}

/// Main entry function for the parser
pub fn parse(s: &str) -> IResult<Syntax> {
    syntax(s)
}

////////////////////////////////////////////////////////////////////////////////

// 124
fn digit(s: &str) -> IResult<char> {
    nom::character::complete::one_of("0123456789")(s)
}

// 125 digits
// skipped due to using fast_float

// 126
fn encoded_character(s: &str) -> IResult<char> {
    map(recognize(tuple((octet, octet, octet, octet))),
        |v| std::char::from_u32(u32::from_str_radix(v, 16).unwrap()).unwrap())
        (s)
}

// 127
fn hex_digit(s: &str) -> IResult<char> {
    alt((digit, nom::character::complete::one_of("abcdef")))(s)
}

// 128
fn letter(s: &str) -> IResult<char> {
    nom::character::complete::one_of("abcdefghijklmnopqrstuvwxyz")(s)
}

// 132
fn not_paren_star_quote_special(s: &str) -> IResult<char> {
    nom::character::complete::one_of("!\"#$%&+,-./:;<=>?@[\\]^_‘{|}~")(s)
}

// 134
fn not_quote(s: &str) -> IResult<char> {
    alt((not_paren_star_quote_special, letter, digit,
         nom::character::complete::one_of("()*")))(s)
}

// 136
fn octet(s: &str) -> IResult<&str> {
    recognize(pair(hex_digit, hex_digit))(s)
}

// 139
fn binary_literal(s: &str) -> IResult<usize> {
    let bits = fold_many1(alt((char('0'), char('1'))), 0,
        |acc, item| acc * 2 + item.to_digit(10).unwrap() as usize);
    preceded(char('%'), bits)(s)
}

// 140
fn encoded_string_literal(s: &str) -> IResult<String> {
    delimited(
        char('"'),
        fold_many0(encoded_character, String::new(),
            |mut s: String, c: char| { s.push(c); s }),
        char('"'))(s)
}

// 141 integer_literal = digits .
// skipped because we're using fast_float instead

// 142
fn real_literal_(s: &str) -> IResult<f64> {
    match fast_float::parse_partial::<f64, _>(s) {
        Err(_) => build_err(s, "Could not parse float"),
        Ok((x, n)) => Ok((&s[n..], x)),
    }
}
fn real_literal(s: &str) -> IResult<f64> {
    ws(real_literal_)(s)
}

// 143 simple_id = letter { letter | digit | ’_’ } .
#[derive(Debug, Eq, PartialEq)]
pub struct SimpleId<'a>(pub &'a str);
impl<'a> SimpleId<'a> {
    fn parse(s: &'a str) -> IResult<Self> {
        let r = ws(map(pair(
                letter,
                many0_count(alt((letter, digit, char('_'))))),
            |(_c, i)| SimpleId(&s[..(i + 1)])))(s)?;
        // Refuse to match language keywords
        match r.1.0 {
            "abs" | "abstract" | "acos" | "aggregate" | "alias" | "and" |
            "andor" | "array" | "as" | "asin" | "atan" | "bag" | "based_on" |
            "begin" | "binary" | "blength" | "boolean" | "by" | "case" |
            "const_e" | "constant" | "cos" | "derive" | "div" | "else" |
            "end" | "end_case" | "end_constant" | "end_entity" |
            "end_function" | "end_if" | "end_local" | "end_procedure" |
            "end_repeat" | "end_rule" | "end_schema" |
            "end_subtype_constraint escape" | "end_type" | "entity" |
            "enumeration" | "exists" | "exp" | "extensible" | "false" |
            "fixed" | "for" | "format" | "from" | "function" | "generic" |
            "generic_entity list" | "hibound" | "hiindex" | "if" | "in" |
            "integer" | "inverse" | "length" | "like" | "lobound" | "local" |
            "log" | "log10" | "log2" | "logical" | "loindex" | "mod" | "not" |
            "number" | "nvl" | "odd" | "of" | "oneof" | "optional" | "or" |
            "otherwise" | "pi" | "procedure reference schema" | "query" |
            "real" | "renamed" | "repeat" | "return" | "rolesof" | "rule" |
            "select" | "self" | "set" | "sin" | "sizeof" | "skip" | "sqrt" |
            "string" | "subtype" | "subtype_constraint" | "supertype" | "tan" |
            "then" | "to" | "total_over" | "true" | "type" | "unique" |
            "unknown" | "until" | "use" | "usedin" | "value" | "value_in" |
            "value_unique" | "var" | "where" | "while" | "with" | "xor"
              => build_err(s, "keyword"),
            _ => Ok(r)
        }
    }
}
fn simple_id(s: &str) -> IResult<SimpleId> { SimpleId::parse(s) }

// 144 simple_string_literal = \q { ( \q \q ) | not_quote | \s | \x9 | \xA | \xD } \q .
fn simple_string_literal(s: &str) -> IResult<String> {
    let f = alt((
        map(tag("''"), |_| '\''),
        not_quote,
        nom::character::complete::one_of(" \t\n\r")
    ));
    delimited(
            char('\''),
            fold_many0(f, String::new(), |mut s, c| { s.push(c); s }),
            char('\''))(s)
}

// 145-149 (remarks) are parsed beforehand

// 150-163
id_type!(AttributeRef, attribute_ref);
id_type!(ConstantRef, constant_ref);
id_type!(EntityRef, entity_ref);
id_type!(EnumerationRef, enumeration_ref);
id_type!(FunctionRef, function_ref);
id_type!(ParameterRef);
id_type!(ProcedureRef, procedure_ref);
id_type!(RuleLabelRef);
id_type!(RuleRef);
id_type!(SchemaRef, schema_ref);
id_type!(SubtypeConstraintRef);
id_type!(TypeLabelRef);
id_type!(TypeRef, type_ref);
id_type!(VariableRef);

// 164 abstract_entity_declaration = ABSTRACT .
fn abstract_entity_declaration(s: &str) -> IResult<()> {
    map(kw("abstract"), |_| ())(s)
}

// 165 abstract_supertype = ABSTRACT SUPERTYPE ’;’ .
fn abstract_supertype(s: &str) -> IResult<()> {
    map(tuple((
        kw("abstract"),
        kw("supertype"),
        char(';')
    )), |_| ())(s)
}

// 166 abstract_supertype_declaration = ABSTRACT SUPERTYPE [ subtype_constraint ] .
#[derive(Debug)]
pub struct AbstractSupertypeDeclaration<'a>(Option<SubtypeConstraint<'a>>);
fn abstract_supertype_declaration(s: &str) -> IResult<AbstractSupertypeDeclaration> {
    map(tuple((
        kw("abstract"),
        kw("supertype"),
        opt(subtype_constraint),
    )), |(_, _, a)| AbstractSupertypeDeclaration(a))(s)
}

// 167 actual_parameter_list = ’(’ parameter { ’,’ parameter } ’)’ .
#[derive(Debug)]
pub struct ActualParameterList<'a>(Vec<Parameter<'a>>);
fn actual_parameter_list(s: &str) -> IResult<ActualParameterList> {
    map(parens(list1(',', parameter)), ActualParameterList)(s)
}

// 168
#[derive(Debug)]
pub enum AddLikeOp { Add, Sub, Or, Xor }
fn add_like_op(s: &str) -> IResult<AddLikeOp> {
    use AddLikeOp::*;
    alt((
        map(char('+'),  |_| Add),
        map(char('-'),  |_| Sub),
        map(kw("or"),  |_| Or),
        map(kw("xor"), |_| Xor),
    ))(s)
}

// 169
#[derive(Debug)]
pub struct AggregateInitializer<'a>(Vec<Element<'a>>);
fn aggregate_initializer(s: &str) -> IResult<AggregateInitializer> {
    map(delimited(
            char('['),
            list0(',', element),
            char(']')),
        AggregateInitializer)(s)
}

// 170
alias!(AggregateSource<'a>, SimpleExpression, aggregate_source);

// 171 aggregate_type = AGGREGATE [ ’:’ type_label ] OF parameter_type .
#[derive(Debug)]
pub struct AggregateType<'a>(Option<TypeLabel<'a>>, Box<ParameterType<'a>>);
fn aggregate_type(s: &str) -> IResult<AggregateType> {
    map(tuple((
        kw("aggregate"),
        opt(preceded(char(':'), type_label)),
        kw("of"),
        parameter_type,
    )), |(_, t, _, p)| AggregateType(t, Box::new(p)))(s)
}

// 172
#[derive(Debug)]
pub enum AggregationTypes<'a> {
    Array(ArrayType<'a>),
    Bag(BagType<'a>),
    List(ListType<'a>),
    Set(SetType<'a>),
}
fn aggregation_types(s: &str) -> IResult<AggregationTypes> {
    use AggregationTypes::*;
    alt((
        map(array_type, Array),
        map(bag_type, Bag),
        map(list_type, List),
        map(set_type, Set),
    ))(s)
}

// 173
#[derive(Debug)]
pub struct AlgorithmHead<'a> {
    pub declaration: Vec<Declaration<'a>>,
    pub constant: Option<ConstantDecl<'a>>,
    pub local: Option<LocalDecl<'a>>,
}
fn algorithm_head(s: &str) -> IResult<AlgorithmHead> {
    map(tuple((
        many0(declaration),
        opt(constant_decl),
        opt(local_decl),
    )), |(d, c, l)| AlgorithmHead {
        declaration: d,
        constant: c,
        local: l
    })(s)
}

// 174 alias_stmt = ALIAS variable_id FOR general_ref { qualifier } ’;’ stmt { stmt }
//                  END_ALIAS ’;’ .
#[derive(Debug)]
pub struct AliasStmt<'a> {
    pub variable: VariableId<'a>,
    pub general: GeneralRef<'a>,
    pub qualifiers: Vec<Qualifier<'a>>,
    pub stmts: Vec<Stmt<'a>>,
}
fn alias_stmt(s: &str) -> IResult<AliasStmt> {
    map(tuple((
        kw("alias"),
        variable_id,
        kw("for"),
        general_ref,
        many0(qualifier),
        char(';'),
        many0(stmt),
    )), |(_, v, _, g, q, _, s)| AliasStmt {
        variable: v,
        general: g,
        qualifiers: q,
        stmts: s,
    })(s)
}

// 175
#[derive(Debug)]
pub struct ArrayType<'a> {
    pub bounds: BoundSpec<'a>,
    pub optional: bool,
    pub unique: bool,
    pub instantiable_type: Box<InstantiableType<'a>>,
}
fn array_type(s: &str) -> IResult<ArrayType> {
    map(tuple((
        kw("array"),
        bound_spec,
        kw("of"),
        opt(kw("optional")),
        opt(kw("unique")),
        instantiable_type,
    )),
    |(_, b, _, opt, uniq, t)| ArrayType {
        bounds: b,
        optional: opt.is_some(),
        unique: uniq.is_some(),
        instantiable_type: Box::new(t),
    })(s)
}

// 176 assignment_stmt = general_ref { qualifier } ’:=’ expression ’;’ .
#[derive(Debug)]
pub struct AssignmentStmt<'a> {
    pub general_ref: GeneralRef<'a>,
    pub qualifiers: Vec<Qualifier<'a>>,
    pub expression: Expression<'a>,
}
fn assignment_stmt(s: &str) -> IResult<AssignmentStmt> {
    map(tuple((
        general_ref,
        many0(qualifier),
        tag(":="),
        expression,
        char(';'),
    )), |(g, q, _, e, _)| AssignmentStmt {
        general_ref: g,
        qualifiers: q,
        expression: e,
    })(s)
}

// 177 attribute_decl = attribute_id | redeclared_attribute .
#[derive(Debug)]
pub enum AttributeDecl<'a> {
    Id(AttributeId<'a>),
    Redeclared(RedeclaredAttribute<'a>),
}
fn attribute_decl(s: &str) -> IResult<AttributeDecl> {
    use AttributeDecl::*;
    alt((
        map(attribute_id, Id),
        map(redeclared_attribute, Redeclared),
    ))(s)
}

// 178
id_type!(AttributeId, attribute_id);

// 179
#[derive(Debug)]
pub struct AttributeQualifier<'a>(pub AttributeRef<'a>);
fn attribute_qualifier(s: &str) -> IResult<AttributeQualifier> {
    map(preceded(char('.'), attribute_ref), AttributeQualifier)(s)
}

// 180
#[derive(Debug)]
pub struct BagType<'a>(Option<BoundSpec<'a>>, pub Box<InstantiableType<'a>>);
fn bag_type(s: &str) -> IResult<BagType> {
    map(tuple((
            kw("bag"),
            opt(bound_spec),
            kw("of"),
            instantiable_type
        )), |(_, b, _, t)| BagType(b, Box::new(t)))
        (s)
}

// 181 binary_type = BINARY [ width_spec ] .
#[derive(Debug)]
pub struct BinaryType<'a>(Option<WidthSpec<'a>>);
fn binary_type(s: &str) -> IResult<BinaryType> {
    map(preceded(kw("binary"), opt(width_spec)), BinaryType)(s)
}

// 182 boolean_type = BOOLEAN .
fn boolean_type(s: &str) -> IResult<()> {
    map(kw("boolean"), |_| ())(s)
}

// 183
alias!(Bound1<'a>, NumericExpression, bound_1);

// 184
alias!(Bound2<'a>, NumericExpression, bound_2);

// 185
#[derive(Debug)]
pub struct BoundSpec<'a>(Bound1<'a>, pub Bound2<'a>);
fn bound_spec(s: &str) -> IResult<BoundSpec> {
    map(tuple((
        char('['),
        bound_1,
        char(':'),
        bound_2,
        char(']'),
    )), |(_, b1, _, b2, _)| BoundSpec(b1, b2))(s)
}

// 186
#[derive(Debug)]
pub enum BuiltInConstant { ConstE, Pi, Self_, Indeterminant }
fn built_in_constant(s: &str) -> IResult<BuiltInConstant> {
    use BuiltInConstant::*;
    alt((
        map(kw("const_e"), |_| ConstE),
        map(kw("pi"),      |_| Pi),
        map(kw("self"),    |_| Self_),
        map(char('?'),      |_| Indeterminant),
    ))(s)
}

// 187
#[derive(Debug)]
pub enum BuiltInFunction {
    Abs, Acos, Asin, Atan, Blength, Cos, Exists, Exp, Format, Hibound, HiIndex,
    Length, LoBound, LoIndex, Log, Log2, Log10, Nvl, Odd, RolesOf, Sin, SizeOf,
    Sqrt, Tan, Typeof, Usedin, Value, ValueIn, ValueUnique
}
fn to_built_in_function(s: &str) -> Option<BuiltInFunction> {
    use BuiltInFunction::*;
    Some(match s {
        "abs" => Abs,
        "acos" => Acos,
        "asin" => Asin,
        "atan" => Atan,
        "blength" => Blength,
        "cos" => Cos,
        "exists" => Exists,
        "exp" => Exp,
        "format" => Format,
        "hibound" => Hibound,
        "hiindex" => HiIndex,

        "length" => Length,
        "lobound" => LoBound,
        "loindex" => LoIndex,
        "log" => Log,
        "log2" => Log2,
        "log10" => Log10,
        "nvl" => Nvl,
        "odd" => Odd,
        "rolesof" => RolesOf,
        "sin" => Sin,
        "sizeof" => SizeOf,

        "sqrt" => Sqrt,
        "tan" => Tan,
        "typeof" => Typeof,
        "usedin" => Usedin,
        "value" => Value,
        "value_in" => ValueIn,
        "value_unique" => ValueUnique,
        _ => return None,
    })
}
fn built_in_function(s: &str) -> IResult<BuiltInFunction> {
    // Tokenize then match the keyword, instead of doing a huge alt(...)
    ws(map_opt(alpha1, to_built_in_function))(s)
}

// 188 built_in_procedure = INSERT | REMOVE .
#[derive(Debug)]
pub enum BuiltInProcedure { Insert, Remove }
fn built_in_procedure(s: &str) -> IResult<BuiltInProcedure> {
    use BuiltInProcedure::*;
    alt((
        map(kw("insert"), |_| Insert),
        map(kw("remove"), |_| Remove),
    ))(s)
}

// 189 case_action = case_label { ’,’ case_label } ’:’ stmt .
#[derive(Debug)]
pub struct CaseAction<'a>(Vec<CaseLabel<'a>>, Stmt<'a>);
fn case_action(s: &str) -> IResult<CaseAction> {
    map(tuple((
        list1(',', case_label),
        char(':'),
        stmt,
    )), |(a, _, b)| CaseAction(a, b))(s)
}

// 190 case_label = expression .
alias!(CaseLabel<'a>, Expression, case_label);

// 191 case_stmt = CASE selector OF { case_action } [ OTHERWISE ’:’ stmt ]
//                  END_CASE ’;’ .
#[derive(Debug)]
pub struct CaseStmt<'a> {
    pub selector: Selector<'a>,
    pub actions: Vec<CaseAction<'a>>,
    pub otherwise: Option<Box<Stmt<'a>>>,
}
fn case_stmt(s: &str) -> IResult<CaseStmt> {
    map(tuple((
        kw("case"),
        selector,
        kw("of"),
        many0(case_action),
        opt(map(tuple((
            kw("otherwise"),
            char(':'),
            stmt)), |(_, _, s)| s)),
        kw("end_case"),
        char(';'))),
        |(_, s, _, a, t, _, _)| CaseStmt {
            selector: s,
            actions: a,
            otherwise: t.map(Box::new),
        })(s)
}

// 192 compound_stmt = BEGIN stmt { stmt } END ’;’ .
#[derive(Debug)]
pub struct CompoundStmt<'a>(Vec<Stmt<'a>>);
fn compound_stmt(s: &str) -> IResult<CompoundStmt> {
    map(delimited(
            kw("begin"),
            many1(stmt),
            pair(kw("end"), char(';'))),
        CompoundStmt)(s)
}

// 193
#[derive(Debug)]
pub enum ConcreteTypes<'a> {
    Aggregation(AggregationTypes<'a>),
    Simple(SimpleTypes<'a>),
    TypeRef(TypeRef<'a>),
}
fn concrete_types(s: &str) -> IResult<ConcreteTypes> {
    use ConcreteTypes::*;
    alt((
        map(aggregation_types, Aggregation),
        map(simple_types, Simple),
        map(type_ref, TypeRef),
    ))(s)
}

// 194 constant_body = constant_id ’:’ instantiable_type ’:=’ expression ’;’
#[derive(Debug)]
pub struct ConstantBody<'a> {
    pub constant_id: ConstantId<'a>,
    pub instantiable_type: InstantiableType<'a>,
    pub expression: Expression<'a>,
}
fn constant_body(s: &str) -> IResult<ConstantBody> {
    map(tuple((
        constant_id,
        char(':'),
        instantiable_type,
        tag(":="),
        expression,
        char(';')
    )), |(a, _, t, _, e, _)| ConstantBody {
        constant_id: a,
        instantiable_type: t,
        expression: e,
    })(s)
}

// 195
#[derive(Debug)]
pub struct ConstantDecl<'a>(Vec<ConstantBody<'a>>);
fn constant_decl(s: &str) -> IResult<ConstantDecl> {
    map(tuple((
        kw("constant"),
        many1(constant_body),
        kw("end_constant"),
        char(';'),
    )), |(_, b, _, _)| ConstantDecl(b))(s)
}

// 196 constant_factor = built_in_constant | constant_ref .
#[derive(Debug)]
pub enum ConstantFactor<'a> {
    BuiltIn(BuiltInConstant),
    ConstantRef(ConstantRef<'a>),
}
fn constant_factor(s: &str) -> IResult<ConstantFactor> {
    use ConstantFactor::*;
    alt((
        map(built_in_constant, BuiltIn),
        map(constant_ref, ConstantRef),
    ))(s)
}

// 197
id_type!(ConstantId, constant_id);

// 198
#[derive(Debug)]
pub enum ConstructedTypes<'a> {
    Enumeration(EnumerationType<'a>),
    Select(SelectType<'a>),
}
fn constructed_types(s: &str) -> IResult<ConstructedTypes> {
    use ConstructedTypes::*;
    alt((
        map(enumeration_type, Enumeration),
        map(select_type, Select),
    ))(s)
}

// 199 declaration = entity_decl | function_decl | procedure_decl |
//                   subtype_constraint_decl | type_decl .
#[derive(Debug)]
pub enum Declaration<'a> {
    Entity(EntityDecl<'a>),
    Function(FunctionDecl<'a>),
    Procedure(ProcedureDecl<'a>),
    SubtypeConstraint(SubtypeConstraintDecl<'a>),
    Type(TypeDecl<'a>),
}
fn declaration(s: &str) -> IResult<Declaration> {
    use Declaration::*;
    alt((
        map(entity_decl, Entity),
        map(function_decl, Function),
        map(procedure_decl, Procedure),
        map(subtype_constraint_decl, SubtypeConstraint),
        map(type_decl, Type),
    ))(s)
}

// 200 derived_attr = attribute_decl ’:’ parameter_type ’:=’ expression ’;’ .
#[derive(Debug)]
pub struct DerivedAttr<'a>(pub AttributeDecl<'a>,
                           ParameterType<'a>,
                           Expression<'a>);
fn derived_attr(s: &str) -> IResult<DerivedAttr> {
    map(tuple((
        attribute_decl,
        char(':'),
        parameter_type,
        tag(":="),
        expression,
        char(';'),
    )), |(a, _, b, _, e, _)| DerivedAttr(a, b, e))(s)
}

// 201 derive_clause = DERIVE derived_attr { derived_attr } .
#[derive(Debug)]
pub struct DeriveClause<'a>(pub Vec<DerivedAttr<'a>>);
fn derive_clause(s: &str) -> IResult<DeriveClause> {
    map(preceded(kw("derive"), many1(derived_attr)), DeriveClause)(s)
}

// 202 domain_rule = [ rule_label_id ’:’ ] expression .
#[derive(Debug)]
pub struct DomainRule<'a> {
    pub rule_label_id: Option<RuleLabelId<'a>>,
    pub expression: Expression<'a>,
}
fn domain_rule(s: &str) -> IResult<DomainRule> {
    let (s, rule_label_id) = opt(terminated(rule_label_id, char(':')))(s)?;
    let (s, expression) = expression(s)?;
    Ok((s, DomainRule { rule_label_id, expression }))
}

// 203
#[derive(Debug)]
pub struct Element<'a>(Expression<'a>, Option<Repetition<'a>>);
fn element(s: &str) -> IResult<Element> {
    map(pair(expression, opt(preceded(char(':'), repetition))),
        |(a, b)| Element(a, b))(s)
}

// 204 entity_body = { explicit_attr } [ derive_clause ] [ inverse_clause ]
//                   [ unique_clause ] [ where_clause ] .
#[derive(Debug)]
pub struct EntityBody<'a> {
    pub explicit_attr: Vec<ExplicitAttr<'a>>,
    pub derive: Option<DeriveClause<'a>>,
    pub inverse: Option<InverseClause<'a>>,
    pub unique: Option<UniqueClause<'a>>,
    pub where_: Option<WhereClause<'a>>,
}
fn entity_body(s: &str) -> IResult<EntityBody> {
    let (s, explicit_attr) = many0(explicit_attr)(s)?;
    let (s, derive) = opt(derive_clause)(s)?;
    let (s, inverse) = opt(inverse_clause)(s)?;
    let (s, unique) = opt(unique_clause)(s)?;
    let (s, where_) = opt(where_clause)(s)?;
    Ok((s, EntityBody { explicit_attr, derive, inverse, unique, where_ }))
}

// 205 entity_constructor = entity_ref ’(’ [ expression { ’,’ expression } ] ’)’ .
#[derive(Debug)]
pub struct EntityConstructor<'a> {
    pub entity_ref: EntityRef<'a>,
    pub args: Vec<Expression<'a>>,
}
// We never parse entity_constructor directly, because it's always in parsers
// which could be ambiguous where it could be ambiguous with function_call

// 206 entity_decl = entity_head entity_body END_ENTITY ’;’ .
#[derive(Debug)]
pub struct EntityDecl<'a>(pub EntityHead<'a>, pub EntityBody<'a>);
fn entity_decl(s: &str) -> IResult<EntityDecl> {
    let (s, a) = entity_head(s)?;
    let (s, b) = entity_body(s)?;
    let (s, _) = kw("end_entity")(s)?;
    let (s, _) = char(';')(s)?;
    Ok((s, EntityDecl(a, b)))
}

// 207 entity_head = ENTITY entity_id subsuper ’;’ .
#[derive(Debug)]
pub struct EntityHead<'a>(pub EntityId<'a>, pub Subsuper<'a>);
fn entity_head(s: &str) -> IResult<EntityHead> {
    map(tuple((
        kw("entity"),
        entity_id,
        subsuper,
        char(';'),
    )), |(_, a, b, _)| EntityHead(a, b))(s)
}

// 208
id_type!(EntityId, entity_id);

// 209
#[derive(Debug)]
pub struct EnumerationExtension<'a> {
    pub type_ref: TypeRef<'a>,
    pub enumeration_items: Option<EnumerationItems<'a>>,
}
fn enumeration_extension(s: &str) -> IResult<EnumerationExtension> {
    map(preceded(
        kw("based_on"),
        pair(type_ref, opt(preceded(kw("with"), enumeration_items)))),
        |(a, b)| EnumerationExtension { type_ref: a, enumeration_items: b })(s)
}

// 210
id_type!(EnumerationId, enumeration_id);

// 211 enumeration_items = ’(’ enumeration_id { ’,’ enumeration_id } ’)’ .
#[derive(Debug)]
pub struct EnumerationItems<'a>(pub Vec<EnumerationId<'a>>);
fn enumeration_items(s: &str) -> IResult<EnumerationItems> {
    map(parens(list1(',', enumeration_id)), EnumerationItems)(s)
}

// 212 enumeration_reference = [ type_ref ’.’ ] enumeration_ref .
#[derive(Debug)]
pub struct EnumerationReference<'a>(Option<TypeRef<'a>>, EnumerationRef<'a>);
fn enumeration_reference(s: &str) -> IResult<EnumerationReference> {
    map(tuple((
        opt(terminated(type_ref, char('.'))),
        enumeration_ref
    )), |(a, b)| EnumerationReference(a, b))(s)
}

// 213
#[derive(Debug)]
pub enum EnumerationItemsOrExtension<'a> {
    Items(EnumerationItems<'a>),
    Extension(EnumerationExtension<'a>),
}
#[derive(Debug)]
pub struct EnumerationType<'a> {
    pub extensible: bool,
    pub items_or_extension: Option<EnumerationItemsOrExtension<'a>>
}
fn enumeration_type(s: &str) -> IResult<EnumerationType> {
    map(tuple((
        opt(kw("extensible")),
        kw("enumeration"),
        opt(alt((
            map(preceded(kw("of"), enumeration_items),
                EnumerationItemsOrExtension::Items),
            map(enumeration_extension,
                EnumerationItemsOrExtension::Extension))))
    )), |(e, _, p)| EnumerationType {
        extensible: e.is_some(),
        items_or_extension: p })(s)
}

// 214 escape_stmt = ESCAPE ’;’ .
fn escape_stmt(s: &str) -> IResult<()> {
    map(pair(kw("escape"), char(';')), |_| ())(s)
}

// 215 explicit_attr = attribute_decl { ’,’ attribute_decl } ’:’ [ OPTIONAL ]
//                      parameter_type ’;’ .
#[derive(Debug)]
pub struct ExplicitAttr<'a> {
    pub attributes: Vec<AttributeDecl<'a>>,
    pub optional: bool,
    pub parameter_type: ParameterType<'a>,
}
fn explicit_attr(s: &str) -> IResult<ExplicitAttr> {
    map(tuple((
        list1(',', attribute_decl),
        char(':'),
        opt(kw("optional")),
        parameter_type,
        char(';'),
    )), |(a, _, o, t, _)| ExplicitAttr {
        attributes: a,
        optional: o.is_some(),
        parameter_type: t,
    })(s)
}

// 216 expression = simple_expression [ rel_op_extended simple_expression ] .
#[derive(Debug)]
pub struct Expression<'a>(SimpleExpression<'a>, Option<(RelOpExtended, SimpleExpression<'a>)>);
impl<'a> Expression<'a> {
    fn parse(s: &'a str) -> IResult<Self> {
        let (s, a) = simple_expression(s)?;
        let (s, b) = opt(pair(rel_op_extended, simple_expression))(s)?;
        Ok((s, Self(a, b)))
    }
}
fn expression(s: &str) -> IResult<Expression> { Expression::parse(s) }

// 217 factor = simple_factor [ ’**’ simple_factor ] .
#[derive(Debug)]
pub struct Factor<'a>(pub SimpleFactor<'a>, pub Option<SimpleFactor<'a>>);
fn factor(s: &str) -> IResult<Factor> {
    map(pair(simple_factor, opt(preceded(tag("**"), simple_factor))),
        |(a, b)| Factor(a, b))(s)
}

// 218 formal_parameter = parameter_id { ’,’ parameter_id } ’:’ parameter_type .
#[derive(Debug)]
pub struct FormalParameter<'a>(Vec<ParameterId<'a>>, ParameterType<'a>);
fn formal_parameter(s: &str) -> IResult<FormalParameter> {
    map(tuple((
        list1(',', parameter_id),
        char(':'),
        parameter_type
    )), |(a, _, b)| FormalParameter(a, b))(s)
}

// 219 function_call = ( built_in_function | function_ref ) [ actual_parameter_list ] .
#[derive(Debug)]
pub enum BuiltInOrFunctionRef<'a> {
    BuiltIn(BuiltInFunction),
    Ref(FunctionRef<'a>),
}
#[derive(Debug)]
pub struct FunctionCall<'a>(BuiltInOrFunctionRef<'a>, ActualParameterList<'a>);
fn function_call(s: &str) -> IResult<FunctionCall> {
    map(pair(
            alt((map(built_in_function, BuiltInOrFunctionRef::BuiltIn),
                 map(function_ref, BuiltInOrFunctionRef::Ref))),
            actual_parameter_list),
        |(a, b)| FunctionCall(a, b))(s)
}
// 220 function_decl = function_head algorithm_head stmt { stmt } END_FUNCTION ’;’ .
#[derive(Debug)]
pub struct FunctionDecl<'a> {
    pub function_head: FunctionHead<'a>,
    pub algorithm_head: AlgorithmHead<'a>,
    pub stmts: Vec<Stmt<'a>>,
}
fn function_decl(s: &str) -> IResult<FunctionDecl> {
    map(tuple((
        function_head,
        algorithm_head,
        many1(stmt),
        kw("end_function"),
        char(';'),
    )), |(a, b, c, _, _)| FunctionDecl {
        function_head: a,
        algorithm_head: b,
        stmts: c,
    })(s)
}

// 221 function_head = FUNCTION function_id [ ’(’ formal_parameter
//                     { ’;’ formal_parameter } ’)’ ] ’:’ parameter_type ’;’ .
#[derive(Debug)]
pub struct FunctionHead<'a> {
    pub id: FunctionId<'a>,
    pub params: Option<Vec<FormalParameter<'a>>>,
    pub out: ParameterType<'a>,
}
fn function_head(s: &str) -> IResult<FunctionHead> {
    map(tuple((
        kw("function"),
        function_id,
        opt(parens(list1(';', formal_parameter))),
        char(':'),
        parameter_type,
        char(';'),
    )), |(_, i, a, _, p, _)| FunctionHead {
        id: i,
        params: a,
        out: p,
    })(s)
}

// 222
id_type!(FunctionId, function_id);

// 223 generalized_types = aggregate_type | general_aggregation_types |
//                         generic_entity_type | generic_type .
#[derive(Debug)]
pub enum GeneralizedTypes<'a> {
    Aggregate(AggregateType<'a>),
    GeneralAggregation(GeneralAggregationTypes<'a>),
    GenericEntity(GenericEntityType<'a>),
    Generic(GenericType<'a>),
}
fn generalized_types(s: &str) -> IResult<GeneralizedTypes> {
    use GeneralizedTypes::*;
    alt((
        map(aggregate_type, Aggregate),
        map(general_aggregation_types, GeneralAggregation),
        map(generic_entity_type, GenericEntity),
        map(generic_type, Generic),
    ))(s)
}

// 224 general_aggregation_types = general_array_type | general_bag_type |
//                                 general_list_type | general_set_type .
#[derive(Debug)]
pub enum GeneralAggregationTypes<'a> {
    Array(GeneralArrayType<'a>),
    Bag(GeneralBagType<'a>),
    List(GeneralListType<'a>),
    Set(GeneralSetType<'a>),
}
fn general_aggregation_types(s: &str) -> IResult<GeneralAggregationTypes> {
    use GeneralAggregationTypes::*;
    alt((
        map(general_array_type, Array),
        map(general_bag_type, Bag),
        map(general_list_type, List),
        map(general_set_type, Set),
    ))(s)
}

// 225 general_array_type = ARRAY [ bound_spec ] OF [ OPTIONAL ] [ UNIQUE ]
//                          parameter_type .
#[derive(Debug)]
pub struct GeneralArrayType<'a> {
    pub bounds: BoundSpec<'a>,
    pub optional: bool,
    pub unique: bool,
    pub parameter_type: Box<ParameterType<'a>>,
}
fn general_array_type(s: &str) -> IResult<GeneralArrayType> {
    map(tuple((
        kw("array"),
        bound_spec,
        kw("of"),
        opt(kw("optional")),
        opt(kw("unique")),
        parameter_type,
    )),
    |(_, b, _, opt, uniq, t)| GeneralArrayType {
        bounds: b,
        optional: opt.is_some(),
        unique: uniq.is_some(),
        parameter_type: Box::new(t),
    })(s)
}

// 226 general_bag_type = BAG [ bound_spec ] OF parameter_type .
#[derive(Debug)]
pub struct GeneralBagType<'a>(pub Option<BoundSpec<'a>>,
                              pub Box<ParameterType<'a>>);
fn general_bag_type(s: &str) -> IResult<GeneralBagType> {
    map(tuple((
            kw("bag"),
            opt(bound_spec),
            kw("of"),
            parameter_type
        )), |(_, b, _, t)| GeneralBagType(b, Box::new(t)))
        (s)
}

// 227 general_list_type = LIST [ bound_spec ] OF [ UNIQUE ] parameter_type .
#[derive(Debug)]
pub struct GeneralListType<'a> {
    pub bounds: Option<BoundSpec<'a>>,
    pub unique: bool,
    pub parameter_type: Box<ParameterType<'a>>,
}
fn general_list_type(s: &str) -> IResult<GeneralListType> {
    map(tuple((
        kw("list"),
        opt(bound_spec),
        kw("of"),
        opt(kw("unique")),
        parameter_type,
    )),
    |(_, b, _, uniq, t)| GeneralListType {
        bounds: b,
        unique: uniq.is_some(),
        parameter_type: Box::new(t),
    })(s)
}

// 228 general_ref = parameter_ref | variable_ref .
#[derive(Debug)]
pub enum GeneralRef<'a> {
    Parameter(ParameterRef<'a>),
    Variable(VariableRef<'a>),
    _SimpleId(SimpleId<'a>),
}
fn general_ref(s: &str) -> IResult<GeneralRef> {
    map(simple_id, GeneralRef::_SimpleId)(s)
}

// 229 general_set_type = SET [ bound_spec ] OF parameter_type .
#[derive(Debug)]
pub struct GeneralSetType<'a> {
    pub bounds: Option<BoundSpec<'a>>,
    pub parameter_type: Box<ParameterType<'a>>,
}
fn general_set_type(s: &str) -> IResult<GeneralSetType> {
    map(tuple((
        kw("set"),
        opt(bound_spec),
        kw("of"),
        parameter_type,
    )),
    |(_, b, _, t)| GeneralSetType {
        bounds: b,
        parameter_type: Box::new(t),
    })(s)
}

// 230 generic_entity_type = GENERIC_ENTITY [ ’:’ type_label ] .
#[derive(Debug)]
pub struct GenericEntityType<'a>(Option<TypeLabel<'a>>);
fn generic_entity_type(s: &str) -> IResult<GenericEntityType> {
    map(preceded(kw("generic_entity"),
                 opt(preceded(char(':'), type_label))),
        GenericEntityType)(s)
}

// 231 generic_type = GENERIC [ ’:’ type_label ] .
#[derive(Debug)]
pub struct GenericType<'a>(Option<TypeLabel<'a>>);
fn generic_type(s: &str) -> IResult<GenericType> {
    map(preceded(kw("generic"),
                 opt(preceded(char(':'), type_label))),
        GenericType)(s)
}

// 232 group_qualifier = ’\’ entity_ref .
#[derive(Debug)]
pub struct GroupQualifier<'a>(pub EntityRef<'a>);
fn group_qualifier(s: &str) -> IResult<GroupQualifier> {
    map(preceded(char('\\'), entity_ref), GroupQualifier)(s)
}

// 233 if_stmt = IF logical_expression THEN stmt { stmt } [ ELSE stmt { stmt } ]
//               END_IF ’;’ .
#[derive(Debug)]
pub struct IfStmt<'a>(LogicalExpression<'a>, Vec<Stmt<'a>>, Option<Vec<Stmt<'a>>>);
fn if_stmt(s: &str) -> IResult<IfStmt> {
    map(tuple((
        kw("if"),
        logical_expression,
        kw("then"),
        many1(stmt),
        opt(preceded(kw("else"), many1(stmt))),
        kw("end_if"),
        char(';'),
    )), |(_, cond, _, a, b, _, _)| IfStmt(cond, a, b))(s)
}

// 234
alias!(Increment<'a>, NumericExpression, increment);

// 235 increment_control = variable_id ’:=’ bound_1 TO bound_2 [ BY increment ] .
#[derive(Debug)]
pub struct IncrementControl<'a> {
    pub var: VariableId<'a>,
    pub bound1: Bound1<'a>,
    pub bound2: Bound2<'a>,
    pub increment: Option<Increment<'a>>,
}
fn increment_control(s: &str) -> IResult<IncrementControl> {
    map(tuple((
        variable_id,
        tag(":="),
        bound_1,
        kw("to"),
        bound_2,
        opt(preceded(kw("by"), increment)),
    )), |(v, _, b1, _, b2, i)| IncrementControl {
        var: v,
        bound1: b1,
        bound2: b2,
        increment: i,
    })(s)
}

// 236
alias!(Index<'a>, NumericExpression);

// 237
alias!(Index1<'a>, Index, index_1);

// 238
alias!(Index2<'a>, Index, index_2);

// 239 index_qualifier = ’[’ index_1 [ ’:’ index_2 ] ’]’ .
#[derive(Debug)]
pub struct IndexQualifier<'a>(Index1<'a>, Option<Index2<'a>>);
fn index_qualifier(s: &str) -> IResult<IndexQualifier> {
    let (s, _) = char('[')(s)?;
    let (s, index1) = index_1(s)?;
    let (s, index2) = opt(preceded(char(';'), index_2))(s)?;
    let (s, _) = char(']')(s)?;
    Ok((s, IndexQualifier(index1, index2)))
}

// 240
#[derive(Debug)]
pub enum InstantiableType<'a> {
    Concrete(ConcreteTypes<'a>),
    EntityRef(EntityRef<'a>),
}
fn instantiable_type(s: &str) -> IResult<InstantiableType> {
    use InstantiableType::*;
    alt((
        map(concrete_types, Concrete),
        map(entity_ref, EntityRef),
    ))(s)
}

// 241 integer_type = INTEGER .
fn integer_type(s: &str) -> IResult<()> {
    map(kw("integer"), |_| ())(s)
}

// 242 interface_specification = reference_clause | use_clause .
#[derive(Debug)]
pub enum InterfaceSpecification<'a> {
    ReferenceClause(ReferenceClause<'a>),
    UseClause(UseClause<'a>),
}
fn interface_specification(s: &str) -> IResult<InterfaceSpecification> {
    use InterfaceSpecification::*;
    alt((map(reference_clause, ReferenceClause),
         map(use_clause, UseClause)))(s)
}

// 243
#[derive(Debug)]
pub struct Interval<'a> {
    pub low: IntervalLow<'a>,
    pub op1: IntervalOp,
    pub item: IntervalItem<'a>,
    pub op2: IntervalOp,
    pub high: IntervalHigh<'a>,
}
fn interval(s: &str) -> IResult<Interval> {
    map(delimited(
        char('{'),
        tuple((
            interval_low,
            interval_op,
            interval_item,
            interval_op,
            interval_high,
        )),
        char('}')),
        |(low, op1, item, op2, high)| Interval { low, op1, item, op2, high })
    (s)
}

// 244
alias!(IntervalHigh<'a>, SimpleExpression, interval_high);

// 245
alias!(IntervalItem<'a>, SimpleExpression, interval_item);

// 246
alias!(IntervalLow<'a>, SimpleExpression, interval_low);

// 247
#[derive(Debug)]
pub enum IntervalOp { LessThan, LessThanOrEqual }
fn interval_op(s: &str) -> IResult<IntervalOp> {
    alt((
        // Sort by length to pick the best match
        map(tag("<="), |_| IntervalOp::LessThanOrEqual),
        map(char('<'), |_| IntervalOp::LessThan),
    ))(s)
}

// 248 inverse_attr = attribute_decl ’:’ [ ( SET | BAG ) [ bound_spec ] OF ] entity_ref
//                    FOR [ entity_ref ’.’ ] attribute_ref ’;’ .
#[derive(Debug)]
pub enum SetOrBag { Set, Bag }
#[derive(Debug)]
pub struct InverseAttr<'a> {
    pub attribute_decl: AttributeDecl<'a>,
    pub bounds: Option<(SetOrBag, Option<BoundSpec<'a>>)>,
    pub entity: EntityRef<'a>,
    pub entity_for: Option<EntityRef<'a>>,
    pub attribute_ref: AttributeRef<'a>,
}
fn inverse_attr(s: &str) -> IResult<InverseAttr> {
    map(tuple((
        attribute_decl,
        char(':'),
        opt(map(tuple((
            alt((map(kw("set"), |_| SetOrBag::Set),
                 map(kw("bag"), |_| SetOrBag::Bag))),
            opt(bound_spec),
            kw("of"),
        )), |(t, b, _)| (t, b))),
        entity_ref,
        kw("for"),
        opt(terminated(entity_ref, char('.'))),
        attribute_ref,
        char(';'),
    )), |(a, _, b, c, _, d, e, _)| InverseAttr {
        attribute_decl: a,
        bounds: b,
        entity: c,
        entity_for: d,
        attribute_ref: e,
    })(s)
}

// 249 inverse_clause = INVERSE inverse_attr { inverse_attr } .
#[derive(Debug)]
pub struct InverseClause<'a>(Vec<InverseAttr<'a>>);
fn inverse_clause(s: &str) -> IResult<InverseClause> {
    map(preceded(kw("inverse"), many1(inverse_attr)), InverseClause)(s)
}

// 250
#[derive(Debug)]
pub struct ListType<'a> {
    pub bounds: Option<BoundSpec<'a>>,
    pub unique: bool,
    pub instantiable_type: Box<InstantiableType<'a>>,
}
fn list_type(s: &str) -> IResult<ListType> {
    map(tuple((
        kw("list"),
        opt(bound_spec),
        kw("of"),
        opt(kw("unique")),
        instantiable_type,
    )),
    |(_, b, _, uniq, t)| ListType {
        bounds: b,
        unique: uniq.is_some(),
        instantiable_type: Box::new(t),
    })(s)
}

// 251
#[derive(Debug)]
pub enum Literal {
    String(String),
    Binary(usize),
    Logical(LogicalLiteral),
    Real(f64),
}
fn literal(s: &str) -> IResult<Literal> {
    use Literal::*;
    alt((
        map(binary_literal, Binary),
        map(string_literal, |s| String(s.0)),
        map(logical_literal, Logical),
        map(real_literal, Real)
    ))(s)
}
// 252 local_decl = LOCAL local_variable { local_variable } END_LOCAL ’;’
#[derive(Debug)]
pub struct LocalDecl<'a>(Vec<LocalVariable<'a>>);
fn local_decl(s: &str) -> IResult<LocalDecl> {
    map(tuple((
        kw("local"),
        many1(local_variable),
        kw("end_local"),
        char(';'),
    )), |(_, vs, _, _)| LocalDecl(vs))(s)
}
// 253 local_variable = variable_id { ’,’ variable_id } ’:’ parameter_type
//                      [ ’:=’ expression ] ’;’ .
#[derive(Debug)]
pub struct LocalVariable<'a> {
    pub variable_id: Vec<VariableId<'a>>,
    pub parameter_type: ParameterType<'a>,
    pub expression: Option<Expression<'a>>,
}
fn local_variable(s: &str) -> IResult<LocalVariable> {
    map(tuple((
        list1(',', variable_id),
        char(':'),
        parameter_type,
        opt(preceded(tag(":="), expression)),
        char(';'),
    )), |(vars, _, pt, exp, _)| LocalVariable {
        variable_id: vars,
        parameter_type: pt,
        expression: exp,
    })(s)
}

// 254
alias!(LogicalExpression<'a>, Expression, logical_expression);

// 255
#[derive(Debug)]
pub enum LogicalLiteral {
    True, False, Unknown
}
fn logical_literal(s: &str) -> IResult<LogicalLiteral> {
    alt((map(kw("false"),   |_| LogicalLiteral::False),
         map(kw("true"),    |_| LogicalLiteral::True),
         map(kw("unknown"), |_| LogicalLiteral::Unknown)))(s)
}

// 256 logical_type = LOGICAL .
fn logical_type(s: &str) -> IResult<()> {
    map(kw("logical"), |_| ())(s)
}

// 257
#[derive(Debug)]
pub enum MultiplicationLikeOp {Mul, Div, IntegerDiv, Mod, And, ComplexEntity }
fn multiplication_like_op(s: &str) -> IResult<MultiplicationLikeOp> {
    use MultiplicationLikeOp::*;
    alt((
        map(char('*'),  |_| Mul),
        map(char('/'),  |_| Div),
        map(kw("div"), |_| IntegerDiv),
        map(kw("mod"), |_| Mod),
        map(kw("and"), |_| And),
        map(tag("||"),  |_| ComplexEntity),
    ))(s)
}

// 258
#[derive(Debug)]
pub enum NamedTypes<'a> {
    Entity(EntityRef<'a>),
    Type(TypeRef<'a>),
    _Ambiguous(SimpleId<'a>),
}
fn named_types(s: &str) -> IResult<NamedTypes> {
    map(simple_id, NamedTypes::_Ambiguous)(s)
}

// 259
#[derive(Debug)]
pub enum EntityOrTypeId<'a> {
    Entity(EntityId<'a>),
    Type(EntityId<'a>),
    _Ambiguous(SimpleId<'a>),
}
#[derive(Debug)]
pub struct NamedTypeOrRename<'a> {
    pub named_types: NamedTypes<'a>,
    pub rename: Option<EntityOrTypeId<'a>>,
}
fn named_type_or_rename(s: &str) -> IResult<NamedTypeOrRename> {
    map(pair(
        named_types,
        opt(preceded(kw("as"),
            map(simple_id, EntityOrTypeId::_Ambiguous)))),
        |(a, b)| NamedTypeOrRename { named_types: a, rename: b })(s)
}

// 260 null_stmt = ’;’ .
fn null_stmt(s: &str) -> IResult<()> {
    map(char(';'), |_| ())(s)
}

// 261 number_type = NUMBER .
fn number_type(s: &str) -> IResult<()> {
    map(kw("number"), |_| ())(s)
}

// 262
alias!(NumericExpression<'a>, SimpleExpression);

// 263 one_of = ONEOF ’(’ supertype_expression { ’,’ supertype_expression } ’)’
#[derive(Debug)]
pub struct OneOf<'a>(Vec<SupertypeExpression<'a>>);
fn one_of(s: &str) -> IResult<OneOf> {
    map(preceded(
        kw("oneof"),
        parens(list1(',', supertype_expression)),
    ), OneOf)(s)
}

// 264
alias!(Parameter<'a>, Expression, parameter);

// 265
id_type!(ParameterId, parameter_id);

// 266
#[derive(Debug)]
pub enum ParameterType<'a> {
    Generalized(GeneralizedTypes<'a>),
    Named(NamedTypes<'a>),
    Simple(SimpleTypes<'a>),
}
fn parameter_type(s: &str) -> IResult<ParameterType> {
    use ParameterType::*;
    alt((
        map(generalized_types, Generalized),
        map(named_types, Named),
        map(simple_types, Simple),
    ))(s)
}

// 267
#[derive(Debug)]
pub struct Population<'a>(EntityRef<'a>); // never parsed

// 268
alias!(PrecisionSpec<'a>, NumericExpression, precision_spec);

// 269 primary = literal | ( qualifiable_factor { qualifier } ) .
#[derive(Debug)]
pub enum Primary<'a> {
    Literal(Literal),
    Qualifiable(QualifiableFactor<'a>, Vec<Qualifier<'a>>),
}
fn primary(s: &str) -> IResult<Primary> {
    use Primary::*;
    alt((
        // Order so that the longest parser runs first
        map(pair(qualifiable_factor, many0(qualifier)),
            |(f, qs)| Qualifiable(f, qs)),
        map(literal, Literal),
    ))(s)
}

// 270 procedure_call_stmt = ( built_in_procedure | procedure_ref )
//                           [ actual_parameter_list ] ’;’ .
#[derive(Debug)]
pub enum BuiltInOrProcedureRef<'a> {
    BuiltIn(BuiltInProcedure),
    ProcedureRef(ProcedureRef<'a>),
}
#[derive(Debug)]
pub struct ProcedureCallStmt<'a> {
    pub proc: BuiltInOrProcedureRef<'a>,
    pub params: Option<ActualParameterList<'a>>,
}
fn procedure_call_stmt(s: &str) -> IResult<ProcedureCallStmt> {
    map(tuple((
        alt((map(built_in_procedure, BuiltInOrProcedureRef::BuiltIn),
             map(procedure_ref, BuiltInOrProcedureRef::ProcedureRef),
        )),
        opt(actual_parameter_list),
        char(';'),
    )), |(a, b, _)| ProcedureCallStmt {
        proc: a,
        params: b,
    })(s)
}
// 271 procedure_decl = procedure_head algorithm_head { stmt } END_PROCEDURE ’;’ .
#[derive(Debug)]
pub struct ProcedureDecl<'a>(ProcedureHead<'a>, AlgorithmHead<'a>, Vec<Stmt<'a>>);
fn procedure_decl(s: &str) -> IResult<ProcedureDecl> {
    map(tuple((
        procedure_head,
        algorithm_head,
        many0(stmt),
        kw("end_procedure"),
        char(';'),
    )), |(p, a, s, _, _)| ProcedureDecl(p, a, s))(s)
}

// 272 procedure_head = PROCEDURE procedure_id [ ’(’ [ VAR ] formal_parameter
//                      { ’;’ [ VAR ] formal_parameter } ’)’ ] ’;’ .
#[derive(Debug)]
pub struct ProcedureHead<'a> {
    pub procedure_id: ProcedureId<'a>,
    pub args: Option<Vec<(bool, FormalParameter<'a>)>>,
}
fn procedure_head(s: &str) -> IResult<ProcedureHead> {
    map(tuple((
        kw("procedure"),
        procedure_id,
        opt(parens(list1(';',
                map(tuple((opt(kw("var")), formal_parameter)),
                    |(v, f)| (v.is_some(), f))),
        )),
        char(';'),
    )), |(_, p, args, _)| ProcedureHead {
        procedure_id: p,
        args
    })(s)
}

// 273
id_type!(ProcedureId, procedure_id);

// 274 qualifiable_factor = attribute_ref | constant_factor | function_call |
//                          general_ref | population .
#[derive(Debug)]
pub enum QualifiableFactor<'a> {
    // Function calls should go first, since otherwise they get parsed as a
    // bare ref and leave the `(arg1, arg2, ...)` sitting on the stack
    FunctionCall(FunctionCall<'a>),

    AttributeRef(AttributeRef<'a>),
    ConstantFactor(ConstantFactor<'a>),
    GeneralRef(GeneralRef<'a>),
    Population(Population<'a>),

    // catch-all for attribute, constant, general, population
    _Ambiguous(&'a str),
}
fn qualifiable_factor(s: &str) -> IResult<QualifiableFactor> {
    alt((
        // Try parsing the function call first.  One valid parse is just a
        // function_ref, so we convert that case to _Ambiguous, since it may
        // not actually be a function.
        map(function_call, |b| if b.1.0.is_empty() {
            match b.0 {
                BuiltInOrFunctionRef::BuiltIn(_) =>
                    QualifiableFactor::FunctionCall(b),
                BuiltInOrFunctionRef::Ref(b) =>
                    QualifiableFactor::_Ambiguous(b.0),
            }
        } else {
            QualifiableFactor::FunctionCall(b)
        }),
        // Same thing for constant_factor, which could match a constant_ref
        // (which in fact matches every ref)
        map(constant_factor, |b| match b {
            ConstantFactor::BuiltIn(_) =>
                QualifiableFactor::ConstantFactor(b),
            ConstantFactor::ConstantRef(b) =>
                QualifiableFactor::_Ambiguous(b.0),
        }),
        map(simple_id, |b| QualifiableFactor::_Ambiguous(b.0)),
    ))(s)
}

// 275 qualified_attribute = SELF group_qualifier attribute_qualifier .
#[derive(Debug)]
pub struct QualifiedAttribute<'a>(pub GroupQualifier<'a>,
                                  pub AttributeQualifier<'a>);
fn qualified_attribute(s: &str) -> IResult<QualifiedAttribute> {
    map(tuple((
        kw("self"),
        group_qualifier,
        attribute_qualifier,
    )), |(_, a, b)| QualifiedAttribute(a, b))(s)
}

// 276
#[derive(Debug)]
pub enum Qualifier<'a> {
    Attribute(AttributeQualifier<'a>),
    Group(GroupQualifier<'a>),
    Index(IndexQualifier<'a>),
}
fn qualifier(s: &str) -> IResult<Qualifier> {
    use Qualifier::*;
    alt((
        map(attribute_qualifier, Attribute),
        map(group_qualifier, Group),
        map(index_qualifier, Index),
    ))(s)
}

// 277 query_expression = QUERY ’(’ variable_id ’<*’ aggregate_source ’|’
//                        logical_expression ’)’ .
#[derive(Debug)]
pub struct QueryExpression<'a> {
    pub var: VariableId<'a>,
    pub aggregate: AggregateSource<'a>,
    pub logical_expression: LogicalExpression<'a>,
}
fn query_expression(s: &str) -> IResult<QueryExpression> {
    map(tuple((
        kw("query"),
        char('('),
        variable_id,
        tag("<*"),
        aggregate_source,
        char('|'),
        logical_expression,
        char(')'),
    )), |(_, _, var, _, aggregate, _, log, _)| QueryExpression {
        var,
        aggregate,
        logical_expression: log,
    })(s)
}

// 278 real_type = REAL [ ’(’ precision_spec ’)’ ] .
#[derive(Debug)]
pub struct RealType<'a>(Option<PrecisionSpec<'a>>);
fn real_type(s: &str) -> IResult<RealType> {
    map(preceded(kw("real"),
                 opt(parens(precision_spec))),
        RealType)(s)
}

// 279 redeclared_attribute = qualified_attribute [ RENAMED attribute_id ] .
#[derive(Debug)]
pub struct RedeclaredAttribute<'a>(pub QualifiedAttribute<'a>,
                                   pub Option<AttributeId<'a>>);
fn redeclared_attribute(s: &str) -> IResult<RedeclaredAttribute> {
    map(pair(qualified_attribute,
             opt(preceded(kw("renamed"), attribute_id))),
        |(a, b)| RedeclaredAttribute(a, b))(s)
}

// 280 referenced_attribute = attribute_ref | qualified_attribute .
#[derive(Debug)]
pub enum ReferencedAttribute<'a> {
    Ref(AttributeRef<'a>),
    Qualified(QualifiedAttribute<'a>),
}
fn referenced_attribute(s: &str) -> IResult<ReferencedAttribute> {
    use ReferencedAttribute::*;
    alt((
        map(attribute_ref, Ref),
        map(qualified_attribute, Qualified),
    ))(s)
}

// 281 reference_clause = REFERENCE FROM schema_ref [ ’(’ resource_or_rename
//                        { ’,’ resource_or_rename } ’)’ ] ’;’ .
#[derive(Debug)]
pub struct ReferenceClause<'a> {
    pub schema_ref: SchemaRef<'a>,
    pub resource_or_rename: Option<Vec<ResourceOrRename<'a>>>,
}
fn reference_clause(s: &str) -> IResult<ReferenceClause> {
    map(tuple((
        kw("reference"),
        kw("front"),
        schema_ref,
        opt(parens(list1(',', resource_or_rename))),
        char(';'),
    )), |(_, _, s, r, _)| ReferenceClause {
        schema_ref: s,
        resource_or_rename: r,
    })(s)
}

// 282
#[derive(Debug)]
pub enum RelOp { LessThan, GreaterThan, LessThanOrEqual, GreaterThanOrEqual,
             NotEqual, Equal, InstanceEqual, InstanceNotEqual }
fn rel_op(s: &str) -> IResult<RelOp> {
    use RelOp::*;
    alt((
        // Sorted by length to avoid prefix issues
        map(tag(":<>:"), |_| InstanceEqual),
        map(tag(":=:"),  |_| InstanceNotEqual),
        map(tag("<="),   |_| LessThanOrEqual),
        map(tag(">="),   |_| GreaterThanOrEqual),
        map(tag("<>"),   |_| NotEqual),
        map(char('<'),   |_| LessThan),
        map(char('>'),   |_| GreaterThan),
        map(char('='),   |_| Equal),
    ))(s)
}

// 283
#[derive(Debug)]
pub enum RelOpExtended { RelOp(RelOp), In, Like }
fn rel_op_extended(s: &str) -> IResult<RelOpExtended> {
    use RelOpExtended::*;
    alt((
        map(kw("in"),   |_| In),
        map(kw("like"), |_| Like),
        map(rel_op, RelOp)))(s)
}

// 284
#[derive(Debug)]
pub enum RenameId<'a> {
    Constant(ConstantId<'a>),
    Entity(EntityId<'a>),
    Function(FunctionId<'a>),
    Procedure(ProcedureId<'a>),
    Type(TypeId<'a>),
    _Ambiguous(SimpleId<'a>),
}
fn rename_id(s: &str) -> IResult<RenameId> {
    map(simple_id, RenameId::_Ambiguous)(s)
}

// 285 repeat_control = [ increment_control ] [ while_control ] [ until_control ] .
#[derive(Debug)]
pub struct RepeatControl<'a>(
    Option<IncrementControl<'a>>,
    Option<WhileControl<'a>>,
    Option<UntilControl<'a>>);
fn repeat_control(s: &str) -> IResult<RepeatControl> {
    map(tuple((
        opt(increment_control),
        opt(while_control),
        opt(until_control),
    )), |(a, b, c)| RepeatControl(a, b, c))(s)
}

// 286 repeat_stmt = REPEAT repeat_control ’;’ stmt { stmt } END_REPEAT ’;’ .
#[derive(Debug)]
pub struct RepeatStmt<'a>(RepeatControl<'a>, Vec<Stmt<'a>>);
fn repeat_stmt(s: &str) -> IResult<RepeatStmt> {
    map(tuple((
        kw("repeat"),
        repeat_control,
        char(';'),
        many1(stmt),
        kw("end_repeat"),
        char(';'),
    )), |(_, r, _, s, _, _)| RepeatStmt(r, s))(s)
}

// 287
alias!(Repetition<'a>, NumericExpression, repetition);

// 288
#[derive(Debug)]
pub struct ResourceOrRename<'a>(ResourceRef<'a>, Option<RenameId<'a>>);
fn resource_or_rename(s: &str) -> IResult<ResourceOrRename> {
    map(pair(resource_ref, opt(preceded(kw("as"), rename_id))),
        |(a, b)| ResourceOrRename(a, b))(s)
}

// 289
#[derive(Debug)]
pub enum ResourceRef<'a> {
    Constant(ConstantRef<'a>),
    Entity(EntityRef<'a>),
    Function(FunctionRef<'a>),
    Procedure(ProcedureRef<'a>),
    Type(TypeRef<'a>),

    _Ambiguous(SimpleId<'a>),
}
fn resource_ref(s: &str) -> IResult<ResourceRef> {
    map(simple_id, ResourceRef::_Ambiguous)(s)
}

// 290 return_stmt = RETURN [ ’(’ expression ’)’ ] ’;’ .
#[derive(Debug)]
pub struct ReturnStmt<'a>(Option<Expression<'a>>);
fn return_stmt(s:  &str) -> IResult<ReturnStmt> {
    map(delimited(
        kw("return"),
        opt(parens(expression)),
        char(';')), ReturnStmt)(s)
}

// 291 rule_decl = rule_head algorithm_head { stmt } where_clause END_RULE ’;’ .
#[derive(Debug)]
pub struct RuleDecl<'a> {
    pub rule_head: RuleHead<'a>,
    pub algorithm_head: AlgorithmHead<'a>,
    pub stmt: Vec<Stmt<'a>>,
    pub where_clause: WhereClause<'a>,
}
fn rule_decl(s: &str) -> IResult<RuleDecl> {
    map(tuple((
        rule_head,
        algorithm_head,
        many0(stmt),
        where_clause,
        kw("end_rule"),
        char(';'),
    )), |(r, a, s, w, _, _)| RuleDecl {
        rule_head: r,
        algorithm_head: a,
        stmt: s,
        where_clause: w,
    })(s)
}

// 292 rule_head = RULE rule_id FOR ’(’ entity_ref { ’,’ entity_ref } ’)’ ’;’ .
#[derive(Debug)]
pub struct RuleHead<'a> {
    pub rule_id: RuleId<'a>,
    pub entities: Vec<EntityRef<'a>>,
}
fn rule_head(s: &str) -> IResult<RuleHead> {
    map(tuple((
        kw("rule"),
        rule_id,
        kw("for"),
        parens(list1(',', entity_ref)),
        char(';'),
    )), |(_, id, _, es, _)| RuleHead {
        rule_id: id,
        entities: es,
    })(s)
}

// 293
id_type!(RuleId, rule_id);

// 294
id_type!(RuleLabelId, rule_label_id);

// 295
#[derive(Debug)]
pub enum DeclarationOrRuleDecl<'a> {
    Declaration(Declaration<'a>),
    RuleDecl(RuleDecl<'a>),
}

#[derive(Debug)]
pub struct SchemaBody<'a> {
    pub interfaces: Vec<InterfaceSpecification<'a>>,
    pub constants: Option<ConstantDecl<'a>>,
    pub declarations: Vec<DeclarationOrRuleDecl<'a>>,
}
fn schema_body(s: &str) -> IResult<SchemaBody> {
    map(tuple((
        many0(interface_specification),
        opt(constant_decl),
        many0(alt((
            map(declaration, DeclarationOrRuleDecl::Declaration),
            map(rule_decl, DeclarationOrRuleDecl::RuleDecl),
        ))),
    )), |(a, b, c)| SchemaBody {
        interfaces: a,
        constants: b,
        declarations: c})(s)
}

// 296
#[derive(Debug)]
pub struct SchemaDecl<'a> {
    pub id: SchemaId<'a>,
    pub version: Option<SchemaVersionId>,
    pub body: SchemaBody<'a>,
}
fn schema_decl(s: &str) -> IResult<SchemaDecl> {
    map(tuple((
        kw("schema"),
        schema_id,
        opt(schema_version_id),
        char(';'),
        schema_body,
        kw("end_schema"),
        char(';')
    )), |(_, id, version, _, body, _, _)| SchemaDecl {
        id, version, body
    })(s)
}

// 297
id_type!(SchemaId, schema_id);

// 298
alias!(SchemaVersionId, StringLiteral, schema_version_id);

// 299 selector = expression .
alias!(Selector<'a>, Expression, selector);

// 300
#[derive(Debug)]
pub struct SelectExtension<'a> {
    pub type_ref: TypeRef<'a>,
    pub select_list: Option<SelectList<'a>>,
}
fn select_extension(s: &str) -> IResult<SelectExtension> {
    map(tuple((
        kw("based_on"), type_ref,
        opt(preceded(kw("with"), select_list))
    )), |(_, a, b)| SelectExtension {
        type_ref: a, select_list: b
    })(s)
}

// 301
#[derive(Debug)]
pub struct SelectList<'a>(pub Vec<NamedTypes<'a>>);
fn select_list(s: &str) -> IResult<SelectList> {
    map(parens(list1(',', named_types)), SelectList)(s)
}

// 302 select_type = [ EXTENSIBLE [ GENERIC_ENTITY ] ] SELECT [ select_list
//                   | select_extension ] .
#[derive(Debug)]
pub enum SelectListOrExtension<'a> {
    List(SelectList<'a>),
    Extension(SelectExtension<'a>),
}
#[derive(Debug)]
pub struct SelectType<'a> {
    pub extensible: bool,
    pub generic_entity: bool,
    pub list_or_extension: SelectListOrExtension<'a>,
}
fn select_type(s: &str) -> IResult<SelectType> {
    map(tuple((
        opt(pair(kw("extensible"), opt(kw("generic_entity")))),
        kw("select"),
        alt((
            map(select_list, SelectListOrExtension::List),
            map(select_extension, SelectListOrExtension::Extension),
        ))
    )), |(a, _, c)| SelectType{
        extensible: a.is_some(),
        generic_entity: a.is_some() && a.unwrap().1.is_some(),
        list_or_extension: c
    })(s)
}

// 303
#[derive(Debug)]
pub struct SetType<'a> {
    pub bounds: Option<BoundSpec<'a>>,
    pub instantiable_type: Box<InstantiableType<'a>>,
}
fn set_type(s: &str) -> IResult<SetType> {
    map(tuple((
        kw("set"),
        opt(bound_spec),
        kw("of"),
        instantiable_type,
    )),
    |(_, b, _, t)| SetType {
        bounds: b,
        instantiable_type: Box::new(t),
    })(s)
}

// 304 sign = ’+’ | ’-’ .
// not implemented because we're parsing floats using a separate library

// 305 simple_expression = term { add_like_op term } .
#[derive(Debug)]
pub struct SimpleExpression<'a>(pub Box<Term<'a>>, pub Vec<(AddLikeOp, Term<'a>)>);
impl<'a> SimpleExpression<'a> {
    fn parse(s: &'a str) -> IResult<Self> {
        let (s, a) = term(s)?;
        let (s, b) = many0(pair(add_like_op, term))(s)?;
        Ok((s, SimpleExpression(Box::new(a), b)))
    }
}
fn simple_expression(s: &str) -> IResult<SimpleExpression> {
    SimpleExpression::parse(s)
}

// 306 simple_factor = aggregate_initializer | entity_constructor |
//                     enumeration_reference | interval | query_expression |
//                     ( [ unary_op ] ( ’(’ expression ’)’ | primary ) ) .
#[derive(Debug)]
pub enum ExpressionOrPrimary<'a> {
    Expression(Box<Expression<'a>>),
    Primary(Primary<'a>),
}
#[derive(Debug)]
pub enum SimpleFactor<'a> {
    // Both EntityConstructor and primary -> qualifiable_factor -> function_call
    // can match things of the form function_ref(expression, expression, ...),
    // so we match them with an "ambiguous" branch here

    _AmbiguousFunctionCall(SimpleId<'a>, Vec<Expression<'a>>),
    AggregateInitializer(AggregateInitializer<'a>),
    EntityConstructor(EntityConstructor<'a>),
    EnumerationReference(EnumerationReference<'a>),
    Interval(Interval<'a>),
    QueryExpression(QueryExpression<'a>),
    Unary(Option<UnaryOp>, ExpressionOrPrimary<'a>)
}

fn ambiguous_function_call(s: &str) -> IResult<SimpleFactor> {
    map(terminated(
        // simple_id already refuses to eat built-in functions
        pair(simple_id, parens(list0(',', expression))),
        // ambiguous_function_call has a special-case to avoid eating a primary
        // function call, e.g. "cross_product(axis, ref_direction).magnitude"
        not(peek(alt((char('.'), char('\\')))))),
        |(a, b)| SimpleFactor::_AmbiguousFunctionCall(a, b))(s)
}

fn simple_factor(s: &str) -> IResult<SimpleFactor> {
    use SimpleFactor::*;
    alt((
        map(aggregate_initializer, AggregateInitializer),

        // This produces _AmbiguousFunctionCall objects which can represent
        // either an `EntityConstructor` or a `FunctionCall` without a trailing
        // qualifier.
        ambiguous_function_call,

        map(interval, Interval),
        map(query_expression, QueryExpression),

        map(pair(
            opt(unary_op),
            alt((
                map(parens(expression),
                    |e| ExpressionOrPrimary::Expression(Box::new(e))),
                map(primary, ExpressionOrPrimary::Primary)
            ))), |(op, p)| Unary(op, p)),

        // At the bottom, because this will consume a single ref
        map(enumeration_reference, EnumerationReference),
    ))(s)
}

// 307 simple_types = binary_type | boolean_type | integer_type | logical_type |
//                    number_type | real_type | string_type .
#[derive(Debug)]
pub enum SimpleTypes<'a> {
    Binary(BinaryType<'a>), Boolean, Integer, Logical, Number,
    Real(RealType<'a>), String(StringType<'a>),
}
fn simple_types(s: &str) -> IResult<SimpleTypes> {
    use SimpleTypes::*;
    alt((
        map(binary_type,  Binary),
        map(boolean_type, |_| Boolean),
        map(integer_type, |_| Integer),
        map(logical_type, |_| Logical),
        map(number_type,  |_| Number),
        map(real_type, Real),
        map(string_type, String),
    ))(s)
}

// 308 skip_stmt = SKIP ’;’ .
fn skip_stmt(s: &str) -> IResult<()> {
    map(pair(kw("skip"), char(';')), |_| ())(s)
}

// 309 stmt = alias_stmt | assignment_stmt | case_stmt | compound_stmt | escape_stmt |
//            if_stmt | null_stmt | procedure_call_stmt | repeat_stmt | return_stmt |
//            skip_stmt .
#[derive(Debug)]
pub enum Stmt<'a> {
    Alias(AliasStmt<'a>),
    Assignment(AssignmentStmt<'a>),
    Case(CaseStmt<'a>),
    Compound(CompoundStmt<'a>),
    Escape,
    If(IfStmt<'a>),
    Null,
    ProcedureCall(ProcedureCallStmt<'a>),
    Repeat(RepeatStmt<'a>),
    Return(ReturnStmt<'a>),
    Skip,
}
fn stmt(s: &str) -> IResult<Stmt> {
    use Stmt::*;
    alt((
        map(alias_stmt, Alias),
        map(assignment_stmt, Assignment),
        map(case_stmt, Case),
        map(compound_stmt, Compound),
        map(escape_stmt, |_| Escape),
        map(if_stmt, If),
        map(null_stmt, |_| Null),
        map(procedure_call_stmt, ProcedureCall),
        map(repeat_stmt, Repeat),
        map(return_stmt, Return),
        map(skip_stmt, |_| Skip),
    ))(s)
}

// 310
#[derive(Debug)]
pub struct StringLiteral(String);
impl StringLiteral {
    fn parse(s: &str) -> IResult<Self> {
        map(alt((simple_string_literal, encoded_string_literal)), Self)(s)
    }
}
fn string_literal(s: &str) -> IResult<StringLiteral> { StringLiteral::parse(s) }

// 311 string_type = STRING [ width_spec ] .
#[derive(Debug)]
pub struct StringType<'a>(Option<WidthSpec<'a>>);
fn string_type(s: &str) -> IResult<StringType> {
    map(preceded(kw("string"), opt(width_spec)), StringType)(s)
}

// 312 subsuper = [ supertype_constraint ] [ subtype_declaration ] .
#[derive(Debug)]
pub struct Subsuper<'a>(pub Option<SupertypeConstraint<'a>>,
                        pub Option<SubtypeDeclaration<'a>>);
fn subsuper(s: &str) -> IResult<Subsuper> {
    map(pair(opt(supertype_constraint), opt(subtype_declaration)),
        |(a, b)| Subsuper(a, b))(s)
}

// 313 subtype_constraint = OF ’(’ supertype_expression ’)’ .
#[derive(Debug)]
pub struct SubtypeConstraint<'a>(SupertypeExpression<'a>);
fn subtype_constraint(s: &str) -> IResult<SubtypeConstraint> {
    map(preceded(kw("of"), parens(supertype_expression)),
        SubtypeConstraint)(s)
}

// 314 subtype_constraint_body = [ abstract_supertype ] [ total_over ]
//                               [ supertype_expression ’;’ ] .
#[derive(Debug)]
pub struct SubtypeConstraintBody<'a> {
    pub abstract_super: bool,
    pub total_over: Option<TotalOver<'a>>,
    pub supertype: Option<SupertypeExpression<'a>>,
}
fn subtype_constraint_body(s: &str) -> IResult<SubtypeConstraintBody> {
    map(tuple((
        opt(abstract_supertype),
        opt(total_over),
        opt(terminated(supertype_expression, char(';'))),
    )), |(a, b, c)| SubtypeConstraintBody {
        abstract_super: a.is_some(),
        total_over: b,
        supertype: c
    })(s)
}

// 315 subtype_constraint_decl = subtype_constraint_head subtype_constraint_body
//                               END_SUBTYPE_CONSTRAINT ’;’ .
#[derive(Debug)]
pub struct SubtypeConstraintDecl<'a>(SubtypeConstraintHead<'a>,
                                     SubtypeConstraintBody<'a>);
fn subtype_constraint_decl(s: &str) -> IResult<SubtypeConstraintDecl> {
    map(tuple((
        subtype_constraint_head,
        subtype_constraint_body,
        kw("end_subtype_constraint"),
        char(';')
    )), |(a, b, _, _)| SubtypeConstraintDecl(a, b))(s)
}

// 316 subtype_constraint_head = SUBTYPE_CONSTRAINT subtype_constraint_id FOR
//                               entity_ref ’;’ .
#[derive(Debug)]
pub struct SubtypeConstraintHead<'a>(SubtypeConstraintId<'a>, EntityRef<'a>);
fn subtype_constraint_head(s: &str) -> IResult<SubtypeConstraintHead> {
    map(tuple((
        kw("subtype_constraint"),
        subtype_constraint_id,
        kw("for"),
        entity_ref,
        char(';')
    )), |(_, a, _, b, _)| SubtypeConstraintHead(a, b))(s)
}

// 317
id_type!(SubtypeConstraintId, subtype_constraint_id);

// 318 subtype_declaration = SUBTYPE OF ’(’ entity_ref { ’,’ entity_ref } ’)’ .
#[derive(Debug)]
pub struct SubtypeDeclaration<'a>(pub Vec<EntityRef<'a>>);
fn subtype_declaration(s: &str) -> IResult<SubtypeDeclaration> {
    map(preceded(tuple((kw("subtype"), kw("of"))),
                 parens(list1(',', entity_ref))),
        SubtypeDeclaration)(s)
}

// 319 supertype_constraint = abstract_entity_declaration |
//                            abstract_supertype_declaration | supertype_rule .
#[derive(Debug)]
pub enum SupertypeConstraint<'a> {
    AbstractEntity,
    AbstractSupertype(AbstractSupertypeDeclaration<'a>),
    SupertypeRule(SupertypeRule<'a>)
}
fn supertype_constraint(s: &str) -> IResult<SupertypeConstraint> {
    use SupertypeConstraint::*;
    alt((
        // Ordered so that "abstract supertype" is parsed before "abstract"
        map(abstract_supertype_declaration, AbstractSupertype),
        map(abstract_entity_declaration, |_| AbstractEntity),
        map(supertype_rule, SupertypeRule),
    ))(s)
}

// 320 supertype_expression = supertype_factor { ANDOR supertype_factor } .
#[derive(Debug)]
pub struct SupertypeExpression<'a>(SupertypeFactor<'a>,
                               Vec<SupertypeFactor<'a>>);
fn supertype_expression(s: &str) -> IResult<SupertypeExpression> {
    let (s, a) = supertype_factor(s)?;
    let (s, b) = many0(preceded(kw("andor"), supertype_factor))(s)?;
    Ok((s, SupertypeExpression(a, b)))
}

// 321 supertype_factor = supertype_term { AND supertype_term } .
#[derive(Debug)]
pub struct SupertypeFactor<'a>(Vec<SupertypeTerm<'a>>);
fn supertype_factor(s: &str) -> IResult<SupertypeFactor> {
    map(separated_list1(kw("and"), supertype_term),
        SupertypeFactor)(s)
}

// 322 supertype_rule = SUPERTYPE subtype_constraint .
#[derive(Debug)]
pub struct SupertypeRule<'a>(SubtypeConstraint<'a>);
fn supertype_rule(s: &str) -> IResult<SupertypeRule> {
    map(preceded(kw("supertype"), subtype_constraint), SupertypeRule)(s)
}

// 323 supertype_term = entity_ref | one_of | ’(’ supertype_expression ’)’ .
#[derive(Debug)]
pub enum SupertypeTerm<'a> {
    Entity(EntityRef<'a>),
    OneOf(OneOf<'a>),
    Expression(SupertypeExpression<'a>),
}
fn supertype_term(s: &str) -> IResult<SupertypeTerm> {
    use SupertypeTerm::*;
    alt((
        map(entity_ref, Entity),
        map(one_of, OneOf),
        map(parens(supertype_expression), Expression),
    ))(s)
}

// 324 syntax = schema_decl { schema_decl } .
#[derive(Debug)]
pub struct Syntax<'a>(pub Vec<SchemaDecl<'a>>);
fn syntax(s: &str) -> IResult<Syntax> {
    preceded(multispace0, map(many1(schema_decl), Syntax))(s)
}

// 325 term = factor { multiplication_like_op factor } .
#[derive(Debug)]
pub struct Term<'a>(pub Factor<'a>, pub Vec<(MultiplicationLikeOp, Factor<'a>)>);
fn term(s: &str) -> IResult<Term> {
    map(pair(factor, many0(pair(multiplication_like_op, factor))),
        |(a, b)| Term(a, b))(s)
}

// 326 total_over = TOTAL_OVER ’(’ entity_ref { ’,’ entity_ref } ’)’ ’;’ .
#[derive(Debug)]
pub struct TotalOver<'a>(Vec<EntityRef<'a>>);
fn total_over(s: &str) -> IResult<TotalOver> {
    map(delimited(
            kw("total_over"),
            parens(list1(',', entity_ref)),
            char(';')),
        TotalOver)(s)
}

// 327 type_decl = TYPE type_id ’=’ underlying_type ’;’ [ where_clause ] END_TYPE ’;’ .
#[derive(Debug)]
pub struct TypeDecl<'a> {
    pub type_id: TypeId<'a>,
    pub underlying_type: UnderlyingType<'a>,
    pub where_clause: Option<WhereClause<'a>>,
}
fn type_decl(s: &str) -> IResult<TypeDecl> {
    map(tuple((
        kw("type"),
        type_id,
        char('='),
        underlying_type,
        char(';'),
        opt(where_clause),
        kw("end_type"),
        char(';'),
    )), |(_, t, _, u, _, w, _, _)| TypeDecl {
        type_id: t,
        underlying_type: u,
        where_clause: w,
    })(s)
}

// 328
id_type!(TypeId, type_id);

// 329 type_label = type_label_id | type_label_ref .
#[derive(Debug)]
pub enum TypeLabel<'a> {
    Id(TypeLabelId<'a>),
    Ref(TypeLabelRef<'a>),
    _Ambiguous(SimpleId<'a>),
}
fn type_label(s: &str) -> IResult<TypeLabel> {
    map(simple_id, TypeLabel::_Ambiguous)(s)
}

// 330
#[derive(Debug)]
pub struct TypeLabelId<'a>(SimpleId<'a>);

// 331
#[derive(Debug, Eq, PartialEq)]
pub enum UnaryOp { Add, Sub, Not }
fn unary_op(s: &str) -> IResult<UnaryOp> {
    use UnaryOp::*;
    alt((
        map(char('+'),  |_| Add),
        map(char('-'),  |_| Sub),
        map(kw("not"), |_| Not),
    ))(s)
}

// 332
#[derive(Debug)]
pub enum UnderlyingType<'a> {
    Concrete(ConcreteTypes<'a>),
    Constructed(ConstructedTypes<'a>),
}
fn underlying_type(s: &str) -> IResult<UnderlyingType> {
    use UnderlyingType::*;
    alt((
        // Read constructed types first, so that 'select' doesn't get
        // mis-parsed as a TypeRef
        map(constructed_types, Constructed),
        map(concrete_types, Concrete),
    ))(s)
}

// 333 unique_clause = UNIQUE unique_rule ’;’ { unique_rule ’;’ } .
#[derive(Debug)]
pub struct UniqueClause<'a>(Vec<UniqueRule<'a>>);
fn unique_clause(s: &str) -> IResult<UniqueClause> {
    map(preceded(kw("unique"), many1(terminated(unique_rule, char(';')))), UniqueClause)(s)
}

// 334 unique_rule = [ rule_label_id ’:’ ] referenced_attribute { ’,’
//                   referenced_attribute } .
#[derive(Debug)]
pub struct UniqueRule<'a> {
    pub label: Option<RuleLabelId<'a>>,
    pub attrs: Vec<ReferencedAttribute<'a>>,
}
fn unique_rule(s: &str) -> IResult<UniqueRule> {
    map(pair(opt(terminated(rule_label_id, char(':'))),
             list1(',', referenced_attribute)),
        |(a, b)| UniqueRule { label: a, attrs: b })(s)
}

// 335 until_control = UNTIL logical_expression .
#[derive(Debug)]
pub struct UntilControl<'a>(LogicalExpression<'a>);
fn until_control(s: &str) -> IResult<UntilControl> {
    map(preceded(kw("until"), logical_expression), UntilControl)(s)
}

// 336 use_clause = USE FROM schema_ref [ ’(’ named_type_or_rename
//                  { ’,’ named_type_or_rename } ’)’ ] ’;’ .
#[derive(Debug)]
pub struct UseClause<'a> {
    pub schema_ref: SchemaRef<'a>,
    pub named_type_or_rename: Option<Vec<NamedTypeOrRename<'a>>>,
}
fn use_clause(s: &str) -> IResult<UseClause> {
    map(tuple((
        kw("use"),
        kw("from"),
        schema_ref,
        opt(parens(list1(',', named_type_or_rename))),
        char(';'),
    )), |(_, _, s, r, _)| UseClause {
        schema_ref: s,
        named_type_or_rename: r,
    })(s)
}

// 337 variable_id = simple_id .
id_type!(VariableId, variable_id);

// 338 where_clause = WHERE domain_rule ’;’ { domain_rule ’;’ } .
#[derive(Debug)]
pub struct WhereClause<'a>(Vec<DomainRule<'a>>);
fn where_clause(s: &str) -> IResult<WhereClause> {
    let (s, _) = kw("where")(s)?;
    let (s, v) = many1(terminated(domain_rule, char(';')))(s)?;
    Ok((s, WhereClause(v)))
}

// 339 while_control = WHILE logical_expression .
#[derive(Debug)]
pub struct WhileControl<'a>(LogicalExpression<'a>);
fn while_control(s: &str) -> IResult<WhileControl> {
    map(preceded(kw("while"), logical_expression), WhileControl)(s)
}

// 340
alias!(Width<'a>, NumericExpression, width);

// 341 width_spec = ’(’ width ’)’ [ FIXED ] .
#[derive(Debug)]
pub struct WidthSpec<'a> { pub expression: Width<'a>, pub fixed: bool }
fn width_spec(s: &str) -> IResult<WidthSpec> {
    map(pair(parens(width), opt(kw("fixed"))),
        |(w, f)| WidthSpec { expression: w, fixed: f.is_some() })(s)
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_literal() {
        assert!(real_literal("1.E6").unwrap().1 == 1.0e6);
        assert!(real_literal("3.5e-5").unwrap().1 == 3.5e-5);
        assert!(real_literal("359.62").unwrap().1 == 359.62);
    }

    #[test]
    fn test_derived_attr() {
        let e = derived_attr(r#"users : set of founded_item_select := using_items(self, []);"#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_explicit_attr() {
        let e = explicit_attr(r#"operands :  list [2:2] of generic_expression;"#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_entity_decl() {
        let e = entity_decl(r#"entity action_assignment abstract supertype;
  assigned_action : action;
derive
  role : object_role := get_role(self);
where
  wr1 : sizeof(usedin(self, 
    'automotive_design.role_association.item_with_role')) <= 1;
end_entity;  "#).unwrap();
        assert_eq!(e.0, "");

        let e = entity_decl(r#"entity advanced_brep_shape_representation
subtype of (shape_representation);
where
  wr1 : sizeof(query(it <* self.items | not (sizeof([
    'automotive_design.manifold_solid_brep', 
    'automotive_design.faceted_brep', 'automotive_design.mapped_item', 
    'automotive_design.axis2_placement_3d'] * typeof(it)) = 1))) = 0;
end_entity; "#).unwrap();
        assert!(e.0 == "");

        let e = entity_decl(r#"entity advanced_face
subtype of (face_surface);
where
  wr1 : sizeof(['automotive_design.elementary_surface', 
    'automotive_design.b_spline_surface', 
    'automotive_design.swept_surface'] * typeof(face_geometry)) = 1;
  wr2 : sizeof(query(elp_fbnds <* query(bnds <* bounds | 
    'automotive_design.edge_loop' in typeof(bnds.bound)) | not (sizeof(
    query(oe <* elp_fbnds.bound\path.edge_list | not (
    'automotive_design.edge_curve' in typeof(oe\oriented_edge.edge_element))
    )) = 0))) = 0;
  wr3 : sizeof(query(elp_fbnds <* query(bnds <* bounds | 
    'automotive_design.edge_loop' in typeof(bnds.bound)) | not (sizeof(
    query(oe <* elp_fbnds.bound\path.edge_list | not (sizeof([
    'automotive_design.line', 'automotive_design.conic', 
    'automotive_design.polyline', 'automotive_design.surface_curve', 
    'automotive_design.b_spline_curve'] * typeof(oe.edge_element\edge_curve.
    edge_geometry)) = 1))) = 0))) = 0;
  wr4 : sizeof(query(elp_fbnds <* query(bnds <* bounds | 
    'automotive_design.edge_loop' in typeof(bnds.bound)) | not (sizeof(
    query(oe <* elp_fbnds.bound\path.edge_list | not ((
    'automotive_design.vertex_point' in typeof(oe\edge.edge_start)) and (
    'automotive_design.cartesian_point' in typeof(oe\edge.edge_start\
    vertex_point.vertex_geometry)) and ('automotive_design.vertex_point' in 
    typeof(oe\edge.edge_end)) and ('automotive_design.cartesian_point' in 
    typeof(oe\edge.edge_end\vertex_point.vertex_geometry))))) = 0))) = 0;
  wr5 : sizeof(query(elp_fbnds <* query(bnds <* bounds | 
    'automotive_design.edge_loop' in typeof(bnds.bound)) | 
    'automotive_design.oriented_path' in typeof(elp_fbnds.bound))) = 0;
  wr6 : not ('automotive_design.swept_surface' in typeof(face_geometry)) or 
    (sizeof(['automotive_design.line', 'automotive_design.conic', 
    'automotive_design.polyline', 'automotive_design.b_spline_curve'] * 
    typeof(face_geometry\swept_surface.swept_curve)) = 1);
  wr7 : sizeof(query(vlp_fbnds <* query(bnds <* bounds | 
    'automotive_design.vertex_loop' in typeof(bnds.bound)) | not ((
    'automotive_design.vertex_point' in typeof(vlp_fbnds\face_bound.bound\
    vertex_loop.loop_vertex)) and ('automotive_design.cartesian_point' in 
    typeof(vlp_fbnds\face_bound.bound\vertex_loop.loop_vertex\vertex_point.
    vertex_geometry))))) = 0;
  wr8 : sizeof(query(bnd <* bounds | not (sizeof([
    'automotive_design.edge_loop', 'automotive_design.vertex_loop'] * 
    typeof(bnd.bound)) = 1))) = 0;
  wr9 : sizeof(query(elp_fbnds <* query(bnds <* bounds | 
    'automotive_design.edge_loop' in typeof(bnds.bound)) | not (sizeof(
    query(oe <* elp_fbnds.bound\path.edge_list | (
    'automotive_design.surface_curve' in typeof(oe\oriented_edge.
    edge_element\edge_curve.edge_geometry)) and not (sizeof(query(sc_ag <* oe.
    edge_element\edge_curve.edge_geometry\surface_curve.associated_geometry | 
    not ('automotive_design.pcurve' in typeof(sc_ag)))) = 0))) = 0))) = 0;
  wr10 : (not ('automotive_design.swept_surface' in typeof(face_geometry)) 
    or not ('automotive_design.polyline' in typeof(face_geometry\
    swept_surface.swept_curve)) or (sizeof(face_geometry\swept_surface.
    swept_curve\polyline.points) >= 3)) and (sizeof(query(elp_fbnds <* query(
    bnds <* bounds | 'automotive_design.edge_loop' in typeof(bnds.bound)) | 
    not (sizeof(query(oe <* elp_fbnds.bound\path.edge_list | (
    'automotive_design.polyline' in typeof(oe\oriented_edge.edge_element\
    edge_curve.edge_geometry)) and not (sizeof(oe\oriented_edge.edge_element\
    edge_curve.edge_geometry\polyline.points) >= 3))) = 0))) = 0);
end_entity; "#).unwrap();
        assert_eq!(e.0, "");

        let e = entity_decl(r#"entity alternate_product_relationship;
  name : label;
  definition :  optional text;
  alternate : product;
  base : product;
  basis : text;
unique
  ur1 : alternate, base;
where
  wr1 : alternate :<>: base;
end_entity;  "#).unwrap();
        assert_eq!(e.0, "");

        let e = entity_decl(r#"entity annotation_fill_area
subtype of (geometric_representation_item);
  boundaries : set [1:?] of curve;
where
  wr1 : (self\geometric_representation_item.dim = 3) or (sizeof(query(curve <* 
    self.boundaries | not (('automotive_design.circle' in typeof(curve)) or 
    ('automotive_design.ellipse' in typeof(curve)) or (
    'automotive_design.b_spline_curve' in typeof(curve)) and (curve\
    b_spline_curve.closed_curve = true) or (
    'automotive_design.composite_curve' in typeof(curve)) and (curve\
    composite_curve.closed_curve = true) or ('automotive_design.polyline' in
     typeof(curve)) and (curve\polyline.points[loindex(curve\polyline.points)]
     = curve\polyline.points[hiindex(curve\polyline.points)])))) = 0);
end_entity;  "#).unwrap();
        assert_eq!(e.0, "");

        let e = entity_decl(r#"entity axis2_placement_3d
subtype of (placement);
  axis :  optional direction;
  ref_direction :  optional direction;
derive
  p :  list [3:3] of direction := build_axes(axis, ref_direction);
where
  wr1 : self\placement.location.dim = 3;
  wr2 : not exists(axis) or (axis.dim = 3);
  wr3 : not exists(ref_direction) or (ref_direction.dim = 3);
  wr4 : not exists(axis) or not exists(ref_direction) or (cross_product(axis, 
    ref_direction).magnitude > 0.0);
end_entity;  "#).unwrap();
        assert_eq!(e.0, "");

        let e = entity_decl(r#"entity b_spline_curve
supertype of (oneof (uniform_curve, b_spline_curve_with_knots, 
quasi_uniform_curve, bezier_curve) andor rational_b_spline_curve)
subtype of (bounded_curve);
  degree : integer;
  control_points_list :  list [2:?] of cartesian_point;
  curve_form : b_spline_curve_form;
  closed_curve : logical;
  self_intersect : logical;
derive
  upper_index_on_control_points : integer := sizeof(control_points_list) - 1;
  control_points :  array [0 : upper_index_on_control_points] of 
  cartesian_point := list_to_array(control_points_list, 0, 
  upper_index_on_control_points);
where
  wr1 : ('automotive_design.uniform_curve' in typeof(self)) or (
    'automotive_design.quasi_uniform_curve' in typeof(self)) or (
    'automotive_design.bezier_curve' in typeof(self)) or (
    'automotive_design.b_spline_curve_with_knots' in typeof(self));
end_entity;  "#).unwrap();
        assert_eq!(e.0, "");

        let e = entity_decl(r#"entity binary_generic_expression abstract supertype
subtype of (generic_expression);
  operands :  list [2:2] of generic_expression;
end_entity; "#).unwrap();
        assert_eq!(e.0, "");

        let e = entity_decl(r#"entity composite_hole
subtype of (compound_feature);
where
  wr1 : self\characterized_object.description in ['counterbore', 'countersunk']
    ;
  wr2 : sizeof(query(sa <* get_shape_aspects(self) | ('automotive_design.'
     + 'composite_shape_aspect' in typeof(sa)) and (sa.name = 
    'compound feature in solid') and (sizeof(query(sar <* usedin(sa, 
    'automotive_design.shape_aspect_relationship.relating_shape_aspect') | 
    'automotive_design.' + 'feature_component_relationship' in typeof(sar)))
     = 2))) = 1;
  wr3 : sizeof(query(sa <* get_shape_aspects(self) | ('automotive_design.'
     + 'composite_shape_aspect' in typeof(sa)) and (sa.name = 
    'compound feature in solid') and (sizeof(query(sar <* usedin(sa, 
    'automotive_design.shape_aspect_relationship.relating_shape_aspect') | 
    'automotive_design.' + 'feature_component_relationship' in typeof(sar)))
     = 2) and (sizeof(get_round_holes_for_composite_hole(bag_to_set(usedin(sa, 
    'automotive_design.shape_aspect_relationship.relating_shape_aspect'))))
     = 2))) = 1;
  wr4 : sizeof(query(sa <* get_shape_aspects(self) | ('automotive_design.'
     + 'composite_shape_aspect' in typeof(sa)) and (sa.name = 
    'compound feature in solid') and (sizeof(query(rh1 <* 
    get_round_holes_for_composite_hole(bag_to_set(usedin(sa, 
    'automotive_design.shape_aspect_relationship.relating_shape_aspect')))
     | sizeof(query(rh2 <* get_round_holes_for_composite_hole(bag_to_set(usedin
    (sa, 'automotive_design.shape_aspect_relationship.relating_shape_aspect'
    ))) | (rh1 :<>: rh2) and (get_diameter_for_round_hole(rh1) = 
    get_diameter_for_round_hole(rh2)))) = 0)) = 0))) = 1;
  wr5 : (self.description <> 'countersunk') or (sizeof(query(sa <* 
    get_shape_aspects(self) | ('automotive_design.' + 
    'composite_shape_aspect' in typeof(sa)) and (sa.name = 
    'compound feature in solid') and (sizeof(query(rh <* 
    get_round_holes_for_composite_hole(bag_to_set(usedin(sa, 
    'automotive_design.shape_aspect_relationship.relating_shape_aspect')))
     | sizeof(query(sa1 <* get_shape_aspects(rh) | (sa.description = 
    'change in diameter occurrence') and (sizeof(query(sar <* usedin(sa1, 
    'automotive_design.shape_aspect_relationship.related_shape_aspect') | (
    sar.description = 'taper usage') and ('automotive_design.' + 'taper' in 
    typeof(sar.relating_shape_aspect)))) = 1))) = 1)) = 1))) = 1);
end_entity;  "#).unwrap();
        assert_eq!(e.0, "");

        let e = entity_decl(r#"entity founded_item;
derive
  users : set of founded_item_select := using_items(self, []);
where
  wr1 : sizeof(users) > 0;
  wr2 : not (self in users);
end_entity;  "#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_subsuper() {
        let e = subsuper("abstract supertype;").unwrap();
        assert_eq!(e.0, ";");
    }

    #[test]
    fn test_supertype_constraint() {
        let e = supertype_constraint("abstract supertype;").unwrap();
        assert_eq!(e.0, ";");
    }

    #[test]
    fn test_octet() {
        assert_eq!(octet("00").unwrap().1, "00");
    }

    #[test]
    fn test_encoded_character() {
        assert_eq!(encoded_character("00000041").unwrap().1, 'A');
    }

    #[test]
    fn test_encoded_string_literal() {
        assert_eq!(&encoded_string_literal("\"\"").unwrap().1, "");
        assert_eq!(&encoded_string_literal("\"00000041\"").unwrap().1, "A");
        assert_eq!(&encoded_string_literal("\"0000795e00006238\"").unwrap().1, "神戸");
    }

    #[test]
    fn test_simple_string_literal() {
        assert_eq!(simple_string_literal("'omg'").unwrap().1, "omg");
        assert_eq!(simple_string_literal("'om''g'").unwrap().1, "om'g");
    }

    #[test]
    fn test_type_decl() {
        type_decl(r#"type action_item = select
  (action, action_directive, action_method, action_property,
  shape_representation, versioned_action_request);
end_type;  "#).unwrap();

        let e = type_decl(r#"type day_in_month_number = integer;
where
  wr1 : {1 <= self <= 31};
  end_type;  "#).unwrap();
        assert_eq!(e.0, "");

        let e = type_decl(r#"type non_negative_length_measure = length_measure;
where
  wr1 : self >= 0.0;
end_type;  "#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_where_clause() {
        let e = where_clause(r#"where
          wr1 : {1 <= self <= 31};"#).unwrap();
        assert_eq!(e.0, "");

        let e = where_clause(r#"where
wr1 : self >= 0.0; "#).unwrap();
        assert_eq!(e.0, "");

        let e = where_clause(r#"where
  wr1 : sizeof(query(it <* self.items | not (sizeof([
    'automotive_design.manifold_solid_brep', 
    'automotive_design.faceted_brep', 'automotive_design.mapped_item', 
    'automotive_design.axis2_placement_3d'] * typeof(it)) = 1))) = 0;"#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_domain_rule() {
        let e = domain_rule(r#"wr3 : sizeof(query(msb <* query(it <* self.items | 
    'automotive_design.manifold_solid_brep' in typeof(it)) | not (sizeof(
    query(csh <* msb_shells(msb) | not (sizeof(query(fcs <* csh\
    connected_face_set.cfs_faces | not ('automotive_design.advanced_face' in
     typeof(fcs)))) = 0))) = 0))) = 0;"#).unwrap();
        assert_eq!(e.0, ";");

        let e = domain_rule(r#"wr10 : (not ('automotive_design.swept_surface' in typeof(face_geometry)) 
    or not ('automotive_design.polyline' in typeof(face_geometry\
    swept_surface.swept_curve)) or (sizeof(face_geometry\swept_surface.
    swept_curve\polyline.points) >= 3)) and (sizeof(query(elp_fbnds <* query(
    bnds <* bounds | 'automotive_design.edge_loop' in typeof(bnds.bound)) | 
    not (sizeof(query(oe <* elp_fbnds.bound\path.edge_list | (
    'automotive_design.polyline' in typeof(oe\oriented_edge.edge_element\
    edge_curve.edge_geometry)) and not (sizeof(oe\oriented_edge.edge_element\
    edge_curve.edge_geometry\polyline.points) >= 3))) = 0))) = 0);"#).unwrap();
        assert_eq!(e.0, ";");

        let e = domain_rule(r#"wr4 : sizeof(query(elp_fbnds <* query(bnds <* bounds | 
    'automotive_design.edge_loop' in typeof(bnds.bound)) | not (sizeof(
    query(oe <* elp_fbnds.bound\path.edge_list | not ((
    'automotive_design.vertex_point' in typeof(oe\edge.edge_start)) and (
    'automotive_design.cartesian_point' in typeof(oe\edge.edge_start\
    vertex_point.vertex_geometry)) and ('automotive_design.vertex_point' in 
    typeof(oe\edge.edge_end)) and ('automotive_design.cartesian_point' in 
    typeof(oe\edge.edge_end\vertex_point.vertex_geometry))))) = 0))) = 0;"#).unwrap();
        assert_eq!(e.0, ";");

        let e = domain_rule(r#"wr1 : (self\geometric_representation_item.dim = 3) or (sizeof(query(curve <* 
    self.boundaries | not (('automotive_design.circle' in typeof(curve)) or 
    ('automotive_design.ellipse' in typeof(curve)) or (
    'automotive_design.b_spline_curve' in typeof(curve)) and (curve\
    b_spline_curve.closed_curve = true) or (
    'automotive_design.composite_curve' in typeof(curve)) and (curve\
    composite_curve.closed_curve = true) or ('automotive_design.polyline' in
     typeof(curve)) and (curve\polyline.points[loindex(curve\polyline.points)]
     = curve\polyline.points[hiindex(curve\polyline.points)])))) = 0);"#).unwrap();
        assert_eq!(e.0, ";");

        let e = domain_rule(r#"wr4 : sizeof(query(sa <* get_shape_aspects(self) | ('automotive_design.'
     + 'composite_shape_aspect' in typeof(sa)) and (sa.name = 
    'compound feature in solid') and (sizeof(query(rh1 <* 
    get_round_holes_for_composite_hole(bag_to_set(usedin(sa, 
    'automotive_design.shape_aspect_relationship.relating_shape_aspect')))
     | sizeof(query(rh2 <* get_round_holes_for_composite_hole(bag_to_set(usedin
    (sa, 'automotive_design.shape_aspect_relationship.relating_shape_aspect'
    ))) | (rh1 :<>: rh2) and (get_diameter_for_round_hole(rh1) = 
    get_diameter_for_round_hole(rh2)))) = 0)) = 0))) = 1;"#).unwrap();
        assert_eq!(e.0, ";");
    }

    #[test]
    fn test_aggregate_source() {
        let e = aggregate_source("csh\\connected_face_set.cfs_faces").unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_aggregate_initializer() {
        let e = aggregate_initializer(r#"[d2, normalise(cross_product(d1, d2))\vector.orientation, d1]"#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_query_expression() {
        let e = query_expression(r#"query(fcs <* csh\
    connected_face_set.cfs_faces | not ('automotive_design.advanced_face' in
     typeof(fcs)))"#).unwrap();
        assert_eq!(e.0, "");

        let e = query_expression(r#"query(it <* self.items | not (sizeof([
    'automotive_design.manifold_solid_brep', 
    'automotive_design.faceted_brep', 'automotive_design.mapped_item', 
    'automotive_design.axis2_placement_3d'] * typeof(it)) = 1))"#).unwrap();
        assert_eq!(e.0, "");

        let e = query_expression(r#"query(msb <* query(it <* self.items | 
    'automotive_design.manifold_solid_brep' in typeof(it)) | not (sizeof(
    query(csh <* msb_shells(msb) | not (sizeof(query(fcs <* csh\
    connected_face_set.cfs_faces | not ('automotive_design.advanced_face' in
     typeof(fcs)))) = 0))) = 0))"#).unwrap();
        assert_eq!(e.0, "");

        let e = query_expression(r#"query(elp_fbnds <* query(bnds <* bounds | 
    'automotive_design.edge_loop' in typeof(bnds.bound)) | not (sizeof(
    query(oe <* elp_fbnds.bound\path.edge_list | not ((
    'automotive_design.vertex_point' in typeof(oe\edge.edge_start)) and (
    'automotive_design.cartesian_point' in typeof(oe\edge.edge_start\
    vertex_point.vertex_geometry)) and ('automotive_design.vertex_point' in 
    typeof(oe\edge.edge_end)) and ('automotive_design.cartesian_point' in 
    typeof(oe\edge.edge_end\vertex_point.vertex_geometry))))) = 0))"#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_expression() {
        let e = expression(r#"{1 <= self <= 31}"#).unwrap();
        assert_eq!(e.0, "");

        let e = expression(r#"sizeof(query(msb <* query(it <* self.items | 
    'automotive_design.manifold_solid_brep' in typeof(it)) | not (sizeof(
    query(csh <* msb_shells(msb) | not (sizeof(query(fcs <* csh\
    connected_face_set.cfs_faces | not ('automotive_design.advanced_face' in
     typeof(fcs)))) = 0))) = 0)))"#).unwrap();
        assert_eq!(e.0, "");

        let e = expression(r#"cross_product(axis, 
    ref_direction).magnitude"#).unwrap();
        assert_eq!(e.0, "");

        let e = expression(r#"sizeof(query(sa <* get_shape_aspects(self) | ('automotive_design.'
     + 'composite_shape_aspect' in typeof(sa)) and (sa.name = 
    'compound feature in solid') and (sizeof(query(rh1 <* 
    get_round_holes_for_composite_hole(bag_to_set(usedin(sa, 
    'automotive_design.shape_aspect_relationship.relating_shape_aspect')))
     | sizeof(query(rh2 <* get_round_holes_for_composite_hole(bag_to_set(usedin
    (sa, 'automotive_design.shape_aspect_relationship.relating_shape_aspect'
    ))) | (rh1 :<>: rh2) and (get_diameter_for_round_hole(rh1) = 
    get_diameter_for_round_hole(rh2)))) = 0)) = 0))) = 1"#).unwrap();
        assert_eq!(e.0, "");

        let e = expression(r#"normalise(cross_product(d1, d2))\vector.orientation"#).unwrap();
        assert_eq!(e.0, "");

        let e = expression(r#"[d2, normalise(cross_product(d1, d2))\vector.orientation, d1]"#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_function_decl() {
        let e = function_decl(r#"function acyclic(arg1 : generic_expression; arg2 : set of generic_expression)
   : boolean;
local
  result : boolean;
end_local;
  if 'automotive_design.simple_generic_expression' in typeof(arg1) then
    return (true);
  end_if;
  if arg1 in arg2 then
    return (false);
  end_if;
  if 'automotive_design.unary_generic_expression' in typeof(arg1) then
    return (acyclic(arg1\unary_generic_expression.operand, arg2 + [arg1]));
  end_if;
  if 'automotive_design.binary_generic_expression' in typeof(arg1) then
    return (acyclic(arg1\binary_generic_expression.operands[1], arg2 + [arg1]) 
    and acyclic(arg1\binary_generic_expression.operands[2], arg2 + [arg1]));
  end_if;
  if 'automotive_design.multiple_arity_generic_expression' in typeof(arg1)
   then
    result := true;
    repeat i := 1 to sizeof(arg1\multiple_arity_generic_expression.operands);
      result := result and acyclic(arg1\multiple_arity_generic_expression.
      operands[i], arg2 + [arg1]);
    end_repeat;
    return (result);
  end_if;
end_function; "#).unwrap();
        assert_eq!(e.0, "");

        let e = function_decl(r#"function build_axes(axis : direction; ref_direction : direction) :  list [3:3]
   of direction;
local
  d1 : direction;
  d2 : direction;
end_local;
  d1 := nvl(normalise(axis), dummy_gri||direction([0.0, 0.0, 1.0]));
  d2 := first_proj_axis(d1, ref_direction);
  return ([d2, normalise(cross_product(d1, d2))\vector.orientation, d1]);
end_function;  "#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_return_stmt() {
        let e = return_stmt(r#"return ([d2, normalise(cross_product(d1, d2))\vector.orientation, d1]);"#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_if_stmt() {
        let e = if_stmt(r#"if 'automotive_design.multiple_arity_generic_expression' in typeof(arg1)
   then
    result := true;
    repeat i := 1 to sizeof(arg1\multiple_arity_generic_expression.operands);
      result := result and acyclic(arg1\multiple_arity_generic_expression.
      operands[i], arg2 + [arg1]);
    end_repeat;
    return (result);
  end_if;"#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_repeat_stmt() {
        let e = repeat_stmt(r#"repeat i := 1 to sizeof(arg1\multiple_arity_generic_expression.operands);
      result := result and acyclic(arg1\multiple_arity_generic_expression.
      operands[i], arg2 + [arg1]);
    end_repeat;"#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_function_call() {
        let e = function_call(r#"usedin
    (sa, 'automotive_design.shape_aspect_relationship.relating_shape_aspect'
    )"#).unwrap();
        assert_eq!(e.0, "");

        let e = function_call(r#"sizeof(query(fcs <* csh\
    connected_face_set.cfs_faces | not ('automotive_design.advanced_face' in
     typeof(fcs))))"#).unwrap();
        assert_eq!(e.0, "");

        let e = function_call(r#"sizeof(
    query(csh <* msb_shells(msb) | not (sizeof(query(fcs <* csh\
    connected_face_set.cfs_faces | not ('automotive_design.advanced_face' in
     typeof(fcs)))) = 0)))"#).unwrap();
        assert_eq!(e.0, "");

        let e = function_call(r#"sizeof(query(msb <* query(it <* self.items | 
    'automotive_design.manifold_solid_brep' in typeof(it)) | not (sizeof(
    query(csh <* msb_shells(msb) | not (sizeof(query(fcs <* csh\
    connected_face_set.cfs_faces | not ('automotive_design.advanced_face' in
     typeof(fcs)))) = 0))) = 0)))"#).unwrap();
        assert_eq!(e.0, "");

        let e = function_call(r#"sizeof(query(elp_fbnds <* query(bnds <* bounds | 
    'automotive_design.edge_loop' in typeof(bnds.bound)) | not (sizeof(
    query(oe <* elp_fbnds.bound\path.edge_list | not ((
    'automotive_design.vertex_point' in typeof(oe\edge.edge_start)) and (
    'automotive_design.cartesian_point' in typeof(oe\edge.edge_start\
    vertex_point.vertex_geometry)) and ('automotive_design.vertex_point' in 
    typeof(oe\edge.edge_end)) and ('automotive_design.cartesian_point' in 
    typeof(oe\edge.edge_end\vertex_point.vertex_geometry))))) = 0)))"#).unwrap();
        assert_eq!(e.0, "");

        let e = function_call(r#"using_items(self, [])"#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_actual_parameter_list() {
        let e = actual_parameter_list("(self, [])").unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_parameter() {
        let e = parameter("[]").unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_literal() {
        expression(r#"1 "#).unwrap();
    }

    #[test]
    fn test_interval() {
        interval(r#"{1 <= self <= 31}"#).unwrap();
    }

    #[test]
    fn test_interval_low() {
        let e = interval_low("1 ").unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_term() {
        let e = term("1 ").unwrap();
        assert_eq!(e.0, "");

        let e = term("csh\\connected_face_set.cfs_faces").unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_factor() {
        let e = factor("1 ").unwrap();
        assert_eq!(e.0, "");

        let e = factor("csh\\connected_face_set.cfs_faces").unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_simple_factor() {
        let e = simple_factor("1 ").unwrap();
        assert_eq!(e.0, "");

        let e = simple_factor("csh\\connected_face_set.cfs_faces").unwrap();
        assert_eq!(e.0, "");

        let e = simple_factor(r#"((
    'automotive_design.vertex_point' in typeof(oe\edge.edge_start)) and (
    'automotive_design.cartesian_point' in typeof(oe\edge.edge_start\
    vertex_point.vertex_geometry)) and ('automotive_design.vertex_point' in 
    typeof(oe\edge.edge_end)))"#).unwrap();
        assert_eq!(e.0, "");

        let e = simple_factor(r#"not ((
    'automotive_design.vertex_point' in typeof(oe\edge.edge_start)) and (
    'automotive_design.cartesian_point' in typeof(oe\edge.edge_start\
    vertex_point.vertex_geometry)) and ('automotive_design.vertex_point' in 
    typeof(oe\edge.edge_end)) and ('automotive_design.cartesian_point' in 
    typeof(oe\edge.edge_end\vertex_point.vertex_geometry)))"#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_primary() {
        let e = primary("1 ").unwrap();
        assert_eq!(e.0, "");

        let e = primary("self.items").unwrap();
        assert_eq!(e.0, "");

        let e = primary("csh\\connected_face_set.cfs_faces").unwrap();
        assert_eq!(e.0, "");

        let e = primary(r#"curve\polyline.points[loindex(curve\polyline.points)]"#).unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_interval_op() {
        let e = interval_op("<= ").unwrap();
        assert_eq!(e.0, "");
    }

    #[test]
    fn test_select_type() {
        let e = select_type(r#"select 
  (action, action_directive, action_method, action_property,
  shape_representation, versioned_action_request);"#).unwrap();
        assert_eq!(e.0, ";");
    }

    #[test]
    fn test_underlying_type() {
        let e = underlying_type(r#"select 
  (action, action_directive, action_method, action_property,
  shape_representation, versioned_action_request);"#).unwrap();
        assert_eq!(e.0, ";");
    }
    #[test]
    fn test_simple_id() {
        assert_eq!(simple_id("action").unwrap().1,
                   SimpleId("action"));
        assert_eq!(simple_id("action_directive").unwrap().1,
                   SimpleId("action_directive"));
        assert_eq!(simple_id("action_method").unwrap().1,
                   SimpleId("action_method"));
        assert_eq!(simple_id("action_property").unwrap().1,
                   SimpleId("action_property"));
    }

}
