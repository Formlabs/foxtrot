
import re
import random

def camel_to_snake(s):
    if s == "Axis2Placement3d": return "AXIS2_PLACEMENT_3D"
    return re.sub(r'(?<!^)(?=[A-Z])', '_', s).lower()

def chunks(lst, n):
    """Yield successive n-sized chunks from lst. Try not to yeild chunks of size 1"""
    if n > 1 and len(lst) > n and len(lst) % n == 1:
        yield lst[: n - 1]
        for i in range(n - 1, len(lst), n):
            yield lst[i:i + n]
    else:
        for i in range(0, len(lst), n):
            yield lst[i:i + n]


type_mp = {
    "pair_id_ParameterValue": "(Id, ParameterValue)",
    "str" : "String",
    "id" : "Id",
    "float" : "f64",
    "bool" : "bool",
    "usize" : "i32",
    "opt_id": "Option<Id>",
    "opt_str": "Option<String>",
    "vec_id": "Vec<Id>",
    "vec_vec_id": "Vec<Vec<Id>>",
    "vec_float": "Vec<f64>",
    "vec_usize" : "Vec<i32>"
}
# pf_mp = {
#     "pair_id_ParameterValue": 'delimited(tag("("), tuple((after_ws(step_id), after_wscomma(step_stf_parameter_value))), after_ws(tag(")")))',
#     "*" : "tag(\"*\")",
#     "id" : "step_id",
#     "str" : "step_string",
#     "float" : "step_float",
#     "bool" : "step_bool",
#     "usize" : "step_udecimal",
#     "opt_id": "step_opt(step_id)",
#     "opt_str": "step_opt(step_string)",
#     "vec_id": "step_vec(step_id)",
#     "vec_vec_id": "step_vec(step_vec(step_id))",
#     "vec_float": "step_vec(step_float)",
#     "vec_usize": "step_vec(step_udecimal)"
# }

pf_mp = {
    "pair_id_ParameterValue": 'read_pair_id_ParameterValue',
    "*" : "Parser::read_star",
    "id" : "Parser::read_id",
    "str" : "Parser::read_string",
    "float" : "Parser::read_float",
    "bool" : "Parser::read_bool",
    "usize" : "Parser::read_int",
    "opt_id": "Parser::read_id_or_dollar",
    "opt_str": "Parser::read_string_or_dollar",
    "vec_id": "Parser::read_id_vector",
    "vec_vec_id": "Parser::read_id_vector_vector",
    "vec_float": "Parser::read_float_vector",
    "vec_usize": "Parser::read_int_vector"
}

o = open('generated.rs', 'w')
o.write("""
use crate::hparser::{{Parser, Id}};
""".format())

o.write("""
pub fn read_pair_id_ParameterValue(parser: &mut Parser) -> (Id, ParameterValue) {{
    read_tuple_2(parser, Parser::read_id, read_stf_parameter_value)
}}
""".format())

o.write("pub fn read_tuple_0(parser: &mut Parser) -> () {{\n        parser.read_open_paren(); parser.skip_whitespace(); parser.read_close_paren();\n     \n}}\n\n".format())
for n in [1, 2, 3, 4, 5, 6, 7, 9, 13]:
    o.write("pub fn read_tuple_{n}<{Ts}, {Fs}>(parser: &mut Parser, {funcs}) -> ({Ts}) where {wheres} {{\n        parser.read_open_paren();\n{parser}\n        parser.read_close_paren();\n     ({pack})\n}}\n\n".format(
        n=n,
        Ts=", ".join("T{i}".format(i=i) for i in range(n)),
        Fs=", ".join("F{i}".format(i=i) for i in range(n)),
        funcs=", ".join("func{i}: F{i}".format(i=i) for i in range(n)),
        wheres=", ".join("F{i}: Fn(&mut Parser) -> T{i}".format(i=i) for i in range(n)),
        parser="\nparser.read_comma();\n".join("parser.skip_whitespace();\nlet x{i} = func{i}(parser);\nparser.skip_whitespace();".format(i=i) for i in range(n)),
        pack=", ".join("x{i}".format(i=i) for i in range(n))
    ))


# # add strongly typed floats

strongly_typed_floats = {
    "LengthMeasure", "CountMeasure", "PositiveLengthMeasure", "AreaMeasure", "VolumeMeasure", "ParameterValue",
}

STRONGLY_TYPED_FLOAT_TEMPLATE = """
#[derive(Debug, PartialEq)]
pub struct {cname}(pub f64);
pub fn read_stf_{lname}(parser: &mut Parser) -> {cname} {{
    let (s, v) = parser.read_united_float();
    if s != "{uname}" {{ panic!("unexpected iden"); }}
    {cname}(v)
}}
"""

for name in strongly_typed_floats:
    o.write(STRONGLY_TYPED_FLOAT_TEMPLATE.format(cname=name, uname=camel_to_snake(name).upper(), lname=camel_to_snake(name).lower()))
    type_mp[name] = name
    pf_mp[name] = "read_stf_{lname}".format(lname = camel_to_snake(name).lower())
    


# # add strongly typed floats combinations as enums

strongly_typed_enum_combination = {
    "AreaMeasureOrVolumeMeasure" : [ "AreaMeasure", "VolumeMeasure" ]
}

STRONGLY_TYPED_ENUM_COMBINATION_TEMPLATE = """
pub enum {cname} {{ {enum_vals} }}
pub fn step_c_{lname}(parser: &mut Parser) -> {cname} {{
    let (s, v) = parser.read_united_float();
    match &s[..] {{
        {match_options},
        _ => panic!("unexpected string")
    }}
}}
"""

for name, vals in strongly_typed_enum_combination.items():
    o.write(STRONGLY_TYPED_ENUM_COMBINATION_TEMPLATE.format(
        cname=name,
        lname = camel_to_snake(name).lower(),
        enum_vals = ", ".join(["{cval}({cval})".format(cval=val) for val in vals]),
        # tag_vals = ", ".join(["tag(\"{uval}\")".format(uval=camel_to_snake(val).upper()) for val in vals]),
        match_options = ",\n".join(["\"{uval}\" => {cname}::{cval}({cval}(v))".format(cname=name, cval=val, uval=camel_to_snake(val).upper()) for val in vals])
    ))
    type_mp[name] = name
    pf_mp[name] = "step_c_{lname}".format(lname = camel_to_snake(name).lower())
    


# # add enums

enums = {
    "SurfaceSide": ["Positive", "Negative", "Both"],
    "Source": ["Made", "Bought", "NotKnown"],
    "BSplineEnum1": ["Unspecified", "WeDontSupportOneElmentEnumsYet"],
    "BSplineEnum2": ["PiecewiseBezierKnots", "Unspecified", "QuasiUniformKnots"],
    "TrimmedCurveEnum": ["Parameter", "WeDontSupportOneElmentEnumsYet"],
}
ENUM_TEMPLATE = """
pub enum {cname} {{ {enum_vals} }}
pub fn read_enum_{lname}(parser: &mut Parser) -> {cname} {{
    let s = parser.read_literal();
    match &s[..] {{
        {remaps},
        _ => panic!("unexpected string")
    }}
}}\n"""

for name, vals in enums.items():
    o.write(ENUM_TEMPLATE.format(
        cname = name,
        enum_vals = ", ".join(vals),
        lname = camel_to_snake(name).lower(),
        # tag_options = ", ".join([
        #     "tag(\"{uval}\")".format(uval=camel_to_snake(val).upper()) for val in vals
        # ]),
        remaps = ", ".join([
            "\"{uval}\" => {cname}::{cval}".format(uval = camel_to_snake(val).upper(), cname=name, cval=val) for val in vals
        ])))
    type_mp[name] = name
    pf_mp[name] = "read_enum_{lname}".format(lname = camel_to_snake(name).lower())
    


# generate DataEntity parsers

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
    "VertexPoint": ["str", "id"]
}

data_entity = sorted(list(data_entity.items()))

# # generate big enum to hold values

o.write("""
pub enum DataEntity {{
    ComplexBucketType,
{types}
}}
""".format(
    types=", ".join([
        "    {cname}({args})".format(
            cname=name,
            args=", ".join([
                type_mp[tp] for tp in tps if tp != "*"
            ])
        )
        for name, tps in data_entity
    ])
))



o.write("""
pub fn parse_data_func(iden: &str, parser: &mut Parser) -> DataEntity {
    use DataEntity::*;
    parser.skip_whitespace();
    match iden {
""")

for name, tps in data_entity:
    o.write("        \"{uname}\" => {{\n            let {unpack} = read_tuple_{n}( parser, {parsers} );\n            {cname}({pack})\n        }}\n".format(
        cname=name,
        uname=camel_to_snake(name).upper(),
        n=len(tps),
        unpack=("({})" if sum(t != "*" for t in tps) > 1 else "{}").format(", ".join("x" + str(i) if t != "*" else "_" for i, t in enumerate(tps))),
        pack=", ".join("x" + str(i) for i, t in enumerate(tps) if t != "*"),
        parsers=", ".join("{}".format(pf_mp[t]) for t in tps)
    ))

o.write(""",
        _ => panic!("unexpected string")
    }
}
""")


# # generate parser for each entity

# DATA_ENTITY_FUNCS_TEMPLATE = """
# fn data_entity_{lname}(input: &str) -> Res<&str, DataEntity> {{
#     named_func(
#         "{lname}", "{uname}",
#         {parser},
#     )(input) .map(|(next_input, res)| (next_input, {{
#         let {unpack} = res;
#         DataEntity::{cname}({pack})
#     }}))
# }}
# """

# for name, tps in data_entity:
#     o.write(DATA_ENTITY_FUNCS_TEMPLATE.format(
#         lname=camel_to_snake(name).lower(), uname = camel_to_snake(name).upper(), cname = name,
#         unpack=("({})" if sum(t != "*" for t in tps) > 1 else "{}").format(", ".join("x" + str(i) if t != "*" else "_" for i, t in enumerate(tps))),
#         pack=", ".join("x" + str(i) for i, t in enumerate(tps) if t != "*"),
#         parser="after_ws({})".format(pf_mp[tps[0]]) if len(tps) == 1 else
#                 "tuple(({}))".format(", ".join(["after_ws({})".format(pf_mp[tps[0]])] + ["after_wscomma({})".format(pf_mp[t]) for t in tps[1:]]))
#     ))


# # write functions which gather data entitys together

# o.write("""
# pub fn data_entity_complex_bucket_type(input: &str) -> Res<&str, DataEntity> {
#     terminated( paren_tup, after_ws(tag(";")) ) (input).map(|(next_input, res)| (next_input, DataEntity::ComplexBucketType) )
# }
# """)

# o.write("""
# fn data_entity(input: &str) -> Res<&str, DataEntity> {{
#     alt((
# {outer_options}
#     ))(input)
# }}
# """.format(outer_options=",\n".join([
#         "        alt(( {inner_options} ))".format(inner_options=", ".join(chunk))
#         for chunk in chunks(
#             ["data_entity_complex_bucket_type"] + 
#             ["data_entity_{lname}".format(lname=camel_to_snake(name).lower()) for name, _ in data_entity],
#             14
#         )
#     ])
# ))

# o.write("""
# fn data_line(input: &str) -> Res<&str, (Id, DataEntity)> {
#     tuple((
#         after_ws(step_id), after_ws(tag("=")), after_ws(data_entity)
#     ))(input) .map(|(next_input, res)| (next_input, {
#         let (id, _, ent) = res;
#         (id, ent)
#     }))
# }
# """)

# o.write("""
# pub fn data_block(input: &str) -> Res<&str, Vec<(Id, DataEntity)>> {
#     many0(
#         after_ws(data_line)
#     )(input)
# }
# """)


# # generate test cases

# escape = lambda x: x.replace("\n", "\\n").replace("\"", "\\\"")

# step_files_for_tests_as_str = open('/Users/Henry Heffan/Desktop/foxtrot/KondoMotherboard_RevB_full.step').read() + "\n\n" + open('/Users/Henry Heffan/Desktop/foxtrot/HOLEWIZARD.step').read()


# test_cases_for_entity = {}
# for name, _ in data_entity:
#     m = re.findall(" = " + camel_to_snake(name).upper() + "\\([^;]*;", step_files_for_tests_as_str)
#     seed = 3242562
#     random.Random(seed).shuffle(m)  # make it deterministic so reruning doesnt confuse git
#     test_cases_for_entity[name] = m

# m = re.findall(";\n#\\d* =\\s*[^( ][^;]*;", step_files_for_tests_as_str)
# seed = 3242562
# random.Random(seed).shuffle(m)  # make it deterministic so reruning doesnt confuse git
# line_test_cases = m

# max_num_individual_tests = 25
# max_line_tests_cases = 500

# o.write(
# """
# #[cfg(test)]
# mod tests {{
#     use super::*;

# {individual_tests}

# {line_test}

# }}
# """.format(
#     individual_tests="\n\n".join([
#         "    #[test]\n    fn test_{name}() {{\n{tests}\n    }}".format(
#             name = camel_to_snake(name).lower(),
#             tests = "\n".join(["        assert!(data_entity_{lname}(\"{text}\").is_ok());".format(
#                 lname = camel_to_snake(name).lower(), text = escape(text[3:])) for text in test_cases_for_entity[name][:min(len(m), max_num_individual_tests)]
#             ])
#         )
#         for name, _ in data_entity
#     ]),
#     line_test="    #[test]\n    fn test_data_line() {{\n{tests}\n    }}".format(
#         name = camel_to_snake(name).lower(),
#         tests = "\n".join(["        assert!(data_line(\"{text}\").is_ok());".format(
#             lname = camel_to_snake(name).lower(), text = escape(text[1:].strip())) for text in line_test_cases[:min(len(m), max_line_tests_cases)]
#         ])
#     )
# ))

# # m = re.findall(";\n#\\d* = [^;]*;", step_files_for_tests_as_str)
# # seed = 3242562
# # random.Random(seed).shuffle(m)  # make it deterministic so reruning doesnt confuse git
# # max_num_tests = 200
# # tests = m[:min(len(m), max_num_tests)]
# # o.write("    #[test]\n    fn test_data_block() {{\n{tests}\n    }}\n\n".format(
# #     name = camel_to_snake(name).lower(),
# #     tests = "\n".join([
# #         "        let block = data_line(\"{text}\"));\n        assert!(block.is_ok());\n        assert!(block?.1.len() == 20);".format(
# #             lname = camel_to_snake(name).lower(),
# #             text = "\\n".join(escape(t) for t in test)
# #             )
# #         for test in chunks(tests, 20)
# #     ])
# # ))


# o.close()

