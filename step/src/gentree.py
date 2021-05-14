
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
