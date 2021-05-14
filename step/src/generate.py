
import subprocess
import re
import random


def camel_to_snake(s):
    if s == "Axis2Placement3d":
        return "AXIS2_PLACEMENT_3D"
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
    "str": "&'a str",
    "id": "Id",
    "float": "f64",
    "bool": "bool",
    "usize": "usize",
    "opt_id": "Option<Id>",
    "opt_str": "Option<&'a str>",
    "vec_id": "Vec<Id>",
    "vec_vec_id": "Vec<Vec<Id>>",
    "vec_float": "Vec<f64>",
    "vec_usize": "Vec<usize>"
}
pf_mp = {
    "pair_id_ParameterValue": 'delimited(tag("("), tuple((after_ws(step_id), after_wscomma(step_stf_parameter_value))), after_ws(tag(")")))',
    "*": "tag(\"*\")",
    "id": "step_id",
    "str": "step_string",
    "float": "step_float",
    "bool": "step_bool",
    "usize": "step_udecimal",
    "opt_id": "step_opt(step_id)",
    "opt_str": "step_opt(step_string)",
    "vec_id": "step_vec(step_id)",
    "vec_vec_id": "step_vec(step_vec(step_id))",
    "vec_float": "step_vec(step_float)",
    "vec_usize": "step_vec(step_udecimal)"
}

AP214_AUTOGEN_FILENAME = 'ap214_autogen.rs'
PARSE_AUTOGEN_FILENAME = 'parse_autogen.rs'

o_ap214_autogen = open(AP214_AUTOGEN_FILENAME, 'w')
o_parse_autogen = open(PARSE_AUTOGEN_FILENAME, 'w')

o_ap214_autogen.write("""
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, Ord)]
pub struct Id(pub usize);
""")

o_parse_autogen.write("""
use crate::parse_basics::{{paren_tup, step_opt, Res, after_ws, after_wscomma, step_string, step_vec, step_id, step_identifier, step_float, step_bool, step_udecimal}};
use crate::ap214_autogen::*;
use nom::{{ branch::alt, bytes::complete::{{ tag }}, combinator::opt, sequence::{{ tuple, delimited, terminated }}, Err as NomErr, error::VerboseError }};
""".format())


# add strongly typed floats

strongly_typed_floats = {
    "LengthMeasure", "CountMeasure", "PositiveLengthMeasure", "AreaMeasure", "VolumeMeasure", "ParameterValue",
}

STRONGLY_TYPED_FLOAT_TEMPLATE_T = """
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct {cname}(pub f64);
"""
STRONGLY_TYPED_FLOAT_TEMPLATE_O = """
pub fn step_stf_{lname}(input: &str) -> Res<&str, {cname}> {{
    delimited(tuple((tag("{uname}"), after_ws(tag("(")))), after_ws(step_float), after_ws(tag(")")))(input)
    .map(|(next_input, res)| (next_input, {cname}(res)))
}}
"""

for name in strongly_typed_floats:
    o_ap214_autogen.write(STRONGLY_TYPED_FLOAT_TEMPLATE_T.format(cname=name))
    o_parse_autogen.write(STRONGLY_TYPED_FLOAT_TEMPLATE_O.format(cname=name, uname=camel_to_snake(
        name).upper(), lname=camel_to_snake(name).lower()))
    type_mp[name] = name
    pf_mp[name] = "step_stf_{lname}".format(lname=camel_to_snake(name).lower())


# add strongly typed floats combinations as enums

strongly_typed_enum_combination = {
    "AreaMeasureOrVolumeMeasure": ["AreaMeasure", "VolumeMeasure"]
}
STRONGLY_TYPED_ENUM_COMBINATION_TEMPLATE_T = """
#[derive(Debug, Copy, Clone)]
pub enum {cname} {{ {enum_vals} }}
"""
STRONGLY_TYPED_ENUM_COMBINATION_TEMPLATE_O = """
pub fn step_c_{lname}(input: &str) -> Res<&str, {cname}> {{
    tuple(( alt(( {tag_vals} )), delimited( after_ws(tag("(")), after_ws(step_float), after_ws(tag(")")) ) )) (input)
    .map(|(next_input, res)| (next_input, {{
        let (tg, flt) = res;
        match tg {{
{match_options},
            _ => panic!("unexpected string")
        }}
    }}))
}}
"""

for name, vals in strongly_typed_enum_combination.items():
    o_ap214_autogen.write(STRONGLY_TYPED_ENUM_COMBINATION_TEMPLATE_T.format(
        cname=name,
        enum_vals=", ".join(["{cval}({cval})".format(cval=val)
                            for val in vals])
    ))
    o_parse_autogen.write(STRONGLY_TYPED_ENUM_COMBINATION_TEMPLATE_O.format(
        cname=name,
        lname=camel_to_snake(name).lower(),
        enum_vals=", ".join(["{cval}({cval})".format(cval=val)
                            for val in vals]),
        tag_vals=", ".join(["tag(\"{uval}\")".format(
            uval=camel_to_snake(val).upper()) for val in vals]),
        match_options=",\n".join(["            \"{uval}\" => {cname}::{cval}({cval}(flt))".format(
            cname=name, cval=val, uval=camel_to_snake(val).upper()) for val in vals])
    ))
    type_mp[name] = name
    pf_mp[name] = "step_c_{lname}".format(lname=camel_to_snake(name).lower())


# add enums

enums = {
    "SurfaceSide": ["Positive", "Negative", "Both"],
    "Source": ["Made", "Bought", "NotKnown"],
    "BSplineEnum1": ["Unspecified", "SurfOfLinearExtrusion"],
    "BSplineEnum2": ["PiecewiseBezierKnots", "Unspecified", "QuasiUniformKnots"],
    "TrimmedCurveEnum": ["Parameter", "WeDontSupportOneElmentEnumsYet"],
}
ENUM_TEMPLATE_T = """
#[derive(Debug, Copy, Clone)]
pub enum {cname} {{ {enum_vals} }}
"""
ENUM_TEMPLATE_O = """
pub fn step_enum_{lname}(input: &str) -> Res<&str, {cname}> {{
    delimited(tag("."), alt(({tag_options})), tag("."))(input)
    .map(|(next_input, res)| (next_input, match res {{
        {remaps},
        _ => panic!("unepected string")
    }}))
}}
"""

for name, vals in enums.items():
    o_ap214_autogen.write(ENUM_TEMPLATE_T.format(
        cname=name,
        enum_vals=", ".join(vals),
        lname=camel_to_snake(name).lower(),
        tag_options=", ".join([
            "tag(\"{uval}\")".format(uval=camel_to_snake(val).upper()) for val in vals
        ]),
        remaps=", ".join([
            "\"{uval}\" => {cname}::{cval}".format(uval=camel_to_snake(val).upper(), cname=name, cval=val) for val in vals
        ])))
    o_parse_autogen.write(ENUM_TEMPLATE_O.format(
        cname=name,
        enum_vals=", ".join(vals),
        lname=camel_to_snake(name).lower(),
        tag_options=", ".join([
            "tag(\"{uval}\")".format(uval=camel_to_snake(val).upper()) for val in vals
        ]),
        remaps=", ".join([
            "\"{uval}\" => {cname}::{cval}".format(uval=camel_to_snake(val).upper(), cname=name, cval=val) for val in vals
        ])))
    type_mp[name] = name
    pf_mp[name] = "step_enum_{lname}".format(
        lname=camel_to_snake(name).lower())


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
    "VertexLoop": ["str", "id"],
    "VertexPoint": ["str", "id"]
}

data_entity = sorted(list(data_entity.items()))


# generate big enum to hold values

o_ap214_autogen.write("""
#[derive(Debug)]
pub enum DataEntity<'a> {{
    Null,
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


# generate parser for each entity

DATA_ENTITY_FUNCS_TEMPLATE = """
fn data_entity_{lname}<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {{
    delimited(
        after_ws(tag("(")),
        {parser},
        tuple((after_ws(tag(")")), after_ws(tag(";"))))
    ) (input) .map(|(next_input, res)| (next_input, {{
        let {unpack} = res;
        DataEntity::{cname}({pack})
    }}))
}}
"""

for name, tps in data_entity:
    o_parse_autogen.write(DATA_ENTITY_FUNCS_TEMPLATE.format(
        lname=camel_to_snake(name).lower(), uname=camel_to_snake(name).upper(), cname=name,
        unpack=("({})" if sum(t != "*" for t in tps) > 1 else "{}").format(
            ", ".join("x" + str(i) if t != "*" else "_" for i, t in enumerate(tps))),
        pack=", ".join("x" + str(i) for i, t in enumerate(tps) if t != "*"),
        parser="after_ws({})".format(pf_mp[tps[0]]) if len(tps) == 1 else
        "tuple(({}))".format(", ".join(["after_ws({})".format(
            pf_mp[tps[0]])] + ["after_wscomma({})".format(pf_mp[t]) for t in tps[1:]]))
    ))


# write functions which gather data entitys together

o_parse_autogen.write("""
pub fn data_entity_complex_bucket_type(input: &str) -> Res<&str, DataEntity> {
    terminated( paren_tup, after_ws(tag(";")) ) (input).map(|(next_input, _)| (next_input, DataEntity::ComplexBucketType) )
}
""")

o_parse_autogen.write("""
pub fn data_line(input: &str) -> Res<&str, (Id, DataEntity)> {{
    let res = tuple((
        after_ws(step_id), after_ws(tag("=")), after_ws(opt(step_identifier))
    ))(input);
    if res.is_err() {{ return res.map( |(a, (b, _, _))| (a, (b, DataEntity::ComplexBucketType )) ) }}
    let (next_input, (id, _, opt_iden)) = res.expect("should be ok");

    if opt_iden.is_none() {{
        return data_entity_complex_bucket_type(next_input).map(|(next_input, ent)| (next_input, (id, ent)) );
    }}

    match opt_iden.unwrap() {{
        {cases}
        _ => Err(NomErr::Error(VerboseError{{errors: vec![]}}))
    }}.map(|(next_input, ent)| (next_input, (id, ent)) )
}}
""".format(
    cases="\n".join([
        "        \"{uname}\" => data_entity_{lname}(next_input),".format(lname=camel_to_snake(name).lower(), uname=camel_to_snake(name).upper()) for name, _ in data_entity
    ])
))

def make_packer(t):
    if t == 'id':
        return "x{i}.clone()" 
    elif t == 'vec_id':
        return "*x{i}"
    elif t == 'vec_vec_id':
        return "**x{i}"
    elif t == 'opt_vec_id':
        return "*(match x{i} {{ Some(v) => v, None => vec![] }})"
    elif t == 'opt_id':
        return "*(match x{i} {{ Some(v) => vec![v], None => vec![] }})"
    else:
        raise ValueError(t)
                 
 
o_ap214_autogen.write("""
impl DataEntity<'_> {{
    pub fn upstream(&self) -> Vec<Id> {{
        use DataEntity::*;
        match self {{
            Null | ComplexBucketType => vec![],
            {remaps}
        }}
    }}
}}
""".format(
    remaps = ", ".join([
        "{cname}({unpack}) => vec![{pack}]".format(
            cname=name,
            unpack=", ".join(["x{i}".format(i=i) if (t == 'id' or 'id_' in 't') else "_" for i, t in enumerate(vals) if t != '*']),
            pack=", ".join([
                make_packer(t).format(i=i) for (i, t) in enumerate(vals) if (t == 'id' or 'id_' in 't')
            ])
        )
        for name, vals in data_entity
    ])
))


# gather test cases

step_files_for_tests_as_str = open('/Users/Henry Heffan/Desktop/foxtrot/foxtrot/local/KondoMotherboard_RevB_full.step').read(
) + "\n\n" + open('/Users/Henry Heffan/Desktop/foxtrot/foxtrot/local/HOLEWIZARD.step').read()


def escape(x): return re.sub(r"/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/", "", x).replace("\n", "\\n").replace("\"", "\\\"")


test_cases_for_entity = {}
for name, _ in data_entity:
    m = re.findall(" = " + camel_to_snake(name).upper() +
                   "\\([^;]*;", step_files_for_tests_as_str)
    seed = 3242562
    # make it deterministic so reruning doesnt confuse git
    random.Random(seed).shuffle(m)
    test_cases_for_entity[name] = [escape(
        '(' + test.split('(', 1)[1]) for test in m]

m = re.findall(";\n#\\d* =\\s*[^( ][^;]*;", step_files_for_tests_as_str)
seed = 3242562
# make it deterministic so reruning doesnt confuse git
random.Random(seed).shuffle(m)
line_test_cases = [escape(test[1:].strip()) for test in m]


# generate tests in rust


max_num_individual_tests = 10
max_line_tests_cases = 60

o_parse_autogen.write(
    """
#[cfg(test)]
mod tests {{
    use super::*;

{individual_tests}

{line_test}

}}
""".format(
        individual_tests="\n\n".join([
            "    #[test]\n    fn test_{name}() {{\n{tests}\n    }}".format(
                name=camel_to_snake(name).lower(),
                tests="\n".join(["        assert!(data_entity_{lname}(\"{text}\").is_ok());".format(
                    lname=camel_to_snake(name).lower(), text=test) for test in test_cases_for_entity[name][:min(len(m), max_num_individual_tests)]
                ])
            )
            for name, _ in data_entity
        ]),
        line_test="    #[test]\n    fn test_data_line() {{\n{tests}\n    }}".format(
            name=camel_to_snake(name).lower(),
            tests="\n".join(["        assert!(data_line(\"{text}\").is_ok());".format(
                lname=camel_to_snake(name).lower(), text=test) for test in line_test_cases[:min(len(m), max_line_tests_cases)]
            ])
        )
    ))

o_parse_autogen.close()
o_ap214_autogen.close()

subprocess.run(["rustfmt", AP214_AUTOGEN_FILENAME])
subprocess.run(["rustfmt", PARSE_AUTOGEN_FILENAME])


ENTITY_TREE_FILENAME = 'entity_tree_autogen.rs'


