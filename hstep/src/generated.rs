
use crate::hparser::{Parser, Id};

pub fn read_pair_id_ParameterValue(parser: &mut Parser) -> (Id, ParameterValue) {
    read_tuple_2(parser, Parser::read_id, read_stf_parameter_value)
}
pub fn read_tuple_0(parser: &mut Parser) -> () {
        parser.read_open_paren(); parser.skip_whitespace(); parser.read_close_paren();
     
}

pub fn read_tuple_1<T0, F0>(parser: &mut Parser, func0: F0) -> (T0) where F0: Fn(&mut Parser) -> T0 {
        parser.read_open_paren();
parser.skip_whitespace();
let x0 = func0(parser);
parser.skip_whitespace();
        parser.read_close_paren();
     (x0)
}

pub fn read_tuple_2<T0, T1, F0, F1>(parser: &mut Parser, func0: F0, func1: F1) -> (T0, T1) where F0: Fn(&mut Parser) -> T0, F1: Fn(&mut Parser) -> T1 {
        parser.read_open_paren();
parser.skip_whitespace();
let x0 = func0(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x1 = func1(parser);
parser.skip_whitespace();
        parser.read_close_paren();
     (x0, x1)
}

pub fn read_tuple_3<T0, T1, T2, F0, F1, F2>(parser: &mut Parser, func0: F0, func1: F1, func2: F2) -> (T0, T1, T2) where F0: Fn(&mut Parser) -> T0, F1: Fn(&mut Parser) -> T1, F2: Fn(&mut Parser) -> T2 {
        parser.read_open_paren();
parser.skip_whitespace();
let x0 = func0(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x1 = func1(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x2 = func2(parser);
parser.skip_whitespace();
        parser.read_close_paren();
     (x0, x1, x2)
}

pub fn read_tuple_4<T0, T1, T2, T3, F0, F1, F2, F3>(parser: &mut Parser, func0: F0, func1: F1, func2: F2, func3: F3) -> (T0, T1, T2, T3) where F0: Fn(&mut Parser) -> T0, F1: Fn(&mut Parser) -> T1, F2: Fn(&mut Parser) -> T2, F3: Fn(&mut Parser) -> T3 {
        parser.read_open_paren();
parser.skip_whitespace();
let x0 = func0(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x1 = func1(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x2 = func2(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x3 = func3(parser);
parser.skip_whitespace();
        parser.read_close_paren();
     (x0, x1, x2, x3)
}

pub fn read_tuple_5<T0, T1, T2, T3, T4, F0, F1, F2, F3, F4>(parser: &mut Parser, func0: F0, func1: F1, func2: F2, func3: F3, func4: F4) -> (T0, T1, T2, T3, T4) where F0: Fn(&mut Parser) -> T0, F1: Fn(&mut Parser) -> T1, F2: Fn(&mut Parser) -> T2, F3: Fn(&mut Parser) -> T3, F4: Fn(&mut Parser) -> T4 {
        parser.read_open_paren();
parser.skip_whitespace();
let x0 = func0(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x1 = func1(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x2 = func2(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x3 = func3(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x4 = func4(parser);
parser.skip_whitespace();
        parser.read_close_paren();
     (x0, x1, x2, x3, x4)
}

pub fn read_tuple_6<T0, T1, T2, T3, T4, T5, F0, F1, F2, F3, F4, F5>(parser: &mut Parser, func0: F0, func1: F1, func2: F2, func3: F3, func4: F4, func5: F5) -> (T0, T1, T2, T3, T4, T5) where F0: Fn(&mut Parser) -> T0, F1: Fn(&mut Parser) -> T1, F2: Fn(&mut Parser) -> T2, F3: Fn(&mut Parser) -> T3, F4: Fn(&mut Parser) -> T4, F5: Fn(&mut Parser) -> T5 {
        parser.read_open_paren();
parser.skip_whitespace();
let x0 = func0(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x1 = func1(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x2 = func2(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x3 = func3(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x4 = func4(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x5 = func5(parser);
parser.skip_whitespace();
        parser.read_close_paren();
     (x0, x1, x2, x3, x4, x5)
}

pub fn read_tuple_7<T0, T1, T2, T3, T4, T5, T6, F0, F1, F2, F3, F4, F5, F6>(parser: &mut Parser, func0: F0, func1: F1, func2: F2, func3: F3, func4: F4, func5: F5, func6: F6) -> (T0, T1, T2, T3, T4, T5, T6) where F0: Fn(&mut Parser) -> T0, F1: Fn(&mut Parser) -> T1, F2: Fn(&mut Parser) -> T2, F3: Fn(&mut Parser) -> T3, F4: Fn(&mut Parser) -> T4, F5: Fn(&mut Parser) -> T5, F6: Fn(&mut Parser) -> T6 {
        parser.read_open_paren();
parser.skip_whitespace();
let x0 = func0(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x1 = func1(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x2 = func2(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x3 = func3(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x4 = func4(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x5 = func5(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x6 = func6(parser);
parser.skip_whitespace();
        parser.read_close_paren();
     (x0, x1, x2, x3, x4, x5, x6)
}

pub fn read_tuple_9<T0, T1, T2, T3, T4, T5, T6, T7, T8, F0, F1, F2, F3, F4, F5, F6, F7, F8>(parser: &mut Parser, func0: F0, func1: F1, func2: F2, func3: F3, func4: F4, func5: F5, func6: F6, func7: F7, func8: F8) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8) where F0: Fn(&mut Parser) -> T0, F1: Fn(&mut Parser) -> T1, F2: Fn(&mut Parser) -> T2, F3: Fn(&mut Parser) -> T3, F4: Fn(&mut Parser) -> T4, F5: Fn(&mut Parser) -> T5, F6: Fn(&mut Parser) -> T6, F7: Fn(&mut Parser) -> T7, F8: Fn(&mut Parser) -> T8 {
        parser.read_open_paren();
parser.skip_whitespace();
let x0 = func0(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x1 = func1(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x2 = func2(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x3 = func3(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x4 = func4(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x5 = func5(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x6 = func6(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x7 = func7(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x8 = func8(parser);
parser.skip_whitespace();
        parser.read_close_paren();
     (x0, x1, x2, x3, x4, x5, x6, x7, x8)
}

pub fn read_tuple_13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12>(parser: &mut Parser, func0: F0, func1: F1, func2: F2, func3: F3, func4: F4, func5: F5, func6: F6, func7: F7, func8: F8, func9: F9, func10: F10, func11: F11, func12: F12) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) where F0: Fn(&mut Parser) -> T0, F1: Fn(&mut Parser) -> T1, F2: Fn(&mut Parser) -> T2, F3: Fn(&mut Parser) -> T3, F4: Fn(&mut Parser) -> T4, F5: Fn(&mut Parser) -> T5, F6: Fn(&mut Parser) -> T6, F7: Fn(&mut Parser) -> T7, F8: Fn(&mut Parser) -> T8, F9: Fn(&mut Parser) -> T9, F10: Fn(&mut Parser) -> T10, F11: Fn(&mut Parser) -> T11, F12: Fn(&mut Parser) -> T12 {
        parser.read_open_paren();
parser.skip_whitespace();
let x0 = func0(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x1 = func1(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x2 = func2(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x3 = func3(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x4 = func4(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x5 = func5(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x6 = func6(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x7 = func7(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x8 = func8(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x9 = func9(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x10 = func10(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x11 = func11(parser);
parser.skip_whitespace();
parser.read_comma();
parser.skip_whitespace();
let x12 = func12(parser);
parser.skip_whitespace();
        parser.read_close_paren();
     (x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12)
}


#[derive(Debug, PartialEq)]
pub struct VolumeMeasure(pub f64);
pub fn read_stf_volume_measure(parser: &mut Parser) -> VolumeMeasure {
    let (s, v) = parser.read_united_float();
    if s != "VOLUME_MEASURE" { panic!("unexpected iden"); }
    VolumeMeasure(v)
}

#[derive(Debug, PartialEq)]
pub struct PositiveLengthMeasure(pub f64);
pub fn read_stf_positive_length_measure(parser: &mut Parser) -> PositiveLengthMeasure {
    let (s, v) = parser.read_united_float();
    if s != "POSITIVE_LENGTH_MEASURE" { panic!("unexpected iden"); }
    PositiveLengthMeasure(v)
}

#[derive(Debug, PartialEq)]
pub struct ParameterValue(pub f64);
pub fn read_stf_parameter_value(parser: &mut Parser) -> ParameterValue {
    let (s, v) = parser.read_united_float();
    if s != "PARAMETER_VALUE" { panic!("unexpected iden"); }
    ParameterValue(v)
}

#[derive(Debug, PartialEq)]
pub struct CountMeasure(pub f64);
pub fn read_stf_count_measure(parser: &mut Parser) -> CountMeasure {
    let (s, v) = parser.read_united_float();
    if s != "COUNT_MEASURE" { panic!("unexpected iden"); }
    CountMeasure(v)
}

#[derive(Debug, PartialEq)]
pub struct LengthMeasure(pub f64);
pub fn read_stf_length_measure(parser: &mut Parser) -> LengthMeasure {
    let (s, v) = parser.read_united_float();
    if s != "LENGTH_MEASURE" { panic!("unexpected iden"); }
    LengthMeasure(v)
}

#[derive(Debug, PartialEq)]
pub struct AreaMeasure(pub f64);
pub fn read_stf_area_measure(parser: &mut Parser) -> AreaMeasure {
    let (s, v) = parser.read_united_float();
    if s != "AREA_MEASURE" { panic!("unexpected iden"); }
    AreaMeasure(v)
}

pub enum AreaMeasureOrVolumeMeasure { AreaMeasure(AreaMeasure), VolumeMeasure(VolumeMeasure) }
pub fn step_c_area_measure_or_volume_measure(parser: &mut Parser) -> AreaMeasureOrVolumeMeasure {
    let (s, v) = parser.read_united_float();
    match &s[..] {
        "AREA_MEASURE" => AreaMeasureOrVolumeMeasure::AreaMeasure(AreaMeasure(v)),
"VOLUME_MEASURE" => AreaMeasureOrVolumeMeasure::VolumeMeasure(VolumeMeasure(v)),
        _ => panic!("unexpected string")
    }
}

pub enum SurfaceSide { Positive, Negative, Both }
pub fn read_enum_surface_side(parser: &mut Parser) -> SurfaceSide {
    let s = parser.read_literal();
    match &s[..] {
        "POSITIVE" => SurfaceSide::Positive, "NEGATIVE" => SurfaceSide::Negative, "BOTH" => SurfaceSide::Both,
        _ => panic!("unexpected string")
    }
}

pub enum Source { Made, Bought, NotKnown }
pub fn read_enum_source(parser: &mut Parser) -> Source {
    let s = parser.read_literal();
    match &s[..] {
        "MADE" => Source::Made, "BOUGHT" => Source::Bought, "NOT_KNOWN" => Source::NotKnown,
        _ => panic!("unexpected string")
    }
}

pub enum BSplineEnum1 { Unspecified, WeDontSupportOneElmentEnumsYet }
pub fn read_enum_b_spline_enum1(parser: &mut Parser) -> BSplineEnum1 {
    let s = parser.read_literal();
    match &s[..] {
        "UNSPECIFIED" => BSplineEnum1::Unspecified, "WE_DONT_SUPPORT_ONE_ELMENT_ENUMS_YET" => BSplineEnum1::WeDontSupportOneElmentEnumsYet,
        _ => panic!("unexpected string")
    }
}

pub enum BSplineEnum2 { PiecewiseBezierKnots, Unspecified, QuasiUniformKnots }
pub fn read_enum_b_spline_enum2(parser: &mut Parser) -> BSplineEnum2 {
    let s = parser.read_literal();
    match &s[..] {
        "PIECEWISE_BEZIER_KNOTS" => BSplineEnum2::PiecewiseBezierKnots, "UNSPECIFIED" => BSplineEnum2::Unspecified, "QUASI_UNIFORM_KNOTS" => BSplineEnum2::QuasiUniformKnots,
        _ => panic!("unexpected string")
    }
}

pub enum TrimmedCurveEnum { Parameter, WeDontSupportOneElmentEnumsYet }
pub fn read_enum_trimmed_curve_enum(parser: &mut Parser) -> TrimmedCurveEnum {
    let s = parser.read_literal();
    match &s[..] {
        "PARAMETER" => TrimmedCurveEnum::Parameter, "WE_DONT_SUPPORT_ONE_ELMENT_ENUMS_YET" => TrimmedCurveEnum::WeDontSupportOneElmentEnumsYet,
        _ => panic!("unexpected string")
    }
}

pub enum DataEntity {
    ComplexBucketType,
    AdvancedBrepShapeRepresentation(String, Vec<Id>, Id),     AdvancedFace(String, Vec<Id>, Id, bool),     ApplicationContext(String),     ApplicationProtocolDefinition(String, String, i32, Id),     Axis2Placement3d(String, Id, Id, Id),     BSplineCurveWithKnots(String, i32, Vec<Id>, BSplineEnum1, bool, bool, Vec<i32>, Vec<f64>, BSplineEnum2),     BSplineSurfaceWithKnots(String, i32, i32, Vec<Vec<Id>>, BSplineEnum1, bool, bool, bool, Vec<i32>, Vec<i32>, Vec<f64>, Vec<f64>, BSplineEnum2),     BrepWithVoids(String, Id, Vec<Id>),     CartesianPoint(String, Vec<f64>),     Circle(String, Id, f64),     ClosedShell(String, Vec<Id>),     ColourRgb(String, f64, f64, f64),     ConicalSurface(String, Id, f64, f64),     ContextDependentShapeRepresentation(Id, Id),     CurveStyle(String, Id, PositiveLengthMeasure, Id),     CylindricalSurface(String, Id, f64),     DerivedUnit(Vec<Id>),     DerivedUnitElement(Id, f64),     DescriptiveRepresentationItem(String, String),     Direction(String, Vec<f64>),     DraughtingPreDefinedColour(String),     DraughtingPreDefinedCurveFont(String),     EdgeCurve(String, Id, Id, Id, bool),     EdgeLoop(String, Vec<Id>),     Ellipse(String, Id, f64, f64),     FaceBound(String, Id, bool),     FillAreaStyle(String, Vec<Id>),     FillAreaStyleColour(String, Id),     GeometricCurveSet(String, Vec<Id>),     ItemDefinedTransformation(String, String, Id, Id),     Line(String, Id, Id),     ManifoldSolidBrep(String, Id),     ManifoldSurfaceShapeRepresentation(String, Vec<Id>, Id),     MeasureRepresentationItem(String, AreaMeasureOrVolumeMeasure, Id),     MechanicalDesignGeometricPresentationRepresentation(String, Vec<Id>, Id),     NextAssemblyUsageOccurrence(String, String, String, Id, Id, Option<String>),     OpenShell(String, Vec<Id>),     OrientedClosedShell(String, Id, bool),     OrientedEdge(String, Id, bool),     OverRidingStyledItem(String, Vec<Id>, Id, Id),     Plane(String, Id),     PresentationLayerAssignment(String, String, Vec<Id>),     PresentationStyleAssignment(Vec<Id>),     PresentationStyleByContext(Vec<Id>, Id),     Product(String, String, String, Vec<Id>),     ProductCategory(String, String),     ProductContext(String, Id, String),     ProductDefinition(String, String, Id, Id),     ProductDefinitionContext(String, Id, String),     ProductDefinitionFormation(String, String, Id),     ProductDefinitionFormationWithSpecifiedSource(String, String, Id, Source),     ProductDefinitionShape(String, String, Id),     ProductRelatedProductCategory(String, Option<String>, Vec<Id>),     PropertyDefinition(String, String, Id),     PropertyDefinitionRepresentation(Id, Id),     Representation(Option<String>, Vec<Id>, Option<Id>),     ShapeAspect(String, String, Id, bool),     ShapeDefinitionRepresentation(Id, Id),     ShapeRepresentation(String, Vec<Id>, Id),     ShapeRepresentationRelationship(String, String, Id, Id),     ShellBasedSurfaceModel(String, Vec<Id>),     SphericalSurface(String, Id, f64),     StyledItem(String, Vec<Id>, Id),     SurfaceOfLinearExtrusion(String, Id, Id),     SurfaceSideStyle(String, Vec<Id>),     SurfaceStyleFillArea(Id),     SurfaceStyleUsage(SurfaceSide, Id),     ToroidalSurface(String, Id, f64, f64),     TrimmedCurve(String, Id, (Id, ParameterValue), (Id, ParameterValue), bool, TrimmedCurveEnum),     UncertaintyMeasureWithUnit(LengthMeasure, Id, String, String),     ValueRepresentationItem(String, CountMeasure),     Vector(String, Id, f64),     VertexPoint(String, Id)
}

pub fn parse_data_func(iden: &str, parser: &mut Parser) -> DataEntity {
    use DataEntity::*;
    parser.skip_whitespace();
    match iden {
        "ADVANCED_BREP_SHAPE_REPRESENTATION" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id_vector, Parser::read_id );
            AdvancedBrepShapeRepresentation(x0, x1, x2)
        }
        "ADVANCED_FACE" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_id_vector, Parser::read_id, Parser::read_bool );
            AdvancedFace(x0, x1, x2, x3)
        }
        "APPLICATION_CONTEXT" => {
            let x0 = read_tuple_1( parser, Parser::read_string );
            ApplicationContext(x0)
        }
        "APPLICATION_PROTOCOL_DEFINITION" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_string, Parser::read_int, Parser::read_id );
            ApplicationProtocolDefinition(x0, x1, x2, x3)
        }
        "AXIS2_PLACEMENT_3D" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_id, Parser::read_id, Parser::read_id );
            Axis2Placement3d(x0, x1, x2, x3)
        }
        "B_SPLINE_CURVE_WITH_KNOTS" => {
            let (x0, x1, x2, x3, x4, x5, x6, x7, x8) = read_tuple_9( parser, Parser::read_string, Parser::read_int, Parser::read_id_vector, read_enum_b_spline_enum1, Parser::read_bool, Parser::read_bool, Parser::read_int_vector, Parser::read_float_vector, read_enum_b_spline_enum2 );
            BSplineCurveWithKnots(x0, x1, x2, x3, x4, x5, x6, x7, x8)
        }
        "B_SPLINE_SURFACE_WITH_KNOTS" => {
            let (x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12) = read_tuple_13( parser, Parser::read_string, Parser::read_int, Parser::read_int, Parser::read_id_vector_vector, read_enum_b_spline_enum1, Parser::read_bool, Parser::read_bool, Parser::read_bool, Parser::read_int_vector, Parser::read_int_vector, Parser::read_float_vector, Parser::read_float_vector, read_enum_b_spline_enum2 );
            BSplineSurfaceWithKnots(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12)
        }
        "BREP_WITH_VOIDS" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id, Parser::read_id_vector );
            BrepWithVoids(x0, x1, x2)
        }
        "CARTESIAN_POINT" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_float_vector );
            CartesianPoint(x0, x1)
        }
        "CIRCLE" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id, Parser::read_float );
            Circle(x0, x1, x2)
        }
        "CLOSED_SHELL" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_id_vector );
            ClosedShell(x0, x1)
        }
        "COLOUR_RGB" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_float, Parser::read_float, Parser::read_float );
            ColourRgb(x0, x1, x2, x3)
        }
        "CONICAL_SURFACE" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_id, Parser::read_float, Parser::read_float );
            ConicalSurface(x0, x1, x2, x3)
        }
        "CONTEXT_DEPENDENT_SHAPE_REPRESENTATION" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_id, Parser::read_id );
            ContextDependentShapeRepresentation(x0, x1)
        }
        "CURVE_STYLE" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_id, read_stf_positive_length_measure, Parser::read_id );
            CurveStyle(x0, x1, x2, x3)
        }
        "CYLINDRICAL_SURFACE" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id, Parser::read_float );
            CylindricalSurface(x0, x1, x2)
        }
        "DERIVED_UNIT" => {
            let x0 = read_tuple_1( parser, Parser::read_id_vector );
            DerivedUnit(x0)
        }
        "DERIVED_UNIT_ELEMENT" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_id, Parser::read_float );
            DerivedUnitElement(x0, x1)
        }
        "DESCRIPTIVE_REPRESENTATION_ITEM" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_string );
            DescriptiveRepresentationItem(x0, x1)
        }
        "DIRECTION" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_float_vector );
            Direction(x0, x1)
        }
        "DRAUGHTING_PRE_DEFINED_COLOUR" => {
            let x0 = read_tuple_1( parser, Parser::read_string );
            DraughtingPreDefinedColour(x0)
        }
        "DRAUGHTING_PRE_DEFINED_CURVE_FONT" => {
            let x0 = read_tuple_1( parser, Parser::read_string );
            DraughtingPreDefinedCurveFont(x0)
        }
        "EDGE_CURVE" => {
            let (x0, x1, x2, x3, x4) = read_tuple_5( parser, Parser::read_string, Parser::read_id, Parser::read_id, Parser::read_id, Parser::read_bool );
            EdgeCurve(x0, x1, x2, x3, x4)
        }
        "EDGE_LOOP" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_id_vector );
            EdgeLoop(x0, x1)
        }
        "ELLIPSE" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_id, Parser::read_float, Parser::read_float );
            Ellipse(x0, x1, x2, x3)
        }
        "FACE_BOUND" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id, Parser::read_bool );
            FaceBound(x0, x1, x2)
        }
        "FILL_AREA_STYLE" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_id_vector );
            FillAreaStyle(x0, x1)
        }
        "FILL_AREA_STYLE_COLOUR" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_id );
            FillAreaStyleColour(x0, x1)
        }
        "GEOMETRIC_CURVE_SET" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_id_vector );
            GeometricCurveSet(x0, x1)
        }
        "ITEM_DEFINED_TRANSFORMATION" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_string, Parser::read_id, Parser::read_id );
            ItemDefinedTransformation(x0, x1, x2, x3)
        }
        "LINE" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id, Parser::read_id );
            Line(x0, x1, x2)
        }
        "MANIFOLD_SOLID_BREP" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_id );
            ManifoldSolidBrep(x0, x1)
        }
        "MANIFOLD_SURFACE_SHAPE_REPRESENTATION" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id_vector, Parser::read_id );
            ManifoldSurfaceShapeRepresentation(x0, x1, x2)
        }
        "MEASURE_REPRESENTATION_ITEM" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, step_c_area_measure_or_volume_measure, Parser::read_id );
            MeasureRepresentationItem(x0, x1, x2)
        }
        "MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id_vector, Parser::read_id );
            MechanicalDesignGeometricPresentationRepresentation(x0, x1, x2)
        }
        "NEXT_ASSEMBLY_USAGE_OCCURRENCE" => {
            let (x0, x1, x2, x3, x4, x5) = read_tuple_6( parser, Parser::read_string, Parser::read_string, Parser::read_string, Parser::read_id, Parser::read_id, Parser::read_string_or_dollar );
            NextAssemblyUsageOccurrence(x0, x1, x2, x3, x4, x5)
        }
        "OPEN_SHELL" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_id_vector );
            OpenShell(x0, x1)
        }
        "ORIENTED_CLOSED_SHELL" => {
            let (x0, _, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_star, Parser::read_id, Parser::read_bool );
            OrientedClosedShell(x0, x2, x3)
        }
        "ORIENTED_EDGE" => {
            let (x0, _, _, x3, x4) = read_tuple_5( parser, Parser::read_string, Parser::read_star, Parser::read_star, Parser::read_id, Parser::read_bool );
            OrientedEdge(x0, x3, x4)
        }
        "OVER_RIDING_STYLED_ITEM" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_id_vector, Parser::read_id, Parser::read_id );
            OverRidingStyledItem(x0, x1, x2, x3)
        }
        "PLANE" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_id );
            Plane(x0, x1)
        }
        "PRESENTATION_LAYER_ASSIGNMENT" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_string, Parser::read_id_vector );
            PresentationLayerAssignment(x0, x1, x2)
        }
        "PRESENTATION_STYLE_ASSIGNMENT" => {
            let x0 = read_tuple_1( parser, Parser::read_id_vector );
            PresentationStyleAssignment(x0)
        }
        "PRESENTATION_STYLE_BY_CONTEXT" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_id_vector, Parser::read_id );
            PresentationStyleByContext(x0, x1)
        }
        "PRODUCT" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_string, Parser::read_string, Parser::read_id_vector );
            Product(x0, x1, x2, x3)
        }
        "PRODUCT_CATEGORY" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_string );
            ProductCategory(x0, x1)
        }
        "PRODUCT_CONTEXT" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id, Parser::read_string );
            ProductContext(x0, x1, x2)
        }
        "PRODUCT_DEFINITION" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_string, Parser::read_id, Parser::read_id );
            ProductDefinition(x0, x1, x2, x3)
        }
        "PRODUCT_DEFINITION_CONTEXT" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id, Parser::read_string );
            ProductDefinitionContext(x0, x1, x2)
        }
        "PRODUCT_DEFINITION_FORMATION" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_string, Parser::read_id );
            ProductDefinitionFormation(x0, x1, x2)
        }
        "PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_string, Parser::read_id, read_enum_source );
            ProductDefinitionFormationWithSpecifiedSource(x0, x1, x2, x3)
        }
        "PRODUCT_DEFINITION_SHAPE" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_string, Parser::read_id );
            ProductDefinitionShape(x0, x1, x2)
        }
        "PRODUCT_RELATED_PRODUCT_CATEGORY" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_string_or_dollar, Parser::read_id_vector );
            ProductRelatedProductCategory(x0, x1, x2)
        }
        "PROPERTY_DEFINITION" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_string, Parser::read_id );
            PropertyDefinition(x0, x1, x2)
        }
        "PROPERTY_DEFINITION_REPRESENTATION" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_id, Parser::read_id );
            PropertyDefinitionRepresentation(x0, x1)
        }
        "REPRESENTATION" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string_or_dollar, Parser::read_id_vector, Parser::read_id_or_dollar );
            Representation(x0, x1, x2)
        }
        "SHAPE_ASPECT" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_string, Parser::read_id, Parser::read_bool );
            ShapeAspect(x0, x1, x2, x3)
        }
        "SHAPE_DEFINITION_REPRESENTATION" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_id, Parser::read_id );
            ShapeDefinitionRepresentation(x0, x1)
        }
        "SHAPE_REPRESENTATION" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id_vector, Parser::read_id );
            ShapeRepresentation(x0, x1, x2)
        }
        "SHAPE_REPRESENTATION_RELATIONSHIP" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_string, Parser::read_id, Parser::read_id );
            ShapeRepresentationRelationship(x0, x1, x2, x3)
        }
        "SHELL_BASED_SURFACE_MODEL" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_id_vector );
            ShellBasedSurfaceModel(x0, x1)
        }
        "SPHERICAL_SURFACE" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id, Parser::read_float );
            SphericalSurface(x0, x1, x2)
        }
        "STYLED_ITEM" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id_vector, Parser::read_id );
            StyledItem(x0, x1, x2)
        }
        "SURFACE_OF_LINEAR_EXTRUSION" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id, Parser::read_id );
            SurfaceOfLinearExtrusion(x0, x1, x2)
        }
        "SURFACE_SIDE_STYLE" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_id_vector );
            SurfaceSideStyle(x0, x1)
        }
        "SURFACE_STYLE_FILL_AREA" => {
            let x0 = read_tuple_1( parser, Parser::read_id );
            SurfaceStyleFillArea(x0)
        }
        "SURFACE_STYLE_USAGE" => {
            let (x0, x1) = read_tuple_2( parser, read_enum_surface_side, Parser::read_id );
            SurfaceStyleUsage(x0, x1)
        }
        "TOROIDAL_SURFACE" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, Parser::read_string, Parser::read_id, Parser::read_float, Parser::read_float );
            ToroidalSurface(x0, x1, x2, x3)
        }
        "TRIMMED_CURVE" => {
            let (x0, x1, x2, x3, x4, x5) = read_tuple_6( parser, Parser::read_string, Parser::read_id, read_pair_id_ParameterValue, read_pair_id_ParameterValue, Parser::read_bool, read_enum_trimmed_curve_enum );
            TrimmedCurve(x0, x1, x2, x3, x4, x5)
        }
        "UNCERTAINTY_MEASURE_WITH_UNIT" => {
            let (x0, x1, x2, x3) = read_tuple_4( parser, read_stf_length_measure, Parser::read_id, Parser::read_string, Parser::read_string );
            UncertaintyMeasureWithUnit(x0, x1, x2, x3)
        }
        "VALUE_REPRESENTATION_ITEM" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, read_stf_count_measure );
            ValueRepresentationItem(x0, x1)
        }
        "VECTOR" => {
            let (x0, x1, x2) = read_tuple_3( parser, Parser::read_string, Parser::read_id, Parser::read_float );
            Vector(x0, x1, x2)
        }
        "VERTEX_POINT" => {
            let (x0, x1) = read_tuple_2( parser, Parser::read_string, Parser::read_id );
            VertexPoint(x0, x1)
        }
,
        _ => panic!("unexpected string")
    }
}
