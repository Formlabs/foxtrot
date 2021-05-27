use std::collections::{HashSet, HashMap};
use codegen::{Scope};
use crate::parse::*;

////////////////////////////////////////////////////////////////////////////////
// Helper types to use when doing code-gen
struct TypeHead {
    has_lifetime: bool,
}
enum TypeBody<'a> {
    Entity {
        // In order, with parent attributes first
        attrs: Vec<AttributeData<'a>>,
    },
    // These are all TYPE in EXPRESS, but we unpack them here
    Redeclared(&'a str),
    Enum(Vec<&'a str>),
    Select(Vec<&'a str>),
    Aggregation {
        optional: bool,
        type_: Box<Type<'a>>,
    },

    // Direct Rust type
    Primitive(&'a str),
}
struct Type<'a> {
    head: TypeHead,
    body: TypeBody<'a>,
}
struct TypeMap<'a>(HashMap<&'a str, Type<'a>>, &'a HashMap<&'a str, Ref<'a>>);
impl <'a> TypeMap<'a> {
    fn to_rtype(&self, s: &str) -> String {
        let t = self.0.get(s).expect(&format!("Could not get {:?}", s));
        let lifetime = if t.head.has_lifetime {
            "'a"
        } else {
            ""
        };
        match &t.body {
            TypeBody::Entity { .. } => format!("Id<{}{}>", to_camel(s), lifetime),
            TypeBody::Redeclared(_)
            | TypeBody::Enum(_)
            | TypeBody::Select(_)
            | TypeBody::Primitive(_) => format!("{}{}", to_camel(s), lifetime),
            TypeBody::Aggregation { optional, type_ } => if *optional {
                format!("Vec<Option<{}>>", self.to_inner_rtype(type_))
            } else {
                format!("Vec<{}>", self.to_inner_rtype(type_))
            },
        }
    }
    fn to_inner_rtype(&self, t: &Type<'a>) -> String {
        let lifetime = if t.head.has_lifetime {
            "'a"
        } else {
            ""
        };
        match &t.body {
            TypeBody::Aggregation { optional, type_ } => if *optional {
                format!("Vec<Option<{}>>", self.to_inner_rtype(type_))
            } else {
                format!("Vec<{}>", self.to_inner_rtype(type_))
            },
            TypeBody::Redeclared(r) => {
                if self.is_entity(r) {
                    assert!(!t.head.has_lifetime);
                    format!("Id<{}>{}", to_camel(r), lifetime)
                } else {
                    format!("{}{}", to_camel(r), lifetime)
                }
            },
            TypeBody::Primitive(r) => format!("{}", r),

            TypeBody::Entity { .. } | TypeBody::Enum(_) | TypeBody::Select(_) =>
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
    fn is_entity(&self, s: &'a str) -> bool {
        let t = self.0.get(s).expect(&format!("Could not get {:?}", s));
        if let TypeBody::Entity{..} = t.body {
            true
        } else {
            false
        }
    }
    fn has_lifetime(&mut self, s: &'a str) -> bool {
        if !self.0.contains_key(s) {
            self.build(s);
        }
        let t = self.0.get(s).expect(&format!("Could not get {:?}", s));
        t.head.has_lifetime
    }
    fn attributes(&mut self, s: &'a str) -> Vec<AttributeData<'a>> {
        if !self.0.contains_key(s) {
            self.build(s);
        }
        let t = self.0.get(s).expect(&format!("Could not get {:?}", s));
        if let TypeBody::Entity { attrs } = &t.body {
            attrs.clone()
        } else {
            panic!("Cannot get attributes of a non-entity");
        }
    }
}

impl<'a> Type<'a> {
    fn gen(&self, name: &str, scope: &mut Scope, type_map: &TypeMap) {
        let name = to_camel(name);
        match &self.body {
            TypeBody::Redeclared(c) => {
                let t = scope.new_struct(&name);
                if self.head.has_lifetime {
                    t.generic("'a");
                }
                t.tuple_field(type_map.to_rtype(c));
            },
            TypeBody::Enum(c) => {
                let t = scope.new_enum(&name);
                if self.head.has_lifetime {
                    t.generic("'a");
                }
                for v in c {
                    t.new_variant(&to_camel(v));
                }
            },
            TypeBody::Select(c) => {
                let t = scope.new_enum(&name);
                if self.head.has_lifetime {
                    t.generic("'a");
                }
                for v in c {
                    t.new_variant(&to_camel(v))
                        .tuple(&type_map.to_rtype(v));
                }
            },
            TypeBody::Aggregation { .. } => {
                let t = scope.new_struct(&name);
                if self.head.has_lifetime {
                    t.generic("'a");
                }
                t.tuple_field(type_map.to_inner_rtype(self));
            }
            TypeBody::Entity { attrs } => {
                let t = scope.new_struct(&name);
                if self.head.has_lifetime {
                    t.generic("'a");
                }
                for a in attrs {
                    let attr_type = type_map.to_rtype(a.name);
                    if a.optional {
                        t.field(a.name, &format!("Option<{}>", attr_type));
                    } else {
                        t.field(a.name, &attr_type);
                    }
                }
            },
            TypeBody::Primitive(_) => {},
        };
    }
}

#[derive(Clone, Debug)]
struct AttributeData<'a> {
    name: &'a str, // already camel-case
    type_: String,
    optional: bool,
}

////////////////////////////////////////////////////////////////////////////////

// A reference into an existing `Syntax` tree, for convenient random access
enum Ref<'a> {
    Entity(&'a EntityDecl<'a>),
    Type(&'a UnderlyingType<'a>),
}

////////////////////////////////////////////////////////////////////////////////

pub fn gen(s: &mut Syntax) -> String {
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
    type_map.0.insert("usize", Type {
        head: TypeHead { has_lifetime: false },
        body: TypeBody::Primitive("usize"),
    });
    type_map.0.insert("bool", Type {
        head: TypeHead { has_lifetime: false },
        body: TypeBody::Primitive("bool"),
    });
    type_map.0.insert("i64", Type {
        head: TypeHead { has_lifetime: false },
        body: TypeBody::Primitive("i64"),
    });
    type_map.0.insert("f64", Type {
        head: TypeHead { has_lifetime: false },
        body: TypeBody::Primitive("f64"),
    });
    type_map.0.insert("&'a str", Type {
        head: TypeHead { has_lifetime: true },
        body: TypeBody::Primitive("&'a str"),
    });

    for k in ref_map.keys() {
        type_map.build(k);
    }

    // Step four: do codegen on the completed type map
    let mut scope = Scope::new();
    for (k,v) in type_map.0.iter() {
        v.gen(k, &mut scope, &type_map);
    }
    scope.to_string()
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
            UnderlyingType::Constructed(c) => c.to_type(type_map),
        }
    }
}
impl<'a> ConcreteTypes<'a> {
    fn to_type(&self, type_map: &mut TypeMap<'a>) -> Type {
        match self {
            ConcreteTypes::Aggregation(a) => a.to_type(type_map),
            ConcreteTypes::Simple(s) => s.to_type(),
            ConcreteTypes::TypeRef(t) => {
                let has_lifetime = type_map.has_lifetime(t.0);
                Type {
                    head: TypeHead {
                        has_lifetime,
                    },
                    body: TypeBody::Redeclared(t.0),
                }
            },
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
                Type {
                    head: TypeHead { has_lifetime: type_.head.has_lifetime },
                    body: TypeBody::Aggregation { optional,
                        type_: Box::new(type_),
                    }
                }
            },
            InstantiableType::EntityRef(e) => {
                let has_lifetime = type_map.has_lifetime(e.0);
                Type {
                    head: TypeHead { has_lifetime },
                    body: TypeBody::Redeclared(e.0),
                }
            }
        }
    }
}
impl<'a> ConstructedTypes<'a> {
    fn to_type(&'a self, type_map: &mut TypeMap<'a>) -> Type {
        match self {
            ConstructedTypes::Enumeration(e) => e.to_type(),
            ConstructedTypes::Select(s) => s.to_type(type_map),
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
        Type {
            head: TypeHead { has_lifetime: false },
            body: TypeBody::Enum(out)
        }
    }
}
impl<'a> SelectType<'a> {
    fn to_type(&'a self, type_map: &mut TypeMap<'a>) -> Type {
        assert!(!self.extensible, "Cannot handle extensible lists");
        assert!(!self.generic_entity, "Cannot handle generic entity lists");
        match &self.list_or_extension {
            SelectListOrExtension::List(e) => e.to_type(type_map),
            _ => panic!("Extensions not supported"),
        }
    }
}
impl<'a> SelectList<'a> {
    fn to_type(&'a self, type_map: &mut TypeMap<'a>) -> Type {
        let mut out = Vec::new();
        let mut has_lifetime = false;
        for e in &self.0 {
            has_lifetime |= type_map.has_lifetime(e.name());
            out.push(e.name());
        }
        Type {
            head: TypeHead { has_lifetime },
            body: TypeBody::Select(out)
        }
    }
}
impl<'a> EntityDecl<'a> {
    fn to_type(&'a self, type_map: &mut TypeMap<'a>) -> Type<'a> {
        let subsuper = &self.0.1;
        let mut attrs = Vec::new();
        let mut has_lifetime = false;
        if let Some(subs) = &subsuper.1 {
            for sub in subs.0.iter() {
                attrs.extend(type_map.attributes(sub.0).into_iter());
                has_lifetime |= type_map.has_lifetime(sub.0);
            }
        }
        for attr in &self.1.explicit_attr {
            // TODO
        }
        Type {
            head: TypeHead { has_lifetime },
            body: TypeBody::Entity { attrs },
        }
    }
}
impl <'a> SimpleTypes<'a> {
    fn to_type(&self) -> Type {
        let t = match self {
            SimpleTypes::Binary(_) => "usize",
            SimpleTypes::Boolean => "bool",
            SimpleTypes::Integer => "i64",
            SimpleTypes::Logical => "Option<bool>",
            SimpleTypes::Number => "f64",
            SimpleTypes::Real(_) => "f64",
            SimpleTypes::String(_) => "&'a str",
        };
        let has_lifetime = if let SimpleTypes::String(_) = &self {
            true
        } else {
            false
        };
        Type {
            head: TypeHead {
                has_lifetime,
            },
            body: TypeBody::Primitive(t),
        }
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
