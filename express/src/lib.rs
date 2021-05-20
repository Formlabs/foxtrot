use memchr::{memchr, memchr_iter};
use nom::{
    branch::{alt},
    bytes::complete::{tag},
    character::complete::{one_of, alpha1, multispace0, digit1, char},
    error::*,
    multi::{fold_many1, fold_many0, many0_count, separated_list0, separated_list1, many0, many1},
    combinator::{map, recognize, opt},
    sequence::{delimited, pair, preceded, tuple, terminated},
};

enum Parse {
    LogicalLiteral(LogicalLiteral),
}

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

macro_rules! alias {
    // `()` indicates that the macro takes no argument.
    ($a:ident $(< $lt:lifetime >)?,
     $b:ident, $parse_a:ident) => {
        struct $a $(< $lt >)?($b $(< $lt >)?);
        impl $(< $lt >)? $a $(< $lt >)?  {
            fn parse(s: &$( $lt )? str) -> IResult<Self> {
                map($b::parse, Self)(s)
            }
        }
        fn $parse_a(s: &str) -> IResult<$a> {
            $a::parse(s)
        }
    };
}


////////////////////////////////////////////////////////////////////////////////

// 124
fn digit(s: &str) -> IResult<char> {
    one_of("0123456789")(s)
}

// 125
fn digits(s: &str) -> IResult<usize> {
    map(digit1, |v: &str| v.parse().unwrap())(s)
}

// 126
fn encoded_character(s: &str) -> IResult<char> {
    map(recognize(tuple((octet, octet, octet, octet))),
        |v| std::char::from_u32(u32::from_str_radix(v, 16).unwrap()).unwrap())
        (s)
}

// 127
fn hex_digit(s: &str) -> IResult<char> {
    alt((digit, one_of("abcdef")))(s)
}

// 128
fn letter(s: &str) -> IResult<char> {
    one_of("abcdefghijklmnopqrstuvwxyz")(s)
}

// 132
fn not_paren_star_quote_special(s: &str) -> IResult<char> {
    one_of("!\"#$%&+,-./:;<=>?@[\\]^_‘{|}~")(s)
}

// 134
fn not_quote(s: &str) -> IResult<char> {
    alt((not_paren_star_quote_special, letter, digit, one_of("()*")))(s)
}

// 136
fn octet(s: &str) -> IResult<&str> {
    recognize(pair(hex_digit, hex_digit))(s)
}

// 139
fn binary_literal(s: &str) -> IResult<usize> {
    let bits = fold_many1(alt((char('0'), char('1'))), 0,
        |mut acc, item| acc * 2 + item.to_digit(10).unwrap() as usize);
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

// 141
fn integer_literal(s: &str) -> IResult<usize> {
    digits(s)
}

// 142
fn real_literal(s: &str) -> IResult<f64> {
    match fast_float::parse_partial::<f64, _>(s) {
        Err(_) => build_err(s, "Could not parse float"),
        Ok((x, n)) => Ok((&s[n..], x)),
    }
}

// 143
struct SimpleId<'a>(&'a str);
impl<'a> SimpleId<'a> {
    fn parse(s: &'a str) -> IResult<Self> {
        map(pair(
                alpha1,
                many0_count(alt((letter, digit, char('_'))))),
            |(_c, i)| SimpleId(&s[1..(i + 1)]))(s)
    }
}
fn simple_id(s: &str) -> IResult<SimpleId> { SimpleId::parse(s) }

// 144
fn simple_string_literal(s: &str) -> IResult<String> {
    let f = alt((
        map(tag("''"), |_| '\''),
        not_paren_star_quote_special,
        letter,
        digit,
        one_of("()*")
    ));
    delimited(
            char('\''),
            fold_many0(f, String::new(), |mut s, c| { s.push(c); s }),
            char('\''))(s)
}

// 145-149 (remarks) are parsed beforehand

// 150-163
alias!(AttributeRef<'a>, AttributeId, attribute_ref);
alias!(ConstantRef<'a>, ConstantId, constant_ref);
alias!(EntityRef<'a>, EntityId, entity_ref);
alias!(EnumerationRef<'a>, EnumerationId, enumeration_ref);
alias!(FunctionRef<'a>, FunctionId, function_ref);
alias!(ParameterRef<'a>, ParameterId, parameter_ref);
alias!(ProcedureRef<'a>, ProcedureId, procedure_ref);
alias!(RuleLabelRef<'a>, RuleLabelId, rule_label_ref);
alias!(RuleRef<'a>, RuleId, rule_ref);
alias!(SchemaRef<'a>, SchemaId, schema_ref);
alias!(SubtypeConstraintRef<'a>, SubtypeConstraintId, subtype_constraint_ref);
alias!(TypeLabelRef<'a>, TypeLabelId, type_label_ref);
alias!(TypeRef<'a>, TypeId, type_ref);
alias!(VariableRef<'a>, VariableId, variable_ref);

// 164 abstract_entity_declaration = ABSTRACT .
fn abstract_entity_declaration(s: &str) -> IResult<()> {
    map(ws(tag("abstract")), |_| ())(s)
}

// 165 abstract_supertype = ABSTRACT SUPERTYPE ’;’ .
fn abstract_supertype(s: &str) -> IResult<()> {
    map(tuple((
        ws(tag("abstract")),
        ws(tag("supertype")),
        ws(char(';'))
    )), |_| ())(s)
}

// 166 abstract_supertype_declaration = ABSTRACT SUPERTYPE [ subtype_constraint ] .
struct AbstractSupertypeDeclaration(Option<SubtypeConstraint>);
fn abstract_supertype_declaration(s: &str) -> IResult<AbstractSupertypeDeclaration> {
    map(tuple((
        ws(tag("abstract")),
        ws(tag("supertype")),
        opt(ws(subtype_constraint)),
    )), |(_, _, a)| AbstractSupertypeDeclaration(a))(s)
}

// 167
struct ActualParameterList<'a>(Vec<Parameter<'a>>);
fn actual_parameter_list(s: &str) -> IResult<ActualParameterList> {
    map(delimited(
            ws(char('(')),
            separated_list1(ws(char(',')), parameter),
            ws(char(')'))),
        ActualParameterList)(s)
}

// 168
enum AddLikeOp { Add, Sub, Or, Xor }
fn add_like_op(s: &str) -> IResult<AddLikeOp> {
    use AddLikeOp::*;
    alt((
        map(char('+'),  |_| Add),
        map(char('-'),  |_| Sub),
        map(tag("or"),  |_| Or),
        map(tag("xor"), |_| Xor),
    ))(s)
}

// 169
struct AggregateInitializer<'a>(Vec<Element<'a>>);
fn aggregate_initializer(s: &str) -> IResult<AggregateInitializer> {
    map(delimited(
            ws(char('[')),
            separated_list0(ws(char(',')), ws(element)),
            ws(char(']'))),
        AggregateInitializer)(s)
}

// 170
alias!(AggregateSource<'a>, SimpleExpression, aggregate_source);

// 171 aggregate_type = AGGREGATE [ ’:’ type_label ] OF parameter_type .
struct AggregateType<'a>(Option<TypeLabel<'a>>, ParameterType<'a>);
fn aggregate_type(s: &str) -> IResult<AggregateType> {
    map(tuple((
        ws(tag("aggregate")),
        opt(preceded(ws(char(':')), ws(type_label))),
        ws(tag("of")),
        ws(parameter_type),
    )), |(_, t, _, p)| AggregateType(t, p))(s)
}

// 172
enum AggregationTypes<'a> {
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
struct AlgorithmHead<'a> {
    declaration: Vec<Declaration<'a>>,
    constant: Option<ConstantDecl<'a>>,
    local: Option<LocalDecl<'a>>,
}
fn algorithm_head(s: &str) -> IResult<AlgorithmHead> {
    map(tuple((
        many0(ws(declaration)),
        opt(ws(constant_decl)),
        opt(ws(local_decl)),
    )), |(d, c, l)| AlgorithmHead {
        declaration: d,
        constant: c,
        local: l
    })(s)
}

// 174 alias_stmt = ALIAS variable_id FOR general_ref { qualifier } ’;’ stmt { stmt }
//                  END_ALIAS ’;’ .
struct AliasStmt<'a> {
    variable: VariableId<'a>,
    general: GeneralRef<'a>,
    qualifiers: Vec<Qualifier<'a>>,
    stmts: Vec<Stmt<'a>>,
}
fn alias_stmt(s: &str) -> IResult<AliasStmt> {
    map(tuple((
        ws(tag("alias")),
        ws(variable_id),
        ws(tag("for")),
        ws(general_ref),
        many0(ws(qualifier)),
        ws(char(';')),
        many0(ws(stmt)),
    )), |(_, v, _, g, q, _, s)| AliasStmt {
        variable: v,
        general: g,
        qualifiers: q,
        stmts: s,
    })(s)
}

// 175
struct ArrayType<'a> {
    bounds: BoundSpec<'a>,
    optional: bool,
    unique: bool,
    instantiable_type: Box<InstantiableType<'a>>,
}
fn array_type(s: &str) -> IResult<ArrayType> {
    map(tuple((
        ws(tag("array")),
        ws(bound_spec),
        ws(tag("of")),
        ws(opt(tag("optional"))),
        ws(opt(tag("unique"))),
        ws(instantiable_type),
    )),
    |(_, b, _, opt, uniq, t)| ArrayType {
        bounds: b,
        optional: opt.is_some(),
        unique: uniq.is_some(),
        instantiable_type: Box::new(t),
    })(s)
}

// 176 assignment_stmt = general_ref { qualifier } ’:=’ expression ’;’ .
struct AssignmentStmt<'a> {
    general_ref: GeneralRef<'a>,
    qualifiers: Vec<Qualifier<'a>>,
    expression: Expression<'a>,
}
fn assignment_stmt(s: &str) -> IResult<AssignmentStmt> {
    map(tuple((
        ws(general_ref),
        many0(ws(qualifier)),
        ws(tag(":=")),
        ws(expression),
        ws(char(';')),
    )), |(g, q, _, e, _)| AssignmentStmt {
        general_ref: g,
        qualifiers: q,
        expression: e,
    })(s)
}

// 177 attribute_decl = attribute_id | redeclared_attribute .
enum AttributeDecl<'a> {
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
alias!(AttributeId<'a>, SimpleId, attribute_id);

// 179
alias!(AttributeQualifier<'a>, AttributeRef, attribute_qualifier);

// 180
struct BagType<'a>(Option<BoundSpec<'a>>, Box<InstantiableType<'a>>);
fn bag_type(s: &str) -> IResult<BagType> {
    map(tuple((
            ws(tag("BAG")),
            ws(opt(bound_spec)),
            ws(tag("OF")),
            ws(instantiable_type)
        )), |(_, b, _, t)| BagType(b, Box::new(t)))
        (s)
}

// 181 binary_type = BINARY [ width_spec ] .
struct BinaryType(Option<usize>);
fn binary_type(s: &str) -> IResult<BinaryType> {
    map(preceded(ws(tag("binary")), opt(ws(width_spec))), BinaryType)(s)
}

// 182 boolean_type = BOOLEAN .
fn boolean_type(s: &str) -> IResult<()> {
    map(ws(tag("boolean")), |_| ())(s)
}

// 183
alias!(Bound1<'a>, NumericalExpression, bound_1);

// 184
alias!(Bound2<'a>, NumericalExpression, bound_2);

// 185
struct BoundSpec<'a>(Bound1<'a>, Bound2<'a>);
fn bound_spec(s: &str) -> IResult<BoundSpec> {
    map(tuple((
        ws(char('[')),
        ws(bound_1),
        ws(char(':')),
        ws(bound_2),
        ws(char(']')),
    )), |(_, b1, _, b2, _)| BoundSpec(b1, b2))(s)
}

// 186
enum BuiltInConstant { ConstE, Pi, Self_, Indeterminant }
fn built_in_constant(s: &str) -> IResult<BuiltInConstant> {
    use BuiltInConstant::*;
    alt((
        map(tag("const_e"), |_| ConstE),
        map(tag("pi"),      |_| Pi),
        map(tag("self"),    |_| Self_),
        map(char('?'),      |_| Indeterminant),
    ))(s)
}

// 187
enum BuiltInFunction {
    Abs, Acos, Asin, Atan, Blength, Cos, Exists, Exp, Format, Hibound, HiIndex,
    Length, LoBound, LoIndex, Log, Log2, Log10, Nvl, Odd, RolesOf, Sin, SizeOf,
    Sqrt, Tan, Typeof, Usedin, Value, ValueIn, ValueUnique
}
fn built_in_function(s: &str) -> IResult<BuiltInFunction> {
    use BuiltInFunction::*;
    // Tokenize then match the keyword, instead of doing a huge alt(...)
    let (rest, kw) = alpha1(s)?;
    Ok((rest, match kw {
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
        _ => return build_err(s, "No such built-in function"),
    }))
}

// 188 built_in_procedure = INSERT | REMOVE .
enum BuiltInProcedure { Insert, Remove }
fn built_in_procedure(s: &str) -> IResult<BuiltInProcedure> {
    use BuiltInProcedure::*;
    alt((
        map(ws(tag("insert")), |_| Insert),
        map(ws(tag("remove")), |_| Remove),
    ))(s)
}

// 189 case_action = case_label { ’,’ case_label } ’:’ stmt .
struct CaseAction<'a>(Vec<CaseLabel<'a>>, Stmt<'a>);
fn case_action(s: &str) -> IResult<CaseAction> {
    map(tuple((
        separated_list1(ws(char(',')), ws(case_label)),
        ws(char(':')),
        ws(stmt),
    )), |(a, _, b)| CaseAction(a, b))(s)
}

// 190 case_label = expression .
alias!(CaseLabel<'a>, Expression, case_label);

// 191 case_stmt = CASE selector OF { case_action } [ OTHERWISE ’:’ stmt ]
//                  END_CASE ’;’ .
struct CaseStmt<'a> {
    selector: Selector<'a>,
    actions: Vec<CaseAction<'a>>,
    otherwise: Option<Stmt<'a>>,
}
fn case_stmt(s: &str) -> IResult<CaseStmt> {
    map(tuple((
        ws(tag("case")),
        ws(selector),
        ws(tag("of")),
        many0(ws(case_action)),
        opt(map(tuple((
            ws(tag("otherwise")),
            ws(char(':')),
            ws(stmt))), |(_, _, s)| s)),
        ws(tag("end_case")),
        ws(char(';')))),
        |(_, s, _, a, t, _, _)| CaseStmt {
            selector: s,
            actions: a,
            stmt: t,
        })(s)
}

// 192 compound_stmt = BEGIN stmt { stmt } END ’;’ .
struct CompoundStmt<'a>(Vec<Stmt<'a>>);
fn compount_stmt(s: &str) -> IResult<CompoundStmt> {
    map(delimited(
            ws(tag("begin")),
            many1(ws(stmt)),
            pair(ws(tag("end")), ws(char(';')))),
        CompoundStmt)(s)
}

// 193
enum ConcreteTypes<'a> {
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
struct ConstantBody<'a> {
    constant_id: ConstantId<'a>,
    instantiable_type: InstantiableType<'a>,
    expression: Expression<'a>,
}
fn constant_body(s: &str) -> IResult<ConstantBody> {
    map(tuple((
        ws(constant_id),
        ws(char(':')),
        ws(instantiable_type),
        ws(tag(":=")),
        ws(expression),
        ws(char(';'))
    )), |(a, _, t, _, e, _)| ConstantBody {
        constant_id: a,
        instantiable_type: t,
        expression: e,
    })(s)
}

// 195
struct ConstantDecl<'a>(Vec<ConstantBody<'a>>);
fn constant_decl(s: &str) -> IResult<ConstantDecl> {
    map(tuple((
        ws(tag("constant")),
        many1(ws(constant_body)),
        ws(tag("end_constant")),
        ws(char(';')),
    )), |(_, b, _, _)| ConstantDecl(b))(s)
}

// 196
enum ConstantFactor<'a> {
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
alias!(ConstantId<'a>, SimpleId, constant_id);

// 198
enum ConstructedTypes<'a> {
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
enum Declaration<'a> {
    Entity(EntityDecl<'a>),
    Function(FunctionDecl<'a>),
    Procedure(ProcedureDecl<'a>),
    SubtypeConstraint(SubtypeConstraintDecl<'a>),
    Type(TypeDecl<'a>),
}
fn declaration(s: &str) -> IResult<Declaration> {
    use Declaration::*;
    ws(alt((
        map(entity_decl, Entity),
        map(function_decl, Function),
        map(procedure_decl, Procedure),
        map(subtype_constraint_decl, SubtypeConstraint),
        map(type_decl, Type),
    )))(s)
}

// 200 derived_attr = attribute_decl ’:’ parameter_type ’:=’ expression ’;’ .
struct DerivedAttr<'a>(AttributeDecl<'a>, ParameterType<'a>, Expression<'a>);
fn derived_attr(s: &str) -> IResult<DerivedAttr> {
    map(tuple((
        ws(attribute_decl),
        ws(char(':')),
        ws(parameter_type),
        ws(tag(":=")),
        ws(expression),
        ws(char(';')),
    )), |(a, _, b, _, e, _)| DerivedAttr(a, b, e))(s)
}

// 201 derive_clause = DERIVE derived_attr { derived_attr } .
struct DeriveClause<'a>(Vec<DerivedAttr<'a>>);
fn derive_clause(s: &str) -> IResult<DeriveClause> {
    map(preceded(ws(tag("derive")), many1(ws(derived_attr))), DeriveClause)(s)
}

// 202
struct DomainRule<'a> {
    rule_label_id: Option<RuleLabelId<'a>>,
    expression: Expression<'a>,
}
fn domain_rule(s: &str) -> IResult<DomainRule> {
    map(pair(opt(terminated(ws(rule_label_id), ws(char(':')))), expression),
         |(rule_label_id, expression)| DomainRule {rule_label_id, expression})
        (s)
}

// 203
struct Element<'a>(Expression<'a>, Option<Repetition<'a>>);
fn element(s: &str) -> IResult<Element> {
    map(pair(ws(expression), opt(preceded(ws(char(':')), repetition))),
        |(a, b)| Element(a, b))(s)
}

// 204 entity_body = { explicit_attr } [ derive_clause ] [ inverse_clause ]
//                   [ unique_clause ] [ where_clause ] .
struct EntityBody<'a> {
    explicit_attr: Vec<ExplicitAttr<'a>>,
    derive: Option<DeriveClause<'a>>,
    inverse: Option<InverseClause<'a>>,
    unique: Option<UniqueClause<'a>>,
    where_: Option<WhereClause<'a>>,
}
fn entity_body(s: &str) -> IResult<EntityBody> {
    map(tuple((
        many0(ws(explicit_attr)),
        opt(ws(derive_clause)),
        opt(ws(inverse_clause)),
        opt(ws(unique_clause)),
        opt(ws(where_clause)),
    )), |(a, b, c, d, e)| EntityBody {
        explicit_attr: a,
        derive: b,
        inverse: c,
        unique: d,
        where_: e,
    })(s)
}

// 205
struct EntityConstructor<'a> {
    entity_ref: EntityRef<'a>,
    args: Vec<Expression<'a>>,
}
fn entity_constructor(s: &str) -> IResult<EntityConstructor> {
    map(pair(
        ws(entity_ref),
        delimited(
            ws(char('(')),
            separated_list0(ws(char(',')), ws(expression)),
            ws(char(')')),
        )), |(r, a)| EntityConstructor { entity_ref: r, args: a} )(s)
}

// 206 entity_decl = entity_head entity_body END_ENTITY ’;’ .
struct EntityDecl<'a>(EntityHead<'a>, EntityBody<'a>);
fn entity_decl(s: &str) -> IResult<EntityDecl> {
    map(tuple((
        ws(entity_head),
        ws(entity_body),
        ws(tag("end_entity")),
        ws(char(';')),
    )), |(a, b, _, _)| EntityDecl(a, b))(s)
}

// 207 entity_head = ENTITY entity_id subsuper ’;’ .
struct EntityHead<'a>(EntityId<'a>, Subsuper<'a>);
fn entity_head(s: &str) -> IResult<EntityHead> {
    map(tuple((
        ws(tag("entity")),
        ws(entity_id),
        ws(subsuper),
        ws(char(';')),
    )), |(_, a, b, _)| EntityHead(a, b))(s)
}

// 208
alias!(EntityId<'a>, SimpleId, entity_id);

// 209
struct EnumerationExtension<'a> {
    type_ref: TypeRef<'a>,
    enumeration_items: Option<EnumerationItems<'a>>,
}
fn enumeration_extension(s: &str) -> IResult<EnumerationExtension> {
    map(preceded(
        ws(tag("based_on")),
        pair(type_ref, opt(preceded(ws(tag("with")), enumeration_items)))),
        |(a, b)| EnumerationExtension { type_ref: a, enumeration_items: b })(s)
}

// 210
alias!(EnumerationId<'a>, SimpleId, enumeration_id);

// 211
struct EnumerationItems<'a>(Vec<EnumerationId<'a>>);
fn enumeration_items(s: &str) -> IResult<EnumerationItems> {
    map(delimited(
        ws(char('(')),
        separated_list1(ws(char(',')), ws(enumeration_id)),
        ws(char(')'))), EnumerationItems)(s)
}

// 212
struct EnumerationReference<'a>(Option<TypeRef<'a>>, EnumerationRef<'a>);
fn enumeration_reference(s: &str) -> IResult<EnumerationReference> {
    map(tuple((
        ws(opt(terminated(ws(type_ref), char('.')))),
        enumeration_ref
    )), |(a, b)| EnumerationReference(a, b))(s)
}

// 213
enum EnumerationItemsOrExtension<'a> {
    Items(EnumerationItems<'a>),
    Extension(EnumerationExtension<'a>),
}
struct EnumerationType<'a> {
    extensible: bool,
    items_or_extension: Option<EnumerationItemsOrExtension<'a>>
}
fn enumeration_type(s: &str) -> IResult<EnumerationType> {
    map(tuple((
        ws(opt(tag("extensible"))),
        ws(tag("enumeration")),
        ws(opt(alt((
            map(preceded(ws(tag("of")), enumeration_items),
                EnumerationItemsOrExtension::Items),
            map(enumeration_extension,
                EnumerationItemsOrExtension::Extension)))))
    )), |(e, _, p)| EnumerationType {
        extensible: e.is_some(),
        items_or_extension: p })(s)
}

// 214 escape_stmt = ESCAPE ’;’ .
fn escape_stmt(s: &str) -> IResult<()> {
    map(pair(ws(tag("escape")), ws(char(';'))), |_| ())(s)
}

// 215 explicit_attr = attribute_decl { ’,’ attribute_decl } ’:’ [ OPTIONAL ]
//                      parameter_type ’;’ .
struct ExplicitAttr<'a> {
    attributes: Vec<AttributeDecl<'a>>,
    optional: bool,
    parameter_type: ParameterType<'a>,
}
fn explicit_attr(s: &str) -> IResult<ExplicitAttr> {
    map(tuple((
        separated_list1(ws(','), ws(attribute_decl)),
        ws(char(':')),
        opt(ws(tag("optional"))),
        ws(parameter_type),
        ws(char(';')),
    )), |(a, _, o, t, _)| ExplicitAttr {
        attributes: a,
        optional: o.is_some(),
        parameter_type: t,
    })(s)
}

// 216
struct Expression<'a>(SimpleExpression<'a>, Option<(RelOpExtended, SimpleExpression<'a>)>);
impl<'a> Expression<'a> {
    fn parse(s: &'a str) -> IResult<Self> {
        map(pair(simple_expression,
                 opt(pair(rel_op_extended, simple_expression))),
            |(a, b)| Self(a, b))(s)
    }
}
fn expression(s: &str) -> IResult<Expression> { Expression::parse(s) }

// 217
struct Factor<'a>(SimpleFactor<'a>, Option<SimpleFactor<'a>>);
fn factor(s: &str) -> IResult<Factor> {
    map(pair(simple_factor, opt(preceded(tag("**"), simple_factor))),
        |(a, b)| Factor(a, b))(s)
}

// 218 formal_parameter = parameter_id { ’,’ parameter_id } ’:’ parameter_type .
struct FormalParameter<'a>(Vec<ParameterId<'a>>, ParameterType<'a>);
fn formal_parameter(s: &str) -> IResult<FormalParameter> {
    map(tuple((
        separated_list1(ws(char(',')), ws(parameter_id)),
        ws(char(':')),
        ws(parameter_type)
    )), |(a, _, b)| FormalParameter(a, b))(s)
}

// 219
enum BuiltInOrFunctionRef<'a> {
    BuiltIn(BuiltInFunction),
    Ref(FunctionRef<'a>),
}
struct FunctionCall<'a>(BuiltInOrFunctionRef<'a>, ActualParameterList<'a>);
fn function_call(s: &str) -> IResult<FunctionCall> {
    map(pair(
            ws(alt((map(built_in_function, BuiltInOrFunctionRef::BuiltIn),
                    map(function_ref, BuiltInOrFunctionRef::Ref)))),
            actual_parameter_list),
        |(a, b)| FunctionCall(a, b))(s)
}
// 220 function_decl = function_head algorithm_head stmt { stmt } END_FUNCTION ’;’ .
struct FunctionDecl<'a> {
    function_head: FunctionHead<'a>,
    algorithm_head: AlgorithmHead<'a>,
    stmts: Vec<Stmt<'a>>,
}
fn function_decl(s: &str) -> IResult<FunctionDecl> {
    map(tuple((
        ws(function_head),
        ws(algorithm_head),
        many1(ws(stmt)),
        ws(tag("end_function")),
        ws(char(';')),
    )), |(a, b, c, _, _)| FunctionDecl {
        function_head: a,
        algorithm_head: b,
        stmts: c,
    })(s)
}

// 221 function_head = FUNCTION function_id [ ’(’ formal_parameter
//                     { ’;’ formal_parameter } ’)’ ] ’:’ parameter_type ’;’ .
struct FunctionHead<'a> {
    id: FunctionId<'a>,
    params: Option<Vec<FormalParameter<'a>>>,
    out: ParameterType<'a>,
}
fn function_head(s: &str) -> IResult<FunctionHead> {
    map(tuple((
        ws(tag("function")),
        ws(function_id),
        opt(delimited(
            ws(char('(')),
            separated_list1(ws(char(';')), ws(formal_parameter)),
            ws(char(')')))),
        ws(char(':')),
        ws(parameter_type),
        ws(char(';')),
    )), |(_, i, a, _, p, _)| FunctionHead {
        id: i,
        params: a,
        out: p,
    })(s)
}

// 222
alias!(FunctionId<'a>, SimpleId, function_id);

// 223 generalized_types = aggregate_type | general_aggregation_types |
//                         generic_entity_type | generic_type .
enum GeneralizedTypes<'a> {
    Aggregate(AggregateType<'a>),
    GeneralAggregate(GeneralAggregateType<'a>),
    GenericEntity(GenericEntityType<'a>),
    Generic(GenericType<'a>),
}
fn generalized_types(s: &str) -> IResult<GeneralizedTypes> {
    use GeneralizedTypes::*;
    alt((
        map(aggregate_type, Aggregate),
        map(general_aggregate_type, GeneralAggregate),
        map(generic_entity_type, GenericEntity),
        map(generic_type, Generic),
    ))(s)
}

// 224 general_aggregation_types = general_array_type | general_bag_type |
//                                 general_list_type | general_set_type .
enum GeneralAggregationTypes<'a> {
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
struct GeneralArrayType<'a> {
    bounds: BoundSpec<'a>,
    optional: bool,
    unique: bool,
    parameter_type: Box<ParameterType<'a>>,
}
fn general_array_type(s: &str) -> IResult<GeneralArrayType> {
    map(tuple((
        ws(tag("array")),
        ws(bound_spec),
        ws(tag("of")),
        ws(opt(tag("optional"))),
        ws(opt(tag("unique"))),
        ws(parameter_type),
    )),
    |(_, b, _, opt, uniq, t)| ArrayType {
        bounds: b,
        optional: opt.is_some(),
        unique: uniq.is_some(),
        parameter_type: Box::new(t),
    })(s)
}

// 226 general_bag_type = BAG [ bound_spec ] OF parameter_type .
struct GeneralBagType<'a>(Option<BoundSpec<'a>>, Box<ParameterType<'a>>);
fn general_bag_type(s: &str) -> IResult<GeneralBagType> {
    map(tuple((
            ws(tag("BAG")),
            ws(opt(bound_spec)),
            ws(tag("OF")),
            ws(parameter_type)
        )), |(_, b, _, t)| BagType(b, Box::new(t)))
        (s)
}

// 227 general_list_type = LIST [ bound_spec ] OF [ UNIQUE ] parameter_type .
struct GeneralListType<'a> {
    bounds: Option<BoundSpec<'a>>,
    unique: bool,
    parameter_type: Box<ParameterType<'a>>,
}
fn general_list_type(s: &str) -> IResult<GeneralListType> {
    map(tuple((
        ws(tag("list")),
        opt(ws(bound_spec)),
        ws(tag("of")),
        ws(opt(tag("unique"))),
        ws(parameter_type),
    )),
    |(_, b, _, uniq, t)| GeneralListType {
        bounds: b,
        unique: uniq.is_some(),
        instantiable_type: Box::new(t),
    })(s)
}

// 228 general_ref = parameter_ref | variable_ref .
enum GeneralRef<'a> {
    Parameter(ParameterRef<'a>),
    Variable(VariableRef<'a>),
    _SimpleId(SimpleId<'a>),
}
fn general_ref(s: &str) -> IResult<GeneralRef> {
    map(simple_id, GeneralRef::_SimpleId)(s)
}

// 229 general_set_type = SET [ bound_spec ] OF parameter_type .
struct GeneralSetType<'a> {
    bounds: Option<BoundSpec<'a>>,
    parameter_type: Box<ParameterType<'a>>,
}
fn general_set_type(s: &str) -> IResult<GeneralSetType> {
    map(tuple((
        ws(tag("set")),
        ws(opt(bound_spec)),
        ws(tag("of")),
        ws(parameter_type),
    )),
    |(_, b, _, t)| GeneralSetType {
        bounds: b,
        instantiable_type: Box::new(t),
    })(s)
}

// 230 generic_entity_type = GENERIC_ENTITY [ ’:’ type_label ] .
struct GenericEntityType<'a>(Option<TypeLabel<'a>>);
fn generic_entity_type(s: &str) -> IResult<GenericEntityType> {
    map(preceded(ws(tag("generic_entity")),
                 opt(preceded(ws(char(':')), ws(type_label)))),
        GenericEntityType)(s)
}

// 231 generic_type = GENERIC [ ’:’ type_label ] .
struct GenericType<'a>(Option<TypeLabel<'a>>);
fn generic_type(s: &str) -> IResult<GenericType> {
    map(preceded(ws(tag("generic")),
                 opt(preceded(ws(char(':')), ws(type_label)))),
        GenericType)(s)
}

// 232
struct GroupQualifier<'a>(EntityRef<'a>);
fn group_qualifier(s: &str) -> IResult<GroupQualifier> {
    map(preceded(ws(char('\\')), ws(entity_ref)), GroupQualifier)(s)
}

// 233 if_stmt = IF logical_expression THEN stmt { stmt } [ ELSE stmt { stmt } ]
//               END_IF ’;’ .
struct IfStmt<'a>(LogicalExpression<'a>, Vec<Stmt<'a>>, Opt<Vec<Stmt<'a>>>);
fn if_stmt(s: &str) -> IResult<IfStmt> {
    map(tuple((
        ws(tag("if")),
        ws(logical_expression),
        ws(tag("then")),
        many1(ws(stmt)),
        opt(preceded(ws(tag("else")), many1(ws(stmt)))),
        ws(tag("end_if")),
        ws(char(';')),
    )), |(_, cond, _, a, b, _, _)| IfStmt(cond, a, b))(s)
}

// 234
alias!(Increment<'a>, NumericalExpression, increment);

// 235 increment_control = variable_id ’:=’ bound_1 TO bound_2 [ BY increment ] .
struct IncrementControl<'a> {
    var: VariableId<'a>,
    bound1: Bound1<'a>,
    bound2: Bound2<'a>,
    increment: Option<Increment<'a>>,
}
fn increment_control(s: &str) -> IResult<IncrementControl> {
    map(tuple((
        ws(variable_id),
        ws(tag(":=")),
        ws(bound_1),
        ws(tag("to")),
        ws(bound_2),
        opt(preceded(ws(tag("by")), ws(increment))),
    )), |(v, _, b1, _, b2, i)| IncrementControl {
        var: v,
        bound1: b1,
        bound2: b2,
        increment: i,
    })(s)
}

// 236
alias!(Index<'a>, NumericalExpression, index);

// 237
alias!(Index1<'a>, Index, index_1);

// 238
alias!(Index2<'a>, Index, index_2);

// 239
struct IndexQualifier<'a>(Index1<'a>, Index2<'a>);
fn index_qualifier(s: &str) -> IResult<IndexQualifier> {
    map(tuple((
        ws(char('[')),
        ws(index_1),
        ws(char(';')),
        ws(index_2),
        ws(char(']')),
    )), |(_, a, _, b, _)| IndexQualifier(a, b))(s)
}

// 240
enum InstantiableType<'a> {
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
    map(ws(tag("integer")), |_| ())(s)
}

// 242
enum InterfaceSpecification<'a> {
    ReferenceClause(ReferenceClause<'a>),
    UseClause(UseClause<'a>),
}
fn interface_specification(s: &str) -> IResult<InterfaceSpecification> {
    use InterfaceSpecification::*;
    alt((map(reference_clause, ReferenceClause),
         map(use_clause, UseClause)))(s)
}

// 243
struct Interval<'a> {
    low: IntervalLow<'a>,
    op1: IntervalOp,
    item: IntervalItem<'a>,
    op2: IntervalOp,
    high: IntervalHigh<'a>,
}
fn interval(s: &str) -> IResult<Interval> {
    map(delimited(
        ws(char('{')),
        ws(tuple((
            interval_low,
            interval_op,
            interval_item,
            interval_op,
            interval_high,
        ))),
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
enum IntervalOp { LessThan, LessThanOrEqual }
fn interval_op(s: &str) -> IResult<IntervalOp> {
    alt((
        map(char('<'), |_| IntervalOp::LessThan),
        map(tag("<="), |_| IntervalOp::LessThanOrEqual),
    ))(s)
}

// 250
struct ListType<'a> {
    bounds: Option<BoundSpec<'a>>,
    unique: bool,
    instantiable_type: Box<InstantiableType<'a>>,
}
fn list_type(s: &str) -> IResult<ListType> {
    map(tuple((
        ws(tag("list")),
        ws(opt(bound_spec)),
        ws(tag("of")),
        ws(opt(tag("unique"))),
        ws(instantiable_type),
    )),
    |(_, b, _, uniq, t)| ListType {
        bounds: b,
        unique: uniq.is_some(),
        instantiable_type: Box::new(t),
    })(s)
}

// 251
enum Literal {
    String(StringLiteral),
    Binary(usize),
    Logical(LogicalLiteral),
    Real(f64),
}
fn literal(s: &str) -> IResult<Literal> {
    use Literal::*;
    alt((
        map(binary_literal, Binary),
        map(string_literal, String),
        map(logical_literal, Logical),
        map(real_literal, Real)
    ))(s)
}
// 252 local_decl = LOCAL local_variable { local_variable } END_LOCAL ’;’
struct LocalDecl<'a>(Vec<LocalVariable<'a>>);
fn local_decl(s: &str) -> IResult<LocalDecl> {
    map(tuple((
        ws(tag("local")),
        many1(ws(local_variable)),
        ws(tag("end_local")),
        ws(char(';')),
    )), |(_, vs, _, _)| LocalDecl(vs))(s)
}
// 253 local_variable = variable_id { ’,’ variable_id } ’:’ parameter_type
//                      [ ’:=’ expression ] ’;’ .
struct LocalVariable<'a> {
    variable_id: Vec<VariableId<'a>>,
    parameter_type: ParameterType<'a>,
    expression: Option<Expression<'a>>,
}
fn local_variable(s: &str) -> IResult<LocalVariable> {
    map(tuple((
        separated_list1(ws(char(',')), ws(variable_id)),
        ws(char(':')),
        ws(parameter_type),
        opt(preceded(ws(tag(":=")), ws(expression))),
        ws(char(';')),
    )), |(vars, _, pt, exp, _)| LocalVariable {
        variable_id: vars,
        parameter_type: pt,
        expression: exp,
    })(s)
}

// 254
alias!(LogicalExpression<'a>, Expression, logical_expression);

// 255
enum LogicalLiteral {
    True, False, Unknown
}
fn logical_literal(s: &str) -> IResult<LogicalLiteral> {
    alt((map(tag("false"),   |_| LogicalLiteral::False),
         map(tag("true"),    |_| LogicalLiteral::True),
         map(tag("unknown"), |_| LogicalLiteral::Unknown)))(s)
}

// 256 logical_type = LOGICAL .
fn logical_type(s: &str) -> IResult<()> {
    map(ws(tag("logical")), |_| ())(s)
}

// 257
enum MultiplicationLikeOp {Mul, Div, IntegerDiv, Mod, And, ComplexEntity }
fn multiplication_like_op(s: &str) -> IResult<MultiplicationLikeOp> {
    use MultiplicationLikeOp::*;
    alt((
        map(char('*'),  |_| Mul),
        map(char('/'),  |_| Div),
        map(tag("div"), |_| IntegerDiv),
        map(tag("mod"), |_| Mod),
        map(tag("||"),  |_| ComplexEntity),
    ))(s)
}

// 258
enum NamedTypes<'a> {
    Entity(EntityRef<'a>),
    Type(TypeRef<'a>),
    _Ambiguous(SimpleId<'a>),
}
fn named_types(s: &str) -> IResult<NamedTypes> {
    map(simple_id, NamedTypes::_Ambiguous)(s)
}

// 259
enum EntityOrTypeId<'a> {
    Entity(EntityId<'a>),
    Type(EntityId<'a>),
    _Ambiguous(SimpleId<'a>),
}
struct NamedTypeOrRename<'a> {
    named_types: NamedTypes<'a>,
    rename: Option<EntityOrTypeId<'a>>,
}
fn named_type_or_rename(s: &str) -> IResult<NamedTypeOrRename> {
    map(pair(
        ws(named_types),
        opt(preceded(ws(tag("as")),
            map(ws(simple_id), EntityOrTypeId::_Ambiguous)))),
        |(a, b)| NamedTypeOrRename { named_types: a, rename: b })(s)
}

// 260 null_stmt = ’;’ .
fn null_stmt(s: &str) -> IResult<()> {
    map(ws(char(';')), |_| ())(s)
}

// 261 number_type = NUMBER .
fn number_type(s: &str) -> IResult<()> {
    map(ws(tag("number")), |_| ())(s)
}

// 262
alias!(NumericalExpression<'a>, SimpleExpression, numerical_expression);

// 264
alias!(Parameter<'a>, Expression, parameter);

// 265
alias!(ParameterId<'a>, SimpleId, parameter_id);

// 266
enum ParameterType<'a> {
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
alias!(Population<'a>, EntityRef, population);

// 268
alias!(PrecisionSpec<'a>, NumericalExpression, precision_spec);

// 269
enum Primary<'a> {
    Literal(Literal),
    Qualifiable(QualifiableFactor<'a>, Vec<Qualifier<'a>>),
}
fn primary(s: &str) -> IResult<Primary> {
    use Primary::*;
    alt((
        map(literal, Literal),
        map(pair(qualifiable_factor, many0(qualifier)),
            |(f, qs)| Qualifiable(f, qs))
    ))(s)
}

// 268
alias!(ProcedureId<'a>, SimpleId, procedure_id);

// 274
enum QualifiableFactor<'a> {
    AttributeRef(AttributeRef<'a>),
    ConstantFactor(ConstantFactor<'a>),
    FunctionCall(FunctionCall<'a>),
    GeneralRef(GeneralRef<'a>),
    Population(Population<'a>),

    // catch-all for attribute, constant, general, population
    _SimpleId(SimpleId<'a>),
}
fn qualifiable_factor(s: &str) -> IResult<QualifiableFactor> {
    use QualifiableFactor::*;
    alt((
        map(simple_id, _SimpleId),
        map(constant_factor, ConstantFactor),
        map(function_call, FunctionCall),
    ))(s)
}

// 276
enum Qualifier<'a> {
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

// 277
struct QueryExpression<'a> {
    var: VariableId<'a>,
    aggregate: AggregateSource<'a>,
    logical_expression: LogicalExpression<'a>,
}
fn query_expression(s: &str) -> IResult<QueryExpression> {
    map(tuple((
        ws(tag("QUERY")),
        ws(char('(')),
        ws(variable_id),
        ws(tag("<*")),
        ws(aggregate_source),
        ws(char('|')),
        ws(logical_expression,),
        ws(char(')')),
    )), |(_, _, var, _, aggregate, _, log, _)| QueryExpression {
        var,
        aggregate,
        logical_expression: log,
    })(s)
}

// 281
struct ReferenceClause<'a> {
    schema_ref: SchemaRef<'a>,
    resource_or_rename: Option<Vec<ResourceOrRename<'a>>>,
}
fn reference_clause(s: &str) -> IResult<ReferenceClause> {
    map(tuple((
        ws(tag("reference")),
        ws(tag("front")),
        ws(schema_ref),
        opt(delimited(
            ws(char('[')),
            separated_list1(ws(char(',')), ws(resource_or_rename)),
            ws(char(']')))),
        ws(char(';')),
    )), |(_, _, s, r, _)| ReferenceClause {
        schema_ref: s,
        resource_or_rename: r,
    })(s)
}

// 282
enum RelOp { LessThan, GreaterThan, LessThanOrEqual, GreaterThanOrEqual,
             NotEqual, Equal, InstanceEqual, InstanceNotEqual }
fn rel_op(s: &str) -> IResult<RelOp> {
    use RelOp::*;
    alt((
        map(char('<'),   |_| LessThan),
        map(char('>'),   |_| GreaterThan),
        map(tag("<="),   |_| LessThanOrEqual),
        map(tag(">="),   |_| GreaterThanOrEqual),
        map(tag("<>"),   |_| NotEqual),
        map(char('='),   |_| Equal),
        map(tag(":<>:"), |_| InstanceEqual),
        map(tag(":=:"),  |_| InstanceNotEqual),
    ))(s)
}

// 283
enum RelOpExtended { RelOp(RelOp), In, Like }
fn rel_op_extended(s: &str) -> IResult<RelOpExtended> {
    use RelOpExtended::*;
    alt((
        map(tag("in"),   |_| In),
        map(tag("like"), |_| Like),
        map(rel_op, RelOp)))(s)
}

// 289
enum RenameId<'a> {
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

// 287
alias!(Repetition<'a>, NumericalExpression, repetition);

// 288
struct ResourceOrRename<'a>(ResourceRef<'a>, Option<RenameId<'a>>);
fn resource_or_rename(s: &str) -> IResult<ResourceOrRename> {
    map(pair(ws(resource_ref), opt(preceded(ws(tag("as")), ws(rename_id)))),
        |(a, b)| ResourceOrRename(a, b))(s)
}

// 289
enum ResourceRef<'a> {
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

// 291 rule_decl = rule_head algorithm_head { stmt } where_clause END_RULE ’;’ .
struct RuleDecl<'a> {
    rule_head: RuleHead<'a>,
    algorithm_head: AlgorithmHead<'a>,
    stmt: Vec<Stmt<'a>>,
    where_clause: WhereClause<'a>,
}
fn rule_decl(s: &str) -> IResult<RuleDecl> {
    map(tuple((
        ws(rule_head),
        ws(algorithm_head),
        many0(ws(stmt)),
        ws(where_clause),
        ws(tag("end_rule")),
        ws(char(';')),
    )), |(r, a, s, w, _, _)| RuleDecl {
        rule_head: r,
        algorithm_head: a,
        stmt: s,
        where_clause: w,
    })(s)
}

// 292 rule_head = RULE rule_id FOR ’(’ entity_ref { ’,’ entity_ref } ’)’ ’;’ .
struct RuleHead<'a> {
    rule_id: RuleId<'a>,
    entities: Vec<EntityRef<'a>>,
}
fn rule_head(s: &str) -> IResult<RuleHead> {
    map(tuple((
        ws(tag("rule")),
        ws(rule_id),
        ws(tag("for")),
        delimited(
            ws(char('(')),
            separated_list1(ws(char(',')), ws(entity_ref)),
            ws(char(')')),
        ),
        ws(char(';')),
    )), |(_, id, _, es, _)| RuleHead {
        rule_id: id,
        entities: es,
    })(s)
}

// 293
alias!(RuleId<'a>, SimpleId, rule_id);

// 294
alias!(RuleLabelId<'a>, SimpleId, rule_label_id);

// 295
enum DeclarationOrRuleDecl<'a> {
    Declaration(Declaration<'a>),
    RuleDecl(RuleDecl<'a>),
}
struct SchemaBody<'a> {
    interfaces: Vec<InterfaceSpecification<'a>>,
    constants: Option<ConstantDecl<'a>>,
    declarations: Vec<DeclarationOrRuleDecl<'a>>,
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
struct SchemaDecl<'a> {
    id: SchemaId<'a>,
    version: Option<SchemaVersionId>,
    body: SchemaBody<'a>,
}
fn schema_decl(s: &str) -> IResult<SchemaDecl> {
    map(tuple((
        ws(tag("schema")),
        ws(schema_id),
        opt(ws(schema_version_id)),
        ws(char(';')),
        ws(schema_body),
        ws(tag("end_schema")),
        ws(char(';'))
    )), |(_, id, version, _, body, _, _)| SchemaDecl {
        id, version, body
    })(s)
}

// 297
alias!(SchemaId<'a>, SimpleId, schema_id);

// 298
alias!(SchemaVersionId, StringLiteral, schema_version_id);

// 299 selector = expression .
alias!(Selector<'a>, Expression, selector);

// 300
struct SelectExtension<'a> {
    type_ref: TypeRef<'a>,
    select_list: Option<SelectList<'a>>,
}
fn select_extension(s: &str) -> IResult<SelectExtension> {
    map(tuple((
        ws(tag("based_on")), type_ref,
        opt(preceded(ws(tag("with")), select_list))
    )), |(_, a, b)| SelectExtension {
        type_ref: a, select_list: b
    })(s)
}

// 301
struct SelectList<'a>(Vec<NamedTypes<'a>>);
fn select_list(s: &str) -> IResult<SelectList> {
    map(delimited(
        ws(char('(')),
        separated_list1(ws(char(',')), ws(named_types)),
        char(')')),
        SelectList)(s)
}

// 302
enum SelectListOrExtension<'a> {
    List(SelectList<'a>),
    Extension(SelectExtension<'a>),
}
struct SelectType<'a> {
    extensible: bool,
    generic_entity: bool,
    list_or_extension: SelectListOrExtension<'a>,
}
fn select_type(s: &str) -> IResult<SelectType> {
    map(tuple((
        opt(pair(ws(tag("extensible")), opt(ws(tag("generic_entity"))))),
        ws(tag("select")),
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
struct SetType<'a> {
    bounds: Option<BoundSpec<'a>>,
    instantiable_type: Box<InstantiableType<'a>>,
}
fn set_type(s: &str) -> IResult<SetType> {
    map(tuple((
        ws(tag("set")),
        ws(opt(bound_spec)),
        ws(tag("of")),
        ws(instantiable_type),
    )),
    |(_, b, _, t)| SetType {
        bounds: b,
        instantiable_type: Box::new(t),
    })(s)
}

// 304 sign = ’+’ | ’-’ .
enum Sign { Plus, Minus }
fn sign(s: &str) -> IResult<Sign> {
    use Sign::*;
    alt((
        map(ws(char('+')), |_| Plus),
        map(ws(char('-')), |_| Minus),
    ))(s)
}

// 305
struct SimpleExpression<'a>(Box<Term<'a>>, Option<(AddLikeOp, Box<Term<'a>>)>);
impl<'a> SimpleExpression<'a> {
    fn parse(s: &'a str) -> IResult<Self> {
        map(pair(term, opt(pair(add_like_op, term))),
            |(a, b)| SimpleExpression(Box::new(a),
                                      b.map(|p| (p.0, Box::new(p.1)))))(s)
    }
}
fn simple_expression(s: &str) -> IResult<SimpleExpression> {
    SimpleExpression::parse(s)
}

// 306
enum ExpressionOrPrimary<'a> {
    Expression(Box<Expression<'a>>),
    Primary(Primary<'a>),
}
enum SimpleFactor<'a> {
    AggregateInitializer(AggregateInitializer<'a>),
    EntityConstructor(EntityConstructor<'a>),
    EnumerationReference(EnumerationReference<'a>),
    Interval(Interval<'a>),
    QueryExpression(QueryExpression<'a>),
    Unary(Option<UnaryOp>, ExpressionOrPrimary<'a>)
}
fn simple_factor(s: &str) -> IResult<SimpleFactor> {
    use SimpleFactor::*;
    alt((
        map(aggregate_initializer, AggregateInitializer),
        map(entity_constructor, EntityConstructor),
        map(enumeration_reference, EnumerationReference),
        map(interval, Interval),
        map(query_expression, QueryExpression),
        map(pair(opt(ws(unary_op)), alt((
            map(delimited(ws(char('(')), ws(expression), char(')')),
                |e| ExpressionOrPrimary::Expression(Box::new(e))),
            map(primary, ExpressionOrPrimary::Primary)))),
            |(op, p)| Unary(op, p))
    ))(s)
}

// 307
enum SimpleTypes<'a> {
    Binary(Option<WidthSpec<'a>>), Boolean, Integer, Logical, Number,
    Real(Option<PrecisionSpec<'a>>), String(Option<WidthSpec<'a>>),
}
fn simple_types(s: &str) -> IResult<SimpleTypes> {
    use SimpleTypes::*;
    alt((
        map(preceded(ws(tag("binary")), opt(width_spec)), Binary),
        map(tag("boolean"), |_| Boolean),
        map(tag("integer"), |_| Integer),
        map(tag("logical"), |_| Logical),
        map(tag("number"), |_| Number),
        map(preceded(ws(tag("real")), opt(
            delimited(
                ws(char('(')),
                ws(precision_spec),
                char(')')),
            )), Real),
        map(preceded(ws(tag("string")), opt(width_spec)), String),
    ))(s)
}

// 308 skip_stmt = SKIP ’;’ .
fn skip_stmt(s: &str) -> IResult<()> {
    map(pair(ws(tag("skip")), ws(char(';'))), |_| ())(s)
}

// 309 stmt = alias_stmt | assignment_stmt | case_stmt | compound_stmt | escape_stmt |
//            if_stmt | null_stmt | procedure_call_stmt | repeat_stmt | return_stmt |
//            skip_stmt .
enum Stmt<'a> {
    Alias(AliasStmt<'a>),
    Assignment(AssignmentStmt<'a>),
    Case(CaseStmt<'a>),
    Compound(CompoundStmt<'a>),
    Escape(EscapeStmt<'a>),
    If(IfStmt<'a>),
    Null(NullStmt<'a>),
    ProcedureCall(ProcedureCallStmt<'a>),
    Repeat(RepeatStmt<'a>),
    Return(ReturnStmt<'a>),
    Skip(SkipStmt<'a>),
}
fn stmt(s: &str) -> IResult<Stmt> {
    use Stmt::*;
    alt((
        map(alias_stmt, Alias),
        map(assignment_stmt, Assignment),
        map(case_stmt, Case),
        map(compound_stmt, Compound),
        map(escape_stmt, Escape),
        map(if_stmt, If),
        map(null_stmt, Null),
        map(procedure_call_stmt, ProcedureCall),
        map(repeat_stmt, Repeat),
        map(return_stmt, Return),
        map(skip_stmt, Skip),
    ))(s)
}

// 310
struct StringLiteral(String);
impl StringLiteral {
    fn parse(s: &str) -> IResult<Self> {
        map(alt((simple_string_literal, encoded_string_literal)), Self)(s)
    }
}
fn string_literal(s: &str) -> IResult<StringLiteral> { StringLiteral::parse(s) }

// 317
alias!(SubtypeConstraintId<'a>, SimpleId, subtype_constraint_id);

// 325
struct Term<'a>(Factor<'a>, Option<(MultiplicationLikeOp, Factor<'a>)>);
fn term(s: &str) -> IResult<Term> {
    map(pair(factor, opt(pair(multiplication_like_op, factor))),
        |(a, b)| Term(a, b))(s)
}

// 327
struct TypeDecl<'a> {
    type_id: TypeId<'a>,
    underlying_type: UnderlyingType<'a>,
    where_clause: Option<WhereClause<'a>>,
}
fn type_decl(s: &str) -> IResult<TypeDecl> {
    map(tuple((
        ws(tag("type")),
        ws(type_id),
        ws(char('=')),
        ws(underlying_type),
        ws(char(';')),
        ws(opt(where_clause)),
        ws(tag("end_type")),
        ws(char(';')),
    )), |(_, t, _, u, _, w, _, _)| TypeDecl {
        type_id: t,
        underlying_type: u,
        where_clause: w,
    })(s)
}

// 328
alias!(TypeId<'a>, SimpleId, type_id);

// 330
alias!(TypeLabelId<'a>, SimpleId, type_label_id);

// 331
enum UnaryOp { Add, Sub, Not }
fn unary_op(s: &str) -> IResult<UnaryOp> {
    use UnaryOp::*;
    alt((
        map(char('+'),  |_| Add),
        map(char('-'),  |_| Sub),
        map(tag("not"), |_| Not),
    ))(s)
}

// 332
enum UnderlyingType<'a> {
    Concrete(ConcreteTypes<'a>),
    Constructed(ConstructedTypes<'a>),
}
fn underlying_type(s: &str) -> IResult<UnderlyingType> {
    use UnderlyingType::*;
    alt((
        map(concrete_types, Concrete),
        map(constructed_types, Constructed),
    ))(s)
}

// 338
struct WhereClause<'a>(Vec<DomainRule<'a>>);
fn where_clause(s: &str) -> IResult<WhereClause> {
    map(preceded(
            ws(tag("where")),
            many1(terminated(ws(domain_rule), ws(char(';'))))),
        |v| WhereClause(v))(s)
}

// 336
struct UseClause<'a> {
    schema_ref: SchemaRef<'a>,
    named_type_or_rename: Option<Vec<NamedTypeOrRename<'a>>>,
}
fn use_clause(s: &str) -> IResult<UseClause> {
    map(tuple((
        ws(tag("use")),
        ws(tag("from")),
        ws(schema_ref),
        opt(delimited(ws(char(')')),
            separated_list1(ws(char(',')), ws(named_type_or_rename)),
            ws(char(')')))),
        ws(char(';')),
    )), |(_, _, s, r, _)| UseClause {
        schema_ref: s,
        named_type_or_rename: r,
    })(s)
}

// 337
alias!(VariableId<'a>, SimpleId, variable_id);

// 339 while_control = WHILE logical_expression .
struct WhileControl<'a>(LogicalExpression<'a>);
fn while_control(s: &str) -> IResult<WhileControl> {
    map(preceded(ws(tag("while")), ws(logical_expression)), WhileControl)(s)
}

// 340
alias!(Width<'a>, NumericalExpression, width);

// 341
struct WidthSpec<'a> { expression: Width<'a>, fixed: bool }
fn width_spec(s: &str) -> IResult<WidthSpec> {
    map(tuple((
        ws(char('(')),
        ws(width),
        ws(char(')')),
        opt(tag("fixed"))
    )), |(_, w, _, f)| WidthSpec { expression: w, fixed: f.is_some() })(s)
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
}

////////////////////////////////////////////////////////////////////////////////

/// Remove comments from an EXPRESS file and converts to lower-case
pub fn strip_flatten(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        match data[i] {
            // Block comments
            b'(' => if i + 1 < data.len() && data[i + 1] == b'*' {
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
                i += newline.unwrap_or(0) + 3;
            },
            c => out.push(c.to_ascii_lowercase())
        }
        i += 1;
    }
    out
}

