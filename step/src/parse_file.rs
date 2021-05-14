
use crate::basic_parse::{paren_tup, step_opt, Id, Res, after_ws, after_wscomma, named_func, step_string, step_vec, step_id, step_float, step_bool, step_udecimal};
use nom::{ branch::alt, bytes::complete::{ tag }, multi::{ many0 }, sequence::{ tuple, delimited, terminated }, };

#[derive(Debug, PartialEq)]
pub struct VolumeMeasure(pub f64);
pub fn step_stf_volume_measure(input: &str) -> Res<&str, VolumeMeasure> {
    delimited(tuple((tag("VOLUME_MEASURE"), after_ws(tag("(")))), after_ws(step_float), after_ws(tag(")")))(input)
    .map(|(next_input, res)| (next_input, VolumeMeasure(res)))
}

#[derive(Debug, PartialEq)]
pub struct PositiveLengthMeasure(pub f64);
pub fn step_stf_positive_length_measure(input: &str) -> Res<&str, PositiveLengthMeasure> {
    delimited(tuple((tag("POSITIVE_LENGTH_MEASURE"), after_ws(tag("(")))), after_ws(step_float), after_ws(tag(")")))(input)
    .map(|(next_input, res)| (next_input, PositiveLengthMeasure(res)))
}

#[derive(Debug, PartialEq)]
pub struct AreaMeasure(pub f64);
pub fn step_stf_area_measure(input: &str) -> Res<&str, AreaMeasure> {
    delimited(tuple((tag("AREA_MEASURE"), after_ws(tag("(")))), after_ws(step_float), after_ws(tag(")")))(input)
    .map(|(next_input, res)| (next_input, AreaMeasure(res)))
}

#[derive(Debug, PartialEq)]
pub struct LengthMeasure(pub f64);
pub fn step_stf_length_measure(input: &str) -> Res<&str, LengthMeasure> {
    delimited(tuple((tag("LENGTH_MEASURE"), after_ws(tag("(")))), after_ws(step_float), after_ws(tag(")")))(input)
    .map(|(next_input, res)| (next_input, LengthMeasure(res)))
}

#[derive(Debug, PartialEq)]
pub struct CountMeasure(pub f64);
pub fn step_stf_count_measure(input: &str) -> Res<&str, CountMeasure> {
    delimited(tuple((tag("COUNT_MEASURE"), after_ws(tag("(")))), after_ws(step_float), after_ws(tag(")")))(input)
    .map(|(next_input, res)| (next_input, CountMeasure(res)))
}

#[derive(Debug, PartialEq)]
pub struct ParameterValue(pub f64);
pub fn step_stf_parameter_value(input: &str) -> Res<&str, ParameterValue> {
    delimited(tuple((tag("PARAMETER_VALUE"), after_ws(tag("(")))), after_ws(step_float), after_ws(tag(")")))(input)
    .map(|(next_input, res)| (next_input, ParameterValue(res)))
}

pub enum AreaMeasureOrVolumeMeasure { AreaMeasure(AreaMeasure), VolumeMeasure(VolumeMeasure) }
pub fn step_c_area_measure_or_volume_measure(input: &str) -> Res<&str, AreaMeasureOrVolumeMeasure> {
    tuple(( alt(( tag("AREA_MEASURE"), tag("VOLUME_MEASURE") )), delimited( after_ws(tag("(")), after_ws(step_float), after_ws(tag(")")) ) )) (input)
    .map(|(next_input, res)| (next_input, {
        let (tg, flt) = res;
        match tg {
            "AREA_MEASURE" => AreaMeasureOrVolumeMeasure::AreaMeasure(AreaMeasure(flt)),
            "VOLUME_MEASURE" => AreaMeasureOrVolumeMeasure::VolumeMeasure(VolumeMeasure(flt)),
            _ => panic!("unexpected string")
        }
    }))
}

pub enum SurfaceSide { Positive, Negative, Both }
pub fn step_enum_surface_side(input: &str) -> Res<&str, SurfaceSide> {
    delimited(tag("."), alt((tag("POSITIVE"), tag("NEGATIVE"), tag("BOTH"))), tag("."))(input)
    .map(|(next_input, res)| (next_input, match res {
        "POSITIVE" => SurfaceSide::Positive, "NEGATIVE" => SurfaceSide::Negative, "BOTH" => SurfaceSide::Both,
        _ => panic!("unepected string")
    }))
}

pub enum Source { Made, Bought, NotKnown }
pub fn step_enum_source(input: &str) -> Res<&str, Source> {
    delimited(tag("."), alt((tag("MADE"), tag("BOUGHT"), tag("NOT_KNOWN"))), tag("."))(input)
    .map(|(next_input, res)| (next_input, match res {
        "MADE" => Source::Made, "BOUGHT" => Source::Bought, "NOT_KNOWN" => Source::NotKnown,
        _ => panic!("unepected string")
    }))
}

pub enum BSplineEnum1 { Unspecified, WeDontSupportOneElmentEnumsYet }
pub fn step_enum_b_spline_enum1(input: &str) -> Res<&str, BSplineEnum1> {
    delimited(tag("."), alt((tag("UNSPECIFIED"), tag("WE_DONT_SUPPORT_ONE_ELMENT_ENUMS_YET"))), tag("."))(input)
    .map(|(next_input, res)| (next_input, match res {
        "UNSPECIFIED" => BSplineEnum1::Unspecified, "WE_DONT_SUPPORT_ONE_ELMENT_ENUMS_YET" => BSplineEnum1::WeDontSupportOneElmentEnumsYet,
        _ => panic!("unepected string")
    }))
}

pub enum BSplineEnum2 { PiecewiseBezierKnots, Unspecified, QuasiUniformKnots }
pub fn step_enum_b_spline_enum2(input: &str) -> Res<&str, BSplineEnum2> {
    delimited(tag("."), alt((tag("PIECEWISE_BEZIER_KNOTS"), tag("UNSPECIFIED"), tag("QUASI_UNIFORM_KNOTS"))), tag("."))(input)
    .map(|(next_input, res)| (next_input, match res {
        "PIECEWISE_BEZIER_KNOTS" => BSplineEnum2::PiecewiseBezierKnots, "UNSPECIFIED" => BSplineEnum2::Unspecified, "QUASI_UNIFORM_KNOTS" => BSplineEnum2::QuasiUniformKnots,
        _ => panic!("unepected string")
    }))
}

pub enum TrimmedCurveEnum { Parameter, WeDontSupportOneElmentEnumsYet }
pub fn step_enum_trimmed_curve_enum(input: &str) -> Res<&str, TrimmedCurveEnum> {
    delimited(tag("."), alt((tag("PARAMETER"), tag("WE_DONT_SUPPORT_ONE_ELMENT_ENUMS_YET"))), tag("."))(input)
    .map(|(next_input, res)| (next_input, match res {
        "PARAMETER" => TrimmedCurveEnum::Parameter, "WE_DONT_SUPPORT_ONE_ELMENT_ENUMS_YET" => TrimmedCurveEnum::WeDontSupportOneElmentEnumsYet,
        _ => panic!("unepected string")
    }))
}

pub enum DataEntity {
    ComplexBucketType,
    AdvancedBrepShapeRepresentation(String, Vec<Id>, Id),     AdvancedFace(String, Vec<Id>, Id, bool),     ApplicationContext(String),     ApplicationProtocolDefinition(String, String, usize, Id),     Axis2Placement3d(String, Id, Id, Id),     BSplineCurveWithKnots(String, usize, Vec<Id>, BSplineEnum1, bool, bool, Vec<usize>, Vec<f64>, BSplineEnum2),     BSplineSurfaceWithKnots(String, usize, usize, Vec<Vec<Id>>, BSplineEnum1, bool, bool, bool, Vec<usize>, Vec<usize>, Vec<f64>, Vec<f64>, BSplineEnum2),     BrepWithVoids(String, Id, Vec<Id>),     CartesianPoint(String, Vec<f64>),     Circle(String, Id, f64),     ClosedShell(String, Vec<Id>),     ColourRgb(String, f64, f64, f64),     ConicalSurface(String, Id, f64, f64),     ContextDependentShapeRepresentation(Id, Id),     CurveStyle(String, Id, PositiveLengthMeasure, Id),     CylindricalSurface(String, Id, f64),     DerivedUnit(Vec<Id>),     DerivedUnitElement(Id, f64),     DescriptiveRepresentationItem(String, String),     Direction(String, Vec<f64>),     DraughtingPreDefinedColour(String),     DraughtingPreDefinedCurveFont(String),     EdgeCurve(String, Id, Id, Id, bool),     EdgeLoop(String, Vec<Id>),     Ellipse(String, Id, f64, f64),     FaceBound(String, Id, bool),     FillAreaStyle(String, Vec<Id>),     FillAreaStyleColour(String, Id),     GeometricCurveSet(String, Vec<Id>),     ItemDefinedTransformation(String, String, Id, Id),     Line(String, Id, Id),     ManifoldSolidBrep(String, Id),     ManifoldSurfaceShapeRepresentation(String, Vec<Id>, Id),     MeasureRepresentationItem(String, AreaMeasureOrVolumeMeasure, Id),     MechanicalDesignGeometricPresentationRepresentation(String, Vec<Id>, Id),     NextAssemblyUsageOccurrence(String, String, String, Id, Id, Option<String>),     OpenShell(String, Vec<Id>),     OrientedClosedShell(String, Id, bool),     OrientedEdge(String, Id, bool),     OverRidingStyledItem(String, Vec<Id>, Id, Id),     Plane(String, Id),     PresentationLayerAssignment(String, String, Vec<Id>),     PresentationStyleAssignment(Vec<Id>),     PresentationStyleByContext(Vec<Id>, Id),     Product(String, String, String, Vec<Id>),     ProductCategory(String, String),     ProductContext(String, Id, String),     ProductDefinition(String, String, Id, Id),     ProductDefinitionContext(String, Id, String),     ProductDefinitionFormation(String, String, Id),     ProductDefinitionFormationWithSpecifiedSource(String, String, Id, Source),     ProductDefinitionShape(String, String, Id),     ProductRelatedProductCategory(String, Option<String>, Vec<Id>),     PropertyDefinition(String, String, Id),     PropertyDefinitionRepresentation(Id, Id),     Representation(Option<String>, Vec<Id>, Option<Id>),     ShapeAspect(String, String, Id, bool),     ShapeDefinitionRepresentation(Id, Id),     ShapeRepresentation(String, Vec<Id>, Id),     ShapeRepresentationRelationship(String, String, Id, Id),     ShellBasedSurfaceModel(String, Vec<Id>),     SphericalSurface(String, Id, f64),     StyledItem(String, Vec<Id>, Id),     SurfaceOfLinearExtrusion(String, Id, Id),     SurfaceSideStyle(String, Vec<Id>),     SurfaceStyleFillArea(Id),     SurfaceStyleUsage(SurfaceSide, Id),     ToroidalSurface(String, Id, f64, f64),     TrimmedCurve(String, Id, (Id, ParameterValue), (Id, ParameterValue), bool, TrimmedCurveEnum),     UncertaintyMeasureWithUnit(LengthMeasure, Id, String, String),     ValueRepresentationItem(String, CountMeasure),     Vector(String, Id, f64),     VertexPoint(String, Id)
}

fn data_entity_advanced_brep_shape_representation(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "advanced_brep_shape_representation", "ADVANCED_BREP_SHAPE_REPRESENTATION",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::AdvancedBrepShapeRepresentation(x0, x1, x2)
    }))
}

fn data_entity_advanced_face(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "advanced_face", "ADVANCED_FACE",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)), after_wscomma(step_id), after_wscomma(step_bool))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::AdvancedFace(x0, x1, x2, x3)
    }))
}

fn data_entity_application_context(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "application_context", "APPLICATION_CONTEXT",
        after_ws(step_string),
    )(input) .map(|(next_input, res)| (next_input, {
        let x0 = res;
        DataEntity::ApplicationContext(x0)
    }))
}

fn data_entity_application_protocol_definition(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "application_protocol_definition", "APPLICATION_PROTOCOL_DEFINITION",
        tuple((after_ws(step_string), after_wscomma(step_string), after_wscomma(step_udecimal), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::ApplicationProtocolDefinition(x0, x1, x2, x3)
    }))
}

fn data_entity_axis2_placement_3d(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "axis2_placement_3d", "AXIS2_PLACEMENT_3D",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_id), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::Axis2Placement3d(x0, x1, x2, x3)
    }))
}

fn data_entity_b_spline_curve_with_knots(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "b_spline_curve_with_knots", "B_SPLINE_CURVE_WITH_KNOTS",
        tuple((after_ws(step_string), after_wscomma(step_udecimal), after_wscomma(step_vec(step_id)), after_wscomma(step_enum_b_spline_enum1), after_wscomma(step_bool), after_wscomma(step_bool), after_wscomma(step_vec(step_udecimal)), after_wscomma(step_vec(step_float)), after_wscomma(step_enum_b_spline_enum2))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3, x4, x5, x6, x7, x8) = res;
        DataEntity::BSplineCurveWithKnots(x0, x1, x2, x3, x4, x5, x6, x7, x8)
    }))
}

fn data_entity_b_spline_surface_with_knots(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "b_spline_surface_with_knots", "B_SPLINE_SURFACE_WITH_KNOTS",
        tuple((after_ws(step_string), after_wscomma(step_udecimal), after_wscomma(step_udecimal), after_wscomma(step_vec(step_vec(step_id))), after_wscomma(step_enum_b_spline_enum1), after_wscomma(step_bool), after_wscomma(step_bool), after_wscomma(step_bool), after_wscomma(step_vec(step_udecimal)), after_wscomma(step_vec(step_udecimal)), after_wscomma(step_vec(step_float)), after_wscomma(step_vec(step_float)), after_wscomma(step_enum_b_spline_enum2))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12) = res;
        DataEntity::BSplineSurfaceWithKnots(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12)
    }))
}

fn data_entity_brep_with_voids(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "brep_with_voids", "BREP_WITH_VOIDS",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_vec(step_id)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::BrepWithVoids(x0, x1, x2)
    }))
}

fn data_entity_cartesian_point(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "cartesian_point", "CARTESIAN_POINT",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_float)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::CartesianPoint(x0, x1)
    }))
}

fn data_entity_circle(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "circle", "CIRCLE",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_float))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::Circle(x0, x1, x2)
    }))
}

fn data_entity_closed_shell(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "closed_shell", "CLOSED_SHELL",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::ClosedShell(x0, x1)
    }))
}

fn data_entity_colour_rgb(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "colour_rgb", "COLOUR_RGB",
        tuple((after_ws(step_string), after_wscomma(step_float), after_wscomma(step_float), after_wscomma(step_float))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::ColourRgb(x0, x1, x2, x3)
    }))
}

fn data_entity_conical_surface(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "conical_surface", "CONICAL_SURFACE",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_float), after_wscomma(step_float))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::ConicalSurface(x0, x1, x2, x3)
    }))
}

fn data_entity_context_dependent_shape_representation(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "context_dependent_shape_representation", "CONTEXT_DEPENDENT_SHAPE_REPRESENTATION",
        tuple((after_ws(step_id), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::ContextDependentShapeRepresentation(x0, x1)
    }))
}

fn data_entity_curve_style(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "curve_style", "CURVE_STYLE",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_stf_positive_length_measure), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::CurveStyle(x0, x1, x2, x3)
    }))
}

fn data_entity_cylindrical_surface(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "cylindrical_surface", "CYLINDRICAL_SURFACE",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_float))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::CylindricalSurface(x0, x1, x2)
    }))
}

fn data_entity_derived_unit(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "derived_unit", "DERIVED_UNIT",
        after_ws(step_vec(step_id)),
    )(input) .map(|(next_input, res)| (next_input, {
        let x0 = res;
        DataEntity::DerivedUnit(x0)
    }))
}

fn data_entity_derived_unit_element(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "derived_unit_element", "DERIVED_UNIT_ELEMENT",
        tuple((after_ws(step_id), after_wscomma(step_float))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::DerivedUnitElement(x0, x1)
    }))
}

fn data_entity_descriptive_representation_item(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "descriptive_representation_item", "DESCRIPTIVE_REPRESENTATION_ITEM",
        tuple((after_ws(step_string), after_wscomma(step_string))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::DescriptiveRepresentationItem(x0, x1)
    }))
}

fn data_entity_direction(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "direction", "DIRECTION",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_float)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::Direction(x0, x1)
    }))
}

fn data_entity_draughting_pre_defined_colour(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "draughting_pre_defined_colour", "DRAUGHTING_PRE_DEFINED_COLOUR",
        after_ws(step_string),
    )(input) .map(|(next_input, res)| (next_input, {
        let x0 = res;
        DataEntity::DraughtingPreDefinedColour(x0)
    }))
}

fn data_entity_draughting_pre_defined_curve_font(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "draughting_pre_defined_curve_font", "DRAUGHTING_PRE_DEFINED_CURVE_FONT",
        after_ws(step_string),
    )(input) .map(|(next_input, res)| (next_input, {
        let x0 = res;
        DataEntity::DraughtingPreDefinedCurveFont(x0)
    }))
}

fn data_entity_edge_curve(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "edge_curve", "EDGE_CURVE",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_id), after_wscomma(step_id), after_wscomma(step_bool))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3, x4) = res;
        DataEntity::EdgeCurve(x0, x1, x2, x3, x4)
    }))
}

fn data_entity_edge_loop(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "edge_loop", "EDGE_LOOP",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::EdgeLoop(x0, x1)
    }))
}

fn data_entity_ellipse(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "ellipse", "ELLIPSE",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_float), after_wscomma(step_float))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::Ellipse(x0, x1, x2, x3)
    }))
}

fn data_entity_face_bound(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "face_bound", "FACE_BOUND",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_bool))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::FaceBound(x0, x1, x2)
    }))
}

fn data_entity_fill_area_style(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "fill_area_style", "FILL_AREA_STYLE",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::FillAreaStyle(x0, x1)
    }))
}

fn data_entity_fill_area_style_colour(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "fill_area_style_colour", "FILL_AREA_STYLE_COLOUR",
        tuple((after_ws(step_string), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::FillAreaStyleColour(x0, x1)
    }))
}

fn data_entity_geometric_curve_set(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "geometric_curve_set", "GEOMETRIC_CURVE_SET",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::GeometricCurveSet(x0, x1)
    }))
}

fn data_entity_item_defined_transformation(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "item_defined_transformation", "ITEM_DEFINED_TRANSFORMATION",
        tuple((after_ws(step_string), after_wscomma(step_string), after_wscomma(step_id), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::ItemDefinedTransformation(x0, x1, x2, x3)
    }))
}

fn data_entity_line(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "line", "LINE",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::Line(x0, x1, x2)
    }))
}

fn data_entity_manifold_solid_brep(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "manifold_solid_brep", "MANIFOLD_SOLID_BREP",
        tuple((after_ws(step_string), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::ManifoldSolidBrep(x0, x1)
    }))
}

fn data_entity_manifold_surface_shape_representation(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "manifold_surface_shape_representation", "MANIFOLD_SURFACE_SHAPE_REPRESENTATION",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::ManifoldSurfaceShapeRepresentation(x0, x1, x2)
    }))
}

fn data_entity_measure_representation_item(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "measure_representation_item", "MEASURE_REPRESENTATION_ITEM",
        tuple((after_ws(step_string), after_wscomma(step_c_area_measure_or_volume_measure), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::MeasureRepresentationItem(x0, x1, x2)
    }))
}

fn data_entity_mechanical_design_geometric_presentation_representation(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "mechanical_design_geometric_presentation_representation", "MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::MechanicalDesignGeometricPresentationRepresentation(x0, x1, x2)
    }))
}

fn data_entity_next_assembly_usage_occurrence(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "next_assembly_usage_occurrence", "NEXT_ASSEMBLY_USAGE_OCCURRENCE",
        tuple((after_ws(step_string), after_wscomma(step_string), after_wscomma(step_string), after_wscomma(step_id), after_wscomma(step_id), after_wscomma(step_opt(step_string)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3, x4, x5) = res;
        DataEntity::NextAssemblyUsageOccurrence(x0, x1, x2, x3, x4, x5)
    }))
}

fn data_entity_open_shell(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "open_shell", "OPEN_SHELL",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::OpenShell(x0, x1)
    }))
}

fn data_entity_oriented_closed_shell(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "oriented_closed_shell", "ORIENTED_CLOSED_SHELL",
        tuple((after_ws(step_string), after_wscomma(tag("*")), after_wscomma(step_id), after_wscomma(step_bool))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, _, x2, x3) = res;
        DataEntity::OrientedClosedShell(x0, x2, x3)
    }))
}

fn data_entity_oriented_edge(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "oriented_edge", "ORIENTED_EDGE",
        tuple((after_ws(step_string), after_wscomma(tag("*")), after_wscomma(tag("*")), after_wscomma(step_id), after_wscomma(step_bool))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, _, _, x3, x4) = res;
        DataEntity::OrientedEdge(x0, x3, x4)
    }))
}

fn data_entity_over_riding_styled_item(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "over_riding_styled_item", "OVER_RIDING_STYLED_ITEM",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)), after_wscomma(step_id), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::OverRidingStyledItem(x0, x1, x2, x3)
    }))
}

fn data_entity_plane(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "plane", "PLANE",
        tuple((after_ws(step_string), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::Plane(x0, x1)
    }))
}

fn data_entity_presentation_layer_assignment(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "presentation_layer_assignment", "PRESENTATION_LAYER_ASSIGNMENT",
        tuple((after_ws(step_string), after_wscomma(step_string), after_wscomma(step_vec(step_id)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::PresentationLayerAssignment(x0, x1, x2)
    }))
}

fn data_entity_presentation_style_assignment(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "presentation_style_assignment", "PRESENTATION_STYLE_ASSIGNMENT",
        after_ws(step_vec(step_id)),
    )(input) .map(|(next_input, res)| (next_input, {
        let x0 = res;
        DataEntity::PresentationStyleAssignment(x0)
    }))
}

fn data_entity_presentation_style_by_context(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "presentation_style_by_context", "PRESENTATION_STYLE_BY_CONTEXT",
        tuple((after_ws(step_vec(step_id)), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::PresentationStyleByContext(x0, x1)
    }))
}

fn data_entity_product(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "product", "PRODUCT",
        tuple((after_ws(step_string), after_wscomma(step_string), after_wscomma(step_string), after_wscomma(step_vec(step_id)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::Product(x0, x1, x2, x3)
    }))
}

fn data_entity_product_category(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "product_category", "PRODUCT_CATEGORY",
        tuple((after_ws(step_string), after_wscomma(step_string))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::ProductCategory(x0, x1)
    }))
}

fn data_entity_product_context(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "product_context", "PRODUCT_CONTEXT",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_string))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::ProductContext(x0, x1, x2)
    }))
}

fn data_entity_product_definition(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "product_definition", "PRODUCT_DEFINITION",
        tuple((after_ws(step_string), after_wscomma(step_string), after_wscomma(step_id), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::ProductDefinition(x0, x1, x2, x3)
    }))
}

fn data_entity_product_definition_context(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "product_definition_context", "PRODUCT_DEFINITION_CONTEXT",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_string))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::ProductDefinitionContext(x0, x1, x2)
    }))
}

fn data_entity_product_definition_formation(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "product_definition_formation", "PRODUCT_DEFINITION_FORMATION",
        tuple((after_ws(step_string), after_wscomma(step_string), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::ProductDefinitionFormation(x0, x1, x2)
    }))
}

fn data_entity_product_definition_formation_with_specified_source(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "product_definition_formation_with_specified_source", "PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE",
        tuple((after_ws(step_string), after_wscomma(step_string), after_wscomma(step_id), after_wscomma(step_enum_source))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::ProductDefinitionFormationWithSpecifiedSource(x0, x1, x2, x3)
    }))
}

fn data_entity_product_definition_shape(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "product_definition_shape", "PRODUCT_DEFINITION_SHAPE",
        tuple((after_ws(step_string), after_wscomma(step_string), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::ProductDefinitionShape(x0, x1, x2)
    }))
}

fn data_entity_product_related_product_category(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "product_related_product_category", "PRODUCT_RELATED_PRODUCT_CATEGORY",
        tuple((after_ws(step_string), after_wscomma(step_opt(step_string)), after_wscomma(step_vec(step_id)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::ProductRelatedProductCategory(x0, x1, x2)
    }))
}

fn data_entity_property_definition(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "property_definition", "PROPERTY_DEFINITION",
        tuple((after_ws(step_string), after_wscomma(step_string), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::PropertyDefinition(x0, x1, x2)
    }))
}

fn data_entity_property_definition_representation(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "property_definition_representation", "PROPERTY_DEFINITION_REPRESENTATION",
        tuple((after_ws(step_id), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::PropertyDefinitionRepresentation(x0, x1)
    }))
}

fn data_entity_representation(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "representation", "REPRESENTATION",
        tuple((after_ws(step_opt(step_string)), after_wscomma(step_vec(step_id)), after_wscomma(step_opt(step_id)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::Representation(x0, x1, x2)
    }))
}

fn data_entity_shape_aspect(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "shape_aspect", "SHAPE_ASPECT",
        tuple((after_ws(step_string), after_wscomma(step_string), after_wscomma(step_id), after_wscomma(step_bool))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::ShapeAspect(x0, x1, x2, x3)
    }))
}

fn data_entity_shape_definition_representation(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "shape_definition_representation", "SHAPE_DEFINITION_REPRESENTATION",
        tuple((after_ws(step_id), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::ShapeDefinitionRepresentation(x0, x1)
    }))
}

fn data_entity_shape_representation(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "shape_representation", "SHAPE_REPRESENTATION",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::ShapeRepresentation(x0, x1, x2)
    }))
}

fn data_entity_shape_representation_relationship(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "shape_representation_relationship", "SHAPE_REPRESENTATION_RELATIONSHIP",
        tuple((after_ws(step_string), after_wscomma(step_string), after_wscomma(step_id), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::ShapeRepresentationRelationship(x0, x1, x2, x3)
    }))
}

fn data_entity_shell_based_surface_model(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "shell_based_surface_model", "SHELL_BASED_SURFACE_MODEL",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::ShellBasedSurfaceModel(x0, x1)
    }))
}

fn data_entity_spherical_surface(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "spherical_surface", "SPHERICAL_SURFACE",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_float))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::SphericalSurface(x0, x1, x2)
    }))
}

fn data_entity_styled_item(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "styled_item", "STYLED_ITEM",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::StyledItem(x0, x1, x2)
    }))
}

fn data_entity_surface_of_linear_extrusion(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "surface_of_linear_extrusion", "SURFACE_OF_LINEAR_EXTRUSION",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::SurfaceOfLinearExtrusion(x0, x1, x2)
    }))
}

fn data_entity_surface_side_style(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "surface_side_style", "SURFACE_SIDE_STYLE",
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::SurfaceSideStyle(x0, x1)
    }))
}

fn data_entity_surface_style_fill_area(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "surface_style_fill_area", "SURFACE_STYLE_FILL_AREA",
        after_ws(step_id),
    )(input) .map(|(next_input, res)| (next_input, {
        let x0 = res;
        DataEntity::SurfaceStyleFillArea(x0)
    }))
}

fn data_entity_surface_style_usage(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "surface_style_usage", "SURFACE_STYLE_USAGE",
        tuple((after_ws(step_enum_surface_side), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::SurfaceStyleUsage(x0, x1)
    }))
}

fn data_entity_toroidal_surface(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "toroidal_surface", "TOROIDAL_SURFACE",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_float), after_wscomma(step_float))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::ToroidalSurface(x0, x1, x2, x3)
    }))
}

fn data_entity_trimmed_curve(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "trimmed_curve", "TRIMMED_CURVE",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(delimited(tag("("), tuple((after_ws(step_id), after_wscomma(step_stf_parameter_value))), after_ws(tag(")")))), after_wscomma(delimited(tag("("), tuple((after_ws(step_id), after_wscomma(step_stf_parameter_value))), after_ws(tag(")")))), after_wscomma(step_bool), after_wscomma(step_enum_trimmed_curve_enum))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3, x4, x5) = res;
        DataEntity::TrimmedCurve(x0, x1, x2, x3, x4, x5)
    }))
}

fn data_entity_uncertainty_measure_with_unit(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "uncertainty_measure_with_unit", "UNCERTAINTY_MEASURE_WITH_UNIT",
        tuple((after_ws(step_stf_length_measure), after_wscomma(step_id), after_wscomma(step_string), after_wscomma(step_string))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2, x3) = res;
        DataEntity::UncertaintyMeasureWithUnit(x0, x1, x2, x3)
    }))
}

fn data_entity_value_representation_item(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "value_representation_item", "VALUE_REPRESENTATION_ITEM",
        tuple((after_ws(step_string), after_wscomma(step_stf_count_measure))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::ValueRepresentationItem(x0, x1)
    }))
}

fn data_entity_vector(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "vector", "VECTOR",
        tuple((after_ws(step_string), after_wscomma(step_id), after_wscomma(step_float))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1, x2) = res;
        DataEntity::Vector(x0, x1, x2)
    }))
}

fn data_entity_vertex_point(input: &str) -> Res<&str, DataEntity> {
    named_func(
        "vertex_point", "VERTEX_POINT",
        tuple((after_ws(step_string), after_wscomma(step_id))),
    )(input) .map(|(next_input, res)| (next_input, {
        let (x0, x1) = res;
        DataEntity::VertexPoint(x0, x1)
    }))
}

pub fn data_entity_complex_bucket_type(input: &str) -> Res<&str, DataEntity> {
    terminated( paren_tup, after_ws(tag(";")) ) (input).map(|(next_input, _)| (next_input, DataEntity::ComplexBucketType) )
}

fn data_entity(input: &str) -> Res<&str, DataEntity> {
    alt((
        alt(( data_entity_complex_bucket_type, data_entity_advanced_brep_shape_representation, data_entity_advanced_face, data_entity_application_context, data_entity_application_protocol_definition, data_entity_axis2_placement_3d, data_entity_b_spline_curve_with_knots, data_entity_b_spline_surface_with_knots, data_entity_brep_with_voids, data_entity_cartesian_point, data_entity_circle, data_entity_closed_shell, data_entity_colour_rgb, data_entity_conical_surface )),
        alt(( data_entity_context_dependent_shape_representation, data_entity_curve_style, data_entity_cylindrical_surface, data_entity_derived_unit, data_entity_derived_unit_element, data_entity_descriptive_representation_item, data_entity_direction, data_entity_draughting_pre_defined_colour, data_entity_draughting_pre_defined_curve_font, data_entity_edge_curve, data_entity_edge_loop, data_entity_ellipse, data_entity_face_bound, data_entity_fill_area_style )),
        alt(( data_entity_fill_area_style_colour, data_entity_geometric_curve_set, data_entity_item_defined_transformation, data_entity_line, data_entity_manifold_solid_brep, data_entity_manifold_surface_shape_representation, data_entity_measure_representation_item, data_entity_mechanical_design_geometric_presentation_representation, data_entity_next_assembly_usage_occurrence, data_entity_open_shell, data_entity_oriented_closed_shell, data_entity_oriented_edge, data_entity_over_riding_styled_item, data_entity_plane )),
        alt(( data_entity_presentation_layer_assignment, data_entity_presentation_style_assignment, data_entity_presentation_style_by_context, data_entity_product, data_entity_product_category, data_entity_product_context, data_entity_product_definition, data_entity_product_definition_context, data_entity_product_definition_formation, data_entity_product_definition_formation_with_specified_source, data_entity_product_definition_shape, data_entity_product_related_product_category, data_entity_property_definition, data_entity_property_definition_representation )),
        alt(( data_entity_representation, data_entity_shape_aspect, data_entity_shape_definition_representation, data_entity_shape_representation, data_entity_shape_representation_relationship, data_entity_shell_based_surface_model, data_entity_spherical_surface, data_entity_styled_item, data_entity_surface_of_linear_extrusion, data_entity_surface_side_style, data_entity_surface_style_fill_area, data_entity_surface_style_usage, data_entity_toroidal_surface, data_entity_trimmed_curve )),
        alt(( data_entity_uncertainty_measure_with_unit, data_entity_value_representation_item, data_entity_vector, data_entity_vertex_point ))
    ))(input)
}

fn data_line(input: &str) -> Res<&str, (Id, DataEntity)> {
    tuple((
        after_ws(step_id), after_ws(tag("=")), after_ws(data_entity)
    ))(input) .map(|(next_input, res)| (next_input, {
        let (id, _, ent) = res;
        (id, ent)
    }))
}

pub fn data_block(input: &str) -> Res<&str, Vec<(Id, DataEntity)>> {
    many0(
        after_ws(data_line)
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_brep_shape_representation() {
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#162601),#162751);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#172997),#173477);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#177013),#177555);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#164807),#164957);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#369341),#371680);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#163891),#164041);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#8249),#8791);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#162171),#162321);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#381801),#382783);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#35262),#39038);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#159437),#159719);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#174414),#176988);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#180249),#180791);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#322309),#322599);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#155097),#155907);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#129055),#129205);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#130148),#130298);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#364898),#369278);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#159788),#160697);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#163031),#163181);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#202851),#203001);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#383868),#384850);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#154878),#155028);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#40572),#41735);").is_ok());
        assert!(data_entity_advanced_brep_shape_representation("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#158630),#158780);").is_ok());
    }

    #[test]
    fn test_advanced_face() {
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#171209),#171227,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#5709),#5715,.F.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#278724),#278735,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#310987),#310998,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#164638),#164663,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#64437),#64474,.F.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#171305),#171327,.F.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#363889),#363905,.F.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#388369),#388396,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#355788),#355799,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#119434),#119445,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#273910),#273921,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#57231),#57250,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#174887),#174898,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#247282),#247300,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#330771),#330782,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#67008),#67033,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#8597),#8616,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#273115),#273126,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#11551),#11557,.F.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#309631),#309659,.F.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#394413),#394427,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#25740),#25752,.T.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#363789),#363819,.F.);").is_ok());
        assert!(data_entity_advanced_face("ADVANCED_FACE('',(#256034),#256040,.T.);").is_ok());
    }

    #[test]
    fn test_application_context() {
        assert!(data_entity_application_context("APPLICATION_CONTEXT(\n  'core data for automotive mechanical design processes');").is_ok());
    }

    #[test]
    fn test_application_protocol_definition() {
        assert!(data_entity_application_protocol_definition("APPLICATION_PROTOCOL_DEFINITION('international standard',\n  'automotive_design',2000,#2);").is_ok());
    }

    #[test]
    fn test_axis2_placement_3d() {
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#348424,#348425,#348426);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#16936,#16937,#16938);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#249822,#249823,#249824);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#340719,#340720,#340721);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#82806,#82807,#82808);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#151744,#151745,#151746);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#278029,#278030,#278031);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#144629,#144630,#144631);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#204620,#204621,#204622);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#170730,#170731,#170732);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#390141,#390142,#390143);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#41565,#41566,#41567);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#267816,#267817,#267818);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#393936,#393937,#393938);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#47006,#47007,#47008);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#113337,#113338,#113339);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#278483,#278484,#278485);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#313240,#313241,#313242);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#265820,#265821,#265822);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#74656,#74657,#74658);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#146604,#146605,#146606);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#62913,#62914,#62915);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#262995,#262996,#262997);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#211671,#211672,#211673);").is_ok());
        assert!(data_entity_axis2_placement_3d("AXIS2_PLACEMENT_3D('',#72653,#72654,#72655);").is_ok());
    }

    #[test]
    fn test_b_spline_curve_with_knots() {
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#273781,#273782,#273783,\n    #273784),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#252094,#252095,#252096,\n    #252097),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#269158,#269159,#269160,\n    #269161),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#181755,#181756,#181757,\n    #181758),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,0.10471975512),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#60082,#60083,#60084,#60085,\n    #60086,#60087,#60088,#60089,#60090,#60091),.UNSPECIFIED.,.F.,.F.,(4,\n    2,2,2,4),(5.205788810599E-012,1.241232100071E-003,\n    2.482464194937E-003,3.723696289803E-003,4.964928384668E-003),\n  .UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#51619,#51620,#51621,#51622,\n    #51623,#51624),.UNSPECIFIED.,.F.,.F.,(4,2,4),(1.737237004028E-016,\n    1.961965290198E-004,3.923930580394E-004),.UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#266473,#266474,#266475,\n    #266476),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#6407,#6408,#6409,#6410),\n  .UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,0.10471975512),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#132351,#132352,#132353,\n    #132354),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,0.10471975512),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',4,(#182468,#182469,#182470,\n    #182471,#182472),.UNSPECIFIED.,.F.,.F.,(5,5),(0.E+000,1.570796326795\n    ),.PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#273375,#273376,#273377,\n    #273378),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#269970,#269971,#269972,\n    #269973),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#286562,#286563,#286564,\n    #286565,#286566),.UNSPECIFIED.,.F.,.F.,(4,1,4),(0.E+000,0.5,1.),\n  .QUASI_UNIFORM_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',4,(#6984,#6985,#6986,#6987,#6988),\n  .UNSPECIFIED.,.F.,.F.,(5,5),(0.E+000,1.570796326795),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',4,(#45051,#45052,#45053,#45054,\n    #45055),.UNSPECIFIED.,.F.,.F.,(5,5),(0.E+000,1.559869909752),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',4,(#176074,#176075,#176076,\n    #176077,#176078),.UNSPECIFIED.,.F.,.F.,(5,5),(0.E+000,1.570796326795\n    ),.PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#262819,#262820,#262821,\n    #262822),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#393860,#393861,#393862,\n    #393863),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.099543901406E-005),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#134762,#134763,#134764,\n    #134765),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,0.10471975512),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#286485,#286486,#286487,\n    #286488,#286489),.UNSPECIFIED.,.F.,.F.,(4,1,4),(0.E+000,0.5,1.),\n  .QUASI_UNIFORM_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#183031,#183032,#183033,\n    #183034),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,0.10471975512),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#393793,#393794,#393795,\n    #393796),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,5.998181191398E-006),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',4,(#44559,#44560,#44561,#44562,\n    #44563),.UNSPECIFIED.,.F.,.F.,(5,5),(0.E+000,1.570796326795),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#61675,#61676,#61677,#61678,\n    #61679,#61680,#61681,#61682,#61683,#61684),.UNSPECIFIED.,.F.,.F.,(4,\n    2,2,2,4),(2.424349759031E-019,1.241242893482E-003,\n    2.482485786963E-003,3.723728680445E-003,4.964971573926E-003),\n  .UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("B_SPLINE_CURVE_WITH_KNOTS('',3,(#185588,#185589,#185590,\n    #185591),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,0.10471975512),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
    }

    #[test]
    fn test_b_spline_surface_with_knots() {
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#131931,#131932,#131933,#131934,#131935,#131936,#131937,#131938\n      ,#131939)\n    ,(#131940,#131941,#131942,#131943,#131944,#131945,#131946,#131947\n      ,#131948)\n    ,(#131949,#131950,#131951,#131952,#131953,#131954,#131955,#131956\n      ,#131957)\n    ,(#131958,#131959,#131960,#131961,#131962,#131963,#131964,#131965\n      ,#131966)\n    ,(#131967,#131968,#131969,#131970,#131971,#131972,#131973,#131974\n      ,#131975)\n    ,(#131976,#131977,#131978,#131979,#131980,#131981,#131982,#131983\n      ,#131984)\n    ,(#131985,#131986,#131987,#131988,#131989,#131990,#131991,#131992\n      ,#131993)\n    ,(#131994,#131995,#131996,#131997,#131998,#131999,#132000,#132001\n      ,#132002)\n    ,(#132003,#132004,#132005,#132006,#132007,#132008,#132009,#132010\n      ,#132011\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-7.778174593052E-002,\n    7.778174593052E-002),(-7.315613219613E-003,8.855910256723E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',3,3,(\n    (#57194,#57195,#57196,#57197)\n    ,(#57198,#57199,#57200,#57201)\n    ,(#57202,#57203,#57204,#57205)\n    ,(#57206,#57207,#57208,#57209)\n    ,(#57210,#57211,#57212,#57213)\n    ,(#57214,#57215,#57216,#57217)\n    ,(#57218,#57219,#57220,#57221)\n    ,(#57222,#57223,#57224,#57225)\n    ,(#57226,#57227,#57228,#57229\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,1,1,1,4),(4,4),(-0.419707394122,\n    0.E+000,1.923076923077E-002,0.5,0.980769230769,1.,1.463233547653),(\n    0.215417803204,0.784582200388),.UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#132647,#132648,#132649,#132650,#132651,#132652,#132653,#132654\n      ,#132655)\n    ,(#132656,#132657,#132658,#132659,#132660,#132661,#132662,#132663\n      ,#132664)\n    ,(#132665,#132666,#132667,#132668,#132669,#132670,#132671,#132672\n      ,#132673)\n    ,(#132674,#132675,#132676,#132677,#132678,#132679,#132680,#132681\n      ,#132682)\n    ,(#132683,#132684,#132685,#132686,#132687,#132688,#132689,#132690\n      ,#132691)\n    ,(#132692,#132693,#132694,#132695,#132696,#132697,#132698,#132699\n      ,#132700)\n    ,(#132701,#132702,#132703,#132704,#132705,#132706,#132707,#132708\n      ,#132709)\n    ,(#132710,#132711,#132712,#132713,#132714,#132715,#132716,#132717\n      ,#132718)\n    ,(#132719,#132720,#132721,#132722,#132723,#132724,#132725,#132726\n      ,#132727\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-7.778174593052E-002,\n    7.778174593052E-002),(-8.855910256723E-003,7.315613219614E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#45154,#45155,#45156,#45157,#45158,#45159,#45160,#45161,#45162)\n    ,(#45163,#45164,#45165,#45166,#45167,#45168,#45169,#45170,#45171)\n    ,(#45172,#45173,#45174,#45175,#45176,#45177,#45178,#45179,#45180)\n    ,(#45181,#45182,#45183,#45184,#45185,#45186,#45187,#45188,#45189)\n    ,(#45190,#45191,#45192,#45193,#45194,#45195,#45196,#45197,#45198)\n    ,(#45199,#45200,#45201,#45202,#45203,#45204,#45205,#45206,#45207)\n    ,(#45208,#45209,#45210,#45211,#45212,#45213,#45214,#45215,#45216)\n    ,(#45217,#45218,#45219,#45220,#45221,#45222,#45223,#45224,#45225)\n    ,(#45226,#45227,#45228,#45229,#45230,#45231,#45232,#45233,#45234\n  )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-7.778174593052E-002,\n    7.778174593052E-002),(-8.855910256723E-003,7.315613219615E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',3,3,(\n    (#59855,#59856,#59857,#59858)\n    ,(#59859,#59860,#59861,#59862)\n    ,(#59863,#59864,#59865,#59866)\n    ,(#59867,#59868,#59869,#59870)\n    ,(#59871,#59872,#59873,#59874)\n    ,(#59875,#59876,#59877,#59878\n  )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,4),(4,4),(-2.000202717131E-002,\n    0.E+000,1.,1.020000040435),(0.21531949858,0.784693586529),\n  .UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',3,3,(\n    (#58127,#58128,#58129,#58130)\n    ,(#58131,#58132,#58133,#58134)\n    ,(#58135,#58136,#58137,#58138)\n    ,(#58139,#58140,#58141,#58142)\n    ,(#58143,#58144,#58145,#58146)\n    ,(#58147,#58148,#58149,#58150)\n    ,(#58151,#58152,#58153,#58154)\n    ,(#58155,#58156,#58157,#58158)\n    ,(#58159,#58160,#58161,#58162\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,1,1,1,4),(4,4),(-0.4269626569,\n    0.E+000,1.923076923076E-002,0.5,0.980769230769,1.,1.447328917973),(\n    0.215417802387,0.784582200455),.UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#182685,#182686,#182687,#182688,#182689,#182690,#182691,#182692\n      ,#182693)\n    ,(#182694,#182695,#182696,#182697,#182698,#182699,#182700,#182701\n      ,#182702)\n    ,(#182703,#182704,#182705,#182706,#182707,#182708,#182709,#182710\n      ,#182711)\n    ,(#182712,#182713,#182714,#182715,#182716,#182717,#182718,#182719\n      ,#182720)\n    ,(#182721,#182722,#182723,#182724,#182725,#182726,#182727,#182728\n      ,#182729)\n    ,(#182730,#182731,#182732,#182733,#182734,#182735,#182736,#182737\n      ,#182738)\n    ,(#182739,#182740,#182741,#182742,#182743,#182744,#182745,#182746\n      ,#182747)\n    ,(#182748,#182749,#182750,#182751,#182752,#182753,#182754,#182755\n      ,#182756)\n    ,(#182757,#182758,#182759,#182760,#182761,#182762,#182763,#182764\n      ,#182765\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-3.889087296526E-002,\n    3.889087296526E-002),(-3.626740088442E-003,4.427879780914E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#181969,#181970,#181971,#181972,#181973,#181974,#181975,#181976\n      ,#181977)\n    ,(#181978,#181979,#181980,#181981,#181982,#181983,#181984,#181985\n      ,#181986)\n    ,(#181987,#181988,#181989,#181990,#181991,#181992,#181993,#181994\n      ,#181995)\n    ,(#181996,#181997,#181998,#181999,#182000,#182001,#182002,#182003\n      ,#182004)\n    ,(#182005,#182006,#182007,#182008,#182009,#182010,#182011,#182012\n      ,#182013)\n    ,(#182014,#182015,#182016,#182017,#182018,#182019,#182020,#182021\n      ,#182022)\n    ,(#182023,#182024,#182025,#182026,#182027,#182028,#182029,#182030\n      ,#182031)\n    ,(#182032,#182033,#182034,#182035,#182036,#182037,#182038,#182039\n      ,#182040)\n    ,(#182041,#182042,#182043,#182044,#182045,#182046,#182047,#182048\n      ,#182049\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-3.889087296526E-002,\n    3.889087296526E-002),(-3.626740088442E-003,4.427879780914E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#377673,#377674,#377675,#377676,#377677,#377678,#377679,#377680\n      ,#377681)\n    ,(#377682,#377683,#377684,#377685,#377686,#377687,#377688,#377689\n      ,#377690)\n    ,(#377691,#377692,#377693,#377694,#377695,#377696,#377697,#377698\n      ,#377699)\n    ,(#377700,#377701,#377702,#377703,#377704,#377705,#377706,#377707\n      ,#377708)\n    ,(#377709,#377710,#377711,#377712,#377713,#377714,#377715,#377716\n      ,#377717)\n    ,(#377718,#377719,#377720,#377721,#377722,#377723,#377724,#377725\n      ,#377726)\n    ,(#377727,#377728,#377729,#377730,#377731,#377732,#377733,#377734\n      ,#377735)\n    ,(#377736,#377737,#377738,#377739,#377740,#377741,#377742,#377743\n      ,#377744)\n    ,(#377745,#377746,#377747,#377748,#377749,#377750,#377751,#377752\n      ,#377753\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-7.778174593052E-002,\n    7.778174593052E-002),(-8.855910256723E-003,7.315613219613E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',3,3,(\n    (#56832,#56833,#56834,#56835)\n    ,(#56836,#56837,#56838,#56839)\n    ,(#56840,#56841,#56842,#56843)\n    ,(#56844,#56845,#56846,#56847)\n    ,(#56848,#56849,#56850,#56851)\n    ,(#56852,#56853,#56854,#56855)\n    ,(#56856,#56857,#56858,#56859\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,1,4),(4,4),(5.323075212263E-002,\n    0.148700901957,0.503777838603,0.85885477525,0.981649641487),(\n    -6.575958308042E-003,0.708622850024),.UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',3,3,(\n    (#61634,#61635,#61636,#61637)\n    ,(#61638,#61639,#61640,#61641)\n    ,(#61642,#61643,#61644,#61645)\n    ,(#61646,#61647,#61648,#61649)\n    ,(#61650,#61651,#61652,#61653)\n    ,(#61654,#61655,#61656,#61657\n  )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,4),(4,4),(-1.997895337656E-002,\n    0.E+000,1.,1.019999588909),(0.215319037225,0.784693736573),\n  .UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#182329,#182330,#182331,#182332,#182333,#182334,#182335,#182336\n      ,#182337)\n    ,(#182338,#182339,#182340,#182341,#182342,#182343,#182344,#182345\n      ,#182346)\n    ,(#182347,#182348,#182349,#182350,#182351,#182352,#182353,#182354\n      ,#182355)\n    ,(#182356,#182357,#182358,#182359,#182360,#182361,#182362,#182363\n      ,#182364)\n    ,(#182365,#182366,#182367,#182368,#182369,#182370,#182371,#182372\n      ,#182373)\n    ,(#182374,#182375,#182376,#182377,#182378,#182379,#182380,#182381\n      ,#182382)\n    ,(#182383,#182384,#182385,#182386,#182387,#182388,#182389,#182390\n      ,#182391)\n    ,(#182392,#182393,#182394,#182395,#182396,#182397,#182398,#182399\n      ,#182400)\n    ,(#182401,#182402,#182403,#182404,#182405,#182406,#182407,#182408\n      ,#182409\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-3.889087296526E-002,\n    3.889087296526E-002),(-3.626740088442E-003,4.427879780914E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',3,3,(\n    (#286972,#286973,#286974,#286975)\n    ,(#286976,#286977,#286978,#286979)\n    ,(#286980,#286981,#286982,#286983)\n    ,(#286984,#286985,#286986,#286987)\n    ,(#286988,#286989,#286990,#286991)\n    ,(#286992,#286993,#286994,#286995)\n    ,(#286996,#286997,#286998,#286999\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,1,4),(4,4),(-0.37565647779,\n    0.E+000,0.5,1.,1.359521715808),(-0.24442252123,1.243482740852),\n  .UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',3,3,(\n    (#58318,#58319,#58320,#58321)\n    ,(#58322,#58323,#58324,#58325)\n    ,(#58326,#58327,#58328,#58329)\n    ,(#58330,#58331,#58332,#58333)\n    ,(#58334,#58335,#58336,#58337)\n    ,(#58338,#58339,#58340,#58341)\n    ,(#58342,#58343,#58344,#58345)\n    ,(#58346,#58347,#58348,#58349)\n    ,(#58350,#58351,#58352,#58353\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,1,1,1,4),(4,4),(-0.41970739634,\n    0.E+000,1.923076923077E-002,0.5,0.980769230769,1.,1.463233549611),(\n    0.215417803194,0.78458220039),.UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#131804,#131805,#131806,#131807,#131808,#131809,#131810,#131811\n      ,#131812)\n    ,(#131813,#131814,#131815,#131816,#131817,#131818,#131819,#131820\n      ,#131821)\n    ,(#131822,#131823,#131824,#131825,#131826,#131827,#131828,#131829\n      ,#131830)\n    ,(#131831,#131832,#131833,#131834,#131835,#131836,#131837,#131838\n      ,#131839)\n    ,(#131840,#131841,#131842,#131843,#131844,#131845,#131846,#131847\n      ,#131848)\n    ,(#131849,#131850,#131851,#131852,#131853,#131854,#131855,#131856\n      ,#131857)\n    ,(#131858,#131859,#131860,#131861,#131862,#131863,#131864,#131865\n      ,#131866)\n    ,(#131867,#131868,#131869,#131870,#131871,#131872,#131873,#131874\n      ,#131875)\n    ,(#131876,#131877,#131878,#131879,#131880,#131881,#131882,#131883\n      ,#131884\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-7.778174593052E-002,\n    7.778174593052E-002),(-7.315613219614E-003,8.855910256723E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#132520,#132521,#132522,#132523,#132524,#132525,#132526,#132527\n      ,#132528)\n    ,(#132529,#132530,#132531,#132532,#132533,#132534,#132535,#132536\n      ,#132537)\n    ,(#132538,#132539,#132540,#132541,#132542,#132543,#132544,#132545\n      ,#132546)\n    ,(#132547,#132548,#132549,#132550,#132551,#132552,#132553,#132554\n      ,#132555)\n    ,(#132556,#132557,#132558,#132559,#132560,#132561,#132562,#132563\n      ,#132564)\n    ,(#132565,#132566,#132567,#132568,#132569,#132570,#132571,#132572\n      ,#132573)\n    ,(#132574,#132575,#132576,#132577,#132578,#132579,#132580,#132581\n      ,#132582)\n    ,(#132583,#132584,#132585,#132586,#132587,#132588,#132589,#132590\n      ,#132591)\n    ,(#132592,#132593,#132594,#132595,#132596,#132597,#132598,#132599\n      ,#132600\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-7.778174593052E-002,\n    7.778174593052E-002),(-8.855910256722E-003,7.315613219614E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',3,3,(\n    (#57445,#57446,#57447,#57448)\n    ,(#57449,#57450,#57451,#57452)\n    ,(#57453,#57454,#57455,#57456)\n    ,(#57457,#57458,#57459,#57460)\n    ,(#57461,#57462,#57463,#57464)\n    ,(#57465,#57466,#57467,#57468)\n    ,(#57469,#57470,#57471,#57472)\n    ,(#57473,#57474,#57475,#57476)\n    ,(#57477,#57478,#57479,#57480\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,1,1,1,4),(4,4),(-0.419707393786,\n    0.E+000,1.923076923077E-002,0.5,0.980769230769,1.,1.463233547313),(\n    0.21541780321,0.784582200406),.UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#182941,#182942,#182943,#182944,#182945,#182946,#182947,#182948\n      ,#182949)\n    ,(#182950,#182951,#182952,#182953,#182954,#182955,#182956,#182957\n      ,#182958)\n    ,(#182959,#182960,#182961,#182962,#182963,#182964,#182965,#182966\n      ,#182967)\n    ,(#182968,#182969,#182970,#182971,#182972,#182973,#182974,#182975\n      ,#182976)\n    ,(#182977,#182978,#182979,#182980,#182981,#182982,#182983,#182984\n      ,#182985)\n    ,(#182986,#182987,#182988,#182989,#182990,#182991,#182992,#182993\n      ,#182994)\n    ,(#182995,#182996,#182997,#182998,#182999,#183000,#183001,#183002\n      ,#183003)\n    ,(#183004,#183005,#183006,#183007,#183008,#183009,#183010,#183011\n      ,#183012)\n    ,(#183013,#183014,#183015,#183016,#183017,#183018,#183019,#183020\n      ,#183021\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-3.889087296526E-002,\n    3.889087296526E-002),(-4.427879780914E-003,3.626740088442E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',3,3,(\n    (#56117,#56118,#56119,#56120)\n    ,(#56121,#56122,#56123,#56124)\n    ,(#56125,#56126,#56127,#56128)\n    ,(#56129,#56130,#56131,#56132)\n    ,(#56133,#56134,#56135,#56136)\n    ,(#56137,#56138,#56139,#56140)\n    ,(#56141,#56142,#56143,#56144\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,1,4),(4,4),(4.783655954484E-002,\n    0.14777794034,0.502427406438,0.857076872536,0.974686530813),(\n    -6.49043687887E-003,0.708582280707),.UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#377800,#377801,#377802,#377803,#377804,#377805,#377806,#377807\n      ,#377808)\n    ,(#377809,#377810,#377811,#377812,#377813,#377814,#377815,#377816\n      ,#377817)\n    ,(#377818,#377819,#377820,#377821,#377822,#377823,#377824,#377825\n      ,#377826)\n    ,(#377827,#377828,#377829,#377830,#377831,#377832,#377833,#377834\n      ,#377835)\n    ,(#377836,#377837,#377838,#377839,#377840,#377841,#377842,#377843\n      ,#377844)\n    ,(#377845,#377846,#377847,#377848,#377849,#377850,#377851,#377852\n      ,#377853)\n    ,(#377854,#377855,#377856,#377857,#377858,#377859,#377860,#377861\n      ,#377862)\n    ,(#377863,#377864,#377865,#377866,#377867,#377868,#377869,#377870\n      ,#377871)\n    ,(#377872,#377873,#377874,#377875,#377876,#377877,#377878,#377879\n      ,#377880\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-7.778174593052E-002,\n    7.778174593052E-002),(-8.855910256723E-003,7.315613219614E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#185337,#185338,#185339,#185340,#185341,#185342,#185343,#185344\n      ,#185345)\n    ,(#185346,#185347,#185348,#185349,#185350,#185351,#185352,#185353\n      ,#185354)\n    ,(#185355,#185356,#185357,#185358,#185359,#185360,#185361,#185362\n      ,#185363)\n    ,(#185364,#185365,#185366,#185367,#185368,#185369,#185370,#185371\n      ,#185372)\n    ,(#185373,#185374,#185375,#185376,#185377,#185378,#185379,#185380\n      ,#185381)\n    ,(#185382,#185383,#185384,#185385,#185386,#185387,#185388,#185389\n      ,#185390)\n    ,(#185391,#185392,#185393,#185394,#185395,#185396,#185397,#185398\n      ,#185399)\n    ,(#185400,#185401,#185402,#185403,#185404,#185405,#185406,#185407\n      ,#185408)\n    ,(#185409,#185410,#185411,#185412,#185413,#185414,#185415,#185416\n      ,#185417\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-3.889087296526E-002,\n    3.889087296526E-002),(-3.626740088442E-003,4.427879780914E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',3,3,(\n    (#56696,#56697,#56698,#56699)\n    ,(#56700,#56701,#56702,#56703)\n    ,(#56704,#56705,#56706,#56707)\n    ,(#56708,#56709,#56710,#56711)\n    ,(#56712,#56713,#56714,#56715)\n    ,(#56716,#56717,#56718,#56719)\n    ,(#56720,#56721,#56722,#56723)\n    ,(#56724,#56725,#56726,#56727\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,1,1,4),(4,4),(\n    4.287996563273E-002,0.145075654521,0.502399320551,0.859722986581,1.,\n    1.003032731689),(-6.491751211166E-003,0.708256948002),\n  .UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#44438,#44439,#44440,#44441,#44442,#44443,#44444,#44445,#44446)\n    ,(#44447,#44448,#44449,#44450,#44451,#44452,#44453,#44454,#44455)\n    ,(#44456,#44457,#44458,#44459,#44460,#44461,#44462,#44463,#44464)\n    ,(#44465,#44466,#44467,#44468,#44469,#44470,#44471,#44472,#44473)\n    ,(#44474,#44475,#44476,#44477,#44478,#44479,#44480,#44481,#44482)\n    ,(#44483,#44484,#44485,#44486,#44487,#44488,#44489,#44490,#44491)\n    ,(#44492,#44493,#44494,#44495,#44496,#44497,#44498,#44499,#44500)\n    ,(#44501,#44502,#44503,#44504,#44505,#44506,#44507,#44508,#44509)\n    ,(#44510,#44511,#44512,#44513,#44514,#44515,#44516,#44517,#44518\n  )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-7.778174593052E-002,\n    7.778174593052E-002),(-7.315613219614E-003,8.855910256722E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',3,3,(\n    (#56285,#56286,#56287,#56288)\n    ,(#56289,#56290,#56291,#56292)\n    ,(#56293,#56294,#56295,#56296)\n    ,(#56297,#56298,#56299,#56300)\n    ,(#56301,#56302,#56303,#56304)\n    ,(#56305,#56306,#56307,#56308)\n    ,(#56309,#56310,#56311,#56312)\n    ,(#56313,#56314,#56315,#56316\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,1,1,4),(4,4),(\n    4.287996561525E-002,0.145075654521,0.502399320551,0.859722986581,1.,\n    1.003032731453),(-6.491751211156E-003,0.708256948002),\n  .UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#182789,#182790,#182791,#182792,#182793,#182794,#182795,#182796\n      ,#182797)\n    ,(#182798,#182799,#182800,#182801,#182802,#182803,#182804,#182805\n      ,#182806)\n    ,(#182807,#182808,#182809,#182810,#182811,#182812,#182813,#182814\n      ,#182815)\n    ,(#182816,#182817,#182818,#182819,#182820,#182821,#182822,#182823\n      ,#182824)\n    ,(#182825,#182826,#182827,#182828,#182829,#182830,#182831,#182832\n      ,#182833)\n    ,(#182834,#182835,#182836,#182837,#182838,#182839,#182840,#182841\n      ,#182842)\n    ,(#182843,#182844,#182845,#182846,#182847,#182848,#182849,#182850\n      ,#182851)\n    ,(#182852,#182853,#182854,#182855,#182856,#182857,#182858,#182859\n      ,#182860)\n    ,(#182861,#182862,#182863,#182864,#182865,#182866,#182867,#182868\n      ,#182869\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-3.889087296526E-002,\n    3.889087296526E-002),(-4.427879780914E-003,3.626740088442E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
    }

    #[test]
    fn test_brep_with_voids() {
        assert!(data_entity_brep_with_voids("BREP_WITH_VOIDS('',#67616,(#89927,#90077));").is_ok());
    }

    #[test]
    fn test_cartesian_point() {
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(6.,2.65,1.6));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(5.9199999994,11.67498095209,-2.6699999998)\n  );").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(-6.675,5.089123043511E-017,-5.09));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(33.5,-0.5,11.75));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(-6.83,0.68,-2.1));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(-1.875,-2.1,-7.05));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(0.14294104,-5.315711999999E-002,0.E+000));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(32.,-2.05,9.9));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(6.201663060342,1.801193405256E-012,\n    -13.9061182435));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(-0.2499995,-2.89999928,0.E+000));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(16.91592428742,8.951657804289,\n    -21.11046879016));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(1.175,1.49,-0.84));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(-2.625,1.2,-3.05));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(0.E+000,3.2,0.775));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(-4.15000000001,7.30000000001,0.15000000001)\n  );").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(-30.075,0.85,-6.95));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(3.199666448243,5.531029750749E-002,\n    0.772574954701));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(6.325,0.11,-5.2));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(-6.499999999996E-002,0.720743124491,\n    -11.49099999985));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(-4.675,0.95,-0.820036725769));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(7.109001805981E-004,5.573656791976E-002,\n    0.770584088296));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(2.203101555092E-002,3.191814220383,\n    0.980117378007));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(0.E+000,0.E+000,0.E+000));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(6.550273515208,9.634980952092,\n    -3.349092668505));").is_ok());
        assert!(data_entity_cartesian_point("CARTESIAN_POINT('',(4.257302746373,11.50039416076,\n    -19.76500600004));").is_ok());
    }

    #[test]
    fn test_circle() {
        assert!(data_entity_circle("CIRCLE('',#133619,0.1);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#320468,4.2);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#39471,5.E-002);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#67776,0.36);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#386299,5.E-002);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#18218,0.15);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#193668,5.E-002);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#60959,0.3);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#394352,1.2E-002);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#348101,0.25);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#84216,0.36);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#317775,4.2);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#316019,1.3);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#25997,1.62E-002);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#372202,5.E-002);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#244810,1.8);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#42422,0.25);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#334479,1.5);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#3283,0.3999992);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#290737,0.5);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#61077,0.25);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#320385,4.2);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#379892,5.E-002);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#141338,0.3);").is_ok());
        assert!(data_entity_circle("CIRCLE('',#168666,0.8);").is_ok());
    }

    #[test]
    fn test_closed_shell() {
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#381803,#381875,#381923,#381947,#381972,\n    #381996,#382037,#382061,#382101,#382118,#382143,#382167,#382209,\n    #382240,#382265,#382314,#382331,#382380,#382396,#382421,#382454,\n    #382471,#382487,#382518,#382551,#382575,#382593,#382617,#382651,\n    #382668,#382685,#382703,#382720,#382743,#382761,#382772));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#160741,#160781,#160812,#160843,#160874,\n    #160905,#160936,#160967,#160998,#161029,#161060,#161091,#161122,\n    #161153,#161184,#161215,#161246,#161277,#161308,#161339,#161370,\n    #161401,#161432,#161463,#161494,#161525,#161556,#161578,#161613));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#170967,#171009,#171049,#171080,#171113,\n    #171144,#171175,#171208,#171232,#171249,#171273,#171304,#171332,\n    #171350,#171367,#171379,#171397,#171408));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#11154,#11194,#11234,#11267,#11300,#11331,\n    #11362,#11395,#11428,#11472,#11496,#11533,#11550,#11562));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#391605,#391645,#391676,#391707,#391738,\n    #391769,#391800,#391831,#391862,#391893,#391924,#391946,#391965));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#193483,#193525,#193565,#193596,#193629,\n    #193660,#193691,#193724,#193748,#193765,#193789,#193820,#193848,\n    #193866,#193883,#193895,#193913,#193924));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#374392,#374432,#374463,#374494,#374516,\n    #374528));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#134199,#134239,#134272,#134305,#134338,\n    #134371,#134388,#134405,#134436,#134467,#134484,#134515,#134532,\n    #134563,#134582,#134601,#134634,#134653,#134686,#134705,#134738,\n    #134771,#134867,#134963,#134994,#135090,#135121,#135217,#135248,\n    #135279,#135298,#135317,#135350,#135369,#135402,#135421,#135454,\n    #135487,#135583,#135679,#135710,#135806,#135837,#135933,#135964,\n    #135995,#136013,#136031,#136064,#136082,#136113,#136131,#136162,\n    #136191,#136202,#136213,#136225,#136236));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#26706,#26746,#26786,#26817,#26846,#26863,\n    #26894,#26906,#26924,#26942));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#162603,#162643,#162674,#162705,#162727,\n    #162739));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#202853,#202893,#202924,#202955,#202977,\n    #202989));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#184180,#184220,#184269,#184293,#184335,\n    #184352,#184384,#184408,#184433,#184464,#184496,#184520,#184545,\n    #184576,#184601,#184625,#184650,#184667,#184691,#184722,#184746,\n    #184771,#184795,#184820,#184837,#184868,#184894,#184952,#184976,\n    #184995,#185053,#185069,#185085,#185097,#185123,#185149,#185166,\n    #185182,#185198,#185210,#185314,#185418,#185449,#185474,#185570,\n    #185674,#185705,#185722,#185748,#185774,#185807,#185831,#185850,\n    #185876,#185909,#185926,#186030,#186134,#186165,#186190,#186286,\n    #186390,#186421,#186438,#186463,#186488,#186521,#186545,#186563,\n    #186588,#186621,#186638,#186655,#186672,#186694,#186712,#186723,\n    #186740));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#150355,#150395,#150435,#150498,#150529,\n    #150611,#150623,#150654,#150687,#150711,#150735,#150747,#150787,\n    #150870,#150953,#150984,#151076,#151168,#151190,#151221,#151313,\n    #151344,#151427,#151444,#151461,#151478,#151490,#151512,#151529,\n    #151541,#151553,#151570,#151587,#151604,#151621,#151633,#151650,\n    #151667,#151684,#151701,#151713,#151730,#151747,#151764,#151776,\n    #151798,#151815,#151832,#151844));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#90271,#90311,#90398,#90422,#90446,#90519,\n    #90541,#90572,#91549,#91571,#91588,#91600,#91612,#92614,#92638,\n    #92655,#92721,#92799,#92877,#92955,#92986,#93064,#93142,#93220,\n    #93251,#93275,#93306,#93384,#93408,#93486,#93510,#93581,#93612,\n    #93643,#93674,#93698,#93715,#93793,#93864,#93935,#93966,#94044,\n    #94068,#94139,#94210,#94288,#94319,#94350,#94381,#94405,#94483,\n    #94561,#94585,#94616,#94694,#94765,#94796,#94827,#94851,#94922,\n    #94953,#95016,#95047,#95071,#95149,#95173,#95232,#95249,#95320,\n    #95391,#95422,#95446,#95524,#95548,#95644,#95715,#95786,#95857,\n    #95935,#95959,#96030,#96054,#96078,#96102,#96173,#96197,#96221,\n    #96245,#96316,#96387,#96458,#96529,#96553,#96617,#96688,#96759,\n    #96776,#96847,#96864,#96881,#96898,#96969,#97072,#97136,#97160,\n    #97177,#97241,#97305,#97322,#97386,#97450,#97474,#97538,#97602,\n    #97619,#97690,#97714,#97778,#97795,#97812,#97883,#97900,#97964,\n    #98035,#98099,#98116,#98180,#98244,#98261,#98278,#98295,#98359,\n    #98376,#98393,#98410,#98422,#98434,#98446,#98458,#98475,#98492,\n    #98523,#98535,#98547,#98559,#98576,#98588,#98605,#98617,#98634,\n    #98646,#98670,#98682,#98699,#98723,#98740,#98752,#98769,#98781,\n    #98793,#98810,#98827,#98844,#98861,#98873,#98890,#98907,#98919,\n    #98936,#98948,#98960,#98977,#98989,#99001,#99018,#99035,#99052,\n    #99069,#99081,#99093,#99110,#99122,#99139,#99156,#99173,#99185,\n    #99197,#99214,#99226,#99243,#99260,#99272,#99289,#99306,#99323,\n    #99340,#99352,#99364,#99381,#99398,#99415,#99432,#99444,#99461,\n    #99483,#99495,#99512,#99524,#99541,#99563,#99580,#99592,#99604,\n    #99626,#99638,#99660,#99672,#99684,#99701,#99718,#99740,#99757,\n    #99769,#99781,#99798,#99815,#99837,#99849,#99861,#99883,#99900,\n    #99917,#99929,#99941,#99958,#99980,#99992,#100014,#100026,#100038,\n    #100055,#100072,#100094,#100111,#100123,#100135,#100157,#100174,\n    #100186,#100198,#100220,#100237,#100254,#100266,#100278,#100295,\n    #100312,#100329,#100346,#100358,#100375,#100392,#100409,#100426,\n    #100438,#100455,#100477,#100494,#100506,#100518,#100535,#100552,\n    #100574,#100591,#100603,#100620,#100632,#100649,#100666,#100683,\n    #100700,#100712,#100734,#100751,#100768,#100780,#100792,#100809,\n    #100826,#100843,#100867,#100891,#100915,#102027,#102044,#102061,\n    #102083,#102095,#102112,#102124,#102146,#102163,#102175,#102187,\n    #102204,#102221,#102238,#102250,#102267,#102289,#102306,#102323,\n    #102335,#102347,#102394,#102447,#102478,#102554,#102571,#102679,\n    #102744,#102777,#102794,#102816,#102833,#102845,#102857,#102874,\n    #102896,#102908,#102920,#102942,#102959,#102976,#102988,#103000,\n    #103047,#103071,#103088,#103157,#103190,#103212,#103229,#103241,\n    #103253,#103275,#103287,#103304,#103316,#103333,#103350,#103367,\n    #103379,#103401,#103418,#103430,#103442,#103464,#103481,#103493,\n    #103505,#103536,#103567,#103630,#103661,#103692,#103723,#103754,\n    #103778,#103817,#103841,#103872,#103903,#103966,#103997,#104060,\n    #104084,#104147,#104171,#104227,#104251,#104282,#104345,#104376,\n    #104407,#104438,#104501,#104536,#104599,#104630,#104654,#104685,\n    #104748,#104772,#104796,#104827,#104850,#104913,#104937,#104961,\n    #105024,#105048,#105072,#105128,#105152,#105176,#105193,#105224,\n    #105255,#105286,#105317,#105341,#105372,#105403,#105434,#105465,\n    #105482,#105538,#105563,#105580,#105604,#105621,#105677,#105701,\n    #105732,#105756,#105780,#105797,#105821,#105838,#105894,#105925,\n    #105981,#105998,#106033,#106050,#106106,#106137,#106168,#106185,\n    #106202,#106226,#106250,#106299,#106355,#106372,#106404,#106435,\n    #106484,#106501,#106550,#106574,#106598,#106622,#106671,#106688,\n    #106737,#106754,#106771,#106796,#106852,#106869,#106918,#106942,\n    #106991,#107047,#107064,#107088,#107112,#107136,#107153,#107170,\n    #107226,#107250,#107313,#107337,#107354,#107371,#107395,#107412,\n    #107429,#107446,#107463,#107480,#107497,#107509,#107526,#107543,\n    #107560,#107577,#107594,#107643,#107660,#107684,#107701,#107732,\n    #107782,#107832,#107849,#107896,#107908,#107932,#107972,#107989,\n    #108966,#108983,#108995,#109012,#109024,#109048,#109065,#109096,\n    #109136,#109160,#109172,#109205,#109217,#112038,#112078,#112118,\n    #112135,#112176,#112216,#112256,#112273,#112313,#112353,#112370,\n    #112410,#112450,#112467,#112507,#112531,#112564,#112595,#112628,\n    #112661,#112701,#112725,#112758,#112798,#112838,#112855,#112895,\n    #112935,#112952,#112992,#113016,#113049,#113089,#113129,#113146,\n    #113186,#113210,#113243,#113283,#113307,#113340,#113371,#113404,\n    #113437,#113477,#113517,#113534,#113565,#113598,#113631,#113662,\n    #113695,#113728,#113768,#113792,#113825,#113865,#113905,#113922,\n    #113963,#113994,#114027,#114060,#114100,#114140,#114157,#114197,\n    #114221,#114254,#114294,#114318,#114351,#114382,#114415,#114448,\n    #114488,#114512,#114545,#114585,#114625,#114642,#114682,#114722,\n    #114739,#114770,#114803,#114836,#114876,#114916,#114933,#114973,\n    #114997,#115030,#115047,#115064,#115081,#115093,#115115,#115127,\n    #115139,#115170,#115201,#115255,#115286,#115317,#115371,#115402,\n    #115433,#115487,#115518,#115572,#115603,#115634,#115688,#115719,\n    #115773,#115827,#115881,#115935,#115966,#116020,#116074,#116098,\n    #116152,#116183,#116237,#116284,#116315,#116362,#116409,#116433,\n    #116473,#116504,#116551,#116582,#116606,#116660,#116700,#116747,\n    #116771,#116802,#116833,#116857,#116874,#116928,#116952,#116976,\n    #117000,#117024,#117071,#117125,#117156,#117196,#117236,#117283,\n    #117307,#117361,#117415,#117432,#117449,#117480,#117534,#117551,\n    #117591,#117631,#117662,#117686,#117733,#117780,#117834,#117851,\n    #117891,#117915,#117955,#117979,#118003,#118027,#118067,#118084,\n    #118131,#118178,#118195,#118219,#118259,#118299,#118316,#118370,\n    #118394,#118441,#118488,#118535,#118575,#118622,#118639,#118656,\n    #118696,#118743,#118790,#118807,#118831,#118871,#118911,#118951,\n    #118991,#119031,#119048,#119072,#119089,#119113,#119130,#119170,\n    #119187,#119204,#119244,#119261,#119278,#119318,#119335,#119359,\n    #119399,#119416,#119433,#119450,#119462,#119509,#119556,#119603,\n    #119634,#119665,#119696,#119727,#119758,#119798,#119810,#119827,\n    #119839,#119856,#119873,#119920,#119967,#120014,#120061,#120085,\n    #120132,#120163,#120203,#120234,#120251,#120263,#120280,#120292,\n    #120304,#120316,#120333,#120350,#120397,#120444,#120461,#120478,\n    #120495,#120507,#120519,#120531,#120562,#120609,#120640,#120657,\n    #120674,#120691,#120708,#120725,#120737,#120749,#120773,#120797,\n    #120844,#120884,#120931,#120971,#120988,#121005,#121022,#121039,\n    #121070,#121101,#121141,#121158,#121175,#121192,#121209,#121226,\n    #121243,#121255,#121302,#121349,#121373,#121404,#121437,#121468,\n    #121508,#121525,#121542,#121559,#121571,#121588,#121600,#121631,\n    #121655,#121702,#121749,#121773,#121797,#121844,#121891,#121922,\n    #121969,#122009,#122033,#122057,#122074,#122091,#122108,#122125,\n    #122137,#122149,#122166,#122206,#122230,#122254,#122271,#122311,\n    #122328,#122368,#122408,#122425,#122437,#122449,#122461,#122473,\n    #122490,#122537,#122577,#122617,#122641,#122658,#122675,#122692,\n    #122709,#122726,#122743,#122767,#122807,#122831,#122855,#122872,\n    #122884,#122901,#122913,#122937,#122970,#122994,#123018,#123035,\n    #123052,#123092,#123132,#123149,#123166,#123190,#123207,#123231,\n    #123264,#123281,#123314,#123331,#123348,#123365,#123382,#123399,\n    #123411,#123451,#123484,#123501,#123518,#123530,#123547,#123564,\n    #123588,#123621,#123638,#123678,#123711,#123728,#123761,#123801,\n    #123818,#123842,#123882,#123906,#123939,#123956,#123968,#123985,\n    #124018,#124051,#124068,#124108,#124155,#124172,#124189,#124229,\n    #124262,#124279,#124312,#124352,#124369,#124381,#124393,#124410,\n    #124427,#124460,#124493,#124510,#124527,#124544,#124561,#124578,\n    #124595,#124612,#124629,#124646,#124663,#124675,#124687,#124699,\n    #124716,#124728,#124745,#124757,#124774,#124786,#124803,#124815,\n    #124832,#124844,#124861,#124873,#124890,#124902,#124919,#124931,\n    #124948,#124960,#124977,#124989,#125006,#125018,#125035,#125047,\n    #125064,#125076,#125093,#125105,#125122,#125134,#125151,#125163,\n    #125180,#125192,#125209,#125221,#125238,#125250,#125267,#125279,\n    #125296,#125308,#125325,#125337,#125354,#125366,#125383,#125395,\n    #125412,#125424,#125441,#125453,#125470,#125482,#125499,#125511,\n    #125528,#125540,#125557,#125569,#125586,#125598,#125615,#125627,\n    #125644,#125661,#125673,#125690,#125707,#125724,#125736,#125753,\n    #125770,#125782,#125799,#125816,#125828,#125845,#125862,#125874,\n    #125891,#125903,#125920,#125932,#125949,#125961,#125978,#125995,\n    #126007,#126024,#126041,#126053,#126065,#126082,#126099,#126111,\n    #126128,#126140,#126152,#126169,#126181,#126193,#126205,#126222,\n    #126234,#126251,#126263,#126280,#126292,#126309,#126321,#126338,\n    #126350,#126362,#126374,#126391,#126403,#126415,#126427,#126439,\n    #126456,#126468,#126485,#126502,#126514,#126531,#126548,#126560,\n    #126577,#126594,#126611,#126628,#126645,#126662,#126674,#126696,\n    #126708,#126720,#126737,#126754,#126766,#126783,#126800,#126812,\n    #126834,#126846,#126858,#126875,#126892,#126904,#126921,#126938,\n    #126950,#126967,#126989,#127001,#127013,#127030,#127047,#127059,\n    #127076,#127098,#127110,#127122,#127139,#127156,#127168,#127185,\n    #127202,#127214,#127231,#127248,#127265,#127277,#127299,#127311,\n    #127323,#127345,#127357,#127369,#127386,#127398,#127415,#127432,\n    #127444,#127461,#127478,#127495,#127507,#127524,#127541,#127553,\n    #127575,#127587,#127599,#127616,#127633,#127645,#127662,#127679,\n    #127691,#127708,#127720,#127737,#127749,#127766,#127778,#127795,\n    #127807,#127824,#127836));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#333237,#333281,#333325,#333369,#333431,\n    #333487,#333518,#333565,#333587,#333599,#333680,#333704,#333728,\n    #333752,#333776,#333816,#333847,#333864,#333895,#333919,#333943,\n    #333955,#334045,#334085,#334125,#334165,#334205,#334416,#334440,\n    #334464,#334488,#334512,#334536,#334554,#334572,#334590,#334608,\n    #334626,#334644,#334656,#334680,#334698,#334716,#334734,#334752,\n    #334770,#334788,#334800,#334951,#334967,#335003,#335015,#335033,\n    #335051,#335087,#335099,#335135,#335171,#335189,#335225,#335243,\n    #335261,#335297,#335315,#335351,#335363));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#158851,#158891,#158922,#158953,#158984,\n    #159015,#159046,#159077,#159108,#159139,#159170,#159201,#159232,\n    #159263,#159294,#159325,#159347,#159371));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#330033,#330073,#330104,#330135,#330157,\n    #330169));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#168610,#168656,#168688,#168783,#168819,\n    #168851,#169058,#169103,#169152,#169169,#169186,#169203,#169220,\n    #169232,#169249,#169284,#169333,#169391,#169408,#169425,#169442,\n    #169459,#169471,#169500,#169517,#169534,#169551,#169568,#169585,\n    #169603,#169649,#169680,#169713,#169747,#169771,#169798,#169815,\n    #169827,#169839,#169871,#169893,#169911,#169923,#169935,#169967,\n    #169985,#170010));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#164809,#164849,#164880,#164911,#164933,\n    #164945));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#45830,#45870,#45901,#45934,#45965,#45998,\n    #46022,#46047,#46071,#46096,#46120,#46145,#46169,#46194,#46211,\n    #46228,#46245,#46262));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#128809,#128849,#128880,#128911,#128942,\n    #128964,#128977));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#383870,#383942,#383990,#384014,#384039,\n    #384063,#384104,#384128,#384168,#384185,#384210,#384234,#384276,\n    #384307,#384332,#384381,#384398,#384447,#384463,#384488,#384521,\n    #384538,#384554,#384585,#384618,#384642,#384660,#384684,#384718,\n    #384735,#384752,#384770,#384787,#384810,#384828,#384839));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#330347,#330387,#330427,#330467,#330507,\n    #330547,#330587,#330605,#330629,#330647,#330664,#330681,#330699,\n    #330717,#330735,#330753,#330770,#330787));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#372691,#372731,#372798,#372829,#372860,\n    #372909,#372931,#372943,#372955,#372967));").is_ok());
        assert!(data_entity_closed_shell("CLOSED_SHELL('',(#131121,#131161,#131194,#131227,#131260,\n    #131293,#131310,#131327,#131358,#131389,#131406,#131437,#131454,\n    #131485,#131504,#131523,#131556,#131575,#131608,#131627,#131660,\n    #131693,#131789,#131885,#131916,#132012,#132043,#132139,#132170,\n    #132201,#132220,#132239,#132272,#132291,#132324,#132343,#132376,\n    #132409,#132505,#132601,#132632,#132728,#132767,#132863,#132902,\n    #132933,#132951,#132969,#133002,#133020,#133051,#133084,#133102,\n    #133133,#133166,#133195,#133206,#133217,#133268,#133279,#133296,\n    #133339,#133350,#133367,#133407,#133433,#133450,#133491));").is_ok());
    }

    #[test]
    fn test_colour_rgb() {
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.286274522543,0.662745118141,0.329411774874);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.184313729405,0.749019622803,0.580392181873);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.800000011921,0.800000011921,0.800000011921);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.792156875134,0.819607853889,0.933333337307);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.600000023842,0.40000000596,0.20000000298);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.643137276173,0.678431391716,0.698039233685);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.898039221764,0.921568632126,0.929411768913);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.20000000298,0.20000000298,0.20000000298);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',1.,0.937254905701,0.137254908681);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.188235297799,0.188235297799,0.188235297799);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.E+000,0.501960813999,0.E+000);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.752941191196,0.752941191196,0.752941191196);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.313725501299,0.313725501299,0.313725501299);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.941176474094,1.,1.);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.501960813999,0.501960813999,0.501960813999);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.580392181873,0.580392181873,0.627451002598);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.564705908298,0.564705908298,0.564705908298);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.298039227724,0.298039227724,0.298039227724);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.109803922474,0.109803922474,0.109803922474);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.690196096897,0.690196096897,0.690196096897);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.250980407,0.250980407,0.250980407);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.E+000,0.580392181873,0.611764729023);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.647058844566,0.517647087574,0.E+000);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.160784319043,0.160784319043,0.160784319043);").is_ok());
        assert!(data_entity_colour_rgb("COLOUR_RGB('',0.152941182256,0.305882364511,\n  7.450980693102E-002);").is_ok());
    }

    #[test]
    fn test_conical_surface() {
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#241129,0.234530705359,0.523574705607);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#72037,1.28,0.352833819799);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#72904,1.574999999996,0.463647608998);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#72665,1.28,0.352833819799);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#169889,0.974999999529,0.785398163397);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#169907,0.974999999529,0.785398163397);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#336063,4.9,0.785398163397);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#241067,0.634540825454,0.523583912325);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#73062,1.575000000001,0.352833819801);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#241007,0.634540825454,0.523583912325);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#72146,1.825,0.352833819799);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#72792,1.825,0.463647609002);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#336045,4.9,0.785398163397);").is_ok());
        assert!(data_entity_conical_surface("CONICAL_SURFACE('',#241111,0.234530705359,0.523574705607);").is_ok());
    }

    #[test]
    fn test_context_dependent_shape_representation() {
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#198923,#198925);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#137447,#137449);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#337390,#337392);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#200216,#200218);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#166013,#166015);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#333128,#333130);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#24177,#24179);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#389456,#389458);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#198172,#198174);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#323162,#323164);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#138226,#138228);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#323409,#323411);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#395108,#395110);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#129998,#130000);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#43007,#43009);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#202699,#202701);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#42928,#42930);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#8963,#8965);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#376197,#376199);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#202283,#202285);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#202171,#202173);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#201191,#201193);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#153285,#153287);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#323694,#323696);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#165873,#165875);").is_ok());
    }

    #[test]
    fn test_curve_style() {
        assert!(data_entity_curve_style("CURVE_STYLE('',#412981,POSITIVE_LENGTH_MEASURE(0.1),#406004);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#402574,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#402223,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#412935,POSITIVE_LENGTH_MEASURE(0.1),#403072);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#406720,POSITIVE_LENGTH_MEASURE(0.1),#406718);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#402709,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#413044,POSITIVE_LENGTH_MEASURE(0.1),#406004);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#401863,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#413440,POSITIVE_LENGTH_MEASURE(0.1),#406004);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#416525,POSITIVE_LENGTH_MEASURE(0.1),#416523);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#416494,POSITIVE_LENGTH_MEASURE(0.1),#400136);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#401701,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#403063,POSITIVE_LENGTH_MEASURE(0.1),#399650);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#406231,POSITIVE_LENGTH_MEASURE(0.1),#406004);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#413179,POSITIVE_LENGTH_MEASURE(0.1),#406004);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#402475,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#402961,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#401800,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#402781,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#406204,POSITIVE_LENGTH_MEASURE(0.1),#406004);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#412136,POSITIVE_LENGTH_MEASURE(0.1),#403051);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#402151,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#403015,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#416484,POSITIVE_LENGTH_MEASURE(0.1),#403072);").is_ok());
        assert!(data_entity_curve_style("CURVE_STYLE('',#402358,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok());
    }

    #[test]
    fn test_cylindrical_surface() {
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#243866,1.8);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#363261,0.635);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#25154,1.62E-002);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#327547,1.62E-002);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#387251,5.E-002);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#393187,1.2E-002);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#312188,1.3);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#5980,0.1);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#291598,1.3);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#241430,1.8);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#389940,0.126);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#4480,0.4499991);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#394106,0.125);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#133945,0.1);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#169245,0.25);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#332123,5.E-002);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#298239,4.2);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#4282,1.15000024);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#190737,5.E-002);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#4051,0.7999984);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#188570,5.E-002);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#145130,0.3);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#300849,4.2);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#41606,3.E-002);").is_ok());
        assert!(data_entity_cylindrical_surface("CYLINDRICAL_SURFACE('',#260947,8.E-002);").is_ok());
    }

    #[test]
    fn test_derived_unit() {
        assert!(data_entity_derived_unit("DERIVED_UNIT((#427287));").is_ok());
        assert!(data_entity_derived_unit("DERIVED_UNIT((#427258));").is_ok());
        assert!(data_entity_derived_unit("DERIVED_UNIT((#427280));").is_ok());
        assert!(data_entity_derived_unit("DERIVED_UNIT((#427265));").is_ok());
    }

    #[test]
    fn test_derived_unit_element() {
        assert!(data_entity_derived_unit_element("DERIVED_UNIT_ELEMENT(#427288,3.);").is_ok());
        assert!(data_entity_derived_unit_element("DERIVED_UNIT_ELEMENT(#427259,2.);").is_ok());
        assert!(data_entity_derived_unit_element("DERIVED_UNIT_ELEMENT(#427281,2.);").is_ok());
        assert!(data_entity_derived_unit_element("DERIVED_UNIT_ELEMENT(#427266,3.);").is_ok());
    }

    #[test]
    fn test_descriptive_representation_item() {
        assert!(data_entity_descriptive_representation_item("DESCRIPTIVE_REPRESENTATION_ITEM('MOD_NUM','MOD41249');").is_ok());
        assert!(data_entity_descriptive_representation_item("DESCRIPTIVE_REPRESENTATION_ITEM('PART_REV','B');").is_ok());
    }

    #[test]
    fn test_direction() {
        assert!(data_entity_direction("DIRECTION('',(1.,0.E+000,0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.917281684548,-0.E+000,0.398239012644));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,-0.537075932183,0.843533901553));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(-1.,0.E+000,0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(1.,0.E+000,0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(1.,0.E+000,-0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(1.,0.E+000,-0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(1.,0.E+000,0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(-1.,0.E+000,0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(-1.,0.E+000,0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,-5.892403111334E-016,1.));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,1.,-0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(-0.407575994194,-0.913171292232,0.E+000));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_entity_direction("DIRECTION('',(-0.E+000,-1.,0.E+000));").is_ok());
    }

    #[test]
    fn test_draughting_pre_defined_colour() {
        assert!(data_entity_draughting_pre_defined_colour("DRAUGHTING_PRE_DEFINED_COLOUR('green');").is_ok());
        assert!(data_entity_draughting_pre_defined_colour("DRAUGHTING_PRE_DEFINED_COLOUR('white');").is_ok());
        assert!(data_entity_draughting_pre_defined_colour("DRAUGHTING_PRE_DEFINED_COLOUR('black');").is_ok());
        assert!(data_entity_draughting_pre_defined_colour("DRAUGHTING_PRE_DEFINED_COLOUR('yellow');").is_ok());
    }

    #[test]
    fn test_draughting_pre_defined_curve_font() {
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
    }

    #[test]
    fn test_edge_curve() {
        assert!(data_entity_edge_curve("EDGE_CURVE('',#111835,#119485,#119493,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#285778,#286392,#286394,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#83157,#83061,#83175,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#19698,#19729,#19731,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#215243,#215251,#215253,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#187057,#187083,#187085,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#240221,#276642,#276644,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#373112,#373131,#373140,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#73809,#70109,#73811,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#215499,#222691,#222693,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#144644,#144652,#144654,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#56511,#56503,#56513,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#259884,#206446,#259886,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#22515,#21549,#22523,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#58793,#58785,#58795,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#60828,#60837,#60839,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#101010,#101002,#101012,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#86427,#86353,#86429,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#396849,#396880,#396882,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#376613,#376819,#376821,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#6656,#6648,#6658,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#365227,#366098,#366100,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#113032,#113021,#113040,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#368970,#368978,#368980,.T.);").is_ok());
        assert!(data_entity_edge_curve("EDGE_CURVE('',#32121,#32113,#32123,.T.);").is_ok());
    }

    #[test]
    fn test_edge_loop() {
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#72646,#72647,#72656,#72663));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#305283,#305284,#305293,#305302));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#306013,#306014,#306020,#306021));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#29498,#29506,#29514,#29520));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#240673,#240674,#240680,#240681,#240682,#240688,\n    #240689,#240697));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#88124,#88125,#88126,#88127,#88128,#88129,#88137,\n    #88145));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#176525,#176526,#176535,#176543));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#87677,#87678,#87679,#87685));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#54148,#54149,#54157,#54165));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#147470,#147471,#147479,#147487));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#265225,#265226,#265235,#265244,#265252,#265259)\n  );").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#355494,#355502,#355511,#355519,#355520));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#66381,#66390,#66398,#66405));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#112534,#112542,#112543,#112544,#112545,#112553)\n  );").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#11787,#11796,#11797,#11806));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#5954,#5955,#5964,#5972));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#122644,#122645,#122651,#122652));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#307219,#307220,#307226,#307227));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#389069,#389070,#389071,#389072));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#26134,#26135,#26143,#26151));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#382746,#382747,#382754,#382755));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#301175,#301176,#301177,#301185,#301193,#301201,\n    #301209));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#241877,#241878,#241879,#241880));").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#368519,#368520,#368521,#368522,#368523,#368524)\n  );").is_ok());
        assert!(data_entity_edge_loop("EDGE_LOOP('',(#32622,#32630,#32636,#32637));").is_ok());
    }

    #[test]
    fn test_ellipse() {
        assert!(data_entity_ellipse("ELLIPSE('',#181654,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#184302,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#184903,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#175139,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#177791,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#178501,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#133252,0.150260191002,0.10625);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#174489,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#185029,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#185044,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#181780,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#177725,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#184236,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#184253,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#178516,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#175154,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#174472,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#180972,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#181055,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#181765,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#175280,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#184918,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#180989,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#177774,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("ELLIPSE('',#133235,0.150260191002,0.10625);").is_ok());
    }

    #[test]
    fn test_face_bound() {
        assert!(data_entity_face_bound("FACE_BOUND('',#72645,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#305282,.F.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#306012,.F.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#29497,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#240672,.F.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#88123,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#176524,.F.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#87676,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#54147,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#147469,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#265224,.F.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#355493,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#66380,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#112533,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#11786,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#5953,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#122643,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#307218,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#389068,.F.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#26133,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#382745,.F.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#301174,.F.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#241876,.F.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#368518,.T.);").is_ok());
        assert!(data_entity_face_bound("FACE_BOUND('',#32621,.T.);").is_ok());
    }

    #[test]
    fn test_fill_area_style() {
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#412124));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#424065));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#417993));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#400477));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#409795));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#402356));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#421958));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#418273));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#417139));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#400729));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#422175));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#422966));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#408452));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#424534));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#401532));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#407969));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#419554));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#424310));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#404493));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#414294));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#410431));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#407324));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#403560));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#401690));").is_ok());
        assert!(data_entity_fill_area_style("FILL_AREA_STYLE('',(#412345));").is_ok());
    }

    #[test]
    fn test_fill_area_style_colour() {
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#403072);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#416551);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#416551);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#399418);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#399650);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#401618);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#421110);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#416551);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#416551);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#399418);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#421110);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#399418);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#399650);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#416551);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#400881);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#400881);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#416551);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#416551);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#399418);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#403349);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#399650);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#406760);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#399418);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#401618);").is_ok());
        assert!(data_entity_fill_area_style_colour("FILL_AREA_STYLE_COLOUR('',#399418);").is_ok());
    }

    #[test]
    fn test_geometric_curve_set() {
        assert!(data_entity_geometric_curve_set("GEOMETRIC_CURVE_SET('',(#371717,#371725));").is_ok());
        assert!(data_entity_geometric_curve_set("GEOMETRIC_CURVE_SET('',(#371700,#371708));").is_ok());
    }

    #[test]
    fn test_item_defined_transformation() {
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#198913);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#683);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#1283);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#146983);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#166003);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#333114);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#24167);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#387620);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#146691);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#1075);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#138216);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#323399);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#395098);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#129292);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#42997);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#147311);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#423);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#47);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#374606);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#202273);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#202161);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#201181);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#153128);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#1151);").is_ok());
        assert!(data_entity_item_defined_transformation("ITEM_DEFINED_TRANSFORMATION('','',#11,#165863);").is_ok());
    }

    #[test]
    fn test_line() {
        assert!(data_entity_line("LINE('',#205153,#205154);").is_ok());
        assert!(data_entity_line("LINE('',#233846,#233847);").is_ok());
        assert!(data_entity_line("LINE('',#250073,#250074);").is_ok());
        assert!(data_entity_line("LINE('',#206849,#206850);").is_ok());
        assert!(data_entity_line("LINE('',#48223,#48224);").is_ok());
        assert!(data_entity_line("LINE('',#228178,#228179);").is_ok());
        assert!(data_entity_line("LINE('',#110196,#110197);").is_ok());
        assert!(data_entity_line("LINE('',#36891,#36892);").is_ok());
        assert!(data_entity_line("LINE('',#144070,#144071);").is_ok());
        assert!(data_entity_line("LINE('',#91760,#91761);").is_ok());
        assert!(data_entity_line("LINE('',#397906,#397907);").is_ok());
        assert!(data_entity_line("LINE('',#267329,#267330);").is_ok());
        assert!(data_entity_line("LINE('',#140398,#140399);").is_ok());
        assert!(data_entity_line("LINE('',#184450,#184451);").is_ok());
        assert!(data_entity_line("LINE('',#71807,#71808);").is_ok());
        assert!(data_entity_line("LINE('',#272607,#272608);").is_ok());
        assert!(data_entity_line("LINE('',#216534,#216535);").is_ok());
        assert!(data_entity_line("LINE('',#393522,#393523);").is_ok());
        assert!(data_entity_line("LINE('',#353992,#353993);").is_ok());
        assert!(data_entity_line("LINE('',#121426,#121427);").is_ok());
        assert!(data_entity_line("LINE('',#32177,#32178);").is_ok());
        assert!(data_entity_line("LINE('',#205977,#205978);").is_ok());
        assert!(data_entity_line("LINE('',#77176,#77177);").is_ok());
        assert!(data_entity_line("LINE('',#100319,#100320);").is_ok());
        assert!(data_entity_line("LINE('',#57803,#57804);").is_ok());
    }

    #[test]
    fn test_manifold_solid_brep() {
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#395362);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#157779);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#195674);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#374391);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#149884);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#11153);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#152807);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#136273);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#26705);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#202441);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#330032);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#45829);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#332933);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#385683);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#331756);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#373560);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#160740);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#156900);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#193028);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#154027);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#171915);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#164808);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#174415);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#154879);").is_ok());
        assert!(data_entity_manifold_solid_brep("MANIFOLD_SOLID_BREP('',#386536);").is_ok());
    }

    #[test]
    fn test_manifold_surface_shape_representation() {
        assert!(data_entity_manifold_surface_shape_representation("MANIFOLD_SURFACE_SHAPE_REPRESENTATION('',(#11,#383554),#383590\n  );").is_ok());
        assert!(data_entity_manifold_surface_shape_representation("MANIFOLD_SURFACE_SHAPE_REPRESENTATION('',(#11,#385621),#385657\n  );").is_ok());
        assert!(data_entity_manifold_surface_shape_representation("MANIFOLD_SURFACE_SHAPE_REPRESENTATION('',(#11,#131064),#131083\n  );").is_ok());
        assert!(data_entity_manifold_surface_shape_representation("MANIFOLD_SURFACE_SHAPE_REPRESENTATION('',(#11,#5821),#5840);").is_ok());
        assert!(data_entity_manifold_surface_shape_representation("MANIFOLD_SURFACE_SHAPE_REPRESENTATION('',(#11,#129314),#129333\n  );").is_ok());
        assert!(data_entity_manifold_surface_shape_representation("MANIFOLD_SURFACE_SHAPE_REPRESENTATION('',(#11,#389232),#389268\n  );").is_ok());
    }

    #[test]
    fn test_measure_representation_item() {
        assert!(data_entity_measure_representation_item("MEASURE_REPRESENTATION_ITEM('volume measure',VOLUME_MEASURE(\n    109.45690237608),#427286);").is_ok());
        assert!(data_entity_measure_representation_item("MEASURE_REPRESENTATION_ITEM('surface area measure',\n  AREA_MEASURE(3.584814638318E+003),#427257);").is_ok());
        assert!(data_entity_measure_representation_item("MEASURE_REPRESENTATION_ITEM('surface area measure',\n  AREA_MEASURE(328.74256832),#427279);").is_ok());
        assert!(data_entity_measure_representation_item("MEASURE_REPRESENTATION_ITEM('volume measure',VOLUME_MEASURE(\n    1.977283702949E+003),#427264);").is_ok());
    }

    #[test]
    fn test_mechanical_design_geometric_presentation_representation() {
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #416537,#416544,#416552,#416559,#416566,#416573,#416580,#416587,\n    #416594,#416601,#416608,#416615,#416622,#416629,#416636,#416643,\n    #416650,#416657,#416664,#416671,#416678,#416685,#416692,#416699,\n    #416706,#416713,#416720,#416727,#416734,#416741,#416748,#416755,\n    #416762,#416769,#416776,#416783,#416790,#416797,#416804,#416811,\n    #416818,#416825,#416832,#416839,#416846,#416853,#416860,#416867,\n    #416874,#416881,#416888,#416895,#416902,#416909,#416916,#416923,\n    #416930,#416937,#416944,#416951,#416958,#416965,#416972,#416979,\n    #416986,#416993,#417000,#417007,#417014,#417021,#417028,#417035,\n    #417042,#417049,#417056,#417063,#417070,#417077,#417084,#417091,\n    #417098,#417105,#417112,#417119,#417126,#417133,#417140,#417147,\n    #417154,#417161,#417168,#417175,#417182,#417189,#417196,#417203,\n    #417210,#417217,#417224,#417231,#417238,#417245,#417252,#417259,\n    #417266,#417273,#417280,#417287,#417294,#417301,#417308,#417315,\n    #417322,#417329,#417336,#417343,#417350,#417357,#417364,#417371,\n    #417378,#417385,#417392,#417399,#417406,#417413,#417420,#417427,\n    #417434,#417441,#417448,#417455,#417462,#417469,#417476,#417483,\n    #417490,#417497,#417504,#417511,#417518,#417525,#417532,#417539,\n    #417546,#417553,#417560,#417567,#417574,#417581,#417588,#417595,\n    #417602,#417609,#417616,#417623,#417630,#417637,#417644,#417651,\n    #417658,#417665,#417672,#417679,#417686,#417693,#417700,#417707,\n    #417714,#417721,#417728,#417735,#417742,#417749,#417756,#417763,\n    #417770,#417777,#417784,#417791,#417798,#417805,#417812,#417819,\n    #417826,#417833,#417840,#417847,#417854,#417861,#417868,#417875,\n    #417882,#417889,#417896,#417903,#417910,#417917,#417924,#417931,\n    #417938,#417945,#417952,#417959,#417966,#417973,#417980,#417987,\n    #417994,#418001,#418008,#418015,#418022,#418029,#418036,#418043,\n    #418050,#418057,#418064,#418071,#418078,#418085,#418092,#418099,\n    #418106,#418113,#418120,#418127,#418134,#418141,#418148,#418155,\n    #418162,#418169,#418176,#418183,#418190,#418197,#418204,#418211,\n    #418218,#418225,#418232,#418239,#418246,#418253,#418260,#418267,\n    #418274,#418281,#418288,#418295,#418302,#418309,#418316,#418323,\n    #418330,#418337,#418344,#418351,#418358,#418365,#418372,#418379,\n    #418386,#418393,#418400,#418407,#418414,#418421,#418428,#418435,\n    #418442,#418449,#418456,#418463,#418470,#418477,#418484,#418491,\n    #418498,#418505,#418512,#418519,#418526,#418533,#418540,#418547,\n    #418554,#418561,#418568,#418575,#418582,#418589,#418596,#418603,\n    #418610,#418617,#418624,#418631,#418638,#418645,#418652,#418659,\n    #418666,#418673,#418680,#418687,#418694,#418701,#418708,#418715,\n    #418722,#418729,#418736,#418743,#418750,#418757,#418764,#418771,\n    #418778,#418785,#418792,#418799,#418806,#418813,#418820,#418827,\n    #418834,#418841,#418848,#418855,#418862,#418869,#418876,#418883,\n    #418890,#418897,#418904,#418911,#418918,#418925,#418932,#418939,\n    #418946,#418953,#418960,#418967,#418974,#418981,#418988,#418995,\n    #419002,#419009,#419016,#419023,#419030,#419037,#419044,#419051,\n    #419058,#419065,#419072,#419079,#419086,#419093,#419100,#419107,\n    #419114,#419121,#419128,#419135,#419142,#419149,#419156,#419163,\n    #419170,#419177,#419184,#419191,#419198,#419205,#419212,#419219,\n    #419226,#419233,#419240,#419247,#419254,#419261,#419268,#419275,\n    #419282,#419289,#419296,#419303,#419310,#419317,#419324,#419331,\n    #419338,#419345,#419352,#419359,#419366,#419373,#419380,#419387,\n    #419394,#419401,#419408,#419415,#419422,#419429,#419436,#419443,\n    #419450,#419457,#419464,#419471,#419478,#419485,#419492,#419499,\n    #419506,#419513,#419520,#419527,#419534,#419541,#419548,#419555,\n    #419562,#419569,#419576,#419583,#419590,#419597,#419604,#419611,\n    #419618,#419625,#419632,#419639,#419646,#419653,#419660,#419667,\n    #419674,#419681,#419688,#419695,#419702,#419709,#419716,#419723,\n    #419730,#419737,#419744,#419751,#419758,#419765,#419772,#419779,\n    #419786,#419793,#419800,#419807,#419814,#419821,#419828,#419835,\n    #419842,#419849,#419856,#419863,#419870,#419877,#419884,#419891,\n    #419898,#419905,#419912,#419919,#419926,#419933,#419940,#419947,\n    #419954,#419961,#419968,#419975,#419982,#419989,#419996,#420003,\n    #420010,#420017,#420024,#420031,#420038,#420045,#420052,#420059,\n    #420066,#420073,#420080,#420087,#420094,#420101,#420108,#420115,\n    #420122,#420129,#420136,#420143,#420150,#420157,#420164,#420171,\n    #420178,#420185,#420192,#420199,#420206,#420213,#420220,#420227,\n    #420234,#420241,#420248,#420255,#420262,#420269,#420276,#420283,\n    #420290,#420297,#420304,#420311,#420318,#420325,#420332,#420339,\n    #420346,#420353,#420360,#420367,#420374,#420381,#420388,#420395,\n    #420402,#420409,#420416,#420423,#420430,#420437,#420444,#420451,\n    #420458,#420465,#420472,#420479,#420486,#420493,#420500,#420507,\n    #420514,#420521,#420528,#420535,#420542,#420549,#420556,#420563,\n    #420570,#420577,#420584,#420591,#420598,#420605,#420612,#420619,\n    #420626,#420633,#420640,#420647,#420654,#420661,#420668,#420675,\n    #420682,#420689,#420696,#420703,#420710,#420717,#420724,#420731,\n    #420738,#420745,#420752,#420759,#420766,#420773,#420780,#420787,\n    #420794,#420801,#420808,#420815,#420822,#420829,#420836,#420843,\n    #420850,#420857,#420864,#420871,#420878,#420885,#420892,#420899,\n    #420906,#420913,#420920,#420927,#420934,#420941,#420948,#420955,\n    #420962,#420969,#420976,#420983,#420990,#420997,#421004,#421011,\n    #421018,#421025,#421032,#421039,#421046,#421053,#421060,#421067,\n    #421074,#421081,#421088,#421096,#421103,#421111,#421118,#421125,\n    #421133,#421140,#421147,#421154,#421161,#421168,#421175,#421182,\n    #421189,#421196,#421203,#421210,#421217,#421224,#421231,#421238,\n    #421245,#421252,#421259,#421266,#421273,#421280,#421287,#421294,\n    #421301,#421308,#421315,#421322,#421329,#421336,#421343,#421350,\n    #421357,#421364,#421371,#421378,#421385,#421392,#421399,#421406,\n    #421413,#421420,#421427,#421434,#421441,#421448,#421455,#421462,\n    #421469,#421476,#421483,#421490,#421497,#421504,#421511,#421518,\n    #421525,#421532,#421539,#421546,#421553,#421560,#421567,#421574,\n    #421581,#421588,#421595,#421602,#421609,#421616,#421623,#421630,\n    #421637,#421644,#421651,#421658,#421665,#421672,#421679,#421686,\n    #421693,#421700,#421707,#421714,#421721,#421728,#421735,#421742,\n    #421749,#421756,#421763,#421770,#421777,#421784,#421791,#421798,\n    #421805,#421812,#421819,#421826,#421833,#421840,#421847,#421854,\n    #421861,#421868,#421875,#421882,#421889,#421896,#421903,#421910,\n    #421917,#421924,#421931,#421938,#421945,#421952,#421959,#421966,\n    #421973,#421980,#421987,#421994,#422001,#422008,#422015,#422022,\n    #422029,#422036,#422043,#422050,#422057,#422064,#422071,#422078,\n    #422085,#422092,#422099,#422106,#422113,#422120,#422127,#422134,\n    #422141,#422148,#422155,#422162,#422169,#422176,#422183,#422190,\n    #422197,#422204,#422211,#422218,#422225,#422232,#422239,#422246,\n    #422253,#422260,#422267,#422274,#422281,#422288,#422295,#422302,\n    #422309,#422316,#422323,#422330,#422337,#422344,#422351,#422358,\n    #422365,#422372,#422379,#422386,#422393,#422400,#422407,#422414,\n    #422421,#422428,#422435,#422442,#422449,#422456,#422463,#422470,\n    #422477,#422484,#422491,#422498,#422505,#422512,#422519,#422526,\n    #422533,#422540,#422547,#422554,#422561,#422568,#422575,#422582,\n    #422589,#422596,#422603,#422610,#422617,#422624,#422631,#422638,\n    #422645,#422652,#422659,#422666,#422673,#422680,#422687,#422694,\n    #422701,#422708,#422715,#422722,#422729,#422736,#422743,#422750,\n    #422757,#422764,#422771,#422778,#422785,#422792,#422799,#422806,\n    #422813,#422820,#422827,#422834,#422841,#422848,#422855,#422862,\n    #422869,#422876,#422883,#422890,#422897,#422904,#422911,#422918,\n    #422925,#422932,#422939,#422946,#422953,#422960,#422967,#422974,\n    #422981,#422988,#422995,#423002,#423009,#423016,#423023,#423030,\n    #423037,#423044,#423051,#423058,#423065,#423072,#423079,#423086,\n    #423093,#423100,#423107,#423114,#423121,#423128,#423135,#423142,\n    #423149,#423156,#423163,#423170,#423177,#423184,#423191,#423198,\n    #423205,#423212,#423219,#423226,#423233,#423240,#423247,#423254,\n    #423261,#423268,#423275,#423282,#423289,#423296,#423303,#423310,\n    #423317,#423324,#423331,#423338,#423345,#423352,#423359,#423366,\n    #423373,#423380,#423387,#423394,#423401,#423408,#423415,#423422,\n    #423429,#423436,#423443,#423450,#423457,#423464,#423471,#423478,\n    #423485,#423492,#423499,#423506,#423513,#423520,#423527,#423534,\n    #423541,#423548,#423555,#423562,#423569,#423576,#423583,#423590,\n    #423597,#423604,#423611,#423618,#423625,#423632,#423639,#423646,\n    #423653,#423660,#423667,#423674,#423681,#423688,#423695,#423702,\n    #423709,#423716,#423723,#423730,#423737,#423744,#423751,#423758,\n    #423765,#423772,#423779,#423786,#423793,#423800,#423807,#423814,\n    #423821,#423828,#423835,#423842,#423849,#423856,#423863,#423870,\n    #423877,#423884,#423891,#423898,#423905,#423912,#423919,#423926,\n    #423933,#423940,#423947,#423954,#423961,#423968,#423975,#423982,\n    #423989,#423996,#424003,#424010,#424017,#424024,#424031,#424038,\n    #424045,#424052,#424059,#424066,#424073,#424080,#424087,#424094,\n    #424101,#424108,#424115,#424122,#424129,#424136,#424143,#424150,\n    #424157,#424164,#424171,#424178,#424185,#424192,#424199,#424206,\n    #424213,#424220,#424227,#424234,#424241,#424248,#424255,#424262,\n    #424269,#424276,#424283,#424290,#424297,#424304,#424311,#424318,\n    #424325,#424332,#424339,#424346,#424353,#424360,#424367,#424374,\n    #424381,#424388,#424395,#424402,#424409,#424416,#424423,#424430,\n    #424437,#424444,#424451,#424458,#424465,#424472,#424479,#424486,\n    #424493,#424500,#424507,#424514,#424521,#424528,#424535,#424542,\n    #424549,#424556,#424563,#424570,#424577,#424584,#424591,#424598,\n    #424605,#424612,#424619,#424626,#424633,#424640,#424647,#424654,\n    #424661,#424668,#424675,#424682,#424689,#424696,#424703,#424710,\n    #424717,#424724,#424731,#424738,#424745,#424752,#424759,#424766,\n    #424773,#424780,#424787,#424794,#424801,#424808,#424815,#424822,\n    #424829,#424836,#424843,#424850,#424857,#424864,#424871,#424878,\n    #424885,#424892,#424899,#424906,#424913,#424920,#424927,#424934,\n    #424941,#424948,#424955,#424962,#424969,#424976,#424983,#424990,\n    #424997,#425004,#425011,#425018,#425025,#425032,#425039,#425046,\n    #425053,#425060,#425067,#425074,#425081,#425088,#425095,#425102,\n    #425109,#425116,#425123,#425130,#425137,#425144,#425151,#425158,\n    #425165,#425172,#425179,#425186,#425193,#425200,#425207,#425214,\n    #425221,#425228,#425235,#425242,#425249,#425256,#425263,#425270,\n    #425277,#425284,#425291,#425298,#425305,#425312,#425319,#425326,\n    #425333,#425340,#425347,#425354,#425361,#425368,#425375,#425382,\n    #425389,#425396,#425403,#425410,#425417,#425424,#425431,#425438,\n    #425445,#425452,#425459,#425466,#425473,#425480,#425487,#425494,\n    #425501,#425508,#425515,#425522,#425529,#425536,#425543,#425550,\n    #425557,#425564,#425571,#425578,#425585,#425592,#425599,#425606,\n    #425613,#425620,#425627,#425634,#425641,#425648,#425655,#425662,\n    #425669,#425676,#425683,#425690,#425697,#425704,#425711,#425718,\n    #425725,#425732,#425739,#425746,#425753,#425760,#425767,#425774,\n    #425781,#425788,#425795,#425802,#425809,#425816,#425823,#425830,\n    #425837,#425844,#425851,#425858,#425865,#425872,#425879,#425886,\n    #425893,#425900,#425907,#425914,#425921,#425928,#425935,#425942,\n    #425949),#90227);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #411695),#180791);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #416476),#180224);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #412907),#155907);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #403076,#403083,#403090,#403097,#403104,#403111,#403118,#403125,\n    #403132,#403139,#403146,#403153,#403160,#403167,#403174,#403181,\n    #403188,#403195,#403202,#403209,#403216,#403223,#403230,#403237,\n    #403244,#403251,#403258,#403265,#403272,#403279,#403286,#403293,\n    #403300,#403307,#403314,#403321,#403328,#403335,#403342,#403350,\n    #403357,#403364,#403371,#403378,#403385,#403392,#403399,#403406,\n    #403413),#194225);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #426048),#5307);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #411764,#411771,#411778,#411785,#411792,#411799,#411806,#411813,\n    #411820,#411827,#411834,#411841,#411848,#411855,#411862,#411869,\n    #411876,#411883,#411890,#411897,#411904,#411911,#411918,#411925,\n    #411932,#411939,#411946,#411953,#411960,#411967,#411974,#411981,\n    #411988,#411995,#412002,#412009,#412016,#412023,#412030,#412037,\n    #412044,#412051,#412058,#412065,#412072,#412079,#412086,#412093,\n    #412100),#387279);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #401611,#401621,#401630,#401639,#401648,#401657,#401666,#401675,\n    #401684,#401693,#401702,#401711,#401720,#401729,#401738,#401747,\n    #401756,#401765,#401774,#401783,#401792,#401801,#401810,#401819,\n    #401828,#401837,#401846,#401855,#401864,#401873,#401882,#401891,\n    #401900,#401909,#401918,#401927,#401936,#401945,#401954,#401963,\n    #401972,#401981,#401990,#401999,#402008,#402017,#402026,#402035,\n    #402044,#402053,#402062,#402071,#402080,#402089,#402098,#402107,\n    #402116,#402125,#402134,#402143,#402152,#402161,#402170,#402179,\n    #402188,#402197,#402206,#402215,#402224,#402233,#402242,#402251,\n    #402260,#402269,#402278,#402287,#402296,#402305,#402314,#402323,\n    #402332,#402341,#402350,#402359,#402368,#402377,#402386,#402395,\n    #402404,#402413,#402422,#402431,#402440,#402449,#402458,#402467,\n    #402476,#402485,#402494,#402503,#402512,#402521,#402530,#402539,\n    #402548,#402557,#402566,#402575,#402584,#402593,#402602,#402611,\n    #402620,#402629,#402638,#402647,#402656,#402665,#402674,#402683,\n    #402692,#402701,#402710,#402719,#402728,#402737,#402746,#402755,\n    #402764,#402773,#402782,#402791,#402800,#402809,#402818,#402827,\n    #402836,#402845,#402854,#402863,#402872,#402881,#402890,#402899,\n    #402908,#402917,#402926,#402935,#402944,#402953,#402962,#402971,\n    #402980,#402989,#402998,#403007,#403016,#403025,#403034),#145146);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #407083),#399327);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #407053),#384850);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #405997,#406007,#406016,#406025,#406034,#406043,#406052,#406061,\n    #406070,#406079,#406088,#406097,#406106,#406115,#406124,#406133,\n    #406142,#406151,#406160,#406169,#406178,#406187,#406196,#406205,\n    #406214,#406223,#406232,#406241,#406250,#406259,#406268,#406277,\n    #406286,#406295,#406304,#406313,#406322,#406331,#406340,#406349,\n    #406358,#406367,#406376,#406385,#406394,#406403,#406412,#406421,\n    #406430,#406439,#406448,#406457,#406466,#406475,#406484,#406493,\n    #406502,#406511,#406520,#406529),#337371);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #406584,#406591,#406598,#406605,#406612,#406619,#406626,#406633,\n    #406640,#406647,#406654,#406661,#406668,#406675,#406682,#406689,\n    #406696,#406703),#18731);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #416506),#136247);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #412128),#8791);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #411705),#385657);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #415627,#415634,#415641,#415648,#415655,#415662,#415669,#415676,\n    #415683,#415690,#415697,#415704,#415711,#415718,#415725,#415732,\n    #415739,#415746,#415753,#415760,#415767,#415774,#415781,#415788,\n    #415795,#415802,#415809,#415816,#415823,#415830,#415837,#415844,\n    #415851,#415858,#415865,#415872,#415879,#415886,#415893,#415900,\n    #415907,#415914,#415921,#415928,#415935,#415942,#415949,#415956,\n    #415963),#372979);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #406743),#378793);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #426018),#155028);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #426157,#426164,#426171,#426178,#426185,#426192,#426199,#426206,\n    #426213,#426220,#426227,#426234,#426241,#426248),#391984);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #427240),#139672);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #408628),#389268);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #412563,#412570,#412577,#412584,#412591,#412598,#412605,#412612,\n    #412619,#412626,#412633,#412640,#412647,#412654,#412661,#412668,\n    #412675,#412682,#412689,#412696,#412703,#412710,#412717,#412724,\n    #412731,#412738,#412745,#412752,#412759,#412766,#412773,#412780,\n    #412787,#412794,#412801,#412808,#412815,#412822,#412829,#412836,\n    #412843,#412850,#412857,#412864,#412871,#412878,#412885,#412892,\n    #412899),#196417);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #412937,#412946,#412955,#412964,#412973,#412982,#412991,#413000,\n    #413009,#413018,#413027,#413036,#413045,#413054,#413063,#413072,\n    #413081,#413090,#413099,#413108,#413117,#413126,#413135,#413144,\n    #413153,#413162,#413171,#413180,#413189,#413198,#413207,#413216,\n    #413225,#413234,#413243,#413252,#413261,#413270,#413279,#413288,\n    #413297,#413306,#413315,#413324,#413333,#413342,#413351,#413360,\n    #413369,#413378,#413387,#413396,#413405,#413414,#413423,#413432,\n    #413441,#413450,#413459,#413468,#413477,#413486,#413495,#413504,\n    #413513,#413522,#413531,#413540),#335375);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #413943),#162321);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(\n    #412553),#153593);").is_ok());
    }

    #[test]
    fn test_next_assembly_usage_occurrence() {
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('3943','869','',#198907,#153103\n  ,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('2918','','',#5,#137426,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4381','','',#5,#335401,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4036','962','',#145221,#200195\n  ,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('3275','201','',#165997,#150290\n  ,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4369','','',#333108,#332216,$\n  );").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('2579','','',#24161,#19978,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4518','15','',#387602,#389450,\n  $);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('3890','816','',#145221,#198151\n  ,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4270','','',#5,#323141,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('2973','','',#138210,#5554,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4287','','',#323393,#11089,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4573','','',#395092,#381660,$\n  );").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('2831','11','',#129254,#129590,\n  $);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('2746','','',#42991,#19978,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4201','1127','',#145221,\n  #202678,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('2741','','',#5,#42907,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('2550','','',#5,#8942,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4413','','',#374600,#376191,$\n  );").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4183','1109','',#202267,\n  #153103,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4175','1101','',#202155,\n  #153103,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4105','1031','',#201175,\n  #153103,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('3084','10','',#153122,#153279,\n  $);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('4308','','',#5,#323673,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("NEXT_ASSEMBLY_USAGE_OCCURRENCE('3265','191','',#165857,#150290\n  ,$);").is_ok());
    }

    #[test]
    fn test_open_shell() {
        assert!(data_entity_open_shell("OPEN_SHELL('',(#383556));").is_ok());
        assert!(data_entity_open_shell("OPEN_SHELL('',(#385623));").is_ok());
        assert!(data_entity_open_shell("OPEN_SHELL('',(#131066));").is_ok());
        assert!(data_entity_open_shell("OPEN_SHELL('',(#5823));").is_ok());
        assert!(data_entity_open_shell("OPEN_SHELL('',(#129316));").is_ok());
        assert!(data_entity_open_shell("OPEN_SHELL('',(#389234));").is_ok());
    }

    #[test]
    fn test_oriented_closed_shell() {
        assert!(data_entity_oriented_closed_shell("ORIENTED_CLOSED_SHELL('',*,#90078,.F.);").is_ok());
        assert!(data_entity_oriented_closed_shell("ORIENTED_CLOSED_SHELL('',*,#89928,.F.);").is_ok());
    }

    #[test]
    fn test_oriented_edge() {
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#352147,.F.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#372770,.F.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#36809,.T.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#231406,.F.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#2195,.F.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#128924,.F.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#254526,.T.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#282321,.F.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#216634,.T.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#29894,.T.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#314534,.T.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#296836,.F.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#348565,.F.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#254358,.F.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#359954,.T.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#80840,.F.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#91666,.T.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#308557,.F.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#215618,.T.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#203781,.T.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#195374,.T.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#8281,.T.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#333305,.F.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#360031,.T.);").is_ok());
        assert!(data_entity_oriented_edge("ORIENTED_EDGE('',*,*,#234296,.F.);").is_ok());
    }

    #[test]
    fn test_over_riding_styled_item() {
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#420760),#67182,\n  #416537);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#422093),#74868,\n  #420934);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#403008),#145083,\n  #401621);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#426695),#25323,\n  #426631);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#410405),#375266,\n  #410396);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#412003),#386937,\n  #411897);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#408755),#391252,\n  #408648);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#415312),#38000,\n  #414772);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#422100),#74909,\n  #420934);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#425334),#87487,\n  #420934);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#422296),#75810,\n  #420934);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#418975),#60065,\n  #416537);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#424613),#85024,\n  #420934);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#419668),#62967,\n  #416537);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#422324),#75881,\n  #420934);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#406008),#335418,\n  #405997);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#421652),#73066,\n  #420934);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#424200),#83623,\n  #420934);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#420515),#66295,\n  #416537);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#420970),#67785,\n  #420934);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#415263),#37851,\n  #414772);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#418079),#54363,\n  #416537);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#416560),#46418,\n  #416537);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#414719),#40264,\n  #414676);").is_ok());
        assert!(data_entity_over_riding_styled_item("OVER_RIDING_STYLED_ITEM('overriding color',(#422674),#77695,\n  #420934);").is_ok());
    }

    #[test]
    fn test_plane() {
        assert!(data_entity_plane("PLANE('',#34393);").is_ok());
        assert!(data_entity_plane("PLANE('',#63225);").is_ok());
        assert!(data_entity_plane("PLANE('',#15412);").is_ok());
        assert!(data_entity_plane("PLANE('',#306146);").is_ok());
        assert!(data_entity_plane("PLANE('',#370704);").is_ok());
        assert!(data_entity_plane("PLANE('',#106351);").is_ok());
        assert!(data_entity_plane("PLANE('',#82156);").is_ok());
        assert!(data_entity_plane("PLANE('',#123902);").is_ok());
        assert!(data_entity_plane("PLANE('',#393420);").is_ok());
        assert!(data_entity_plane("PLANE('',#322073);").is_ok());
        assert!(data_entity_plane("PLANE('',#106368);").is_ok());
        assert!(data_entity_plane("PLANE('',#281752);").is_ok());
        assert!(data_entity_plane("PLANE('',#102829);").is_ok());
        assert!(data_entity_plane("PLANE('',#127989);").is_ok());
        assert!(data_entity_plane("PLANE('',#316586);").is_ok());
        assert!(data_entity_plane("PLANE('',#364067);").is_ok());
        assert!(data_entity_plane("PLANE('',#109044);").is_ok());
        assert!(data_entity_plane("PLANE('',#330383);").is_ok());
        assert!(data_entity_plane("PLANE('',#220611);").is_ok());
        assert!(data_entity_plane("PLANE('',#363771);").is_ok());
        assert!(data_entity_plane("PLANE('',#308633);").is_ok());
        assert!(data_entity_plane("PLANE('',#254179);").is_ok());
        assert!(data_entity_plane("PLANE('',#304266);").is_ok());
        assert!(data_entity_plane("PLANE('',#324374);").is_ok());
        assert!(data_entity_plane("PLANE('',#259534);").is_ok());
    }

    #[test]
    fn test_presentation_layer_assignment() {
        assert!(data_entity_presentation_layer_assignment("PRESENTATION_LAYER_ASSIGNMENT('NONE','visible',(#391469,\n    #391500,#390251,#390406,#390809,#391603,#390763,#391097,#390973,\n    #390211,#390623,#391946,#390561,#390468,#391314,#390880,#390716,\n    #390849,#391345,#389572,#391252,#391707,#391800,#391190,#391924,\n    #390911,#391159,#390313,#391522,#390344,#390499,#391862,#391965,\n    #391066,#391831,#391128,#391769,#391676,#391738,#390654,#391035,\n    #391553,#391605,#390807,#390282,#391376,#391407,#391645,#391438,\n    #390685,#392003,#390530,#390942,#390437,#390209,#391283,#391221,\n    #390375,#391004,#391893,#390738,#390592,#393264));").is_ok());
    }

    #[test]
    fn test_presentation_style_assignment() {
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#406513,#406518));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#424054));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#407292));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#400473));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#423753));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#402352,#402357));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#414566));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#412011));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#417135));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#400725));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#405506));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#419277));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#414893));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#411197));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#401528));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#417205));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#405999,#406005));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#424299));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#404489));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#425552));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#410247));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#408546));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#403556));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#401686,#401691));").is_ok());
        assert!(data_entity_presentation_style_assignment("PRESENTATION_STYLE_ASSIGNMENT((#418220));").is_ok());
    }

    #[test]
    fn test_presentation_style_by_context() {
        assert!(data_entity_presentation_style_by_context("PRESENTATION_STYLE_BY_CONTEXT((#410648,#410653),#399365);").is_ok());
        assert!(data_entity_presentation_style_by_context("PRESENTATION_STYLE_BY_CONTEXT((#406713,#406719),#399367);").is_ok());
        assert!(data_entity_presentation_style_by_context("PRESENTATION_STYLE_BY_CONTEXT((#426070,#426076),#399363);").is_ok());
    }

    #[test]
    fn test_product() {
        assert!(data_entity_product("PRODUCT('C66','C66','',(#200814));").is_ok());
        assert!(data_entity_product("PRODUCT('BAT1','BAT1','',(#395319));").is_ok());
        assert!(data_entity_product("PRODUCT('R152','R152','',(#153740));").is_ok());
        assert!(data_entity_product("PRODUCT('TP95','TP95','',(#136981));").is_ok());
        assert!(data_entity_product("PRODUCT('R102','R102','',(#322948));").is_ok());
        assert!(data_entity_product("PRODUCT('C176','C176','',(#197734));").is_ok());
        assert!(data_entity_product("PRODUCT('C42','C42','',(#387560));").is_ok());
        assert!(data_entity_product("PRODUCT('Extruded','Extruded','',(#129000));").is_ok());
        assert!(data_entity_product("PRODUCT('RESC310X160X65L45N','RESC310X160X65L45N','',(#27114));").is_ok());
        assert!(data_entity_product("PRODUCT('C250','C250','',(#194450));").is_ok());
        assert!(data_entity_product("PRODUCT('C220','C220','',(#196502));").is_ok());
        assert!(data_entity_product("PRODUCT('R58','R58','',(#323396));").is_ok());
        assert!(data_entity_product("PRODUCT('C170','C170','',(#197902));").is_ok());
        assert!(data_entity_product("PRODUCT('TP54','TP54','',(#137849));").is_ok());
        assert!(data_entity_product("PRODUCT('TP88','TP88','',(#137177));").is_ok());
        assert!(data_entity_product("PRODUCT('7324893888','7324893888','',(#128794));").is_ok());
        assert!(data_entity_product("PRODUCT('Board','Board','',(#148161));").is_ok());
        assert!(data_entity_product("PRODUCT('C240','C240','',(#194730));").is_ok());
        assert!(data_entity_product("PRODUCT('R48','R48','',(#323676));").is_ok());
        assert!(data_entity_product("PRODUCT('U26','U26','',(#129240));").is_ok());
        assert!(data_entity_product("PRODUCT('R153','R153','',(#165188));").is_ok());
        assert!(data_entity_product("PRODUCT('C9C','C9C','',(#395067));").is_ok());
        assert!(data_entity_product("PRODUCT('TP72','TP72','',(#137569));").is_ok());
        assert!(data_entity_product("PRODUCT('U11','U11','',(#130576));").is_ok());
        assert!(data_entity_product("PRODUCT('C22','C22','',(#394759));").is_ok());
    }

    #[test]
    fn test_product_category() {

    }

    #[test]
    fn test_product_context() {
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("PRODUCT_CONTEXT('',#2,'mechanical');").is_ok());
    }

    #[test]
    fn test_product_definition() {
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#200812,#200815);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#395317,#395320);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#153738,#153741);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#136979,#136982);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#322946,#322949);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#197732,#197735);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#387558,#387561);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#128998,#129001);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#27112,#27115);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#194448,#194451);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#196500,#196503);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#323394,#323397);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#197900,#197903);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#137847,#137850);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#137175,#137178);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#128792,#128795);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#148159,#148162);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#194728,#194731);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#323674,#323677);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#129238,#129241);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#165186,#165189);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#395065,#395068);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#137567,#137570);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#130574,#130577);").is_ok());
        assert!(data_entity_product_definition("PRODUCT_DEFINITION('design','',#394757,#394760);").is_ok());
    }

    #[test]
    fn test_product_definition_context() {
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
    }

    #[test]
    fn test_product_definition_formation() {
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#200813);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#395318);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#153739);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#136980);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#322947);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#197733);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#387559);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#128999);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#27113);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#194449);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#196501);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#323395);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#197901);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#137848);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#137176);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#128793);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#148160);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#194729);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#323675);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#129239);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#165187);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#395066);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#137568);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#130575);").is_ok());
        assert!(data_entity_product_definition_formation("PRODUCT_DEFINITION_FORMATION('','',#394758);").is_ok());
    }

    #[test]
    fn test_product_definition_formation_with_specified_source() {

    }

    #[test]
    fn test_product_definition_shape() {
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #167360);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #359400);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #27513);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #165797);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('','',#162783);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #153812);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('','',#194419);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('','',#381707);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #200046);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #24089);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #379022);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #129970);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #328164);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #167864);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('','',#163015);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('','',#166641);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #200443);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #202403);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('','',#12967);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('','',#383819);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #201955);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('','',#202734);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('','',#200923);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('Placement','Placement of an item',\n  #167785);").is_ok());
        assert!(data_entity_product_definition_shape("PRODUCT_DEFINITION_SHAPE('','',#323281);").is_ok());
    }

    #[test]
    fn test_product_related_product_category() {
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#200869));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#396675));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#153795));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#136980));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#322947));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#197789));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#387559));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#127904));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#24994));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#194505));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#196557));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#323395));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#197957));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#137848));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#137176));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#128793));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#148173));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#194785));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#323675));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#129342));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#165243));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#395066));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#137568));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#130765));").is_ok());
        assert!(data_entity_product_related_product_category("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#394758));").is_ok());
    }

    #[test]
    fn test_property_definition() {
        assert!(data_entity_property_definition("PROPERTY_DEFINITION('geometric_validation_property','centroid'\n  ,#427273);").is_ok());
        assert!(data_entity_property_definition("PROPERTY_DEFINITION('material property','material name',\n  #321263);").is_ok());
        assert!(data_entity_property_definition("PROPERTY_DEFINITION('shape with specific properties',\n  'properties for subshape',#427251);").is_ok());
        assert!(data_entity_property_definition("PROPERTY_DEFINITION('geometric_validation_property','volume',\n  #427251);").is_ok());
        assert!(data_entity_property_definition("PROPERTY_DEFINITION('geometric_validation_property','centroid'\n  ,#427251);").is_ok());
        assert!(data_entity_property_definition("PROPERTY_DEFINITION('geometric_validation_property',\n  'surface area',#427273);").is_ok());
        assert!(data_entity_property_definition("PROPERTY_DEFINITION('material property','material name',\n  #356697);").is_ok());
        assert!(data_entity_property_definition("PROPERTY_DEFINITION('geometric_validation_property',\n  'surface area',#427251);").is_ok());
        assert!(data_entity_property_definition("PROPERTY_DEFINITION('shape with specific properties',\n  'properties for subshape',#427273);").is_ok());
        assert!(data_entity_property_definition("PROPERTY_DEFINITION('material property','density',#321263);").is_ok());
        assert!(data_entity_property_definition("PROPERTY_DEFINITION('material property','density',#356697);").is_ok());
        assert!(data_entity_property_definition("PROPERTY_DEFINITION('geometric_validation_property','volume',\n  #427273);").is_ok());
    }

    #[test]
    fn test_property_definition_representation() {
        assert!(data_entity_property_definition_representation("PROPERTY_DEFINITION_REPRESENTATION(#427290,#427291);").is_ok());
        assert!(data_entity_property_definition_representation("PROPERTY_DEFINITION_REPRESENTATION(#399371,#399369);").is_ok());
        assert!(data_entity_property_definition_representation("PROPERTY_DEFINITION_REPRESENTATION(#427254,#427255);").is_ok());
        assert!(data_entity_property_definition_representation("PROPERTY_DEFINITION_REPRESENTATION(#427268,#427269);").is_ok());
        assert!(data_entity_property_definition_representation("PROPERTY_DEFINITION_REPRESENTATION(#427276,#427277);").is_ok());
        assert!(data_entity_property_definition_representation("PROPERTY_DEFINITION_REPRESENTATION(#427283,#427284);").is_ok());
        assert!(data_entity_property_definition_representation("PROPERTY_DEFINITION_REPRESENTATION(#399378,#399376);").is_ok());
        assert!(data_entity_property_definition_representation("PROPERTY_DEFINITION_REPRESENTATION(#427261,#427262);").is_ok());
        assert!(data_entity_property_definition_representation("PROPERTY_DEFINITION_REPRESENTATION(#399373,#399375);").is_ok());
        assert!(data_entity_property_definition_representation("PROPERTY_DEFINITION_REPRESENTATION(#399380,#399382);").is_ok());
    }

    #[test]
    fn test_representation() {
        assert!(data_entity_representation("REPRESENTATION('centroid',(#427292),#359378);").is_ok());
        assert!(data_entity_representation("REPRESENTATION('material name',(#399370),#321256);").is_ok());
        assert!(data_entity_representation("REPRESENTATION('surface area',(#427256),#321256);").is_ok());
        assert!(data_entity_representation("REPRESENTATION('centroid',(#427270),#321256);").is_ok());
        assert!(data_entity_representation("REPRESENTATION('surface area',(#427278),#359378);").is_ok());
        assert!(data_entity_representation("REPRESENTATION('volume',(#427285),#359378);").is_ok());
        assert!(data_entity_representation("REPRESENTATION('material name',(#399377),#356690);").is_ok());
        assert!(data_entity_representation("REPRESENTATION('volume',(#427263),#321256);").is_ok());
        assert!(data_entity_representation("REPRESENTATION($  /*   NUL REF   */,(),$\n    /*   NUL REF   */);").is_ok());
        assert!(data_entity_representation("REPRESENTATION($  /*   NUL REF   */,(),$\n    /*   NUL REF   */);").is_ok());
    }

    #[test]
    fn test_shape_aspect() {
        assert!(data_entity_shape_aspect("SHAPE_ASPECT('','',#359384,.F.);").is_ok());
        assert!(data_entity_shape_aspect("SHAPE_ASPECT('','',#321262,.F.);").is_ok());
    }

    #[test]
    fn test_shape_definition_representation() {
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#27110,#25007);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#191750,#191756);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#168012,#168018);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#388658,#388664);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#43736,#43742);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#24879,#24885);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#323560,#323566);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#166892,#166898);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#167956,#167962);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#139021,#139027);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#378862,#378868);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#197142,#197148);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#380410,#380416);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#199046,#199052);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#191806,#191812);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#194698,#194704);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#188047,#187890);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#196554,#196560);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#42934,#42940);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#162327,#162170);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#137173,#137179);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#166920,#166926);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#184061,#183512);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#192674,#192680);").is_ok());
        assert!(data_entity_shape_definition_representation("SHAPE_DEFINITION_REPRESENTATION(#197478,#197484);").is_ok());
    }

    #[test]
    fn test_shape_representation() {
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#43361),#43365);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#137068),#137072);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#136928),#136932);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#138076),#138080);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#322951),#322955);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#11152,#11574,#11996,#12418),\n  #12960);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#199165),#199169);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#323147),#323151);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#202161),#202165);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#138776),#138780);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#201629),#201633);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#202796),#202800);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#165443),#165447);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#27780),#27784);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#200845),#200849);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#161703),#161707);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#364888),#364892);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#192401),#192405);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#380389),#380393);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#197541),#197545);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#137264),#137268);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#139280),#139284);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#41795),#41799);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#328319),#328323);").is_ok());
        assert!(data_entity_shape_representation("SHAPE_REPRESENTATION('',(#11,#136956),#136960);").is_ok());
    }

    #[test]
    fn test_shape_representation_relationship() {

    }

    #[test]
    fn test_shell_based_surface_model() {
        assert!(data_entity_shell_based_surface_model("SHELL_BASED_SURFACE_MODEL('',(#383555));").is_ok());
        assert!(data_entity_shell_based_surface_model("SHELL_BASED_SURFACE_MODEL('',(#385622));").is_ok());
        assert!(data_entity_shell_based_surface_model("SHELL_BASED_SURFACE_MODEL('',(#131065));").is_ok());
        assert!(data_entity_shell_based_surface_model("SHELL_BASED_SURFACE_MODEL('',(#5822));").is_ok());
        assert!(data_entity_shell_based_surface_model("SHELL_BASED_SURFACE_MODEL('',(#129315));").is_ok());
        assert!(data_entity_shell_based_surface_model("SHELL_BASED_SURFACE_MODEL('',(#389233));").is_ok());
    }

    #[test]
    fn test_spherical_surface() {
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#394324,0.125);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#171404,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#6207,0.1);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#39988,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#170006,0.25);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#39360,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#381307,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#8086,0.1);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#325461,1.38E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#6080,0.1);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#380679,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#175003,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#152045,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#379940,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#171363,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#190121,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#176955,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#188622,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#178239,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#180123,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#45754,0.1);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#324591,1.38E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#193335,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#379194,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("SPHERICAL_SURFACE('',#195366,5.E-002);").is_ok());
    }

    #[test]
    fn test_styled_item() {
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#405958),#188083);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#426004),#5451);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#405998),#335417);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#400141),#359420);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#411716),#11152);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#426080),#150353);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#427231),#163676);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#426390),#190309);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#403442),#168410);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#407474),#380455);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#413592),#373105);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#406562),#153220);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#412149),#24306);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#412918),#162816);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#407064),#389293);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#414411),#39136);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#409166),#170965);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#403493),#376272);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#414240),#19681);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#411666),#127951);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#416528),#164106);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#415894),#372689);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#407084),#399012);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#425997),#5431);").is_ok());
        assert!(data_entity_styled_item("STYLED_ITEM('color',(#405154),#149883);").is_ok());
    }

    #[test]
    fn test_surface_of_linear_extrusion() {
        assert!(data_entity_surface_of_linear_extrusion("SURFACE_OF_LINEAR_EXTRUSION('',#349510,#349517);").is_ok());
        assert!(data_entity_surface_of_linear_extrusion("SURFACE_OF_LINEAR_EXTRUSION('',#349488,#349495);").is_ok());
    }

    #[test]
    fn test_surface_side_style() {
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#412122));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#424063));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#417991));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#400475));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#409793));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#402354));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#421956));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#418271));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#417137));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#400727));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#422173));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#422964));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#408450));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#424532));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#401530));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#407967));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#419552));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#424308));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#404491));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#414292));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#410429));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#407322));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#403558));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#401688));").is_ok());
        assert!(data_entity_surface_side_style("SURFACE_SIDE_STYLE('',(#412343));").is_ok());
    }

    #[test]
    fn test_surface_style_fill_area() {
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#412123);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#424064);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#417992);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#400476);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#409794);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#402355);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#421957);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#418272);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#417138);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#400728);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#422174);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#422965);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#408451);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#424533);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#401531);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#407968);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#419553);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#424309);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#404492);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#414293);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#410430);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#407323);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#403559);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#401689);").is_ok());
        assert!(data_entity_surface_style_fill_area("SURFACE_STYLE_FILL_AREA(#412344);").is_ok());
    }

    #[test]
    fn test_surface_style_usage() {
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#412121);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#424062);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#417990);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#400474);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#409792);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#402353);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#421955);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#418270);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#417136);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#400726);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#422172);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#422963);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#408449);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#424531);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#401529);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#407966);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#419551);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#424307);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#404490);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#414291);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#410428);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#407321);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#403557);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#401687);").is_ok());
        assert!(data_entity_surface_style_usage("SURFACE_STYLE_USAGE(.BOTH.,#412342);").is_ok());
    }

    #[test]
    fn test_toroidal_surface() {
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#394468,0.113,1.2E-002);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#393207,0.113,1.2E-002);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#335221,4.9,0.1);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#335371,4.225,0.1);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#394408,0.113,1.2E-002);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#334532,1.35,0.15);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#333277,4.225,0.1);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#394388,0.113,1.2E-002);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#334652,1.35,0.15);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#333321,1.15,0.1);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#227522,1.4,0.4);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#335347,4.9,0.1);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#42667,3.15,0.5);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#42487,2.9,0.25);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#42725,2.9,0.25);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#394488,0.113,1.2E-002);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#42531,2.9,0.25);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#335131,4.9,0.1);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#227285,1.4,0.4);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#335095,1.15,0.1);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#333427,4.9,0.1);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#335885,1.7,0.2);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#393127,0.113,1.2E-002);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#335239,1.15,0.1);").is_ok());
        assert!(data_entity_toroidal_surface("TOROIDAL_SURFACE('',#42713,2.9,0.25);").is_ok());
    }

    #[test]
    fn test_trimmed_curve() {
        assert!(data_entity_trimmed_curve("TRIMMED_CURVE('',#371726,(#371731,PARAMETER_VALUE(0.E+000)),(\n    #371732,PARAMETER_VALUE(3.141592653589)),.T.,.PARAMETER.);").is_ok());
        assert!(data_entity_trimmed_curve("TRIMMED_CURVE('',#371701,(#371706,PARAMETER_VALUE(0.E+000)),(\n    #371707,PARAMETER_VALUE(3.141592653589)),.T.,.PARAMETER.);").is_ok());
        assert!(data_entity_trimmed_curve("TRIMMED_CURVE('',#371718,(#371723,PARAMETER_VALUE(0.E+000)),(\n    #371724,PARAMETER_VALUE(3.141592653589)),.T.,.PARAMETER.);").is_ok());
        assert!(data_entity_trimmed_curve("TRIMMED_CURVE('',#371709,(#371714,PARAMETER_VALUE(0.E+000)),(\n    #371715,PARAMETER_VALUE(3.141592653589)),.T.,.PARAMETER.);").is_ok());
    }

    #[test]
    fn test_uncertainty_measure_with_unit() {
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#200822,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#395339,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(5.E-006),#153748,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#136989,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(5.E-006),#322956,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#197742,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#387568,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#128991,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(2.E-006),#27105,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#194458,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#196510,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(5.E-006),#323404,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#197910,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#137857,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#137185,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#128802,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#148152,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#194738,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(5.E-006),#323684,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#129248,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(5.E-006),#165196,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#395075,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#137577,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#130584,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_entity_uncertainty_measure_with_unit("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#394767,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
    }

    #[test]
    fn test_value_representation_item() {

    }

    #[test]
    fn test_vector() {
        assert!(data_entity_vector("VECTOR('',#322112,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#270072,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#117143,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#248800,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#339198,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#216792,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#221462,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#292282,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#111916,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#203996,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#65108,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#341329,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#365317,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#184489,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#149049,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#132158,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#258166,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#247477,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#353994,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#377779,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#344019,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#324977,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#204537,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#284546,1.);").is_ok());
        assert!(data_entity_vector("VECTOR('',#295335,1.);").is_ok());
    }

    #[test]
    fn test_vertex_point() {
        assert!(data_entity_vertex_point("VERTEX_POINT('',#13321);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#209043);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#10124);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#375441);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#90916);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#73268);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#363871);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#173364);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#314641);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#74306);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#155673);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#94234);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#8267);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#69818);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#349679);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#209757);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#223660);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#261584);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#38256);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#105056);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#209655);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#190043);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#358001);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#46718);").is_ok());
        assert!(data_entity_vertex_point("VERTEX_POINT('',#180042);").is_ok());
    }

    #[test]
    fn test_data_line() {
        assert!(data_line("#103695 = ORIENTED_EDGE('',*,*,#103696,.T.);").is_ok());
        assert!(data_line("#124123 = VERTEX_POINT('',#124124);").is_ok());
        assert!(data_line("#113965 = EDGE_LOOP('',(#113966,#113974,#113975,#113983));").is_ok());
        assert!(data_line("#234289 = VERTEX_POINT('',#234290);").is_ok());
        assert!(data_line("#188211 = EDGE_LOOP('',(#188212,#188213,#188214,#188215));").is_ok());
        assert!(data_line("#26497 = ADVANCED_FACE('',(#26498),#26510,.T.);").is_ok());
        assert!(data_line("#285683 = AXIS2_PLACEMENT_3D('',#285684,#285685,#285686);").is_ok());
        assert!(data_line("#293207 = CIRCLE('',#293208,4.2);").is_ok());
        assert!(data_line("#255513 = CARTESIAN_POINT('',(7.575,1.251823672866,-3.616043483317));").is_ok());
        assert!(data_line("#381425 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#395859 = LINE('',#395860,#395861);").is_ok());
        assert!(data_line("#80154 = PLANE('',#80155);").is_ok());
        assert!(data_line("#188350 = CARTESIAN_POINT('',(-0.492273030442,0.270494406184,\n    0.392082765346));").is_ok());
        assert!(data_line("#193529 = EDGE_CURVE('',#193530,#193532,#193534,.T.);").is_ok());
        assert!(data_line("#149798 = DIRECTION('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_line("#152676 = CARTESIAN_POINT('',(0.3,-0.15,0.28));").is_ok());
        assert!(data_line("#134405 = ADVANCED_FACE('',(#134406),#134431,.F.);").is_ok());
        assert!(data_line("#331873 = AXIS2_PLACEMENT_3D('',#331874,#331875,#331876);").is_ok());
        assert!(data_line("#90171 = LINE('',#90172,#90173);").is_ok());
        assert!(data_line("#333094 = CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#333095,#333097);").is_ok());
        assert!(data_line("#235265 = CARTESIAN_POINT('',(10.575,-2.447927405784,-7.05));").is_ok());
        assert!(data_line("#68815 = CARTESIAN_POINT('',(11.2999999994,0.7899999999,-7.058485782561)\n  );").is_ok());
        assert!(data_line("#191365 = AXIS2_PLACEMENT_3D('',#191366,#191367,#191368);").is_ok());
        assert!(data_line("#295899 = ORIENTED_EDGE('',*,*,#295797,.T.);").is_ok());
        assert!(data_line("#353585 = VECTOR('',#353586,1.);").is_ok());
        assert!(data_line("#390574 = VERTEX_POINT('',#390575);").is_ok());
        assert!(data_line("#308141 = LINE('',#308142,#308143);").is_ok());
        assert!(data_line("#13026 = EDGE_CURVE('',#13027,#13019,#13029,.T.);").is_ok());
        assert!(data_line("#394503 = CARTESIAN_POINT('',(-98.82047619806,10.644119757988,\n    42.342860302395));").is_ok());
        assert!(data_line("#13726 = ADVANCED_FACE('',(#13727),#13761,.F.);").is_ok());
        assert!(data_line("#199699 = DIRECTION('',(-3.777050848347E-023,8.742273394091E-008,-1.));").is_ok());
        assert!(data_line("#83164 = DIRECTION('',(-0.E+000,-1.,-0.E+000));").is_ok());
        assert!(data_line("#90077 = ORIENTED_CLOSED_SHELL('',*,#90078,.F.);").is_ok());
        assert!(data_line("#406728 = FILL_AREA_STYLE_COLOUR('',#406729);").is_ok());
        assert!(data_line("#368718 = AXIS2_PLACEMENT_3D('',#368719,#368720,#368721);").is_ok());
        assert!(data_line("#263881 = CARTESIAN_POINT('',(-4.875,1.224848688337,-3.335585364953));").is_ok());
        assert!(data_line("#3259 = CARTESIAN_POINT('',(9.24999928,103.50000636,0.E+000));").is_ok());
        assert!(data_line("#268049 = VECTOR('',#268050,1.);").is_ok());
        assert!(data_line("#247563 = ORIENTED_EDGE('',*,*,#247548,.T.);").is_ok());
        assert!(data_line("#165973 = PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_line("#382593 = ADVANCED_FACE('',(#382594),#382612,.T.);").is_ok());
        assert!(data_line("#285325 = LINE('',#285326,#285327);").is_ok());
        assert!(data_line("#256411 = FACE_BOUND('',#256412,.F.);").is_ok());
        assert!(data_line("#90097 = CARTESIAN_POINT('',(16.2499999994,14.99898095209,-9.4499999996)\n  );").is_ok());
        assert!(data_line("#16086 = ORIENTED_EDGE('',*,*,#16087,.T.);").is_ok());
        assert!(data_line("#216011 = VERTEX_POINT('',#216012);").is_ok());
        assert!(data_line("#257199 = ORIENTED_EDGE('',*,*,#257016,.T.);").is_ok());
        assert!(data_line("#355647 = EDGE_CURVE('',#355648,#355552,#355650,.T.);").is_ok());
        assert!(data_line("#332487 = PLANE('',#332488);").is_ok());
        assert!(data_line("#340491 = DIRECTION('',(0.707106781187,0.707106781187,-0.E+000));").is_ok());
        assert!(data_line("#4043 = ORIENTED_EDGE('',*,*,#4044,.F.);").is_ok());
        assert!(data_line("#402556 = DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_line("#301777 = LINE('',#301778,#301779);").is_ok());
        assert!(data_line("#254801 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#170114 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#170111,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_line("#192959 = PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_line("#99033 = DIRECTION('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_line("#359078 = LINE('',#359079,#359080);").is_ok());
        assert!(data_line("#65500 = DIRECTION('',(-0.E+000,-1.,-0.E+000));").is_ok());
        assert!(data_line("#7100 = ADVANCED_FACE('',(#7101),#7128,.T.);").is_ok());
        assert!(data_line("#312537 = CARTESIAN_POINT('',(-14.925,0.396842520354,-1.675362318841));").is_ok());
        assert!(data_line("#102033 = EDGE_CURVE('',#100898,#101938,#102034,.T.);").is_ok());
        assert!(data_line("#180430 = ORIENTED_EDGE('',*,*,#180409,.F.);").is_ok());
        assert!(data_line("#51637 = LINE('',#51638,#51639);").is_ok());
        assert!(data_line("#369979 = CARTESIAN_POINT('',(-1.,0.E+000,-4.8));").is_ok());
        assert!(data_line("#352715 = ORIENTED_EDGE('',*,*,#352716,.F.);").is_ok());
        assert!(data_line("#175896 = CARTESIAN_POINT('',(3.200958799484,3.159769396675,\n    0.774132029091));").is_ok());
        assert!(data_line("#142860 = LINE('',#142861,#142862);").is_ok());
        assert!(data_line("#389314 = VECTOR('',#389315,1.);").is_ok());
        assert!(data_line("#189262 = CARTESIAN_POINT('',(0.8,0.E+000,5.421010862428E-017));").is_ok());
        assert!(data_line("#294763 = ADVANCED_FACE('',(#294764),#294775,.T.);").is_ok());
        assert!(data_line("#12758 = VERTEX_POINT('',#12759);").is_ok());
        assert!(data_line("#70406 = VERTEX_POINT('',#70407);").is_ok());
        assert!(data_line("#224925 = CARTESIAN_POINT('',(2.625,0.12,-0.832311980303));").is_ok());
        assert!(data_line("#111301 = LINE('',#111302,#111303);").is_ok());
        assert!(data_line("#355497 = CARTESIAN_POINT('',(-5.512480768093,-5.25,-9.26));").is_ok());
        assert!(data_line("#254739 = DIRECTION('',(0.E+000,-1.,0.E+000));").is_ok());
        assert!(data_line("#130521 = ORIENTED_EDGE('',*,*,#130454,.T.);").is_ok());
        assert!(data_line("#17612 = EDGE_CURVE('',#17613,#17605,#17615,.T.);").is_ok());
        assert!(data_line("#6224 = EDGE_CURVE('',#6225,#6217,#6227,.T.);").is_ok());
        assert!(data_line("#74708 = DIRECTION('',(1.376764663474E-016,0.E+000,-1.));").is_ok());
        assert!(data_line("#127711 = ORIENTED_EDGE('',*,*,#122178,.T.);").is_ok());
        assert!(data_line("#404310 = FILL_AREA_STYLE('',(#404311));").is_ok());
        assert!(data_line("#318819 = ORIENTED_EDGE('',*,*,#318724,.F.);").is_ok());
        assert!(data_line("#410634 = FILL_AREA_STYLE_COLOUR('',#400881);").is_ok());
        assert!(data_line("#132609 = LINE('',#132610,#132611);").is_ok());
        assert!(data_line("#105717 = CARTESIAN_POINT('',(-1.17,0.68,-2.1));").is_ok());
        assert!(data_line("#105107 = ORIENTED_EDGE('',*,*,#103705,.T.);").is_ok());
        assert!(data_line("#332429 = VERTEX_POINT('',#332430);").is_ok());
        assert!(data_line("#312255 = VERTEX_POINT('',#312256);").is_ok());
        assert!(data_line("#168013 = PRODUCT_DEFINITION('design','',#168014,#168017);").is_ok());
        assert!(data_line("#135045 = CARTESIAN_POINT('',(1.677018063779,2.298193622072E-002,\n    0.612062861623));").is_ok());
        assert!(data_line("#407104 = PRESENTATION_STYLE_ASSIGNMENT((#407105,#407110));").is_ok());
        assert!(data_line("#89040 = AXIS2_PLACEMENT_3D('',#89041,#89042,#89043);").is_ok());
        assert!(data_line("#377202 = EDGE_CURVE('',#376819,#376794,#377203,.T.);").is_ok());
        assert!(data_line("#148532 = CARTESIAN_POINT('',(-0.25,0.24,0.E+000));").is_ok());
        assert!(data_line("#247903 = EDGE_LOOP('',(#247904,#247905,#247906,#247907));").is_ok());
        assert!(data_line("#27030 = ORIENTED_EDGE('',*,*,#27031,.T.);").is_ok());
        assert!(data_line("#373649 = CARTESIAN_POINT('',(0.75,-0.35,0.E+000));").is_ok());
        assert!(data_line("#37112 = EDGE_CURVE('',#35966,#37105,#37113,.T.);").is_ok());
        assert!(data_line("#130887 = VECTOR('',#130888,1.);").is_ok());
        assert!(data_line("#188994 = CARTESIAN_POINT('',(-0.8,0.18,-0.4));").is_ok());
        assert!(data_line("#282173 = PLANE('',#282174);").is_ok());
        assert!(data_line("#225287 = EDGE_CURVE('',#225278,#225288,#225290,.T.);").is_ok());
        assert!(data_line("#346865 = ORIENTED_EDGE('',*,*,#346866,.T.);").is_ok());
        assert!(data_line("#95399 = CARTESIAN_POINT('',(-2.825,0.1,-4.280909090909));").is_ok());
        assert!(data_line("#361382 = AXIS2_PLACEMENT_3D('',#361383,#361384,#361385);").is_ok());
        assert!(data_line("#186204 = B_SPLINE_SURFACE_WITH_KNOTS('',8,8,(\n    (#186205,#186206,#186207,#186208,#186209,#186210,#186211,#186212\n      ,#186213)\n    ,(#186214,#186215,#186216,#186217,#186218,#186219,#186220,#186221\n      ,#186222)\n    ,(#186223,#186224,#186225,#186226,#186227,#186228,#186229,#186230\n      ,#186231)\n    ,(#186232,#186233,#186234,#186235,#186236,#186237,#186238,#186239\n      ,#186240)\n    ,(#186241,#186242,#186243,#186244,#186245,#186246,#186247,#186248\n      ,#186249)\n    ,(#186250,#186251,#186252,#186253,#186254,#186255,#186256,#186257\n      ,#186258)\n    ,(#186259,#186260,#186261,#186262,#186263,#186264,#186265,#186266\n      ,#186267)\n    ,(#186268,#186269,#186270,#186271,#186272,#186273,#186274,#186275\n      ,#186276)\n    ,(#186277,#186278,#186279,#186280,#186281,#186282,#186283,#186284\n      ,#186285\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-3.889087296526E-002,\n    3.889087296526E-002),(-4.427879780914E-003,3.626740088442E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_line("#391052 = VECTOR('',#391053,1.);").is_ok());
        assert!(data_line("#83968 = ORIENTED_EDGE('',*,*,#83969,.F.);").is_ok());
        assert!(data_line("#157533 = ORIENTED_EDGE('',*,*,#157512,.T.);").is_ok());
        assert!(data_line("#332104 = CYLINDRICAL_SURFACE('',#332105,5.E-002);").is_ok());
        assert!(data_line("#156641 = DIRECTION('',(9.8079699557E-002,-0.995178563141,0.E+000));").is_ok());
        assert!(data_line("#109595 = LINE('',#109596,#109597);").is_ok());
        assert!(data_line("#182392 = CARTESIAN_POINT('',(3.200853900841,3.159612086504,\n    0.778189695941));").is_ok());
        assert!(data_line("#63222 = ORIENTED_EDGE('',*,*,#59596,.T.);").is_ok());
        assert!(data_line("#158812 = PRODUCT_DEFINITION('design','',#158813,#158816);").is_ok());
        assert!(data_line("#237443 = LINE('',#237444,#237445);").is_ok());
        assert!(data_line("#303601 = ORIENTED_EDGE('',*,*,#216714,.T.);").is_ok());
        assert!(data_line("#398152 = CARTESIAN_POINT('',(-9.42631846,3.3727898,3.20000122));").is_ok());
        assert!(data_line("#42823 = PRODUCT_DEFINITION('design','',#42824,#42827);").is_ok());
        assert!(data_line("#235385 = CARTESIAN_POINT('',(-30.225,-1.6,-7.05));").is_ok());
        assert!(data_line("#423342 = SURFACE_STYLE_FILL_AREA(#423343);").is_ok());
        assert!(data_line("#210387 = CARTESIAN_POINT('',(-12.225,1.85,-7.4));").is_ok());
        assert!(data_line("#158314 = DIRECTION('',(-0.923868458814,0.382710165542,0.E+000));").is_ok());
        assert!(data_line("#43105 = PRODUCT('R13C','R13C','',(#43106));").is_ok());
        assert!(data_line("#406876 = SURFACE_STYLE_USAGE(.BOTH.,#406877);").is_ok());
        assert!(data_line("#161041 = ORIENTED_EDGE('',*,*,#161042,.F.);").is_ok());
        assert!(data_line("#208627 = CARTESIAN_POINT('',(21.375,-1.6,-7.4));").is_ok());
        assert!(data_line("#227237 = DIRECTION('',(0.E+000,-1.,0.E+000));").is_ok());
        assert!(data_line("#338181 = EDGE_CURVE('',#338182,#338174,#338184,.T.);").is_ok());
        assert!(data_line("#134257 = CARTESIAN_POINT('',(5.296757852314E-002,6.237936363018E-002,\n    0.102547153673));").is_ok());
        assert!(data_line("#81862 = CARTESIAN_POINT('',(6.279268415495,11.744980952091,\n    -2.687060836852));").is_ok());
        assert!(data_line("#323804 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('4315','','',#323785,#11089,$);").is_ok());
        assert!(data_line("#386123 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#384074 = CARTESIAN_POINT('',(0.E+000,0.E+000,0.499916666667));").is_ok());
        assert!(data_line("#56820 = CARTESIAN_POINT('',(16.983023123577,0.404104297332,\n    -21.03781293944));").is_ok());
        assert!(data_line("#123629 = VECTOR('',#123630,1.);").is_ok());
        assert!(data_line("#76424 = EDGE_CURVE('',#76408,#76417,#76425,.T.);").is_ok());
        assert!(data_line("#241475 = PLANE('',#241476);").is_ok());
        assert!(data_line("#14096 = DIRECTION('',(-1.,-0.E+000,0.E+000));").is_ok());
        assert!(data_line("#113815 = EDGE_CURVE('',#113808,#113797,#113816,.T.);").is_ok());
        assert!(data_line("#51535 = VECTOR('',#51536,1.);").is_ok());
        assert!(data_line("#236139 = ORIENTED_EDGE('',*,*,#236140,.F.);").is_ok());
        assert!(data_line("#386777 = DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_line("#195115 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('3758','684','',#145221,#195091\n  ,$);").is_ok());
        assert!(data_line("#154436 = AXIS2_PLACEMENT_3D('',#154437,#154438,#154439);").is_ok());
        assert!(data_line("#379089 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#250871 = ORIENTED_EDGE('',*,*,#250872,.F.);").is_ok());
        assert!(data_line("#295637 = ORIENTED_EDGE('',*,*,#295533,.F.);").is_ok());
        assert!(data_line("#95269 = DIRECTION('',(0.E+000,-0.707106781187,0.707106781187));").is_ok());
        assert!(data_line("#60936 = DIRECTION('',(-0.E+000,-1.,-0.E+000));").is_ok());
        assert!(data_line("#46480 = VECTOR('',#46481,1.);").is_ok());
        assert!(data_line("#35542 = EDGE_CURVE('',#35535,#35543,#35545,.T.);").is_ok());
        assert!(data_line("#321750 = CIRCLE('',#321751,5.E-002);").is_ok());
        assert!(data_line("#377290 = CARTESIAN_POINT('',(1.289640569674,2.999640538362,\n    0.474265856781));").is_ok());
        assert!(data_line("#89713 = EDGE_CURVE('',#89714,#89716,#89718,.T.);").is_ok());
        assert!(data_line("#186758 = PRODUCT_DEFINITION_SHAPE('','',#186759);").is_ok());
        assert!(data_line("#357172 = CARTESIAN_POINT('',(-1.5875,0.3175,-2.286));").is_ok());
        assert!(data_line("#363834 = DIRECTION('',(-1.,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#37778 = LINE('',#37779,#37780);").is_ok());
        assert!(data_line("#23646 = CARTESIAN_POINT('',(0.761988945279,3.00000000001,0.334887468773\n    ));").is_ok());
        assert!(data_line("#407058 = FILL_AREA_STYLE('',(#407059));").is_ok());
        assert!(data_line("#226675 = CARTESIAN_POINT('',(-28.725,0.12,-1.812993439788));").is_ok());
        assert!(data_line("#283135 = CARTESIAN_POINT('',(35.3,-1.8,10.3));").is_ok());
        assert!(data_line("#57150 = CARTESIAN_POINT('',(5.547213959932,0.241574364378,\n    -0.660938750243));").is_ok());
        assert!(data_line("#344661 = EDGE_CURVE('',#344636,#344654,#344662,.T.);").is_ok());
        assert!(data_line("#192358 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('3647','573','',#192339,#153103\n  ,$);").is_ok());
        assert!(data_line("#233869 = CARTESIAN_POINT('',(-4.425,1.28,-2.4));").is_ok());
        assert!(data_line("#209229 = EDGE_CURVE('',#209222,#209230,#209232,.T.);").is_ok());
        assert!(data_line("#202340 = ITEM_DEFINED_TRANSFORMATION('','',#11,#202329);").is_ok());
        assert!(data_line("#334511 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#182212 = EDGE_LOOP('',(#182213,#182214,#182215,#182223));").is_ok());
        assert!(data_line("#310741 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#157381 = VERTEX_POINT('',#157382);").is_ok());
        assert!(data_line("#297255 = AXIS2_PLACEMENT_3D('',#297256,#297257,#297258);").is_ok());
        assert!(data_line("#30250 = PLANE('',#30251);").is_ok());
        assert!(data_line("#161585 = ORIENTED_EDGE('',*,*,#160895,.T.);").is_ok());
        assert!(data_line("#65902 = CARTESIAN_POINT('',(4.3099999994,0.200000000002,-18.25));").is_ok());
        assert!(data_line("#87946 = ORIENTED_EDGE('',*,*,#87894,.F.);").is_ok());
        assert!(data_line("#63298 = CARTESIAN_POINT('',(1.241092536359,15.677913026956,\n    -13.69656132958));").is_ok());
        assert!(data_line("#189993 = ORIENTED_EDGE('',*,*,#189922,.T.);").is_ok());
        assert!(data_line("#309003 = FACE_BOUND('',#309004,.F.);").is_ok());
        assert!(data_line("#172090 = VECTOR('',#172091,1.);").is_ok());
        assert!(data_line("#213659 = FACE_BOUND('',#213660,.F.);").is_ok());
        assert!(data_line("#62612 = CARTESIAN_POINT('',(14.74703715957,17.549998000292,\n    -6.456854120759));").is_ok());
        assert!(data_line("#136892 = SHAPE_DEFINITION_REPRESENTATION(#136893,#136899);").is_ok());
        assert!(data_line("#142138 = CIRCLE('',#142139,0.2);").is_ok());
        assert!(data_line("#135535 = CARTESIAN_POINT('',(1.611547952902E-002,4.362591836717E-002,\n    0.712712907352));").is_ok());
        assert!(data_line("#138774 = PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');").is_ok());
        assert!(data_line("#41132 = ADVANCED_FACE('',(#41133),#41160,.T.);").is_ok());
        assert!(data_line("#185018 = B_SPLINE_CURVE_WITH_KNOTS('',3,(#185019,#185020,#185021,\n    #185022),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,0.10471975512),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_line("#341081 = EDGE_CURVE('',#341074,#341082,#341084,.T.);").is_ok());
        assert!(data_line("#360728 = DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_line("#68625 = EDGE_CURVE('',#68626,#68618,#68628,.T.);").is_ok());
        assert!(data_line("#315651 = CARTESIAN_POINT('',(-20.925,0.396842520354,-1.675362318841));").is_ok());
        assert!(data_line("#136699 = EDGE_CURVE('',#136675,#136700,#136702,.T.);").is_ok());
        assert!(data_line("#377876 = CARTESIAN_POINT('',(1.400530016283,0.109469741579,\n    0.569850585719));").is_ok());
        assert!(data_line("#213579 = CARTESIAN_POINT('',(-11.025,1.85,-6.95));").is_ok());
        assert!(data_line("#258429 = ORIENTED_EDGE('',*,*,#258430,.F.);").is_ok());
        assert!(data_line("#366730 = ORIENTED_EDGE('',*,*,#366731,.T.);").is_ok());
        assert!(data_line("#121021 = DIRECTION('',(0.E+000,-0.E+000,1.));").is_ok());
        assert!(data_line("#373671 = CARTESIAN_POINT('',(0.8,-0.4,0.E+000));").is_ok());
        assert!(data_line("#407760 = OVER_RIDING_STYLED_ITEM('overriding color',(#407761),#381472,\n  #407739);").is_ok());
        assert!(data_line("#293249 = DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_line("#234093 = CARTESIAN_POINT('',(-10.425,1.431392113139,-2.137975188797));").is_ok());
        assert!(data_line("#195035 = PRODUCT_DEFINITION('design','',#195036,#195039);").is_ok());
        assert!(data_line("#284067 = ADVANCED_FACE('',(#284068),#284079,.T.);").is_ok());
        assert!(data_line("#349727 = CARTESIAN_POINT('',(-6.13,4.96,-11.7));").is_ok());
        assert!(data_line("#224427 = FACE_BOUND('',#224428,.F.);").is_ok());
        assert!(data_line("#389410 = CARTESIAN_POINT('',(0.E+000,0.7,1.E-003));").is_ok());
        assert!(data_line("#78294 = ORIENTED_EDGE('',*,*,#78295,.F.);").is_ok());
        assert!(data_line("#193011 = PRODUCT_DEFINITION('design','',#193012,#193015);").is_ok());
        assert!(data_line("#289523 = DIRECTION('',(-1.,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#375638 = ORIENTED_EDGE('',*,*,#375558,.F.);").is_ok());
        assert!(data_line("#24032 = ADVANCED_FACE('',(#24033),#24044,.F.);").is_ok());
        assert!(data_line("#98375 = DIRECTION('',(0.E+000,-1.807003620809E-016,1.));").is_ok());
        assert!(data_line("#237967 = LINE('',#237968,#237969);").is_ok());
        assert!(data_line("#280513 = CARTESIAN_POINT('',(-29.475,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#187495 = CARTESIAN_POINT('',(0.37500052,-0.2499995,0.E+000));").is_ok());
        assert!(data_line("#27996 = CARTESIAN_POINT('',(0.425,1.95,1.05));").is_ok());
        assert!(data_line("#382375 = PLANE('',#382376);").is_ok());
        assert!(data_line("#12936 = EDGE_CURVE('',#12734,#12559,#12937,.T.);").is_ok());
        assert!(data_line("#45551 = ADVANCED_FACE('',(#45552),#45564,.T.);").is_ok());
        assert!(data_line("#205761 = CARTESIAN_POINT('',(-12.075,-2.3,-7.4));").is_ok());
        assert!(data_line("#173841 = FACE_BOUND('',#173842,.F.);").is_ok());
        assert!(data_line("#174724 = VECTOR('',#174725,1.);").is_ok());
        assert!(data_line("#249927 = AXIS2_PLACEMENT_3D('',#249928,#249929,#249930);").is_ok());
        assert!(data_line("#396571 = DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_line("#241933 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#243171 = CARTESIAN_POINT('',(28.725,0.963391180558,-2.694308748263));").is_ok());
        assert!(data_line("#328358 = ITEM_DEFINED_TRANSFORMATION('','',#11,#328347);").is_ok());
        assert!(data_line("#135261 = EDGE_CURVE('',#135262,#135254,#135264,.T.);").is_ok());
        assert!(data_line("#328703 = ORIENTED_EDGE('',*,*,#328606,.F.);").is_ok());
        assert!(data_line("#399396 = PRESENTATION_STYLE_ASSIGNMENT((#399397));").is_ok());
        assert!(data_line("#269645 = DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_line("#72019 = DIRECTION('',(1.,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#45207 = CARTESIAN_POINT('',(4.43196860764E-002,2.983183033386,\n    0.600533877457));").is_ok());
        assert!(data_line("#30802 = EDGE_LOOP('',(#30803,#30804,#30810,#30811));").is_ok());
        assert!(data_line("#123405 = ORIENTED_EDGE('',*,*,#109928,.T.);").is_ok());
        assert!(data_line("#79286 = CARTESIAN_POINT('',(4.087256631863,14.961579570709,\n    -19.6149999998));").is_ok());
        assert!(data_line("#164730 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('3195','32','',#164548,#164721,\n  $);").is_ok());
        assert!(data_line("#404470 = SURFACE_STYLE_FILL_AREA(#404471);").is_ok());
        assert!(data_line("#96481 = VERTEX_POINT('',#96482);").is_ok());
        assert!(data_line("#183896 = VECTOR('',#183897,1.);").is_ok());
        assert!(data_line("#160496 = CARTESIAN_POINT('',(-1.091692,-0.91181682,0.E+000));").is_ok());
        assert!(data_line("#81110 = EDGE_LOOP('',(#81111,#81117,#81118,#81119,#81120,#81121,#81127,\n    #81128,#81129,#81130));").is_ok());
        assert!(data_line("#124533 = LINE('',#124534,#124535);").is_ok());
        assert!(data_line("#7594 = CARTESIAN_POINT('',(3.096487763899,3.041491039171,0.6210337724)\n  );").is_ok());
        assert!(data_line("#80938 = LINE('',#80939,#80940);").is_ok());
        assert!(data_line("#94169 = EDGE_CURVE('',#94170,#94162,#94172,.T.);").is_ok());
        assert!(data_line("#227069 = DIRECTION('',(1.,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#365300 = CARTESIAN_POINT('',(7.,1.E-001,-2.35));").is_ok());
        assert!(data_line("#406640 = OVER_RIDING_STYLED_ITEM('overriding color',(#406641),#18330,\n  #406584);").is_ok());
        assert!(data_line("#122353 = LINE('',#122354,#122355);").is_ok());
        assert!(data_line("#349969 = ORIENTED_EDGE('',*,*,#349970,.T.);").is_ok());
        assert!(data_line("#268539 = ADVANCED_FACE('',(#268540),#268551,.T.);").is_ok());
        assert!(data_line("#364190 = CARTESIAN_POINT('',(1.4732,-0.4064,-2.921));").is_ok());
        assert!(data_line("#388815 = CARTESIAN_POINT('',(0.E+000,0.E+000,0.3333));").is_ok());
        assert!(data_line("#29916 = DIRECTION('',(-1.,-0.E+000,-0.E+000));").is_ok());
        assert!(data_line("#361102 = ORIENTED_EDGE('',*,*,#361065,.T.);").is_ok());
        assert!(data_line("#62728 = CARTESIAN_POINT('',(12.64947297977,16.33417241439,\n    -1.652385959354));").is_ok());
        assert!(data_line("#172866 = ORIENTED_EDGE('',*,*,#172867,.F.);").is_ok());
        assert!(data_line("#295867 = CARTESIAN_POINT('',(22.275,-2.65,0.E+000));").is_ok());
        assert!(data_line("#368526 = AXIS2_PLACEMENT_3D('',#368527,#368528,#368529);").is_ok());
        assert!(data_line("#1841 = VERTEX_POINT('',#1842);").is_ok());
        assert!(data_line("#318415 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#417744 = SURFACE_STYLE_USAGE(.BOTH.,#417745);").is_ok());
        assert!(data_line("#350447 = ADVANCED_FACE('',(#350448),#350459,.F.);").is_ok());
        assert!(data_line("#397062 = EDGE_LOOP('',(#397063,#397064,#397072,#397080));").is_ok());
        assert!(data_line("#162856 = DIRECTION('',(0.E+000,-1.,0.E+000));").is_ok());
        assert!(data_line("#138873 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('3019','','',#138854,#5554,$);").is_ok());
        assert!(data_line("#32956 = ORIENTED_EDGE('',*,*,#32941,.T.);").is_ok());
        assert!(data_line("#90415 = VECTOR('',#90416,1.);").is_ok());
        assert!(data_line("#205077 = EDGE_CURVE('',#205070,#205078,#205080,.T.);").is_ok());
        assert!(data_line("#114015 = DIRECTION('',(0.E+000,-8.470572510271E-017,1.));").is_ok());
        assert!(data_line("#409118 = SURFACE_STYLE_USAGE(.BOTH.,#409119);").is_ok());
        assert!(data_line("#398266 = CARTESIAN_POINT('',(-9.9031933,1.46899884,0.E+000));").is_ok());
        assert!(data_line("#323720 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('4309','','',#323701,#11089,$);").is_ok());
        assert!(data_line("#233527 = ORIENTED_EDGE('',*,*,#230639,.F.);").is_ok());
        assert!(data_line("#326719 = ORIENTED_EDGE('',*,*,#326686,.T.);").is_ok());
        assert!(data_line("#299899 = AXIS2_PLACEMENT_3D('',#299900,#299901,#299902);").is_ok());
        assert!(data_line("#35490 = CARTESIAN_POINT('',(6.,-4.45,0.E+000));").is_ok());
        assert!(data_line("#133597 = AXIS2_PLACEMENT_3D('',#133598,#133599,#133600);").is_ok());
        assert!(data_line("#18106 = EDGE_CURVE('',#18076,#18107,#18109,.T.);").is_ok());
        assert!(data_line("#159810 = DIRECTION('',(0.999988589643,-4.777089408405E-003,0.E+000));").is_ok());
        assert!(data_line("#177802 = PLANE('',#177803);").is_ok());
        assert!(data_line("#363864 = LINE('',#363865,#363866);").is_ok());
        assert!(data_line("#178198 = ORIENTED_EDGE('',*,*,#177997,.T.);").is_ok());
        assert!(data_line("#376432 = DIRECTION('',(-0.E+000,1.,-0.E+000));").is_ok());
        assert!(data_line("#370435 = VERTEX_POINT('',#370436);").is_ok());
        assert!(data_line("#246409 = B_SPLINE_CURVE_WITH_KNOTS('',3,(#246410,#246411,#246412,\n    #246413),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_line("#362084 = CARTESIAN_POINT('',(1.4732,5.334,-0.4318));").is_ok());
        assert!(data_line("#51739 = LINE('',#51740,#51741);").is_ok());
        assert!(data_line("#199801 = SHAPE_DEFINITION_REPRESENTATION(#199802,#199808);").is_ok());
        assert!(data_line("#110285 = DIRECTION('',(-1.,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#16918 = FACE_BOUND('',#16919,.T.);").is_ok());
        assert!(data_line("#288977 = CARTESIAN_POINT('',(33.15,-4.999999999998E-002,10.75));").is_ok());
        assert!(data_line("#89173 = CARTESIAN_POINT('',(14.6999999994,15.2433999999,-3.2508));").is_ok());
        assert!(data_line("#366944 = ORIENTED_EDGE('',*,*,#366923,.F.);").is_ok());
        assert!(data_line("#77358 = ORIENTED_EDGE('',*,*,#71536,.F.);").is_ok());
        assert!(data_line("#91309 = LINE('',#91310,#91311);").is_ok());
        assert!(data_line("#59270 = CARTESIAN_POINT('',(16.9499999994,15.55,-13.87811617523));").is_ok());
        assert!(data_line("#210079 = EDGE_CURVE('',#210072,#210080,#210082,.T.);").is_ok());
        assert!(data_line("#87744 = VECTOR('',#87745,1.);").is_ok());
        assert!(data_line("#12054 = EDGE_CURVE('',#12055,#12045,#12057,.T.);").is_ok());
        assert!(data_line("#190399 = CARTESIAN_POINT('',(0.75,-0.35,0.E+000));").is_ok());
        assert!(data_line("#182536 = CARTESIAN_POINT('',(5.E-002,9.14686469793E-005,0.975870266043)\n  );").is_ok());
        assert!(data_line("#418176 = OVER_RIDING_STYLED_ITEM('overriding color',(#418177),#54830,\n  #416537);").is_ok());
        assert!(data_line("#387765 = CARTESIAN_POINT('',(-3.4,-1.133333333333,0.83325));").is_ok());
        assert!(data_line("#55536 = CARTESIAN_POINT('',(16.999999999399,13.140943941091,\n    -0.625902354647));").is_ok());
        assert!(data_line("#306895 = DIRECTION('',(0.E+000,-0.866025403784,-0.5));").is_ok());
        assert!(data_line("#383143 = EDGE_LOOP('',(#383144,#383152,#383159,#383160));").is_ok());
        assert!(data_line("#353199 = FACE_BOUND('',#353200,.F.);").is_ok());
        assert!(data_line("#417798 = OVER_RIDING_STYLED_ITEM('overriding color',(#417799),#53180,\n  #416537);").is_ok());
        assert!(data_line("#426942 = SURFACE_SIDE_STYLE('',(#426943));").is_ok());
        assert!(data_line("#375692 = VECTOR('',#375693,1.);").is_ok());
        assert!(data_line("#47288 = CARTESIAN_POINT('',(6.5999999994,14.125,0.25));").is_ok());
        assert!(data_line("#21070 = VECTOR('',#21071,1.);").is_ok());
        assert!(data_line("#182476 = CARTESIAN_POINT('',(-6.938893903907E-018,5.E-002,0.775));").is_ok());
        assert!(data_line("#304415 = EDGE_CURVE('',#224940,#304416,#304418,.T.);").is_ok());
        assert!(data_line("#37262 = ORIENTED_EDGE('',*,*,#37263,.T.);").is_ok());
        assert!(data_line("#350699 = FACE_BOUND('',#350700,.F.);").is_ok());
        assert!(data_line("#327828 = VERTEX_POINT('',#327829);").is_ok());
        assert!(data_line("#350201 = ORIENTED_EDGE('',*,*,#350202,.T.);").is_ok());
        assert!(data_line("#372007 = DIRECTION('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_line("#62246 = DIRECTION('',(1.,-0.E+000,0.E+000));").is_ok());
        assert!(data_line("#224283 = VECTOR('',#224284,1.);").is_ok());
        assert!(data_line("#88994 = EDGE_LOOP('',(#88995,#88996,#89002,#89003,#89004,#89005));").is_ok());
        assert!(data_line("#290251 = CARTESIAN_POINT('',(33.80044891521,2.369210361953,14.64));").is_ok());
        assert!(data_line("#56136 = CARTESIAN_POINT('',(17.25290476175,15.54633626435,\n    -20.84815990824));").is_ok());
        assert!(data_line("#178546 = EDGE_CURVE('',#178507,#178176,#178547,.T.);").is_ok());
        assert!(data_line("#147116 = CARTESIAN_POINT('',(0.E+000,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#94439 = CARTESIAN_POINT('',(2.325,0.24,-5.2));").is_ok());
        assert!(data_line("#57468 = CARTESIAN_POINT('',(15.16082156583,-4.228770243986E-002,\n    -0.631098639136));").is_ok());
        assert!(data_line("#316611 = DIRECTION('',(0.E+000,0.335531504429,-0.942028985507));").is_ok());
        assert!(data_line("#195097 = AXIS2_PLACEMENT_3D('',#195098,#195099,#195100);").is_ok());
        assert!(data_line("#104535 = DIRECTION('',(-8.80570292374E-016,-1.,0.E+000));").is_ok());
        assert!(data_line("#218325 = CARTESIAN_POINT('',(26.175,-2.1,-7.05));").is_ok());
        assert!(data_line("#100491 = CARTESIAN_POINT('',(-7.175,-8.131516293641E-017,-4.6));").is_ok());
        assert!(data_line("#100147 = EDGE_CURVE('',#93816,#93871,#100148,.T.);").is_ok());
        assert!(data_line("#214807 = ORIENTED_EDGE('',*,*,#214808,.T.);").is_ok());
        assert!(data_line("#281013 = DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_line("#280095 = LINE('',#280096,#280097);").is_ok());
        assert!(data_line("#322323 = DIRECTION('',(1.,0.E+000,-0.E+000));").is_ok());
        assert!(data_line("#65642 = ORIENTED_EDGE('',*,*,#65643,.T.);").is_ok());
        assert!(data_line("#327758 = ORIENTED_EDGE('',*,*,#327759,.T.);").is_ok());
        assert!(data_line("#329519 = ORIENTED_EDGE('',*,*,#329053,.T.);").is_ok());
        assert!(data_line("#100141 = LINE('',#100142,#100143);").is_ok());
        assert!(data_line("#121677 = EDGE_CURVE('',#121678,#121670,#121680,.T.);").is_ok());
        assert!(data_line("#384052 = EDGE_CURVE('',#384019,#384045,#384053,.T.);").is_ok());
        assert!(data_line("#264675 = VECTOR('',#264676,1.);").is_ok());
        assert!(data_line("#184116 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('3513','439','',#145221,#184092\n  ,$);").is_ok());
        assert!(data_line("#53076 = VECTOR('',#53077,1.);").is_ok());
        assert!(data_line("#357570 = CARTESIAN_POINT('',(-6.0325,-0.3175,1.0668));").is_ok());
        assert!(data_line("#424636 = SURFACE_SIDE_STYLE('',(#424637));").is_ok());
        assert!(data_line("#306131 = ORIENTED_EDGE('',*,*,#306132,.T.);").is_ok());
        assert!(data_line("#16678 = FACE_BOUND('',#16679,.T.);").is_ok());
        assert!(data_line("#191135 = PRODUCT_DEFINITION('design','',#191136,#191139);").is_ok());
        assert!(data_line("#226499 = EDGE_LOOP('',(#226500,#226510,#226518,#226526));").is_ok());
        assert!(data_line("#413474 = FILL_AREA_STYLE_COLOUR('',#406004);").is_ok());
        assert!(data_line("#250985 = VECTOR('',#250986,1.);").is_ok());
        assert!(data_line("#378424 = EDGE_CURVE('',#378394,#378425,#378427,.T.);").is_ok());
        assert!(data_line("#357442 = VECTOR('',#357443,1.);").is_ok());
        assert!(data_line("#276827 = ORIENTED_EDGE('',*,*,#276828,.F.);").is_ok());
        assert!(data_line("#285817 = ORIENTED_EDGE('',*,*,#285691,.F.);").is_ok());
        assert!(data_line("#33128 = CARTESIAN_POINT('',(5.,1.15,5.7));").is_ok());
        assert!(data_line("#388645 = CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#388646,#388648);").is_ok());
        assert!(data_line("#331351 = ADVANCED_FACE('',(#331352),#331364,.F.);").is_ok());
        assert!(data_line("#191793 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#191790,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_line("#246807 = FACE_BOUND('',#246808,.F.);").is_ok());
        assert!(data_line("#209495 = CARTESIAN_POINT('',(2.925,1.85,-7.4));").is_ok());
        assert!(data_line("#44467 = CARTESIAN_POINT('',(1.348001997584E-002,2.959013102755,\n    0.508236237031));").is_ok());
        assert!(data_line("#16920 = ORIENTED_EDGE('',*,*,#16921,.F.);").is_ok());
        assert!(data_line("#156921 = DIRECTION('',(-1.,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#291565 = DIRECTION('',(1.,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#370731 = CARTESIAN_POINT('',(-2.7,12.8,4.8));").is_ok());
        assert!(data_line("#51393 = VECTOR('',#51394,1.);").is_ok());
        assert!(data_line("#115401 = DIRECTION('',(0.E+000,5.233595624294E-002,0.998629534755));").is_ok());
        assert!(data_line("#422600 = SURFACE_STYLE_FILL_AREA(#422601);").is_ok());
        assert!(data_line("#169412 = ORIENTED_EDGE('',*,*,#169297,.F.);").is_ok());
        assert!(data_line("#389169 = EDGE_CURVE('',#389162,#389162,#389170,.T.);").is_ok());
        assert!(data_line("#343263 = ORIENTED_EDGE('',*,*,#342676,.T.);").is_ok());
        assert!(data_line("#137504 = ITEM_DEFINED_TRANSFORMATION('','',#11,#691);").is_ok());
        assert!(data_line("#18827 = AXIS2_PLACEMENT_3D('',#18828,#18829,#18830);").is_ok());
        assert!(data_line("#266543 = ADVANCED_FACE('',(#266544),#266555,.T.);").is_ok());
        assert!(data_line("#249599 = DIRECTION('',(0.E+000,8.668216701143E-002,-0.996236017178));").is_ok());
        assert!(data_line("#364922 = EDGE_CURVE('',#364923,#364915,#364925,.T.);").is_ok());
        assert!(data_line("#195217 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#195214,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_line("#88842 = DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_line("#172698 = DIRECTION('',(-0.19505913029,-0.980791484308,0.E+000));").is_ok());
        assert!(data_line("#7414 = CARTESIAN_POINT('',(3.018459242046,-9.590931489096E-004,\n    0.632674911771));").is_ok());
        assert!(data_line("#171161 = CARTESIAN_POINT('',(0.45,0.2,5.E-002));").is_ok());
        assert!(data_line("#135323 = CARTESIAN_POINT('',(5.478104631727E-004,0.1,0.711712068398));").is_ok());
        assert!(data_line("#135103 = EDGE_CURVE('',#135096,#135104,#135106,.T.);").is_ok());
        assert!(data_line("#385266 = AXIS2_PLACEMENT_3D('',#385267,#385268,#385269);").is_ok());
        assert!(data_line("#235613 = CARTESIAN_POINT('',(-24.225,-1.6,-7.05));").is_ok());
        assert!(data_line("#414886 = SURFACE_STYLE_USAGE(.BOTH.,#414887);").is_ok());
        assert!(data_line("#223673 = ORIENTED_EDGE('',*,*,#223674,.F.);").is_ok());
        assert!(data_line("#289513 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#116827 = DIRECTION('',(1.,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#231101 = VECTOR('',#231102,1.);").is_ok());
        assert!(data_line("#352443 = VERTEX_POINT('',#352444);").is_ok());
        assert!(data_line("#282699 = ORIENTED_EDGE('',*,*,#282700,.T.);").is_ok());
        assert!(data_line("#238561 = DIRECTION('',(1.,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#27536 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(5.E-006),#27533,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_line("#274995 = DIRECTION('',(0.E+000,0.553859353283,-0.832610243019));").is_ok());
        assert!(data_line("#386657 = CIRCLE('',#386658,5.E-002);").is_ok());
        assert!(data_line("#69618 = DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_line("#248621 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#167487 = AXIS2_PLACEMENT_3D('',#167488,#167489,#167490);").is_ok());
        assert!(data_line("#284507 = EDGE_CURVE('',#284508,#284500,#284510,.T.);").is_ok());
        assert!(data_line("#342227 = LINE('',#342228,#342229);").is_ok());
        assert!(data_line("#184902 = ELLIPSE('',#184903,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_line("#346427 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#315227 = EDGE_CURVE('',#315181,#315220,#315228,.T.);").is_ok());
        assert!(data_line("#98959 = DIRECTION('',(1.,0.E+000,-0.E+000));").is_ok());
        assert!(data_line("#363166 = ORIENTED_EDGE('',*,*,#359635,.T.);").is_ok());
        assert!(data_line("#234249 = ORIENTED_EDGE('',*,*,#234250,.F.);").is_ok());
        assert!(data_line("#68257 = DIRECTION('',(-0.E+000,1.,-0.E+000));").is_ok());
        assert!(data_line("#26575 = FACE_BOUND('',#26576,.T.);").is_ok());
        assert!(data_line("#156539 = ORIENTED_EDGE('',*,*,#156540,.F.);").is_ok());
        assert!(data_line("#44339 = CARTESIAN_POINT('',(6.073009183013E-002,-2.023844055306E-018,\n    0.511740777928));").is_ok());
        assert!(data_line("#230759 = CARTESIAN_POINT('',(2.175,1.28,-2.4));").is_ok());
        assert!(data_line("#351985 = DIRECTION('',(0.985821179711,-0.167799289728,0.E+000));").is_ok());
        assert!(data_line("#43473 = AXIS2_PLACEMENT_3D('',#43474,#43475,#43476);").is_ok());
        assert!(data_line("#359288 = EDGE_LOOP('',(#359289,#359290,#359296,#359297));").is_ok());
        assert!(data_line("#96929 = VERTEX_POINT('',#96930);").is_ok());
        assert!(data_line("#322052 = VERTEX_POINT('',#322053);").is_ok());
        assert!(data_line("#154278 = VECTOR('',#154279,1.);").is_ok());
        assert!(data_line("#250917 = LINE('',#250918,#250919);").is_ok());
        assert!(data_line("#265363 = ORIENTED_EDGE('',*,*,#265349,.T.);").is_ok());
        assert!(data_line("#163682 = EDGE_CURVE('',#163683,#163685,#163687,.T.);").is_ok());
        assert!(data_line("#260863 = ORIENTED_EDGE('',*,*,#260831,.T.);").is_ok());
        assert!(data_line("#33418 = VERTEX_POINT('',#33419);").is_ok());
        assert!(data_line("#109757 = VECTOR('',#109758,1.);").is_ok());
        assert!(data_line("#246119 = VECTOR('',#246120,1.);").is_ok());
        assert!(data_line("#173531 = CARTESIAN_POINT('',(-0.2999994,8.750046E-002,0.E+000));").is_ok());
        assert!(data_line("#226429 = DIRECTION('',(1.,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#271121 = ORIENTED_EDGE('',*,*,#228866,.F.);").is_ok());
        assert!(data_line("#240007 = CARTESIAN_POINT('',(-18.075,0.85,-6.95));").is_ok());
        assert!(data_line("#22014 = ORIENTED_EDGE('',*,*,#22015,.F.);").is_ok());
        assert!(data_line("#98863 = EDGE_LOOP('',(#98864,#98865,#98866,#98867));").is_ok());
        assert!(data_line("#108687 = LINE('',#108688,#108689);").is_ok());
        assert!(data_line("#72331 = DIRECTION('',(0.E+000,0.970142500145,-0.242535625036));").is_ok());
        assert!(data_line("#268959 = CARTESIAN_POINT('',(-12.225,0.85,-7.05));").is_ok());
        assert!(data_line("#303429 = PLANE('',#303430);").is_ok());
        assert!(data_line("#238591 = ORIENTED_EDGE('',*,*,#211996,.T.);").is_ok());
        assert!(data_line("#8122 = CARTESIAN_POINT('',(0.333184260627,2.521065191531,1.099662270611\n    ));").is_ok());
        assert!(data_line("#323471 = ITEM_DEFINED_TRANSFORMATION('','',#11,#1119);").is_ok());
        assert!(data_line("#365036 = CARTESIAN_POINT('',(4.,7.7,-2.85));").is_ok());
        assert!(data_line("#424974 = FILL_AREA_STYLE('',(#424975));").is_ok());
        assert!(data_line("#42471 = EDGE_CURVE('',#42472,#42463,#42474,.T.);").is_ok());
        assert!(data_line("#127245 = CARTESIAN_POINT('',(7.325,1.57,-0.345));").is_ok());
        assert!(data_line("#154796 = ORIENTED_EDGE('',*,*,#154709,.T.);").is_ok());
        assert!(data_line("#348815 = DIRECTION('',(1.,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#244381 = CARTESIAN_POINT('',(26.925,1.224848688337,-3.335585364953));").is_ok());
        assert!(data_line("#273197 = ORIENTED_EDGE('',*,*,#273198,.F.);").is_ok());
        assert!(data_line("#391200 = DIRECTION('',(0.273214882168,0.E+000,-0.961953028043));").is_ok());
        assert!(data_line("#10696 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#190195 = ORIENTED_EDGE('',*,*,#190071,.F.);").is_ok());
        assert!(data_line("#191205 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#191202,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(data_line("#390374 = DIRECTION('',(-0.276243742891,0.E+000,0.961087610217));").is_ok());
        assert!(data_line("#137426 = PRODUCT_DEFINITION('design','',#137427,#137430);").is_ok());
        assert!(data_line("#389085 = AXIS2_PLACEMENT_3D('',#389086,#389087,#389088);").is_ok());
        assert!(data_line("#85880 = EDGE_CURVE('',#85842,#85873,#85881,.T.);").is_ok());
        assert!(data_line("#426442 = SURFACE_STYLE_FILL_AREA(#426443);").is_ok());
        assert!(data_line("#247211 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#130180 = EDGE_CURVE('',#130155,#130173,#130181,.T.);").is_ok());
        assert!(data_line("#244765 = ORIENTED_EDGE('',*,*,#244766,.F.);").is_ok());
        assert!(data_line("#344861 = VECTOR('',#344862,1.);").is_ok());
        assert!(data_line("#234715 = VECTOR('',#234716,1.);").is_ok());
        assert!(data_line("#136099 = VECTOR('',#136100,1.);").is_ok());
        assert!(data_line("#65311 = CIRCLE('',#65312,0.4);").is_ok());
        assert!(data_line("#337821 = FACE_BOUND('',#337822,.T.);").is_ok());
        assert!(data_line("#164282 = ITEM_DEFINED_TRANSFORMATION('','',#11,#145395);").is_ok());
        assert!(data_line("#217727 = VECTOR('',#217728,1.);").is_ok());
        assert!(data_line("#372469 = LINE('',#372470,#372471);").is_ok());
        assert!(data_line("#411216 = OVER_RIDING_STYLED_ITEM('overriding color',(#411217),#367799,\n  #410677);").is_ok());
        assert!(data_line("#161913 = PRODUCT_DEFINITION_FORMATION('','',#161914);").is_ok());
        assert!(data_line("#366144 = CARTESIAN_POINT('',(-1.75,0.4,-2.35));").is_ok());
        assert!(data_line("#140722 = CARTESIAN_POINT('',(-2.54,0.4,1.645));").is_ok());
        assert!(data_line("#164905 = DIRECTION('',(-1.,0.E+000,0.E+000));").is_ok());
        assert!(data_line("#372071 = CIRCLE('',#372072,5.E-002);").is_ok());
        assert!(data_line("#280801 = EDGE_LOOP('',(#280802,#280803,#280804,#280805));").is_ok());
        assert!(data_line("#244867 = DIRECTION('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_line("#9579 = ORIENTED_EDGE('',*,*,#9140,.F.);").is_ok());
        assert!(data_line("#226941 = LINE('',#226942,#226943);").is_ok());
        assert!(data_line("#23710 = DIRECTION('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_line("#414610 = SURFACE_STYLE_FILL_AREA(#414611);").is_ok());
        assert!(data_line("#423708 = FILL_AREA_STYLE_COLOUR('',#399418);").is_ok());
        assert!(data_line("#133337 = DIRECTION('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_line("#41845 = PRODUCT_DEFINITION('design','',#41846,#41849);").is_ok());
        assert!(data_line("#172220 = LINE('',#172221,#172222);").is_ok());
        assert!(data_line("#78184 = CARTESIAN_POINT('',(4.4049999997,6.274980952091,-13.3999999998)\n  );").is_ok());
        assert!(data_line("#112221 = VERTEX_POINT('',#112222);").is_ok());
        assert!(data_line("#150463 = DIRECTION('',(0.E+000,-0.121869343405,-0.992546151641));").is_ok());
        assert!(data_line("#384160 = VECTOR('',#384161,1.);").is_ok());
        assert!(data_line("#316489 = ORIENTED_EDGE('',*,*,#316387,.T.);").is_ok());
        assert!(data_line("#74066 = AXIS2_PLACEMENT_3D('',#74067,#74068,#74069);").is_ok());
    }

}
