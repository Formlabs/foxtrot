use std::fmt::Write;
use std::collections::{HashSet, HashMap};
use crate::parse::*;

////////////////////////////////////////////////////////////////////////////////
// Helper types to use when doing code-gen
#[derive(Debug)]
enum Type<'a> {
    Entity {
        // In order, with parent attributes first
        attrs: Vec<AttributeData<'a>>,
    },
    // These are all TYPE in EXPRESS, but we unpack them here
    Redeclared(&'a str),
    RedeclaredPrimitive(&'a str),
    Enum(Vec<&'a str>),
    Select(Vec<&'a str>),
    Aggregation {
        optional: bool,
        type_: Box<Type<'a>>,
    },

    // Direct Rust type
    Primitive(&'a str),
}
struct TypeMap<'a>(HashMap<&'a str, Type<'a>>, &'a HashMap<&'a str, Ref<'a>>);
impl <'a> TypeMap<'a> {
    fn to_rtype_build(&mut self, s: &'a str) -> String {
        if !self.0.contains_key(s) {
            self.build(s);
        }
        self.to_rtype(s)
    }
    fn to_rtype(&self, s: &str) -> String {
        let t = self.0.get(s).expect(&format!("Could not get {:?}", s));
        match &t {
            Type::Entity { .. }
            | Type::Redeclared(_)
            | Type::RedeclaredPrimitive(_)
            | Type::Enum(_)
            | Type::Select(_) => format!("{}<'a>", to_camel(s)),

            Type::Primitive(s) => s.to_string(),

            Type::Aggregation { optional, type_ } => if *optional {
                format!("Vec<Option<{}>>", self.to_inner_rtype(type_))
            } else {
                format!("Vec<{}>", self.to_inner_rtype(type_))
            },
        }
    }
    fn to_inner_rtype(&self, t: &Type<'a>) -> String {
        match &t {
            Type::Aggregation { optional, type_ } => if *optional {
                format!("Vec<Option<{}>>", self.to_inner_rtype(type_))
            } else {
                format!("Vec<{}>", self.to_inner_rtype(type_))
            },
            Type::Redeclared(r) => {
                format!("{}<'a>", to_camel(r))
            },
            Type::RedeclaredPrimitive(r) => r.to_string(),
            Type::Primitive(r) => r.to_string(),

            Type::Entity { .. } | Type::Enum(_) | Type::Select(_) =>
                panic!("Invalid inner type"),
        }
    }
    fn build(&mut self, s: &'a str) {
        let v = self.1.get(s).unwrap();
        let m = match v {
            Ref::Entity(e) => e.to_type(self),
            Ref::Type(t) => t.to_type(self),
        };
        self.0.insert(s, m);
    }
    fn attributes(&mut self, s: &'a str) -> Vec<AttributeData<'a>> {
        if !self.0.contains_key(s) {
            self.build(s);
        }
        let t = self.0.get(s).expect(&format!("Could not get {:?}", s));
        if let Type::Entity { attrs } = &t {
            attrs.clone()
        } else {
            panic!("Cannot get attributes of a non-entity");
        }
    }
}

impl<'a> Type<'a> {
    fn enum_variant<W>(&self, name: &str, buf: &mut W) -> std::fmt::Result
        where W: std::fmt::Write
    {
        match self {
            Type::Entity{..} => writeln!(buf, "    {0}({0}<'a>),",
                                         to_camel(name)),
            _ => Ok(()),
        }
    }
    fn enum_match<W>(&self, name: &str, buf: &mut W) -> std::fmt::Result
        where W: std::fmt::Write
    {
        match self {
            Type::Entity{..} => writeln!(buf,
                r#"            "{0}" => map({1}::parse, Entity::{1})(s),"#,
                capitalize(name), to_camel(name)),
            _ => Ok(()),
        }
    }
    fn gen<W>(&self, name: &str, buf: &mut W, type_map: &TypeMap) -> std::fmt::Result
        where W: std::fmt::Write
    {
        let camel_name = to_camel(name);
        match self {
            Type::Redeclared(c) => {
                writeln!(buf,r#"
pub struct {0}<'a>(pub {1}, std::marker::PhantomData<&'a ()>); // redeclared
impl<'a> Parse<'a> for {0}<'a> {{
    fn parse(s: &'a str) -> IResult<'a, Self> {{
        delimited(tag("{2}("), Self::parse_inner, char(')'))(s)
    }}
}}
impl<'a> {0}<'a> {{
    pub fn parse_inner(s: &'a str) -> IResult<'a, Self> {{
        map({3}::parse_inner, |r| Self(r, std::marker::PhantomData))(s)
    }}
}}"#,
                camel_name, type_map.to_rtype(c),
                capitalize(&name), to_camel(c))?;
            },
            Type::RedeclaredPrimitive(c) => {
                writeln!(buf, r#"pub struct {0}<'a>(pub {1}, std::marker::PhantomData<&'a ()>); // primitive
impl<'a> Parse<'a> for {0}<'a> {{
    fn parse(s: &'a str) -> IResult<'a, Self> {{
        Self::parse_inner(s)
    }}
}}
impl<'a> {0}<'a> {{
    pub fn parse_inner(s: &'a str) -> IResult<'a, Self> {{
        map(<{2}>::parse, |r| Self(r, std::marker::PhantomData))(s)
    }}
}}"#,
                    camel_name, c, strip_lifetime(c))?;
            },

            Type::Enum(c) => {
                writeln!(buf, "pub enum {}<'a> {{ // enum", camel_name)?;
                for v in c {
                    writeln!(buf, "    {},", to_camel(v))?;
                }
                writeln!(buf,
                    r#"    _Unused(std::marker::PhantomData<&'a ()>),
}}
impl<'a> Parse<'a> for {0}<'a> {{
    fn parse(s: &'a str) -> IResult<'a, Self> {{
        use {0}::*;"#, camel_name)?;

                // Extremely awkward code to account for the fact that nom's
                // `alt` combinator only accepts tuples of size 21 or smaller,
                // and doesn't accept tuples of size 1.
                if c.len() == 1 {
                    write!(buf,
                        r#"        map(tag(".{}."), |_| {})"#,
                        capitalize(c[0]), to_camel(c[0]))?;
                } else {
                    const NOM_MAX_ALT: usize = 21;
                    let mut offset = 0;
                    while offset < c.len() {
                        writeln!(buf, "        alt((")?;
                        let mut i = 0;
                        while i < NOM_MAX_ALT - 2 && i + offset < c.len() {
                            let v = c[i + offset];
                            writeln!(buf,
                                r#"            map(tag(".{}."), |_| {}),"#,
                                capitalize(v), to_camel(v))?;
                            i += 1;
                        }
                        offset += NOM_MAX_ALT - 2;
                    }
                    write!(buf, "        ")?;
                    for _ in 0..(offset / (NOM_MAX_ALT - 2)) {
                        write!(buf, "))")?;
                    }
                }
                writeln!(buf, "(s)
    }}
}}")?;
            },

            Type::Select(c) => {
                writeln!(buf, "pub enum {}<'a> {{ // select", camel_name)?;
                for v in c {
                    writeln!(buf, "    {}({}),", to_camel(v),
                             type_map.to_rtype(v))?;
                }
                writeln!(buf,
                    r#"    _Unused(std::marker::PhantomData<&'a ()>)
}}
impl<'a> Parse<'a> for {}<'a> {{
    fn parse(s: &'a str) -> IResult<'a, Self> {{"#, camel_name)?;

                // Same logic as above
                if c.len() == 1 {
                    write!(buf,
                        r#"        map(<{}>::parse, |r| {}::{}(r))"#,
                        type_map.to_rtype(c[0]), camel_name, to_camel(c[0]))?;
                } else {
                    const NOM_MAX_ALT: usize = 21;
                    let mut offset = 0;
                    while offset < c.len() {
                        writeln!(buf, "        alt((")?;
                        let mut i = 0;
                        while i < NOM_MAX_ALT - 2 && i + offset < c.len() {
                            let v = c[i + offset];
                            writeln!(buf,
                                "            map(<{}>::parse, |r| {}::{}(r)),",
                                type_map.to_rtype(v), camel_name, to_camel(v))?;
                            i += 1;
                        }
                        offset += NOM_MAX_ALT - 2;
                    }
                    write!(buf, "        ")?;
                    for _ in 0..(offset / (NOM_MAX_ALT - 2)) {
                        write!(buf, "))")?;
                    }
                }
                writeln!(buf, "(s)
    }}
}}")?;
            },

            Type::Aggregation { type_, .. } => {
                writeln!(buf,
                    r#"pub struct {0}<'a>(pub {1}, std::marker::PhantomData<&'a ()>); // aggregation
impl<'a> Parse<'a> for {0}<'a> {{
    fn parse(s: &'a str) -> IResult<'a, Self> {{
        map(many0(<{2}>::parse), |r| Self(r, std::marker::PhantomData))(s)
    }}
}}"#,
                    camel_name, type_map.to_inner_rtype(self),
                    type_map.to_inner_rtype(&*type_))?;
            }

            Type::Entity { attrs } => {
                if attrs.iter().any(|a| a.dupe) {
                    writeln!(buf, "#[allow(non_snake_case)]")?;
                }
                writeln!(buf, "pub struct {}_<'a> {{ // entity", camel_name)?;
                for a in attrs {
                    // Skip derived attributes in the struct
                    if a.derived {
                        continue;
                    }
                    if a.dupe {
                        write!(buf, "    pub {}__{}: ", a.from.unwrap(), a.name)?;
                    } else {
                        write!(buf, "    pub {}: ", a.name)?;
                    }
                    if a.optional {
                        writeln!(buf, "Option<{}>,", a.type_)?;
                    } else {
                        writeln!(buf, "{},", a.type_)?;
                    }
                }
                writeln!(buf, r#"    _marker: std::marker::PhantomData<&'a ()>,
}}
pub type {0}<'a> = Id<{0}_<'a>>;
impl<'a> Parse<'a> for {0}_<'a> {{
    fn parse(s: &'a str) -> IResult<'a, Self> {{
        let (s, _) = tag("{1}(")(s)?;"#, camel_name, capitalize(&name))?;
                // Then, write a series of parsers which build the whole struct
                for (i,a) in attrs.iter().enumerate() {
                    if a.derived {
                        writeln!(buf,
                                 r#"        let (s, _) = tag("*,")(s)?;"#)?;
                    } else {
                        if a.dupe {
                            writeln!(buf, "        #[allow(non_snake_case)]")?;
                            write!(buf, "        let (s, {}__{})",
                                   a.from.unwrap(), a.name)?;
                        } else {
                            write!(buf, "        let (s, {})", a.name)?;
                        }
                        if a.optional {
                            writeln!(buf, " = param::<Option<{}>>({}, s)?;", a.type_, i == attrs.len() - 1)?;
                        } else {
                            writeln!(buf, " = param::<{}>({}, s)?;", a.type_, i == attrs.len() - 1)?;
                        }
                    }
                }
                writeln!(buf, "        Ok((s, Self {{")?;
                for a in attrs {
                    if a.derived {
                        continue;
                    } else if a.dupe {
                        // TODO make this a function on `a`
                        writeln!(buf, "            {}__{},",
                                 a.from.unwrap(), a.name)?;
                    } else {
                        writeln!(buf, "            {},", a.name)?;
                    }
                }
                writeln!(buf, "            _marker: std::marker::PhantomData}}))
    }}
}}")?;
            },
            Type::Primitive(_) => (),
        };
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct AttributeData<'a> {
    name: &'a str, // already camel-case
    from: Option<&'a str>, // original class, or None
    type_: String,
    optional: bool,
    dupe: bool, // inherited from different parents with the same name
    derived: bool, // marked whether this is a derived attribute
}

////////////////////////////////////////////////////////////////////////////////

// A reference into an existing `Syntax` tree, for convenient random access
enum Ref<'a> {
    Entity(&'a EntityDecl<'a>),
    Type(&'a UnderlyingType<'a>),
}

////////////////////////////////////////////////////////////////////////////////

pub fn gen(s: &mut Syntax) -> Result<String, std::fmt::Error> {
    assert!(s.0.len() == 1, "Multiple schemas are unsupported");

    // First pass: collect entity names, then convert ambiguous IDs in SELECT
    // data types into Entity or Type refs
    let mut entity_names = HashSet::new();
    s.collect_entity_names(&mut entity_names);
    s.disambiguate(&entity_names);

    // From this point on, `s` is becomes immutable.  We build a map from type
    // names (in camel_case) to references into `s`, for ease of access.
    let mut ref_map = HashMap::new();
    s.build_ref_map(&mut ref_map);

    // Finally, we can build out the type map
    let mut type_map = TypeMap(HashMap::new(), &ref_map);
    type_map.0.insert("usize", Type::Primitive("usize"));
    type_map.0.insert("bool", Type::Primitive("bool"));
    type_map.0.insert("i64", Type::Primitive("i64"));
    type_map.0.insert("f64", Type::Primitive("f64"));
    type_map.0.insert("&'a str", Type::Primitive("&'a str"));

    for k in ref_map.keys() {
        type_map.build(k);
    }

    // Step four: do codegen on the completed type map (sorted for determinism)
    let mut keys: Vec<&str> = type_map.0.keys().cloned().collect();
    keys.sort();
    let mut buf = String::new();
    writeln!(&mut buf, "// Autogenerated file, do not hand-edit!
use crate::parse::{{Id, IResult, Logical, Parse, param}};
use nom::{{
    branch::{{alt}},
    bytes::complete::{{tag}},
    character::complete::{{alpha1, alphanumeric1, char}},
    combinator::{{map, recognize}},
    multi::{{many0}},
    sequence::{{delimited, pair}},
}};")?;

    for k in &keys {
        type_map.0[k].gen(k, &mut buf, &type_map)?;
    }
    writeln!(&mut buf, "pub enum Entity<'a> {{")?;
    for k in &keys {
        type_map.0[k].enum_variant(k, &mut buf)?;
    }
    writeln!(&mut buf, r#"}}
impl<'a> Parse<'a> for Entity<'a> {{
    fn parse(s: &'a str) -> IResult<'a, Self> {{
        let (_, r) = recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        ))(s)?;
        match r {{"#)?;
    for k in &keys {
        type_map.0[k].enum_match(k, &mut buf)?;
    }
    writeln!(&mut buf, r#"            _ => panic!("Invalid case"),
        }}
    }}
}}"#)?;
    Ok(buf)
}

fn capitalize(s: &str) -> String {
    s.chars().map(|c| c.to_uppercase().next().unwrap()).collect()
}

// TODO: this is awkward; it would be cleaner to store lifetime separately
// in the `ReplacedPrimitive` enum
fn strip_lifetime(s: &str) -> String {
    if s.starts_with("&'a ") {
        s.replacen("&'a ", "&", 1)
    } else {
        s.to_owned()
    }
}

fn to_camel(s: &str) -> String {
    let mut out = String::new();
    let mut cap = true;
    for c in s.chars() {
        if c == '_' {
            cap = true;
        } else if cap {
            out.push(c.to_uppercase().next().unwrap());
            cap = false;
        } else {
            out.push(c);
        }
    }
    out
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> Syntax<'a> {
    fn collect_entity_names(&self, entity_names: &mut HashSet<&'a str>) {
        for v in &self.0 {
            v.collect_entity_names(entity_names);
        }
    }
    fn build_ref_map(&'a self, ref_map: &mut HashMap<&'a str, Ref<'a>>) {
        for v in &self.0 {
            v.build_ref_map(ref_map);
        }
    }
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        for v in &mut self.0 {
            v.disambiguate(entity_names);
        }
    }
}
impl<'a> SchemaDecl<'a> {
    fn collect_entity_names(&self, entity_names: &mut HashSet<&'a str>) {
        self.body.collect_entity_names(entity_names);
    }
    fn build_ref_map(&'a self, ref_map: &mut HashMap<&'a str, Ref<'a>>) {
        self.body.build_ref_map(ref_map);
    }
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        self.body.disambiguate(entity_names)
    }
}
impl<'a> SchemaBody<'a> {
    fn collect_entity_names(&self, entity_names: &mut HashSet<&'a str>) {
        for d in &self.declarations {
            match d {
                DeclarationOrRuleDecl::Declaration(d) =>
                    d.collect_entity_names(entity_names),
                DeclarationOrRuleDecl::RuleDecl(_) => (),
            }
        }
    }
    fn build_ref_map(&'a self, ref_map: &mut HashMap<&'a str, Ref<'a>>) {
        for d in &self.declarations {
            match d {
                DeclarationOrRuleDecl::Declaration(d) =>
                    d.build_ref_map(ref_map),
                DeclarationOrRuleDecl::RuleDecl(_) => (),
            }
        }
    }
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        for d in &mut self.declarations {
            match d {
                DeclarationOrRuleDecl::Declaration(d) =>
                    d.disambiguate(entity_names),
                DeclarationOrRuleDecl::RuleDecl(_) => (),
            }
        }
    }
}
impl<'a> Declaration<'a> {
    fn collect_entity_names(&self, entity_names: &mut HashSet<&'a str>) {
        match self {
            Declaration::Entity(d) => { entity_names.insert(d.0.0.0); },
            _ => (),
        }
    }
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        match self {
            Declaration::Type(d) => d.disambiguate(entity_names),
            _ => (),
        }
    }
    fn build_ref_map(&'a self, ref_map: &mut HashMap<&'a str, Ref<'a>>) {
        match self {
            Declaration::Entity(d) => {
                ref_map.insert(d.0.0.0, Ref::Entity(d));
            },
            Declaration::Type(d) => {
                ref_map.insert(d.type_id.0, Ref::Type(&d.underlying_type));
            },
            _ => (),
        }
    }
}
impl<'a> TypeDecl<'a> {
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        match &mut self.underlying_type {
            UnderlyingType::Constructed(c) => {
                c.disambiguate(entity_names);
            },
            _ => (),
        }
    }
}
impl<'a> UnderlyingType<'a> {
    fn to_type(&'a self, type_map: &mut TypeMap<'a>) -> Type {
        match self {
            UnderlyingType::Concrete(c) => c.to_type(type_map),
            UnderlyingType::Constructed(c) => c.to_type(),
        }
    }
}
impl<'a> ConcreteTypes<'a> {
    fn to_type(&self, type_map: &mut TypeMap<'a>) -> Type {
        match self {
            ConcreteTypes::Aggregation(a) => a.to_type(type_map),
            ConcreteTypes::Simple(s) => s.to_type(),
            ConcreteTypes::TypeRef(t) => Type::Redeclared(t.0),
        }
    }
}
impl<'a> AggregationTypes<'a> {
    fn to_type(&self, type_map: &mut TypeMap<'a>) -> Type {
        let (optional, instantiable) = match self {
            AggregationTypes::Array(a) => (a.optional, &a.instantiable_type),
            AggregationTypes::Bag(a) => (false,  &a.1),
            AggregationTypes::List(a) => (false, &a.instantiable_type),
            AggregationTypes::Set(a) => (false, &a.instantiable_type),
        };
        match &**instantiable {
            InstantiableType::Concrete(c) => {
                let type_ = c.to_type(type_map);
                Type::Aggregation { optional,  type_: Box::new(type_) }
            },
            InstantiableType::EntityRef(e) => Type::Aggregation {
                optional, type_: Box::new(Type::Redeclared(e.0))
            }
        }
    }
}
impl<'a> ConstructedTypes<'a> {
    fn to_type(&'a self) -> Type {
        match self {
            ConstructedTypes::Enumeration(e) => e.to_type(),
            ConstructedTypes::Select(s) => s.to_type(),
        }
    }
}
impl<'a> EnumerationType<'a> {
    fn to_type(&self) -> Type {
        assert!(!self.extensible, "Extensible enumerations are not supported");
        match self.items_or_extension.as_ref().unwrap() {
            EnumerationItemsOrExtension::Items(e) => e.to_type(),
            _ => panic!("Extensions not supported")
        }
    }
}
impl<'a> EnumerationItems<'a> {
    fn to_type(&self) -> Type {
        let mut out = Vec::new();
        for e in &self.0 {
            out.push(e.0);
        }
        Type::Enum(out)
    }
}
impl<'a> SelectType<'a> {
    fn to_type(&'a self) -> Type {
        assert!(!self.extensible, "Cannot handle extensible lists");
        assert!(!self.generic_entity, "Cannot handle generic entity lists");
        match &self.list_or_extension {
            SelectListOrExtension::List(e) => e.to_type(),
            _ => panic!("Extensions not supported"),
        }
    }
}
impl<'a> SelectList<'a> {
    fn to_type(&'a self) -> Type {
        let mut out = Vec::new();
        for e in &self.0 {
            out.push(e.name());
        }
        Type::Select(out)
    }
}
impl<'a> EntityDecl<'a> {
    fn to_type(&'a self, type_map: &mut TypeMap<'a>) -> Type<'a> {
        // Derived values from parents shouldn't be stored in the struct, so
        // we build a map of them here and skip them when collecting attributes
        // from superclasses.
        let mut derived: HashSet<(&str, &str)> = HashSet::new();
        if let Some(derive) = &self.1.derive {
            for d in &derive.0 {
                match &d.0 {
                    AttributeDecl::Redeclared(r) => {
                        // There can't be a RENAMED clause here
                        assert!(r.1.is_none());
                        derived.insert((r.0.0.0.0, r.0.1.0.0));
                    }
                    AttributeDecl::Id(_) => continue,
                }
            }
        }

        // Skip attributes when we have multiple inheritance with a common
        // base class.
        let mut seen: HashSet<(&str, &str)> = HashSet::new();

        // Tag any inherited attribute names with > 1 occurence so we can
        // special-case them in the struct
        let subsuper = &self.0.1;
        let mut inherited_name_count: HashMap<&str, usize> = HashMap::new();
        if let Some(subs) = &subsuper.1 {
            for sub in &subs.0 {
                for a in type_map.attributes(sub.0) {
                    *inherited_name_count.entry(a.name).or_insert(0) += 1;
                }
            }
        }
        let inherited_names: HashSet<&str> = inherited_name_count.into_iter()
            .filter(|a| a.1 > 1)
            .map(|a| a.0)
            .collect();

        let mut attrs = Vec::new();
        if let Some(subs) = &subsuper.1 {
            for sub in subs.0.iter() {
                // Import attributes from parent classes, patching the
                // `from` field to indicate that it's from a superclass
                attrs.extend(type_map
                    .attributes(sub.0)
                    .into_iter()
                    .map(|a|
                        if a.from.is_none() {
                            AttributeData {
                                from: Some(sub.0),
                                dupe: inherited_names.contains(a.name),
                                derived: derived.contains(&(sub.0, a.name)),
                                ..a
                            }
                        } else {
                            a
                        }
                    )
                    // Skip values that have already been seen (if we have
                    // multiple inheritance from a common base class)
                    .filter(|a| seen.insert((a.from.unwrap(), a.name))));
            }
        }

        for attr in &self.1.explicit_attr {
            let attr_type = attr.parameter_type.to_attr_type_str(type_map);
            for a in &attr.attributes {
                if a.is_redeclared() {
                    // TODO: tweak existing attr type
                    continue;
                }
                attrs.push(AttributeData {
                    name: a.name(),
                    from: None,
                    dupe: false,
                    derived: false,
                    type_: attr_type.clone(),
                    optional: attr.optional,
                });
            }
        }
        Type::Entity { attrs }
    }
}
impl<'a> AttributeDecl<'a> {
    fn name(&self) -> &str {
        match self {
            AttributeDecl::Id(i) => i.0,
            AttributeDecl::Redeclared(_) =>
                panic!("No support for renamed attributes"),
        }
    }
    fn is_redeclared(&self) -> bool {
        match self {
            AttributeDecl::Id(_) => false,
            AttributeDecl::Redeclared(_) => true,
        }
    }
}
impl<'a> GeneralizedTypes<'a> {
    fn to_attr_type_str(&'a self, type_map: &mut TypeMap<'a>) -> String {
        match self {
            GeneralizedTypes::Aggregate(_) =>
                panic!("No support for aggregate type"),
            GeneralizedTypes::GeneralAggregation(a) =>
                a.to_attr_type_str(type_map),
            GeneralizedTypes::GenericEntity(_) =>
                panic!("No support for generic entity type"),
            GeneralizedTypes::Generic(_) =>
                panic!("No support for generic generalized type"),
        }
    }
}
impl<'a> ParameterType<'a> {
    fn to_attr_type_str(&'a self, type_map: &mut TypeMap<'a>) -> String {
        match self {
            ParameterType::Generalized(g) => g.to_attr_type_str(type_map),
            ParameterType::Named(e) => type_map.to_rtype_build(e.name()),
            ParameterType::Simple(e) => e.to_attr_type_str().to_owned(),
        }
    }
}
impl<'a> GeneralAggregationTypes<'a> {
    fn to_attr_type_str(&'a self, type_map: &mut TypeMap<'a>) -> String {
        let (optional, param_type) = match self {
            GeneralAggregationTypes::Array(a) => (a.optional, &a.parameter_type),
            GeneralAggregationTypes::Bag(a) => (false,  &a.1),
            GeneralAggregationTypes::List(a) => (false, &a.parameter_type),
            GeneralAggregationTypes::Set(a) => (false, &a.parameter_type),
        };
        let t = param_type.to_attr_type_str(type_map);
        if optional {
            format!("Option<Vec<{}>>", t)
        } else {
            format!("Vec<{}>", t)
        }
    }
}
impl <'a> SimpleTypes<'a> {
    fn to_attr_type_str(&self) -> &str {
        match self {
            SimpleTypes::Binary(_) => "usize",
            SimpleTypes::Boolean => "bool",
            SimpleTypes::Integer => "i64",
            SimpleTypes::Logical => "Logical",
            SimpleTypes::Number => "f64",
            SimpleTypes::Real(_) => "f64",
            SimpleTypes::String(_) => "&'a str",
        }
    }
    fn to_type(&self) -> Type {
        Type::RedeclaredPrimitive(self.to_attr_type_str())
    }
}
impl<'a> ConstructedTypes<'a> {
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        match self {
            ConstructedTypes::Select(e) => e.disambiguate(entity_names),
            _ => (),
        }
    }
}
impl<'a> SelectType<'a> {
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        match &mut self.list_or_extension {
            SelectListOrExtension::List(items) => {
                for t in &mut items.0 {
                    t.disambiguate(entity_names);
                }
            }
            _ => panic!("Nope nope nope"),
        }
    }
}
impl<'a> NamedTypes<'a> {
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        if let NamedTypes::_Ambiguous(r) = self {
            *self = if entity_names.contains(r.0) {
                 NamedTypes::Entity(EntityRef(r.0))
            } else {
                 NamedTypes::Type(TypeRef(r.0))
            };
        }
    }
    fn name(&self) -> &str {
        match self {
            NamedTypes::Entity(e) => e.0,
            NamedTypes::Type(e) => e.0,
            NamedTypes::_Ambiguous(e) => e.0,
        }
    }
}
