use std::collections::HashSet;
use codegen::{Enum, Scope, Struct};
use crate::parse::*;

struct EntityData<'a> {
    name: &'a str,
    inherited_attrs: Vec<AttributeData<'a>>,
    attrs: Vec<AttributeData<'a>>,
}
struct AttributeData<'a> {
    name: &'a str,
    type_: String,
    optional: bool,
}

impl<'a> EntityData<'a> {
    /// Builds an initial `EntityData` object, with name and `attrs` populated
    fn from_entity_decl(d: &'a EntityDecl, entity_names: &HashSet<&str>) -> Self {
        let name = d.0.0.0;
        let attrs = d.1.explicit_attr.iter()
            .flat_map(ExplicitAttr::to_attrs)
            .collect();
        assert!(entity_names.contains(name));
        Self {
            name, attrs, inherited_attrs: vec![],
        }
    }
}
impl<'a> ExplicitAttr<'a> {
    fn to_attrs(&self) -> impl Iterator<Item=AttributeData> + '_ {
        self.attributes.iter().map(move |a|
            AttributeData {
                name: "", // TODO,
                type_: "".to_owned(), // TODO
                optional: self.optional,
            })
    }
}

pub fn gen(s: &mut Syntax) -> String {
    assert!(s.0.len() == 1, "Multiple schemas are unsupported");

    let mut entity_names = HashSet::new();
    s.collect_entity_names(&mut entity_names);

    // Convert ambiguous ids to entity names
    s.disambiguate(&entity_names);

    let mut scope = Scope::new();
    s.gen(&mut scope);
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
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        for v in &mut self.0 {
            v.disambiguate(entity_names);
        }
    }
    fn gen(&self, scope: &mut Scope) {
        for v in &self.0 {
            v.gen(scope);
        }
    }
}
impl<'a> SchemaDecl<'a> {
    fn collect_entity_names(&self, entity_names: &mut HashSet<&'a str>) {
        self.body.collect_entity_names(entity_names);
    }
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        self.body.disambiguate(entity_names)
    }
    fn gen(&self, scope: &mut Scope) {
        self.body.gen(scope);
    }
}
impl<'a> SchemaBody<'a> {
    fn gen(&self, scope: &mut Scope) {
        for d in &self.declarations {
            match d {
                DeclarationOrRuleDecl::Declaration(d) => d.gen(scope),
                DeclarationOrRuleDecl::RuleDecl(_) => (), // no code-gen for rules
            }
        }
    }
    fn collect_entity_names(&self, entity_names: &mut HashSet<&'a str>) {
        for d in &self.declarations {
            match d {
                DeclarationOrRuleDecl::Declaration(d) =>
                    d.collect_entity_names(entity_names),
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
    fn gen(&self, scope: &mut Scope) {
        match self {
            Declaration::Type(d) => d.gen(scope),
            _ => (), // only code-gen for types right now
        }
    }
    fn collect_entity_names(&self, entity_names: &mut HashSet<&'a str>) {
        match self {
            Declaration::Entity(d) => { entity_names.insert(d.0.0.0); },
            _ => (),
        }
    }
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        match self {
            Declaration::Type(d) => d.disambiguate(entity_names),
            _ => (), // only code-gen for types right now
        }
    }
}
impl<'a> TypeDecl<'a> {
    fn gen(&self, scope: &mut Scope) {
        match &self.underlying_type {
            UnderlyingType::Concrete(c) => {
                let mut t = scope.new_struct(&to_camel(self.type_id.0));
                c.gen(&mut t);
            },
            UnderlyingType::Constructed(c) => {
                let mut t = scope.new_enum(&to_camel(self.type_id.0));
                c.gen(&mut t);
            },
        };
    }
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        match &mut self.underlying_type {
            UnderlyingType::Constructed(c) => {
                c.disambiguate(entity_names);
            },
            _ => (),
        }
    }
}
impl<'a> ConcreteTypes<'a> {
    fn gen(&self, s: &mut Struct) {
        match self {
            ConcreteTypes::Aggregation(_) => (), // TODO
            ConcreteTypes::Simple(a) => a.gen(s),
            ConcreteTypes::TypeRef(a) => a.gen(s),
        }
    }
}
impl <'a> SimpleTypes<'a> {
    fn gen(&self, s: &mut Struct) {
        s.tuple_field(match self {
            SimpleTypes::Binary(_) => "usize",
            SimpleTypes::Boolean => "bool",
            SimpleTypes::Integer => "i64",
            SimpleTypes::Logical => "Option<bool>",
            SimpleTypes::Number => "f64",
            SimpleTypes::Real(_) => "f64",
            SimpleTypes::String(_) => "String",
        });
    }
}
impl <'a> TypeRef<'a> {
    fn gen(&self, s: &mut Struct) {
        s.tuple_field(&to_camel(self.0));
    }
}
impl<'a> ConstructedTypes<'a> {
    fn gen(&self, s: &mut Enum) {
        match self {
            ConstructedTypes::Enumeration(e) => e.gen(s),
            ConstructedTypes::Select(e) => e.gen(s),
        }
    }
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        match self {
            ConstructedTypes::Select(e) => e.disambiguate(entity_names),
            _ => (),
        }
    }
}
impl<'a> EnumerationType<'a> {
    fn gen(&self, s: &mut Enum) {
        if let Some(ioe) = &self.items_or_extension {
            match ioe {
                EnumerationItemsOrExtension::Items(items) => {
                    for i in &items.0 {
                        s.new_variant(&to_camel(i.0));
                    }
                }
                _ => panic!("Codegen for extensions not implemented"),
            }
        }
    }
}
impl<'a> SelectType<'a> {
    fn gen(&self, s: &mut Enum) {
        match &self.list_or_extension {
            SelectListOrExtension::List(items) => {
                for t in &items.0 {
                    let name = t.to_camel();
                    let inner = if t.is_entity() {
                        format!("Id<{}>", name)
                    } else {
                        name.clone()
                    };
                    s.new_variant(&name).tuple(&inner);
                }
            }
            _ => panic!("Codegen for extensions not implemented"),
        }
    }
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
impl <'a> NamedTypes<'a> {
    fn disambiguate(&mut self, entity_names: &HashSet<&str>) {
        if let NamedTypes::_Ambiguous(r) = self {
            *self = if entity_names.contains(r.0) {
                 NamedTypes::Entity(EntityRef(r.0))
            } else {
                 NamedTypes::Type(TypeRef(r.0))
            };
        }
    }
    fn is_entity(&self) -> bool {
        match self {
            NamedTypes::Entity(_) => true,
            NamedTypes::Type(_) => false,
            NamedTypes::_Ambiguous(_) => panic!("Ambiguous named type"),
        }
    }
    fn to_camel(&self) -> String {
        match self {
            NamedTypes::Entity(r) => to_camel(r.0),
            NamedTypes::Type(r) => to_camel(r.0),
            NamedTypes::_Ambiguous(_) => panic!("Ambiguous named type"),
        }
    }
}
