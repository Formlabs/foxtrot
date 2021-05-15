import subprocess

#### Formatting functions:

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


#### Spec Data:

direct_parents = {
    'AdvancedBrepShapeRepresentation': ['ShapeRepresentation'],
    'AdvancedFace': ['FaceSurface'],
    'ApplicationContext': [],
    'ApplicationContextElement': [],
    'ApplicationProtocolDefinition': [],
    'Axis2Placement2d': ['Placement'],
    'Axis2Placement3d': ['Placement'],
    'CartesianPoint': ['Point'],
    'Circle': ['Conic'],
    'ClosedShell': ['ConnectedFaceSet'],
    'Colour': [],
    'ColourRgb': ['ColourSpecification'],
    'ColourSpecification': ['Colour'],
    'Conic': ['Curve'],
    'ConnectedFaceSet': ['TopologicalRepresentationItem'],
    'Curve': ['GeometricRepresentationItem'],
    'CylindricalSurface': ['ElementarySurface'],
    'Direction': ['GeometricRepresentationItem'],
    'DimensionalExponents': [],
    'Edge': ['TopologicalRepresentationItem'],
    'EdgeCurve': ['Edge', 'GeometricRepresentationItem'],
    'EdgeLoop': ['Loop', 'Path'],
    'ElementarySurface': ['Surface'],
    'Face': ['TopologicalRepresentationItem'],
    'FaceBound': ['TopologicalRepresentationItem'],
    'FaceSurface': ['Face', 'GeometricRepresentationItem'],
    'FillAreaStyle': ['FoundedItem'],
    'FillAreaStyleColour': [],
    'FoundedItem': [],
    'GeometricRepresentationItem': ['RepresentationItem'],
    'Line': ['Curve'],
    'Loop': ['TopologicalRepresentationItem'],
    'ManifoldSolidBrep': ['SolidModel'],
    'MeasureWithUnit': [],
    'MechanicalDesignGeometricPresentationArea': ['PresentationArea'],
    'NamedUnit': [],
    'OrientedEdge': ['Edge'],
    'Path': ['GeometricRepresentationItem'],
    'Placement': ['GeometricRepresentationItem'],
    'Plane': ['ElementarySurface'],
    'PresentationArea': ['PresentationRepresentation'],
    'PresentationRepresentation': ['Representation'],
    'PresentationStyleAssignment': ['FoundedItem'],
    'Product': [],
    'ProductCategory': [],
    'ProductContext': ['ApplicationContextElement'],
    'ProductDefinition': [],
    'ProductDefinitionContext': ['ApplicationContextElement'],
    'ProductDefinitionFormation': [],
    'ProductDefinitionFormationWithSpecifiedSource': ['ProductDefinitionFormation'],
    'ProductDefinitionShape': ['PropertyDefinition'],
    'ProductRelatedProductCategory': ['ProductCategory'],
    'PropertyDefinition': [],
    'PropertyDefinitionRepresentation': [],
    'Point': ['GeometricRepresentationItem'],
    'Representation': [],
    'RepresentationContext': [],
    'RepresentationItem': [],
    'RepresentationRelationship': [],
    'ShapeDefinitionRepresentation': ['PropertyDefinitionRepresentation'],
    'ShapeRepresentation': ['Representation'],
    'ShapeRepresentationRelationship': ['RepresentationRelationship'],
    'SolidModel': ['GeometricRepresentationItem'],
    'StyledItem': ['RepresentationItem'],
    'Surface': ['GeometricRepresentationItem'],
    'SurfaceStyleUsage': ['FoundedItem'],
    'SurfaceSideStyle': ['FoundedItem'],
    'SurfaceStyleFillArea': ['FoundedItem'],
    'TopologicalRepresentationItem': ['RepresentationItem'],
    'UncertaintyMeasureWithUnit': ['MeasureWithUnit'],
    'ValueRepresentationItem': ['RepresentationItem'],
    'Vector': ['GeometricRepresentationItem'],
    'Vertex': ['TopologicalRepresentationItem'],
    'VertexPoint': ['Vertex', 'GeometricRepresentationItem']
}

get_indirect_pars = lambda x: [] if len(direct_parents[x]) == 0 else direct_parents[x] + [p for par in direct_parents[x] for p in get_indirect_pars(par) ]
indirect_parents = { k: set(get_indirect_pars(k)) for k in direct_parents }

structs = {
    'AdvancedBrepShapeRepresentation': [
        ('name', 'str'),
        ('items', 'Vec<RepresentationItem>'),
        ('context_of_items', 'RepresentationContext')
    ],
    'ApplicationContext': [
        ('application', 'str')
    ],
    'ApplicationContextElement': [
        ('name', 'str'),
        ('frame_of_reference', 'ApplicationContext')
    ],
    'AdvancedFace': [
        ('name', 'str'),
        ('face_geometry', 'Surface'),
        ('same_sense', 'bool')
    ],
    'ApplicationProtocolDefinition': [
        ('name', 'str'),
        ('description', 'Option<str>'),
        ('relating_context', 'ApplicationContext'),
        ('related_context', 'ApplicationContext')
    ],
    'Axis2Placement2d': [
        ('name', 'str'),
        ('location', 'CartesianPoint'),
        ('ref_direction', 'Option<Direction>')
    ],
    'Axis2Placement3d': [
        ('name', 'str'),
        ('location', 'CartesianPoint'),
        ('ref_direction', 'Option<Direction>')
    ],
    'CartesianPoint': [
        ('name', 'str'),
        ('coordinates', 'Vec<LengthMeasure>')
    ],
    'Circle': [
        ('name', 'str'),
        ('position', 'Axis2Placement3d'),
        ('radius', 'PositiveLengthMeasure')
    ],
    'ClosedShell': [
        ('name', 'str'),
        ('cfs_faces', 'Vec<Face>')
    ],
    'Colour': [],
    'ColourRgb': [
        ('name', 'str'),
        ('red', 'float'),
        ('green', 'float'),
        ('blue', 'float')
    ],
    'ColourSpecification': [
        ('name', 'str')
    ],
    'Conic': [
        ('name', 'str'),
        ('position', 'Axis2Placement3d')
    ],
    'ConnectedFaceSet': [
        ('name', 'str'),
        ('cfs_faces', 'Vec<Face>')
    ],
    'Curve': [
        ('name', 'str')
    ],
    'CylindricalSurface': [
        ('name', 'str'),
        ('position', 'Axis2Placement3d'),
        ('radius', 'PositiveLengthMeasure')
    ],
    'Direction': [
        ('name', 'str'),
        ('direction_ratios', 'Vec<float>')
    ],
    'DimensionalExponents': [
        ('length_exponent', 'float'),
        ('mass_exponent', 'float'),
        ('time_exponent', 'float'),
        ('electric_current_exponent', 'float'),
        ('thermodynamic_temperature_exponent', 'float'),
        ('amount_of_substance_exponent', 'float'),
        ('luminous_intensity_exponent', 'float')
    ],
    'Edge': [
        ('name', 'str'),
        ('edge_start', 'Vertex'),
        ('edge_end', 'Vertex')
    ],
    'EdgeCurve': [
        ('name', 'str'),
        ('edge_end', 'Vertex'),
        ('edge_geometry', 'Curve'),
        ('same_sense', 'bool')
    ],
    'EdgeLoop': [
        ('name', 'str')
    ],
    'ElementarySurface': [
        ('name', 'str'),
        ('position', 'Axis2Placement3d')
    ],
    'Face': [
        ('name', 'str'),
        ('bounds', 'Vec<FaceBound>')
    ],
    'FaceBound': [
        ('name', 'str'),
        ('bound', 'Loop'),
        ('orientation', 'bool')
    ],
    'FaceSurface': [
        ('name', 'str'),
        ('face_geometry', 'Surface'),
        ('same_sense', 'bool')
    ],
    'FillAreaStyle': [
        ('name', 'str'),
        ('fill_stypes', 'Vec<FillStyleSelect>')
    ],
    'FillAreaStyleColour': [
        ('name', 'str'),
        ('fill_color', 'Colour')
    ],
    'FoundedItem': [],
    'GeometricRepresentationItem': [
        ('name', 'str')
    ],
    'Line': [
        ('name', 'str'),
        ('pnt', 'CartesianPoint'),
        ('dir', 'Vector')
    ],
    'Loop': [
        ('name', 'str')
    ],
    'ManifoldSolidBrep': [
        ('name', 'str'),
        ('outer', 'ClosedShell')
    ],
    'MeasureWithUnit': [
        ('value_component', 'MeasureValue'),
        ('unit_component', 'Unit')
    ],
    'MechanicalDesignGeometricPresentationArea': [
        ('name', 'str'),
        ('items', 'Vec<RepresentationItem>'),
        ('context_of_items', 'RepresentationContext')
    ],
    'NamedUnit': [
        ('dimensions', 'DimensionalExponents')
    ],
    'OrientedEdge': [
        ('name', 'str'),
        ('edge_start', 'Vertex'),
        ('edge_end', 'Vertex'),
        ('edge_element', 'Edge'),
        ('orientation', 'bool')
    ],
    'Path': [
        ('name', 'str'),
        ('edge_list', 'Vec<OrientedEdge>')
    ],
    'Placement': [
        ('name', 'str'),
        ('location', 'CartesianPoint')
    ],
    'Plane': [
        ('name', 'str'),
        ('position', 'Axis2Placement3d')
    ],
    'PresentationArea': [
        ('name', 'str'),
        ('items', 'Vec<RepresentationItem>'),
        ('context_of_items', 'RepresentationContext')
    ],
    'PresentationRepresentation': [
        ('name', 'str'),
        ('items', 'Vec<RepresentationItem>'),
        ('context_of_items', 'RepresentationContext')
    ],
    'PresentationStyleAssignment': [
        ('styles', 'Vec<PresentationStyleSelect>')
    ],
    'Product': [
        ('id', 'Identifier'),
        ('name', 'str'),
        ('description', 'Option<str>'),
        ('frame_of_reference', 'Vec<ProductContext>')
    ],
    'ProductCategory': [
        ('name', 'str'),
        ('description', 'Option<str>')
    ],
    'ProductContext': [
        ('name', 'str'),
        ('frame_of_reference', 'ApplicationContext'),
        ('discipline_type', 'str')
    ],
    'ProductDefinition': [
        ('id', 'Identifier'),
        ('description', 'Option<str>'),
        ('formation', 'ProductDefinitionFormation'),
        ('frame_of_reference', 'ProductDefinitionContext')
    ],
    'ProductDefinitionContext': [
        ('name', 'str'),
        ('frame_of_reference', 'ApplicationContext'),
        ('life_cycle_stage', 'str')
    ],
    'ProductDefinitionFormation': [
        ('id', 'Identifier'),
        ('description', 'Option<str>'),
        ('of_product', 'Product')
    ],
    'ProductDefinitionFormationWithSpecifiedSource': [
        ('id', 'Identifier'),
        ('description', 'Option<str>'),
        ('of_product', 'Product'),
        ('make_or_buy', 'Source')
    ],
    'ProductDefinitionShape': [
        ('name', 'str'),
        ('description', 'Option<str>'),
        ('definition', 'CharacterizedDefinition')
    ],
    'ProductRelatedProductCategory': [
        ('name', 'str'),
        ('description', 'Option<str>'),
        ('products', 'Vec<Product>')
    ],
    'PropertyDefinition': [
        ('name', 'str'),
        ('description', 'Option<str>'),
        ('definition', 'CharacterizedDefinition')
    ],
    'PropertyDefinitionRepresentation': [
        ('definition', 'RepresentedDefinition'),
        ('used_representation', 'Representation')
    ],
    'Point': [
        ('name', 'str')
    ],
    'Representation': [
        ('name', 'str'),
        ('items', 'Vec<RepresentationItem>'),
        ('context_of_items', 'RepresentationContext')
    ],
    'RepresentationContext': [
        ('context_identifier', 'Identifier'),
        ('context_type', 'str')
    ],
    'RepresentationItem': [
        ('name', 'str')
    ],
    'RepresentationRelationship': [
        ('name', 'str'),
        ('description', 'Option<str>'),
        ('rep_1', 'Representation'),
        ('rep_2', 'Representation')
    ],
    'ShapeDefinitionRepresentation': [
        ('definition', 'RepresentedDefinition'),
        ('used_representation', 'Representation')
    ],
    'ShapeRepresentation': [
        ('name', 'str'),
        ('items', 'Vec<RepresentationItem>'),
        ('context_of_items', 'RepresentationContext')
    ],
    'ShapeRepresentationRelationship': [
        ('name', 'str'),
        ('description', 'Option<str>'),
        ('rep_1', 'Representation'),
        ('rep_2', 'Representation')
    ],
    'SolidModel': [
        ('name', 'str')
    ],
    'StyledItem': [
        ('name', 'str'),
        ('styles', 'Vec<PresentationStyleAssignment>'),
        ('item', 'RepresentationItem')
    ],
    'Surface': [
        ('name', 'str')
    ],
    'SurfaceStyleUsage': [
        ('side', 'SurfaceSide'),
        ('style', 'SurfaceSideStyleSelect')
    ],
    'SurfaceSideStyle': [
        ('name', 'str'),
        ('styles', 'Vec<SurfaceStyleElementSelect>')
    ],
    'SurfaceStyleFillArea': [
        ('fill_area', 'FillAreaStyle')
    ],
    'TopologicalRepresentationItem': [
        ('name', 'str')
    ],
    'UncertaintyMeasureWithUnit': [
        ('value_component', 'MeasureValue'),
        ('unit_component', 'Unit'),
        ('name', 'str'),
        ('description', 'Option<str>')
    ],
    'ValueRepresentationItem': [
        ('name', 'str'),
        ('value_component', 'MeasureValue')
    ],
    'Vector': [
        ('name', 'str'),
        ('orientation', 'Direction'),
        ('magnitude', 'LengthMeasure')
    ],
    'Vertex': [
        ('name', 'str')
    ],
    'VertexPoint': [
        ('name', 'str'),
        ('vertex_geometry', 'Point')
    ]
}


# TYPE IfcFillStyleSelect = SELECT
# (   IfcFillAreaStyleHatching,
# IfcFillAreaStyleTiles,
# IfcColour,
# IfcExternallyDefinedHatchStyle);

# validate
types = list(set(structs)) + ['str', 'float', 'bool']
types += ["LengthMeasure", "AreaMeasure", "PositiveLengthMeasure", "MeasureValue", "Unit", "FillStyleSelect", "PresentationStyleSelect", "Identifier",
          "Source", "SurfaceSide", "CharacterizedDefinition", "RepresentedDefinition", "SurfaceSideStyleSelect", "SurfaceStyleElementSelect"]
types = types + ["Vec<{}>".format(s) for s in types] + ["Option<{}>".format(s) for s in types]

for name, fields in structs.items():
    for fname, ftype in fields:
        if ftype not in types:
            raise ValueError(ftype)



data_entity = {
    "AdvancedBrepShapeRepresentation": ["str", "vec_id", "id"],
    "AdvancedFace": ["str", "vec_id", "id", "bool"],
    "ApplicationContext": ["str"],
    "ApplicationProtocolDefinition": ["str", "str", "usize", "id"],
    "Axis2Placement3d": ["str", "id", "id", "id"],
    "BrepWithVoids": ["str", "id", "vec_id"],
    "BSplineCurveWithKnots": ["str", "usize", "vec_id", "BSplineEnum1", "bool", "bool", "vec_usize", "vec_float", "BSplineEnum2"],
    "BSplineSurfaceWithKnots": ["str", "usize", "usize", "vec_vec_id", "BSplineEnum1", "bool", "bool", "bool", "vec_usize", "vec_usize", "vec_float", "vec_float", "BSplineEnum2"],
    "CartesianPoint": ["str", "vec_float"],
    "Circle": ["str", "id", "float"],
    "ClosedShell": ["str", "vec_id"],
    "ColourRgb": ["str", "float", "float", "float"],
    "ConicalSurface": ["str", "id", "float", "float"],
    "ContextDependentShapeRepresentation": ["id", "id"],
    "CurveStyle": ["str", "id", "PositiveLengthMeasure", "id"],
    "CylindricalSurface": ["str", "id", "float"],
    "DescriptiveRepresentationItem": ["str", "str"],
    "DraughtingPreDefinedColour": ["str"],
    "DraughtingPreDefinedCurveFont": ["str"],
    "DerivedUnit": ["vec_id"],
    "DerivedUnitElement": ["id", "float"],
    "Direction": ["str", "vec_float"],
    "Ellipse": ["str", "id", "float", "float"],
    "EdgeCurve": ["str", "id", "id", "id", "bool"],
    "EdgeLoop": ["str", "vec_id"],
    "FaceBound": ["str", "id", "bool"],
    "FillAreaStyle": ["str", "vec_id"],
    "FillAreaStyleColour": ["str", "id"],
    "GeometricCurveSet": ["str", "vec_id"],
    "ItemDefinedTransformation": ["str", "str", "id", "id"],
    "Line": ["str", "id", "id"],
    "ManifoldSolidBrep": ["str", "id"],
    "ManifoldSurfaceShapeRepresentation": ["str", "vec_id", "id"],
    "MeasureRepresentationItem": ["str", "AreaMeasureOrVolumeMeasure", "id"],
    "MechanicalDesignGeometricPresentationRepresentation": ["str", "vec_id", "id"],
    "NextAssemblyUsageOccurrence": ["str", "str", "str", "id", "id", "opt_str"],
    "OpenShell": ["str", "vec_id"],
    "OrientedEdge": ["str", "*", "*", "id", "bool"],
    "OrientedClosedShell": ["str", "*", "id", "bool"],
    "OverRidingStyledItem": ["str", "vec_id", "id", "id"],
    "Plane": ["str", "id"],
    "PresentationLayerAssignment": ["str", "str", "vec_id"],
    "PresentationStyleAssignment": ["vec_id"],
    "PresentationStyleByContext": ["vec_id", "id"],
    "Product": ["str", "str", "str", "vec_id"],
    "ProductCategory": ["str", "str"],
    "ProductContext": ["str", "id", "str"],
    "ProductDefinition": ["str", "str", "id", "id"],
    "ProductDefinitionContext": ["str", "id", "str"],
    "ProductDefinitionFormation": ["str", "str", "id"],
    "ProductDefinitionFormationWithSpecifiedSource": ["str", "str", "id", "Source"],
    "ProductDefinitionShape": ["str", "str", "id"],
    "ProductRelatedProductCategory": ["str", "opt_str", "vec_id"],
    "PropertyDefinition": ["str", "str", "id"],
    "PropertyDefinitionRepresentation": ["id", "id"],
    "Representation": ["opt_str", "vec_id", "opt_id"],
    "ShellBasedSurfaceModel": ["str", "vec_id"],
    "SphericalSurface": ["str", "id", "float"],
    "ShapeAspect": ["str", "str", "id", "bool"],
    "ShapeDefinitionRepresentation": ["id", "id"],
    "ShapeRepresentation": ["str", "vec_id", "id"],
    "ShapeRepresentationRelationship": ["str", "str", "id", "id"],
    "StyledItem": ["str", "vec_id", "id"],
    "SurfaceStyleUsage": ["SurfaceSide", "id"],
    "SurfaceSideStyle": ["str", "vec_id"],
    "SurfaceStyleFillArea": ["id"],
    "SurfaceOfLinearExtrusion": ["str", "id", "id"],
    "TrimmedCurve": ["str", "id", "pair_id_ParameterValue", "pair_id_ParameterValue", "bool", "TrimmedCurveEnum"],
    "ToroidalSurface": ["str", "id", "float", "float"],
    "UncertaintyMeasureWithUnit": ["LengthMeasure", "id", "str", "str"],
    "ValueRepresentationItem": ["str", "CountMeasure"],
    "Vector": ["str", "id", "float"],
    "VertexLoop": ["str", "id"],
    "VertexPoint": ["str", "id"]
}

print (sorted(list(set(data_entity).difference(set(structs)))))


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

generate_delegation_example()
