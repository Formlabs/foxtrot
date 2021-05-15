import subprocess


def format_fn_signature(name, arg_list, return_type):
    return "fn {name}({args}) -> {return_type}".format(
        name=name,
        args=", ".join(arg_list),
        return_type=return_type
    )

def format_fn(signature, body_list):
    template = """
{signature} {{
  {body}
}}"""
    return template.format(signature=signature, body="\n  ".join(body_list))


def format_entity_struct(struct_name, entity_fields_list, ancestor_dict) -> str:
    """
    ancestor_dict: dict from the name of a direct parent to the names of all ancestors of that
    parent (in no particular order)
    """
    template = """
#[derive(Clone, Debug, {maybe_delegate})]
{delegations}
pub struct {struct_name} {{
  {parent_data_fields}
  {entity_fields}
}}"""

    entity_fields="\n  ".join(["pub {}: {},".format(field_name, type_name)
                             for (field_name, type_name) in entity_fields_list])
    parent_data_fields="\n  ".join(["pub {}: {},".format("parent_{}".format(name.lower()), "{}Data".format(name))
                                  for name in ancestor_dict.keys()])

    delegation_template = '#[delegate({ancestor}Trait, target = "parent_{parent}")]'
    delegation_list = []
    # We need to avoid the "diamond problem" when two parents are both subtypes of the same
    # ancestor: we should only try to derive that ancestor's trait once, through one of the parents.
    ancestors_already_inherited_from = set()

    for (parent, ancestors) in ancestor_dict.items():
        for ancestor in [parent] + ancestors:
            if not ancestor in ancestors_already_inherited_from:
                delegation_list.append(delegation_template.format(ancestor=ancestor, parent=parent.lower()))
                ancestors_already_inherited_from.add(ancestor)

    maybe_delegate = "Delegate" if delegation_list else ""
    return template.format(
        delegations="\n".join(delegation_list),
        struct_name=struct_name,
        entity_fields=entity_fields,
        parent_data_fields=parent_data_fields,
        maybe_delegate=maybe_delegate
    )


def format_entity_trait(entity_name, entity_fields_list):
    template = """
#[delegatable_trait]
pub trait {entity_name}Trait {{
  {methods}
}}"""
    methods = "\n  ".join([format_fn_signature(field_name, ["&self"], "&"+type_name)+";"
                           for (field_name, type_name) in entity_fields_list])
    return template.format(entity_name=entity_name, methods=methods)


def format_entity_trait_impl(entity_name, struct_name, entity_fields_list):
    template = """
impl {entity_name}Trait for {struct_name} {{
    {getters}
}}
"""
    getters = "\n".join([format_fn(format_fn_signature(field_name, ["&self"], "&"+type_name),
                                   ["&self.{}".format(field_name)])
                         for (field_name, type_name) in entity_fields_list])
    return template.format(entity_name=entity_name, struct_name=struct_name, getters=getters)


def format_entity_enum(entity_name, direct_children_list, ancestor_list):
    template = """
#[derive(Clone, Debug, Delegate)]
{delegations}
pub enum {entity_name} {{
    {variants}
}}
"""
    delegation_template = "#[delegate({ancestor}Trait)]"
    delegation_list = [delegation_template.format(ancestor=ancestor) for ancestor in ancestor_list]
    delegation_list.append(delegation_template.format(ancestor=entity_name))

    variants = "\n  ".join(["{child}({child}),".format(child=child) for child in direct_children_list])
    return template.format(entity_name=entity_name, variants=variants, delegations="\n".join(delegation_list))


def format_entity(entity_name, entity_fields_list, ancestor_dict, ancestor_list, direct_children_list):
    blocks = [""]
    blocks.append("// Types for {entity_name} entity:".format(entity_name=entity_name))
    if direct_children_list:
        # "non-leaf" entities with subtypes are represented by an enum with a variant for each of
        # their children. Their fields are defined on a "*Data" struct.
        struct_name = "{}Data".format(entity_name)
    else:
        # "leaf" entities with no subtypes don't need an enum, they're just represented by structs with the same name as the entity.
        struct_name = entity_name

    blocks.append(format_entity_struct(struct_name, entity_fields_list, ancestor_dict))
    blocks.append(format_entity_trait(entity_name, entity_fields_list))
    blocks.append(format_entity_trait_impl(entity_name, struct_name, entity_fields_list))

    if direct_children_list:
        blocks.append(format_entity_enum(entity_name, direct_children_list, ancestor_list))
    return "\n".join(blocks)


def format_lookup_method(non_abstract_entities, direct_parents_dict):
    template = """
pub (crate) fn lookup_autogen<T>(id: &Id<T>, storage: &HashMap<usize, Box<dyn Any>>) -> Option<T>
where
    T: Any + Clone,
{{
  let dynamic_entity: &Box<dyn Any> = storage.get(&id.raw())?;

  let entity_type_id = (&**dynamic_entity).type_id();
  let requested_type_id = TypeId::of::<T>();

  {outer_if_block}
}}
"""

    outer_if_template = """
  if entity_type_id == TypeId::of::<{entity}>() {{
    {outer_if_body}
  }}
"""

    outer_if_block = []
    for entity in non_abstract_entities:
        outer_body = []
        outer_body.append("""let static_entity = dynamic_entity.downcast_ref::<{entity}>().expect("downcasting failed");""".format(entity=entity))

        inner_ifs = []
        inner_if_template = """
  if requested_type_id == TypeId::of::<{ancestor}>() {{
    {inner_if_body}
  }}
"""
        same_type_inner_if_body = "Some(dynamic_cast::<{entity}, T>(static_entity).to_owned())".format(entity=entity)
        inner_ifs.append(inner_if_template.format(ancestor=entity, inner_if_body=same_type_inner_if_body))

        already_used_ancestors = set()
        for path in get_paths_to_roots(entity, direct_parents_dict):
            for start_index in range(len(path)-1):
                requested_ancestor = path[start_index]
                if requested_ancestor in already_used_ancestors:
                    continue
                already_used_ancestors.add(requested_ancestor)
                enum_wrapper = wrap_in_enums("static_entity.to_owned()", path[start_index:])
                inner_if_body_template = """
    let static_entity = {enum_wrapper};
    Some(dynamic_cast::<{ancestor}, T>(&static_entity).to_owned())
"""
                inner_if_body = inner_if_body_template.format(enum_wrapper=enum_wrapper, ancestor=requested_ancestor)
                inner_ifs.append(inner_if_template.format(ancestor=requested_ancestor, inner_if_body=inner_if_body))
        inner_ifs.append("{ None }")
        outer_body.append(" else ".join(inner_ifs))
        outer_if_block.append(outer_if_template.format(outer_if_body="\n".join(outer_body), entity=entity))
    outer_if_block.append("{ None }")
    return template.format(entity=entity, outer_if_block=" else ".join(outer_if_block))

def wrap_in_enums(wrapped, hierarchy):
    if len(hierarchy) == 1:
        return wrapped
    new_wrapped = "{parent_name}::{entity_name}({wrapped})".format(
        parent_name=hierarchy[-2], entity_name=hierarchy[-1], wrapped=wrapped)
    return wrap_in_enums(new_wrapped, hierarchy[:-1])


def get_paths_to_roots(entity, direct_parents_dict):
    if not direct_parents_dict[entity]:
        return [[entity]]
    out = []
    for parent in direct_parents_dict[entity]:
        for path in get_paths_to_roots(parent, direct_parents_dict):
            out.append(path + [entity])
    return out



#### Demonstrate formatting functions:


def generate_delegation_example():
    # A bunch of these dicts can be derived from each other, but I decided to write them all out anyway
    direct_parents = {
        "Animal" : [],
        "Mammal" : ["Animal"],
        "ExtraCategory": [],
        "Dog" : ["Mammal", "ExtraCategory"],
        "Cat" : ["Mammal"],
        "VetRecord" : [],
    }
    indirect_parents = {
        "Animal" : [],
        "Mammal" : ["Animal"],
        "ExtraCategory": [],
        "Dog" : ["Mammal", "ExtraCategory", "Animal"],
        "Cat" : ["Mammal", "Animal"],
        "VetRecord" : [],
    }
    direct_children = {
        "Animal" : ["Mammal"],
        "Mammal" : ["Dog", "Cat"],
        "ExtraCategory": ["Dog"],
        "Dog" : [],
        "Cat" : [],
        "VetRecord" : [],
    }
    fields = {
        "Animal" : [("name", "String")],
        "Mammal" : [("hair_type", "String")],
        "ExtraCategory": [("extra", "usize")],
        "Dog" : [("dog_breed", "String")],
        "Cat" : [("cat_breed", "String")],
        "VetRecord" : [("diagnosis", "String"),
                       ("patient", "Id<Mammal>")],
    }
    non_abstract_entities = ["Dog", "Cat", "VetRecord"]

    EXAMPLE_AUTOGEN_FILENAME= "delegation_example_autogen.rs"
    with open(EXAMPLE_AUTOGEN_FILENAME, "w") as f:
        f.write("""
// This file was autogenerated, do not modify
use ambassador::{delegatable_trait, Delegate};
use std::collections::HashMap;
use std::any::{Any, TypeId};
use crate::id::{Id, dynamic_cast};
""")
        for entity in direct_parents.keys():
            ancestor_dict = {}
            for parent in direct_parents[entity]:
                ancestor_dict[parent] =  indirect_parents[parent]
            f.write(format_entity(entity, fields[entity], ancestor_dict, indirect_parents[entity], direct_children[entity]))
        f.write(format_lookup_method(non_abstract_entities, direct_parents))
    subprocess.run(["rustfmt", EXAMPLE_AUTOGEN_FILENAME])


if __name__ == "__main__":
    generate_delegation_example()
