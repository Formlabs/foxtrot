use crate::ap214_autogen::*;
use crate::parse_basics::{
    after_ws, after_wscomma, paren_tup, step_bool, step_float, step_id, step_identifier, step_opt,
    step_string, step_udecimal, step_vec, Res,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::opt,
    error::VerboseError,
    sequence::{delimited, terminated, tuple},
    Err as NomErr,
};

pub fn step_stf_area_measure(input: &str) -> Res<&str, AreaMeasure> {
    delimited(
        tuple((tag("AREA_MEASURE"), after_ws(tag("(")))),
        after_ws(step_float),
        after_ws(tag(")")),
    )(input)
    .map(|(next_input, res)| (next_input, AreaMeasure(res)))
}

pub fn step_stf_count_measure(input: &str) -> Res<&str, CountMeasure> {
    delimited(
        tuple((tag("COUNT_MEASURE"), after_ws(tag("(")))),
        after_ws(step_float),
        after_ws(tag(")")),
    )(input)
    .map(|(next_input, res)| (next_input, CountMeasure(res)))
}

pub fn step_stf_length_measure(input: &str) -> Res<&str, LengthMeasure> {
    delimited(
        tuple((tag("LENGTH_MEASURE"), after_ws(tag("(")))),
        after_ws(step_float),
        after_ws(tag(")")),
    )(input)
    .map(|(next_input, res)| (next_input, LengthMeasure(res)))
}

pub fn step_stf_parameter_value(input: &str) -> Res<&str, ParameterValue> {
    delimited(
        tuple((tag("PARAMETER_VALUE"), after_ws(tag("(")))),
        after_ws(step_float),
        after_ws(tag(")")),
    )(input)
    .map(|(next_input, res)| (next_input, ParameterValue(res)))
}

pub fn step_stf_positive_length_measure(input: &str) -> Res<&str, PositiveLengthMeasure> {
    delimited(
        tuple((tag("POSITIVE_LENGTH_MEASURE"), after_ws(tag("(")))),
        after_ws(step_float),
        after_ws(tag(")")),
    )(input)
    .map(|(next_input, res)| (next_input, PositiveLengthMeasure(res)))
}

pub fn step_stf_volume_measure(input: &str) -> Res<&str, VolumeMeasure> {
    delimited(
        tuple((tag("VOLUME_MEASURE"), after_ws(tag("(")))),
        after_ws(step_float),
        after_ws(tag(")")),
    )(input)
    .map(|(next_input, res)| (next_input, VolumeMeasure(res)))
}

pub fn step_c_area_measure_or_volume_measure(input: &str) -> Res<&str, AreaMeasureOrVolumeMeasure> {
    tuple((
        alt((tag("AREA_MEASURE"), tag("VOLUME_MEASURE"))),
        delimited(after_ws(tag("(")), after_ws(step_float), after_ws(tag(")"))),
    ))(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (tg, flt) = res;
            match tg {
                "AREA_MEASURE" => AreaMeasureOrVolumeMeasure::AreaMeasure(AreaMeasure(flt)),
                "VOLUME_MEASURE" => AreaMeasureOrVolumeMeasure::VolumeMeasure(VolumeMeasure(flt)),
                _ => panic!("unexpected string"),
            }
        })
    })
}

pub fn step_enum_surface_side(input: &str) -> Res<&str, SurfaceSide> {
    delimited(
        tag("."),
        alt((tag("POSITIVE"), tag("NEGATIVE"), tag("BOTH"))),
        tag("."),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            match res {
                "POSITIVE" => SurfaceSide::Positive,
                "NEGATIVE" => SurfaceSide::Negative,
                "BOTH" => SurfaceSide::Both,
                _ => panic!("unepected string"),
            },
        )
    })
}

pub fn step_enum_source(input: &str) -> Res<&str, Source> {
    delimited(
        tag("."),
        alt((tag("MADE"), tag("BOUGHT"), tag("NOT_KNOWN"))),
        tag("."),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            match res {
                "MADE" => Source::Made,
                "BOUGHT" => Source::Bought,
                "NOT_KNOWN" => Source::NotKnown,
                _ => panic!("unepected string"),
            },
        )
    })
}

pub fn step_enum_b_spline_enum1(input: &str) -> Res<&str, BSplineEnum1> {
    delimited(
        tag("."),
        alt((tag("UNSPECIFIED"), tag("SURF_OF_LINEAR_EXTRUSION"))),
        tag("."),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            match res {
                "UNSPECIFIED" => BSplineEnum1::Unspecified,
                "SURF_OF_LINEAR_EXTRUSION" => BSplineEnum1::SurfOfLinearExtrusion,
                _ => panic!("unepected string"),
            },
        )
    })
}

pub fn step_enum_b_spline_enum2(input: &str) -> Res<&str, BSplineEnum2> {
    delimited(
        tag("."),
        alt((
            tag("PIECEWISE_BEZIER_KNOTS"),
            tag("UNSPECIFIED"),
            tag("QUASI_UNIFORM_KNOTS"),
        )),
        tag("."),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            match res {
                "PIECEWISE_BEZIER_KNOTS" => BSplineEnum2::PiecewiseBezierKnots,
                "UNSPECIFIED" => BSplineEnum2::Unspecified,
                "QUASI_UNIFORM_KNOTS" => BSplineEnum2::QuasiUniformKnots,
                _ => panic!("unepected string"),
            },
        )
    })
}

pub fn step_enum_trimmed_curve_enum(input: &str) -> Res<&str, TrimmedCurveEnum> {
    delimited(
        tag("."),
        alt((
            tag("PARAMETER"),
            tag("WE_DONT_SUPPORT_ONE_ELMENT_ENUMS_YET"),
        )),
        tag("."),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            match res {
                "PARAMETER" => TrimmedCurveEnum::Parameter,
                "WE_DONT_SUPPORT_ONE_ELMENT_ENUMS_YET" => {
                    TrimmedCurveEnum::WeDontSupportOneElmentEnumsYet
                }
                _ => panic!("unepected string"),
            },
        )
    })
}

fn data_entity_advanced_brep_shape_representation<'a>(
    input: &'a str,
) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_vec(step_id)),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::AdvancedBrepShapeRepresentation(x0, x1, x2)
        })
    })
}

fn data_entity_advanced_face<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_vec(step_id)),
            after_wscomma(step_id),
            after_wscomma(step_bool),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::AdvancedFace(x0, x1, x2, x3)
        })
    })
}

fn data_entity_application_context<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        after_ws(step_string),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let x0 = res;
            DataEntity::ApplicationContext(x0)
        })
    })
}

fn data_entity_application_protocol_definition<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_string),
            after_wscomma(step_udecimal),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::ApplicationProtocolDefinition(x0, x1, x2, x3)
        })
    })
}

fn data_entity_axis2_placement_3d<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_id),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::Axis2Placement3d(x0, x1, x2, x3)
        })
    })
}

fn data_entity_b_spline_curve_with_knots<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_udecimal),
            after_wscomma(step_vec(step_id)),
            after_wscomma(step_enum_b_spline_enum1),
            after_wscomma(step_bool),
            after_wscomma(step_bool),
            after_wscomma(step_vec(step_udecimal)),
            after_wscomma(step_vec(step_float)),
            after_wscomma(step_enum_b_spline_enum2),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3, x4, x5, x6, x7, x8) = res;
            DataEntity::BSplineCurveWithKnots(x0, x1, x2, x3, x4, x5, x6, x7, x8)
        })
    })
}

fn data_entity_b_spline_surface_with_knots<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_udecimal),
            after_wscomma(step_udecimal),
            after_wscomma(step_vec(step_vec(step_id))),
            after_wscomma(step_enum_b_spline_enum1),
            after_wscomma(step_bool),
            after_wscomma(step_bool),
            after_wscomma(step_bool),
            after_wscomma(step_vec(step_udecimal)),
            after_wscomma(step_vec(step_udecimal)),
            after_wscomma(step_vec(step_float)),
            after_wscomma(step_vec(step_float)),
            after_wscomma(step_enum_b_spline_enum2),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12) = res;
            DataEntity::BSplineSurfaceWithKnots(
                x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12,
            )
        })
    })
}

fn data_entity_brep_with_voids<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_vec(step_id)),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::BrepWithVoids(x0, x1, x2)
        })
    })
}

fn data_entity_cartesian_point<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_vec(step_float)))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::CartesianPoint(x0, x1)
        })
    })
}

fn data_entity_circle<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_float),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::Circle(x0, x1, x2)
        })
    })
}

fn data_entity_closed_shell<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::ClosedShell(x0, x1)
        })
    })
}

fn data_entity_colour_rgb<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_float),
            after_wscomma(step_float),
            after_wscomma(step_float),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::ColourRgb(x0, x1, x2, x3)
        })
    })
}

fn data_entity_conical_surface<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_float),
            after_wscomma(step_float),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::ConicalSurface(x0, x1, x2, x3)
        })
    })
}

fn data_entity_context_dependent_shape_representation<'a>(
    input: &'a str,
) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_id), after_wscomma(step_id))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::ContextDependentShapeRepresentation(x0, x1)
        })
    })
}

fn data_entity_curve_style<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_stf_positive_length_measure),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::CurveStyle(x0, x1, x2, x3)
        })
    })
}

fn data_entity_cylindrical_surface<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_float),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::CylindricalSurface(x0, x1, x2)
        })
    })
}

fn data_entity_derived_unit<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        after_ws(step_vec(step_id)),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let x0 = res;
            DataEntity::DerivedUnit(x0)
        })
    })
}

fn data_entity_derived_unit_element<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_id), after_wscomma(step_float))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::DerivedUnitElement(x0, x1)
        })
    })
}

fn data_entity_descriptive_representation_item<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_string))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::DescriptiveRepresentationItem(x0, x1)
        })
    })
}

fn data_entity_direction<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_vec(step_float)))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::Direction(x0, x1)
        })
    })
}

fn data_entity_draughting_pre_defined_colour<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        after_ws(step_string),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let x0 = res;
            DataEntity::DraughtingPreDefinedColour(x0)
        })
    })
}

fn data_entity_draughting_pre_defined_curve_font<'a>(
    input: &'a str,
) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        after_ws(step_string),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let x0 = res;
            DataEntity::DraughtingPreDefinedCurveFont(x0)
        })
    })
}

fn data_entity_edge_curve<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_id),
            after_wscomma(step_id),
            after_wscomma(step_bool),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3, x4) = res;
            DataEntity::EdgeCurve(x0, x1, x2, x3, x4)
        })
    })
}

fn data_entity_edge_loop<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::EdgeLoop(x0, x1)
        })
    })
}

fn data_entity_ellipse<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_float),
            after_wscomma(step_float),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::Ellipse(x0, x1, x2, x3)
        })
    })
}

fn data_entity_face_bound<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_bool),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::FaceBound(x0, x1, x2)
        })
    })
}

fn data_entity_fill_area_style<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::FillAreaStyle(x0, x1)
        })
    })
}

fn data_entity_fill_area_style_colour<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_id))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::FillAreaStyleColour(x0, x1)
        })
    })
}

fn data_entity_geometric_curve_set<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::GeometricCurveSet(x0, x1)
        })
    })
}

fn data_entity_item_defined_transformation<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_string),
            after_wscomma(step_id),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::ItemDefinedTransformation(x0, x1, x2, x3)
        })
    })
}

fn data_entity_line<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::Line(x0, x1, x2)
        })
    })
}

fn data_entity_manifold_solid_brep<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_id))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::ManifoldSolidBrep(x0, x1)
        })
    })
}

fn data_entity_manifold_surface_shape_representation<'a>(
    input: &'a str,
) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_vec(step_id)),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::ManifoldSurfaceShapeRepresentation(x0, x1, x2)
        })
    })
}

fn data_entity_measure_representation_item<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_c_area_measure_or_volume_measure),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::MeasureRepresentationItem(x0, x1, x2)
        })
    })
}

fn data_entity_mechanical_design_geometric_presentation_representation<'a>(
    input: &'a str,
) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_vec(step_id)),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::MechanicalDesignGeometricPresentationRepresentation(x0, x1, x2)
        })
    })
}

fn data_entity_next_assembly_usage_occurrence<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_string),
            after_wscomma(step_string),
            after_wscomma(step_id),
            after_wscomma(step_id),
            after_wscomma(step_opt(step_string)),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3, x4, x5) = res;
            DataEntity::NextAssemblyUsageOccurrence(x0, x1, x2, x3, x4, x5)
        })
    })
}

fn data_entity_open_shell<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::OpenShell(x0, x1)
        })
    })
}

fn data_entity_oriented_closed_shell<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(tag("*")),
            after_wscomma(step_id),
            after_wscomma(step_bool),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, _, x2, x3) = res;
            DataEntity::OrientedClosedShell(x0, x2, x3)
        })
    })
}

fn data_entity_oriented_edge<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(tag("*")),
            after_wscomma(tag("*")),
            after_wscomma(step_id),
            after_wscomma(step_bool),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, _, _, x3, x4) = res;
            DataEntity::OrientedEdge(x0, x3, x4)
        })
    })
}

fn data_entity_over_riding_styled_item<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_vec(step_id)),
            after_wscomma(step_id),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::OverRidingStyledItem(x0, x1, x2, x3)
        })
    })
}

fn data_entity_plane<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_id))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::Plane(x0, x1)
        })
    })
}

fn data_entity_presentation_layer_assignment<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_string),
            after_wscomma(step_vec(step_id)),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::PresentationLayerAssignment(x0, x1, x2)
        })
    })
}

fn data_entity_presentation_style_assignment<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        after_ws(step_vec(step_id)),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let x0 = res;
            DataEntity::PresentationStyleAssignment(x0)
        })
    })
}

fn data_entity_presentation_style_by_context<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_vec(step_id)), after_wscomma(step_id))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::PresentationStyleByContext(x0, x1)
        })
    })
}

fn data_entity_product<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_string),
            after_wscomma(step_string),
            after_wscomma(step_vec(step_id)),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::Product(x0, x1, x2, x3)
        })
    })
}

fn data_entity_product_category<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_string))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::ProductCategory(x0, x1)
        })
    })
}

fn data_entity_product_context<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_string),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::ProductContext(x0, x1, x2)
        })
    })
}

fn data_entity_product_definition<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_string),
            after_wscomma(step_id),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::ProductDefinition(x0, x1, x2, x3)
        })
    })
}

fn data_entity_product_definition_context<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_string),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::ProductDefinitionContext(x0, x1, x2)
        })
    })
}

fn data_entity_product_definition_formation<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_string),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::ProductDefinitionFormation(x0, x1, x2)
        })
    })
}

fn data_entity_product_definition_formation_with_specified_source<'a>(
    input: &'a str,
) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_string),
            after_wscomma(step_id),
            after_wscomma(step_enum_source),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::ProductDefinitionFormationWithSpecifiedSource(x0, x1, x2, x3)
        })
    })
}

fn data_entity_product_definition_shape<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_string),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::ProductDefinitionShape(x0, x1, x2)
        })
    })
}

fn data_entity_product_related_product_category<'a>(
    input: &'a str,
) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_opt(step_string)),
            after_wscomma(step_vec(step_id)),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::ProductRelatedProductCategory(x0, x1, x2)
        })
    })
}

fn data_entity_property_definition<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_string),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::PropertyDefinition(x0, x1, x2)
        })
    })
}

fn data_entity_property_definition_representation<'a>(
    input: &'a str,
) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_id), after_wscomma(step_id))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::PropertyDefinitionRepresentation(x0, x1)
        })
    })
}

fn data_entity_representation<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_opt(step_string)),
            after_wscomma(step_vec(step_id)),
            after_wscomma(step_opt(step_id)),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::Representation(x0, x1, x2)
        })
    })
}

fn data_entity_shape_aspect<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_string),
            after_wscomma(step_id),
            after_wscomma(step_bool),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::ShapeAspect(x0, x1, x2, x3)
        })
    })
}

fn data_entity_shape_definition_representation<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_id), after_wscomma(step_id))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::ShapeDefinitionRepresentation(x0, x1)
        })
    })
}

fn data_entity_shape_representation<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_vec(step_id)),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::ShapeRepresentation(x0, x1, x2)
        })
    })
}

fn data_entity_shape_representation_relationship<'a>(
    input: &'a str,
) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_string),
            after_wscomma(step_id),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::ShapeRepresentationRelationship(x0, x1, x2, x3)
        })
    })
}

fn data_entity_shell_based_surface_model<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::ShellBasedSurfaceModel(x0, x1)
        })
    })
}

fn data_entity_spherical_surface<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_float),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::SphericalSurface(x0, x1, x2)
        })
    })
}

fn data_entity_styled_item<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_vec(step_id)),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::StyledItem(x0, x1, x2)
        })
    })
}

fn data_entity_surface_of_linear_extrusion<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_id),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::SurfaceOfLinearExtrusion(x0, x1, x2)
        })
    })
}

fn data_entity_surface_side_style<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_vec(step_id)))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::SurfaceSideStyle(x0, x1)
        })
    })
}

fn data_entity_surface_style_fill_area<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        after_ws(step_id),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let x0 = res;
            DataEntity::SurfaceStyleFillArea(x0)
        })
    })
}

fn data_entity_surface_style_usage<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_enum_surface_side), after_wscomma(step_id))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::SurfaceStyleUsage(x0, x1)
        })
    })
}

fn data_entity_toroidal_surface<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_float),
            after_wscomma(step_float),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::ToroidalSurface(x0, x1, x2, x3)
        })
    })
}

fn data_entity_trimmed_curve<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(delimited(
                tag("("),
                tuple((after_ws(step_id), after_wscomma(step_stf_parameter_value))),
                after_ws(tag(")")),
            )),
            after_wscomma(delimited(
                tag("("),
                tuple((after_ws(step_id), after_wscomma(step_stf_parameter_value))),
                after_ws(tag(")")),
            )),
            after_wscomma(step_bool),
            after_wscomma(step_enum_trimmed_curve_enum),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3, x4, x5) = res;
            DataEntity::TrimmedCurve(x0, x1, x2, x3, x4, x5)
        })
    })
}

fn data_entity_uncertainty_measure_with_unit<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_stf_length_measure),
            after_wscomma(step_id),
            after_wscomma(step_string),
            after_wscomma(step_string),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2, x3) = res;
            DataEntity::UncertaintyMeasureWithUnit(x0, x1, x2, x3)
        })
    })
}

fn data_entity_value_representation_item<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_stf_count_measure))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::ValueRepresentationItem(x0, x1)
        })
    })
}

fn data_entity_vector<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((
            after_ws(step_string),
            after_wscomma(step_id),
            after_wscomma(step_float),
        )),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1, x2) = res;
            DataEntity::Vector(x0, x1, x2)
        })
    })
}

fn data_entity_vertex_loop<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_id))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::VertexLoop(x0, x1)
        })
    })
}

fn data_entity_vertex_point<'a>(input: &'a str) -> Res<&'a str, DataEntity<'a>> {
    delimited(
        after_ws(tag("(")),
        tuple((after_ws(step_string), after_wscomma(step_id))),
        tuple((after_ws(tag(")")), after_ws(tag(";")))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (x0, x1) = res;
            DataEntity::VertexPoint(x0, x1)
        })
    })
}

pub fn data_entity_complex_bucket_type(input: &str) -> Res<&str, DataEntity> {
    terminated(paren_tup, after_ws(tag(";")))(input)
        .map(|(next_input, _)| (next_input, DataEntity::ComplexBucketType))
}

pub fn data_line(input: &str) -> Res<&str, (Id, DataEntity)> {
    let res = tuple((
        after_ws(step_id),
        after_ws(tag("=")),
        after_ws(opt(step_identifier)),
    ))(input);
    if res.is_err() {
        return res.map(|(a, (b, _, _))| (a, (b, DataEntity::ComplexBucketType)));
    }
    let (next_input, (id, _, opt_iden)) = res.expect("should be ok");

    if opt_iden.is_none() {
        return data_entity_complex_bucket_type(next_input)
            .map(|(next_input, ent)| (next_input, (id, ent)));
    }

    match opt_iden.unwrap() {
        "ADVANCED_BREP_SHAPE_REPRESENTATION" => {
            data_entity_advanced_brep_shape_representation(next_input)
        }
        "ADVANCED_FACE" => data_entity_advanced_face(next_input),
        "APPLICATION_CONTEXT" => data_entity_application_context(next_input),
        "APPLICATION_PROTOCOL_DEFINITION" => {
            data_entity_application_protocol_definition(next_input)
        }
        "AXIS2_PLACEMENT_3D" => data_entity_axis2_placement_3d(next_input),
        "B_SPLINE_CURVE_WITH_KNOTS" => data_entity_b_spline_curve_with_knots(next_input),
        "B_SPLINE_SURFACE_WITH_KNOTS" => data_entity_b_spline_surface_with_knots(next_input),
        "BREP_WITH_VOIDS" => data_entity_brep_with_voids(next_input),
        "CARTESIAN_POINT" => data_entity_cartesian_point(next_input),
        "CIRCLE" => data_entity_circle(next_input),
        "CLOSED_SHELL" => data_entity_closed_shell(next_input),
        "COLOUR_RGB" => data_entity_colour_rgb(next_input),
        "CONICAL_SURFACE" => data_entity_conical_surface(next_input),
        "CONTEXT_DEPENDENT_SHAPE_REPRESENTATION" => {
            data_entity_context_dependent_shape_representation(next_input)
        }
        "CURVE_STYLE" => data_entity_curve_style(next_input),
        "CYLINDRICAL_SURFACE" => data_entity_cylindrical_surface(next_input),
        "DERIVED_UNIT" => data_entity_derived_unit(next_input),
        "DERIVED_UNIT_ELEMENT" => data_entity_derived_unit_element(next_input),
        "DESCRIPTIVE_REPRESENTATION_ITEM" => {
            data_entity_descriptive_representation_item(next_input)
        }
        "DIRECTION" => data_entity_direction(next_input),
        "DRAUGHTING_PRE_DEFINED_COLOUR" => data_entity_draughting_pre_defined_colour(next_input),
        "DRAUGHTING_PRE_DEFINED_CURVE_FONT" => {
            data_entity_draughting_pre_defined_curve_font(next_input)
        }
        "EDGE_CURVE" => data_entity_edge_curve(next_input),
        "EDGE_LOOP" => data_entity_edge_loop(next_input),
        "ELLIPSE" => data_entity_ellipse(next_input),
        "FACE_BOUND" => data_entity_face_bound(next_input),
        "FILL_AREA_STYLE" => data_entity_fill_area_style(next_input),
        "FILL_AREA_STYLE_COLOUR" => data_entity_fill_area_style_colour(next_input),
        "GEOMETRIC_CURVE_SET" => data_entity_geometric_curve_set(next_input),
        "ITEM_DEFINED_TRANSFORMATION" => data_entity_item_defined_transformation(next_input),
        "LINE" => data_entity_line(next_input),
        "MANIFOLD_SOLID_BREP" => data_entity_manifold_solid_brep(next_input),
        "MANIFOLD_SURFACE_SHAPE_REPRESENTATION" => {
            data_entity_manifold_surface_shape_representation(next_input)
        }
        "MEASURE_REPRESENTATION_ITEM" => data_entity_measure_representation_item(next_input),
        "MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION" => {
            data_entity_mechanical_design_geometric_presentation_representation(next_input)
        }
        "NEXT_ASSEMBLY_USAGE_OCCURRENCE" => data_entity_next_assembly_usage_occurrence(next_input),
        "OPEN_SHELL" => data_entity_open_shell(next_input),
        "ORIENTED_CLOSED_SHELL" => data_entity_oriented_closed_shell(next_input),
        "ORIENTED_EDGE" => data_entity_oriented_edge(next_input),
        "OVER_RIDING_STYLED_ITEM" => data_entity_over_riding_styled_item(next_input),
        "PLANE" => data_entity_plane(next_input),
        "PRESENTATION_LAYER_ASSIGNMENT" => data_entity_presentation_layer_assignment(next_input),
        "PRESENTATION_STYLE_ASSIGNMENT" => data_entity_presentation_style_assignment(next_input),
        "PRESENTATION_STYLE_BY_CONTEXT" => data_entity_presentation_style_by_context(next_input),
        "PRODUCT" => data_entity_product(next_input),
        "PRODUCT_CATEGORY" => data_entity_product_category(next_input),
        "PRODUCT_CONTEXT" => data_entity_product_context(next_input),
        "PRODUCT_DEFINITION" => data_entity_product_definition(next_input),
        "PRODUCT_DEFINITION_CONTEXT" => data_entity_product_definition_context(next_input),
        "PRODUCT_DEFINITION_FORMATION" => data_entity_product_definition_formation(next_input),
        "PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE" => {
            data_entity_product_definition_formation_with_specified_source(next_input)
        }
        "PRODUCT_DEFINITION_SHAPE" => data_entity_product_definition_shape(next_input),
        "PRODUCT_RELATED_PRODUCT_CATEGORY" => {
            data_entity_product_related_product_category(next_input)
        }
        "PROPERTY_DEFINITION" => data_entity_property_definition(next_input),
        "PROPERTY_DEFINITION_REPRESENTATION" => {
            data_entity_property_definition_representation(next_input)
        }
        "REPRESENTATION" => data_entity_representation(next_input),
        "SHAPE_ASPECT" => data_entity_shape_aspect(next_input),
        "SHAPE_DEFINITION_REPRESENTATION" => {
            data_entity_shape_definition_representation(next_input)
        }
        "SHAPE_REPRESENTATION" => data_entity_shape_representation(next_input),
        "SHAPE_REPRESENTATION_RELATIONSHIP" => {
            data_entity_shape_representation_relationship(next_input)
        }
        "SHELL_BASED_SURFACE_MODEL" => data_entity_shell_based_surface_model(next_input),
        "SPHERICAL_SURFACE" => data_entity_spherical_surface(next_input),
        "STYLED_ITEM" => data_entity_styled_item(next_input),
        "SURFACE_OF_LINEAR_EXTRUSION" => data_entity_surface_of_linear_extrusion(next_input),
        "SURFACE_SIDE_STYLE" => data_entity_surface_side_style(next_input),
        "SURFACE_STYLE_FILL_AREA" => data_entity_surface_style_fill_area(next_input),
        "SURFACE_STYLE_USAGE" => data_entity_surface_style_usage(next_input),
        "TOROIDAL_SURFACE" => data_entity_toroidal_surface(next_input),
        "TRIMMED_CURVE" => data_entity_trimmed_curve(next_input),
        "UNCERTAINTY_MEASURE_WITH_UNIT" => data_entity_uncertainty_measure_with_unit(next_input),
        "VALUE_REPRESENTATION_ITEM" => data_entity_value_representation_item(next_input),
        "VECTOR" => data_entity_vector(next_input),
        "VERTEX_LOOP" => data_entity_vertex_loop(next_input),
        "VERTEX_POINT" => data_entity_vertex_point(next_input),
        _ => Err(NomErr::Error(VerboseError { errors: vec![] })),
    }
    .map(|(next_input, ent)| (next_input, (id, ent)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_brep_shape_representation() {
        assert!(
            data_entity_advanced_brep_shape_representation("('',(#11,#162601),#162751);").is_ok()
        );
        assert!(
            data_entity_advanced_brep_shape_representation("('',(#11,#172997),#173477);").is_ok()
        );
        assert!(
            data_entity_advanced_brep_shape_representation("('',(#11,#177013),#177555);").is_ok()
        );
        assert!(
            data_entity_advanced_brep_shape_representation("('',(#11,#164807),#164957);").is_ok()
        );
        assert!(
            data_entity_advanced_brep_shape_representation("('',(#11,#369341),#371680);").is_ok()
        );
        assert!(
            data_entity_advanced_brep_shape_representation("('',(#11,#163891),#164041);").is_ok()
        );
        assert!(data_entity_advanced_brep_shape_representation("('',(#11,#8249),#8791);").is_ok());
        assert!(
            data_entity_advanced_brep_shape_representation("('',(#11,#162171),#162321);").is_ok()
        );
        assert!(
            data_entity_advanced_brep_shape_representation("('',(#11,#381801),#382783);").is_ok()
        );
        assert!(
            data_entity_advanced_brep_shape_representation("('',(#11,#35262),#39038);").is_ok()
        );
    }

    #[test]
    fn test_advanced_face() {
        assert!(data_entity_advanced_face("('',(#171209),#171227,.T.);").is_ok());
        assert!(data_entity_advanced_face("('',(#5709),#5715,.F.);").is_ok());
        assert!(data_entity_advanced_face("('',(#278724),#278735,.T.);").is_ok());
        assert!(data_entity_advanced_face("('',(#310987),#310998,.T.);").is_ok());
        assert!(data_entity_advanced_face("('',(#164638),#164663,.T.);").is_ok());
        assert!(data_entity_advanced_face("('',(#64437),#64474,.F.);").is_ok());
        assert!(data_entity_advanced_face("('',(#171305),#171327,.F.);").is_ok());
        assert!(data_entity_advanced_face("('',(#363889),#363905,.F.);").is_ok());
        assert!(data_entity_advanced_face("('',(#388369),#388396,.T.);").is_ok());
        assert!(data_entity_advanced_face("('',(#355788),#355799,.T.);").is_ok());
    }

    #[test]
    fn test_application_context() {
        assert!(data_entity_application_context(
            "(\n  'core data for automotive mechanical design processes');"
        )
        .is_ok());
    }

    #[test]
    fn test_application_protocol_definition() {
        assert!(data_entity_application_protocol_definition(
            "('international standard',\n  'automotive_design',2000,#2);"
        )
        .is_ok());
    }

    #[test]
    fn test_axis2_placement_3d() {
        assert!(data_entity_axis2_placement_3d("('',#348424,#348425,#348426);").is_ok());
        assert!(data_entity_axis2_placement_3d("('',#16936,#16937,#16938);").is_ok());
        assert!(data_entity_axis2_placement_3d("('',#249822,#249823,#249824);").is_ok());
        assert!(data_entity_axis2_placement_3d("('',#340719,#340720,#340721);").is_ok());
        assert!(data_entity_axis2_placement_3d("('',#82806,#82807,#82808);").is_ok());
        assert!(data_entity_axis2_placement_3d("('',#151744,#151745,#151746);").is_ok());
        assert!(data_entity_axis2_placement_3d("('',#278029,#278030,#278031);").is_ok());
        assert!(data_entity_axis2_placement_3d("('',#144629,#144630,#144631);").is_ok());
        assert!(data_entity_axis2_placement_3d("('',#204620,#204621,#204622);").is_ok());
        assert!(data_entity_axis2_placement_3d("('',#170730,#170731,#170732);").is_ok());
    }

    #[test]
    fn test_b_spline_curve_with_knots() {
        assert!(data_entity_b_spline_curve_with_knots("('',3,(#273781,#273782,#273783,\n    #273784),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("('',3,(#252094,#252095,#252096,\n    #252097),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("('',3,(#269158,#269159,#269160,\n    #269161),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("('',3,(#181755,#181756,#181757,\n    #181758),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,0.10471975512),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("('',3,(#60082,#60083,#60084,#60085,\n    #60086,#60087,#60088,#60089,#60090,#60091),.UNSPECIFIED.,.F.,.F.,(4,\n    2,2,2,4),(5.205788810599E-012,1.241232100071E-003,\n    2.482464194937E-003,3.723696289803E-003,4.964928384668E-003),\n  .UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("('',3,(#51619,#51620,#51621,#51622,\n    #51623,#51624),.UNSPECIFIED.,.F.,.F.,(4,2,4),(1.737237004028E-016,\n    1.961965290198E-004,3.923930580394E-004),.UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("('',3,(#266473,#266474,#266475,\n    #266476),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,1.),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("('',3,(#6407,#6408,#6409,#6410),\n  .UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,0.10471975512),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("('',3,(#132351,#132352,#132353,\n    #132354),.UNSPECIFIED.,.F.,.F.,(4,4),(0.E+000,0.10471975512),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_curve_with_knots("('',4,(#182468,#182469,#182470,\n    #182471,#182472),.UNSPECIFIED.,.F.,.F.,(5,5),(0.E+000,1.570796326795\n    ),.PIECEWISE_BEZIER_KNOTS.);").is_ok());
    }

    #[test]
    fn test_b_spline_surface_with_knots() {
        assert!(data_entity_b_spline_surface_with_knots("('',8,8,(\n    (#131931,#131932,#131933,#131934,#131935,#131936,#131937,#131938\n      ,#131939)\n    ,(#131940,#131941,#131942,#131943,#131944,#131945,#131946,#131947\n      ,#131948)\n    ,(#131949,#131950,#131951,#131952,#131953,#131954,#131955,#131956\n      ,#131957)\n    ,(#131958,#131959,#131960,#131961,#131962,#131963,#131964,#131965\n      ,#131966)\n    ,(#131967,#131968,#131969,#131970,#131971,#131972,#131973,#131974\n      ,#131975)\n    ,(#131976,#131977,#131978,#131979,#131980,#131981,#131982,#131983\n      ,#131984)\n    ,(#131985,#131986,#131987,#131988,#131989,#131990,#131991,#131992\n      ,#131993)\n    ,(#131994,#131995,#131996,#131997,#131998,#131999,#132000,#132001\n      ,#132002)\n    ,(#132003,#132004,#132005,#132006,#132007,#132008,#132009,#132010\n      ,#132011\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-7.778174593052E-002,\n    7.778174593052E-002),(-7.315613219613E-003,8.855910256723E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("('',3,3,(\n    (#57194,#57195,#57196,#57197)\n    ,(#57198,#57199,#57200,#57201)\n    ,(#57202,#57203,#57204,#57205)\n    ,(#57206,#57207,#57208,#57209)\n    ,(#57210,#57211,#57212,#57213)\n    ,(#57214,#57215,#57216,#57217)\n    ,(#57218,#57219,#57220,#57221)\n    ,(#57222,#57223,#57224,#57225)\n    ,(#57226,#57227,#57228,#57229\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,1,1,1,4),(4,4),(-0.419707394122,\n    0.E+000,1.923076923077E-002,0.5,0.980769230769,1.,1.463233547653),(\n    0.215417803204,0.784582200388),.UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("('',8,8,(\n    (#132647,#132648,#132649,#132650,#132651,#132652,#132653,#132654\n      ,#132655)\n    ,(#132656,#132657,#132658,#132659,#132660,#132661,#132662,#132663\n      ,#132664)\n    ,(#132665,#132666,#132667,#132668,#132669,#132670,#132671,#132672\n      ,#132673)\n    ,(#132674,#132675,#132676,#132677,#132678,#132679,#132680,#132681\n      ,#132682)\n    ,(#132683,#132684,#132685,#132686,#132687,#132688,#132689,#132690\n      ,#132691)\n    ,(#132692,#132693,#132694,#132695,#132696,#132697,#132698,#132699\n      ,#132700)\n    ,(#132701,#132702,#132703,#132704,#132705,#132706,#132707,#132708\n      ,#132709)\n    ,(#132710,#132711,#132712,#132713,#132714,#132715,#132716,#132717\n      ,#132718)\n    ,(#132719,#132720,#132721,#132722,#132723,#132724,#132725,#132726\n      ,#132727\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-7.778174593052E-002,\n    7.778174593052E-002),(-8.855910256723E-003,7.315613219614E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("('',8,8,(\n    (#45154,#45155,#45156,#45157,#45158,#45159,#45160,#45161,#45162)\n    ,(#45163,#45164,#45165,#45166,#45167,#45168,#45169,#45170,#45171)\n    ,(#45172,#45173,#45174,#45175,#45176,#45177,#45178,#45179,#45180)\n    ,(#45181,#45182,#45183,#45184,#45185,#45186,#45187,#45188,#45189)\n    ,(#45190,#45191,#45192,#45193,#45194,#45195,#45196,#45197,#45198)\n    ,(#45199,#45200,#45201,#45202,#45203,#45204,#45205,#45206,#45207)\n    ,(#45208,#45209,#45210,#45211,#45212,#45213,#45214,#45215,#45216)\n    ,(#45217,#45218,#45219,#45220,#45221,#45222,#45223,#45224,#45225)\n    ,(#45226,#45227,#45228,#45229,#45230,#45231,#45232,#45233,#45234\n  )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-7.778174593052E-002,\n    7.778174593052E-002),(-8.855910256723E-003,7.315613219615E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("('',3,3,(\n    (#59855,#59856,#59857,#59858)\n    ,(#59859,#59860,#59861,#59862)\n    ,(#59863,#59864,#59865,#59866)\n    ,(#59867,#59868,#59869,#59870)\n    ,(#59871,#59872,#59873,#59874)\n    ,(#59875,#59876,#59877,#59878\n  )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,4),(4,4),(-2.000202717131E-002,\n    0.E+000,1.,1.020000040435),(0.21531949858,0.784693586529),\n  .UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("('',3,3,(\n    (#58127,#58128,#58129,#58130)\n    ,(#58131,#58132,#58133,#58134)\n    ,(#58135,#58136,#58137,#58138)\n    ,(#58139,#58140,#58141,#58142)\n    ,(#58143,#58144,#58145,#58146)\n    ,(#58147,#58148,#58149,#58150)\n    ,(#58151,#58152,#58153,#58154)\n    ,(#58155,#58156,#58157,#58158)\n    ,(#58159,#58160,#58161,#58162\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,1,1,1,4),(4,4),(-0.4269626569,\n    0.E+000,1.923076923076E-002,0.5,0.980769230769,1.,1.447328917973),(\n    0.215417802387,0.784582200455),.UNSPECIFIED.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("('',8,8,(\n    (#182685,#182686,#182687,#182688,#182689,#182690,#182691,#182692\n      ,#182693)\n    ,(#182694,#182695,#182696,#182697,#182698,#182699,#182700,#182701\n      ,#182702)\n    ,(#182703,#182704,#182705,#182706,#182707,#182708,#182709,#182710\n      ,#182711)\n    ,(#182712,#182713,#182714,#182715,#182716,#182717,#182718,#182719\n      ,#182720)\n    ,(#182721,#182722,#182723,#182724,#182725,#182726,#182727,#182728\n      ,#182729)\n    ,(#182730,#182731,#182732,#182733,#182734,#182735,#182736,#182737\n      ,#182738)\n    ,(#182739,#182740,#182741,#182742,#182743,#182744,#182745,#182746\n      ,#182747)\n    ,(#182748,#182749,#182750,#182751,#182752,#182753,#182754,#182755\n      ,#182756)\n    ,(#182757,#182758,#182759,#182760,#182761,#182762,#182763,#182764\n      ,#182765\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-3.889087296526E-002,\n    3.889087296526E-002),(-3.626740088442E-003,4.427879780914E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("('',8,8,(\n    (#181969,#181970,#181971,#181972,#181973,#181974,#181975,#181976\n      ,#181977)\n    ,(#181978,#181979,#181980,#181981,#181982,#181983,#181984,#181985\n      ,#181986)\n    ,(#181987,#181988,#181989,#181990,#181991,#181992,#181993,#181994\n      ,#181995)\n    ,(#181996,#181997,#181998,#181999,#182000,#182001,#182002,#182003\n      ,#182004)\n    ,(#182005,#182006,#182007,#182008,#182009,#182010,#182011,#182012\n      ,#182013)\n    ,(#182014,#182015,#182016,#182017,#182018,#182019,#182020,#182021\n      ,#182022)\n    ,(#182023,#182024,#182025,#182026,#182027,#182028,#182029,#182030\n      ,#182031)\n    ,(#182032,#182033,#182034,#182035,#182036,#182037,#182038,#182039\n      ,#182040)\n    ,(#182041,#182042,#182043,#182044,#182045,#182046,#182047,#182048\n      ,#182049\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-3.889087296526E-002,\n    3.889087296526E-002),(-3.626740088442E-003,4.427879780914E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("('',8,8,(\n    (#377673,#377674,#377675,#377676,#377677,#377678,#377679,#377680\n      ,#377681)\n    ,(#377682,#377683,#377684,#377685,#377686,#377687,#377688,#377689\n      ,#377690)\n    ,(#377691,#377692,#377693,#377694,#377695,#377696,#377697,#377698\n      ,#377699)\n    ,(#377700,#377701,#377702,#377703,#377704,#377705,#377706,#377707\n      ,#377708)\n    ,(#377709,#377710,#377711,#377712,#377713,#377714,#377715,#377716\n      ,#377717)\n    ,(#377718,#377719,#377720,#377721,#377722,#377723,#377724,#377725\n      ,#377726)\n    ,(#377727,#377728,#377729,#377730,#377731,#377732,#377733,#377734\n      ,#377735)\n    ,(#377736,#377737,#377738,#377739,#377740,#377741,#377742,#377743\n      ,#377744)\n    ,(#377745,#377746,#377747,#377748,#377749,#377750,#377751,#377752\n      ,#377753\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(9,9),(9,9),(-7.778174593052E-002,\n    7.778174593052E-002),(-8.855910256723E-003,7.315613219613E-003),\n  .PIECEWISE_BEZIER_KNOTS.);").is_ok());
        assert!(data_entity_b_spline_surface_with_knots("('',3,3,(\n    (#56832,#56833,#56834,#56835)\n    ,(#56836,#56837,#56838,#56839)\n    ,(#56840,#56841,#56842,#56843)\n    ,(#56844,#56845,#56846,#56847)\n    ,(#56848,#56849,#56850,#56851)\n    ,(#56852,#56853,#56854,#56855)\n    ,(#56856,#56857,#56858,#56859\n    )),.UNSPECIFIED.,.F.,.F.,.F.,(4,1,1,1,4),(4,4),(5.323075212263E-002,\n    0.148700901957,0.503777838603,0.85885477525,0.981649641487),(\n    -6.575958308042E-003,0.708622850024),.UNSPECIFIED.);").is_ok());
    }

    #[test]
    fn test_brep_with_voids() {
        assert!(data_entity_brep_with_voids("('',#67616,(#89927,#90077));").is_ok());
    }

    #[test]
    fn test_cartesian_point() {
        assert!(data_entity_cartesian_point("('',(6.,2.65,1.6));").is_ok());
        assert!(data_entity_cartesian_point(
            "('',(5.9199999994,11.67498095209,-2.6699999998)\n  );"
        )
        .is_ok());
        assert!(data_entity_cartesian_point("('',(-6.675,5.089123043511E-017,-5.09));").is_ok());
        assert!(data_entity_cartesian_point("('',(33.5,-0.5,11.75));").is_ok());
        assert!(data_entity_cartesian_point("('',(-6.83,0.68,-2.1));").is_ok());
        assert!(data_entity_cartesian_point("('',(-1.875,-2.1,-7.05));").is_ok());
        assert!(
            data_entity_cartesian_point("('',(0.14294104,-5.315711999999E-002,0.E+000));").is_ok()
        );
        assert!(data_entity_cartesian_point("('',(32.,-2.05,9.9));").is_ok());
        assert!(data_entity_cartesian_point(
            "('',(6.201663060342,1.801193405256E-012,\n    -13.9061182435));"
        )
        .is_ok());
        assert!(data_entity_cartesian_point("('',(-0.2499995,-2.89999928,0.E+000));").is_ok());
    }

    #[test]
    fn test_circle() {
        assert!(data_entity_circle("('',#133619,0.1);").is_ok());
        assert!(data_entity_circle("('',#320468,4.2);").is_ok());
        assert!(data_entity_circle("('',#39471,5.E-002);").is_ok());
        assert!(data_entity_circle("('',#67776,0.36);").is_ok());
        assert!(data_entity_circle("('',#386299,5.E-002);").is_ok());
        assert!(data_entity_circle("('',#18218,0.15);").is_ok());
        assert!(data_entity_circle("('',#193668,5.E-002);").is_ok());
        assert!(data_entity_circle("('',#60959,0.3);").is_ok());
        assert!(data_entity_circle("('',#394352,1.2E-002);").is_ok());
        assert!(data_entity_circle("('',#348101,0.25);").is_ok());
    }

    #[test]
    fn test_closed_shell() {
        assert!(data_entity_closed_shell("('',(#381803,#381875,#381923,#381947,#381972,\n    #381996,#382037,#382061,#382101,#382118,#382143,#382167,#382209,\n    #382240,#382265,#382314,#382331,#382380,#382396,#382421,#382454,\n    #382471,#382487,#382518,#382551,#382575,#382593,#382617,#382651,\n    #382668,#382685,#382703,#382720,#382743,#382761,#382772));").is_ok());
        assert!(data_entity_closed_shell("('',(#160741,#160781,#160812,#160843,#160874,\n    #160905,#160936,#160967,#160998,#161029,#161060,#161091,#161122,\n    #161153,#161184,#161215,#161246,#161277,#161308,#161339,#161370,\n    #161401,#161432,#161463,#161494,#161525,#161556,#161578,#161613));").is_ok());
        assert!(data_entity_closed_shell("('',(#170967,#171009,#171049,#171080,#171113,\n    #171144,#171175,#171208,#171232,#171249,#171273,#171304,#171332,\n    #171350,#171367,#171379,#171397,#171408));").is_ok());
        assert!(data_entity_closed_shell("('',(#11154,#11194,#11234,#11267,#11300,#11331,\n    #11362,#11395,#11428,#11472,#11496,#11533,#11550,#11562));").is_ok());
        assert!(data_entity_closed_shell("('',(#391605,#391645,#391676,#391707,#391738,\n    #391769,#391800,#391831,#391862,#391893,#391924,#391946,#391965));").is_ok());
        assert!(data_entity_closed_shell("('',(#193483,#193525,#193565,#193596,#193629,\n    #193660,#193691,#193724,#193748,#193765,#193789,#193820,#193848,\n    #193866,#193883,#193895,#193913,#193924));").is_ok());
        assert!(data_entity_closed_shell(
            "('',(#374392,#374432,#374463,#374494,#374516,\n    #374528));"
        )
        .is_ok());
        assert!(data_entity_closed_shell("('',(#134199,#134239,#134272,#134305,#134338,\n    #134371,#134388,#134405,#134436,#134467,#134484,#134515,#134532,\n    #134563,#134582,#134601,#134634,#134653,#134686,#134705,#134738,\n    #134771,#134867,#134963,#134994,#135090,#135121,#135217,#135248,\n    #135279,#135298,#135317,#135350,#135369,#135402,#135421,#135454,\n    #135487,#135583,#135679,#135710,#135806,#135837,#135933,#135964,\n    #135995,#136013,#136031,#136064,#136082,#136113,#136131,#136162,\n    #136191,#136202,#136213,#136225,#136236));").is_ok());
        assert!(data_entity_closed_shell(
            "('',(#26706,#26746,#26786,#26817,#26846,#26863,\n    #26894,#26906,#26924,#26942));"
        )
        .is_ok());
        assert!(data_entity_closed_shell(
            "('',(#162603,#162643,#162674,#162705,#162727,\n    #162739));"
        )
        .is_ok());
    }

    #[test]
    fn test_colour_rgb() {
        assert!(
            data_entity_colour_rgb("('',0.286274522543,0.662745118141,0.329411774874);").is_ok()
        );
        assert!(
            data_entity_colour_rgb("('',0.184313729405,0.749019622803,0.580392181873);").is_ok()
        );
        assert!(
            data_entity_colour_rgb("('',0.800000011921,0.800000011921,0.800000011921);").is_ok()
        );
        assert!(
            data_entity_colour_rgb("('',0.792156875134,0.819607853889,0.933333337307);").is_ok()
        );
        assert!(data_entity_colour_rgb("('',0.600000023842,0.40000000596,0.20000000298);").is_ok());
        assert!(
            data_entity_colour_rgb("('',0.643137276173,0.678431391716,0.698039233685);").is_ok()
        );
        assert!(
            data_entity_colour_rgb("('',0.898039221764,0.921568632126,0.929411768913);").is_ok()
        );
        assert!(data_entity_colour_rgb("('',0.20000000298,0.20000000298,0.20000000298);").is_ok());
        assert!(data_entity_colour_rgb("('',1.,0.937254905701,0.137254908681);").is_ok());
        assert!(
            data_entity_colour_rgb("('',0.188235297799,0.188235297799,0.188235297799);").is_ok()
        );
    }

    #[test]
    fn test_conical_surface() {
        assert!(data_entity_conical_surface("('',#241129,0.234530705359,0.523574705607);").is_ok());
        assert!(data_entity_conical_surface("('',#72037,1.28,0.352833819799);").is_ok());
        assert!(data_entity_conical_surface("('',#72904,1.574999999996,0.463647608998);").is_ok());
        assert!(data_entity_conical_surface("('',#72665,1.28,0.352833819799);").is_ok());
        assert!(data_entity_conical_surface("('',#169889,0.974999999529,0.785398163397);").is_ok());
        assert!(data_entity_conical_surface("('',#169907,0.974999999529,0.785398163397);").is_ok());
        assert!(data_entity_conical_surface("('',#336063,4.9,0.785398163397);").is_ok());
        assert!(data_entity_conical_surface("('',#241067,0.634540825454,0.523583912325);").is_ok());
        assert!(data_entity_conical_surface("('',#73062,1.575000000001,0.352833819801);").is_ok());
        assert!(data_entity_conical_surface("('',#241007,0.634540825454,0.523583912325);").is_ok());
    }

    #[test]
    fn test_context_dependent_shape_representation() {
        assert!(data_entity_context_dependent_shape_representation("(#198923,#198925);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("(#137447,#137449);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("(#337390,#337392);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("(#200216,#200218);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("(#166013,#166015);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("(#333128,#333130);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("(#24177,#24179);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("(#389456,#389458);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("(#198172,#198174);").is_ok());
        assert!(data_entity_context_dependent_shape_representation("(#323162,#323164);").is_ok());
    }

    #[test]
    fn test_curve_style() {
        assert!(
            data_entity_curve_style("('',#412981,POSITIVE_LENGTH_MEASURE(0.1),#406004);").is_ok()
        );
        assert!(
            data_entity_curve_style("('',#402574,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok()
        );
        assert!(
            data_entity_curve_style("('',#402223,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok()
        );
        assert!(
            data_entity_curve_style("('',#412935,POSITIVE_LENGTH_MEASURE(0.1),#403072);").is_ok()
        );
        assert!(
            data_entity_curve_style("('',#406720,POSITIVE_LENGTH_MEASURE(0.1),#406718);").is_ok()
        );
        assert!(
            data_entity_curve_style("('',#402709,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok()
        );
        assert!(
            data_entity_curve_style("('',#413044,POSITIVE_LENGTH_MEASURE(0.1),#406004);").is_ok()
        );
        assert!(
            data_entity_curve_style("('',#401863,POSITIVE_LENGTH_MEASURE(0.1),#401618);").is_ok()
        );
        assert!(
            data_entity_curve_style("('',#413440,POSITIVE_LENGTH_MEASURE(0.1),#406004);").is_ok()
        );
        assert!(
            data_entity_curve_style("('',#416525,POSITIVE_LENGTH_MEASURE(0.1),#416523);").is_ok()
        );
    }

    #[test]
    fn test_cylindrical_surface() {
        assert!(data_entity_cylindrical_surface("('',#243866,1.8);").is_ok());
        assert!(data_entity_cylindrical_surface("('',#363261,0.635);").is_ok());
        assert!(data_entity_cylindrical_surface("('',#25154,1.62E-002);").is_ok());
        assert!(data_entity_cylindrical_surface("('',#327547,1.62E-002);").is_ok());
        assert!(data_entity_cylindrical_surface("('',#387251,5.E-002);").is_ok());
        assert!(data_entity_cylindrical_surface("('',#393187,1.2E-002);").is_ok());
        assert!(data_entity_cylindrical_surface("('',#312188,1.3);").is_ok());
        assert!(data_entity_cylindrical_surface("('',#5980,0.1);").is_ok());
        assert!(data_entity_cylindrical_surface("('',#291598,1.3);").is_ok());
        assert!(data_entity_cylindrical_surface("('',#241430,1.8);").is_ok());
    }

    #[test]
    fn test_derived_unit() {
        assert!(data_entity_derived_unit("((#427287));").is_ok());
        assert!(data_entity_derived_unit("((#427258));").is_ok());
        assert!(data_entity_derived_unit("((#427280));").is_ok());
        assert!(data_entity_derived_unit("((#427265));").is_ok());
    }

    #[test]
    fn test_derived_unit_element() {
        assert!(data_entity_derived_unit_element("(#427288,3.);").is_ok());
        assert!(data_entity_derived_unit_element("(#427259,2.);").is_ok());
        assert!(data_entity_derived_unit_element("(#427281,2.);").is_ok());
        assert!(data_entity_derived_unit_element("(#427266,3.);").is_ok());
    }

    #[test]
    fn test_descriptive_representation_item() {
        assert!(data_entity_descriptive_representation_item("('MOD_NUM','MOD41249');").is_ok());
        assert!(data_entity_descriptive_representation_item("('PART_REV','B');").is_ok());
    }

    #[test]
    fn test_direction() {
        assert!(data_entity_direction("('',(1.,0.E+000,0.E+000));").is_ok());
        assert!(data_entity_direction("('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_entity_direction("('',(0.E+000,1.,0.E+000));").is_ok());
        assert!(data_entity_direction("('',(0.917281684548,-0.E+000,0.398239012644));").is_ok());
        assert!(data_entity_direction("('',(0.E+000,-0.537075932183,0.843533901553));").is_ok());
        assert!(data_entity_direction("('',(-1.,0.E+000,0.E+000));").is_ok());
        assert!(data_entity_direction("('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_entity_direction("('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_entity_direction("('',(1.,0.E+000,0.E+000));").is_ok());
        assert!(data_entity_direction("('',(1.,0.E+000,-0.E+000));").is_ok());
    }

    #[test]
    fn test_draughting_pre_defined_colour() {
        assert!(data_entity_draughting_pre_defined_colour("('green');").is_ok());
        assert!(data_entity_draughting_pre_defined_colour("('white');").is_ok());
        assert!(data_entity_draughting_pre_defined_colour("('black');").is_ok());
        assert!(data_entity_draughting_pre_defined_colour("('yellow');").is_ok());
    }

    #[test]
    fn test_draughting_pre_defined_curve_font() {
        assert!(data_entity_draughting_pre_defined_curve_font("('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("('continuous');").is_ok());
        assert!(data_entity_draughting_pre_defined_curve_font("('continuous');").is_ok());
    }

    #[test]
    fn test_edge_curve() {
        assert!(data_entity_edge_curve("('',#111835,#119485,#119493,.T.);").is_ok());
        assert!(data_entity_edge_curve("('',#285778,#286392,#286394,.T.);").is_ok());
        assert!(data_entity_edge_curve("('',#83157,#83061,#83175,.T.);").is_ok());
        assert!(data_entity_edge_curve("('',#19698,#19729,#19731,.T.);").is_ok());
        assert!(data_entity_edge_curve("('',#215243,#215251,#215253,.T.);").is_ok());
        assert!(data_entity_edge_curve("('',#187057,#187083,#187085,.T.);").is_ok());
        assert!(data_entity_edge_curve("('',#240221,#276642,#276644,.T.);").is_ok());
        assert!(data_entity_edge_curve("('',#373112,#373131,#373140,.T.);").is_ok());
        assert!(data_entity_edge_curve("('',#73809,#70109,#73811,.T.);").is_ok());
        assert!(data_entity_edge_curve("('',#215499,#222691,#222693,.T.);").is_ok());
    }

    #[test]
    fn test_edge_loop() {
        assert!(data_entity_edge_loop("('',(#72646,#72647,#72656,#72663));").is_ok());
        assert!(data_entity_edge_loop("('',(#305283,#305284,#305293,#305302));").is_ok());
        assert!(data_entity_edge_loop("('',(#306013,#306014,#306020,#306021));").is_ok());
        assert!(data_entity_edge_loop("('',(#29498,#29506,#29514,#29520));").is_ok());
        assert!(data_entity_edge_loop(
            "('',(#240673,#240674,#240680,#240681,#240682,#240688,\n    #240689,#240697));"
        )
        .is_ok());
        assert!(data_entity_edge_loop(
            "('',(#88124,#88125,#88126,#88127,#88128,#88129,#88137,\n    #88145));"
        )
        .is_ok());
        assert!(data_entity_edge_loop("('',(#176525,#176526,#176535,#176543));").is_ok());
        assert!(data_entity_edge_loop("('',(#87677,#87678,#87679,#87685));").is_ok());
        assert!(data_entity_edge_loop("('',(#54148,#54149,#54157,#54165));").is_ok());
        assert!(data_entity_edge_loop("('',(#147470,#147471,#147479,#147487));").is_ok());
    }

    #[test]
    fn test_ellipse() {
        assert!(data_entity_ellipse("('',#181654,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("('',#184302,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("('',#184903,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("('',#175139,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("('',#177791,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("('',#178501,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("('',#133252,0.150260191002,0.10625);").is_ok());
        assert!(data_entity_ellipse("('',#174489,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("('',#185029,5.027541397818E-002,5.E-002);").is_ok());
        assert!(data_entity_ellipse("('',#185044,5.027541397818E-002,5.E-002);").is_ok());
    }

    #[test]
    fn test_face_bound() {
        assert!(data_entity_face_bound("('',#72645,.T.);").is_ok());
        assert!(data_entity_face_bound("('',#305282,.F.);").is_ok());
        assert!(data_entity_face_bound("('',#306012,.F.);").is_ok());
        assert!(data_entity_face_bound("('',#29497,.T.);").is_ok());
        assert!(data_entity_face_bound("('',#240672,.F.);").is_ok());
        assert!(data_entity_face_bound("('',#88123,.T.);").is_ok());
        assert!(data_entity_face_bound("('',#176524,.F.);").is_ok());
        assert!(data_entity_face_bound("('',#87676,.T.);").is_ok());
        assert!(data_entity_face_bound("('',#54147,.T.);").is_ok());
        assert!(data_entity_face_bound("('',#147469,.T.);").is_ok());
    }

    #[test]
    fn test_fill_area_style() {
        assert!(data_entity_fill_area_style("('',(#412124));").is_ok());
        assert!(data_entity_fill_area_style("('',(#424065));").is_ok());
        assert!(data_entity_fill_area_style("('',(#417993));").is_ok());
        assert!(data_entity_fill_area_style("('',(#400477));").is_ok());
        assert!(data_entity_fill_area_style("('',(#409795));").is_ok());
        assert!(data_entity_fill_area_style("('',(#402356));").is_ok());
        assert!(data_entity_fill_area_style("('',(#421958));").is_ok());
        assert!(data_entity_fill_area_style("('',(#418273));").is_ok());
        assert!(data_entity_fill_area_style("('',(#417139));").is_ok());
        assert!(data_entity_fill_area_style("('',(#400729));").is_ok());
    }

    #[test]
    fn test_fill_area_style_colour() {
        assert!(data_entity_fill_area_style_colour("('',#403072);").is_ok());
        assert!(data_entity_fill_area_style_colour("('',#416551);").is_ok());
        assert!(data_entity_fill_area_style_colour("('',#416551);").is_ok());
        assert!(data_entity_fill_area_style_colour("('',#399418);").is_ok());
        assert!(data_entity_fill_area_style_colour("('',#399650);").is_ok());
        assert!(data_entity_fill_area_style_colour("('',#401618);").is_ok());
        assert!(data_entity_fill_area_style_colour("('',#421110);").is_ok());
        assert!(data_entity_fill_area_style_colour("('',#416551);").is_ok());
        assert!(data_entity_fill_area_style_colour("('',#416551);").is_ok());
        assert!(data_entity_fill_area_style_colour("('',#399418);").is_ok());
    }

    #[test]
    fn test_geometric_curve_set() {
        assert!(data_entity_geometric_curve_set("('',(#371717,#371725));").is_ok());
        assert!(data_entity_geometric_curve_set("('',(#371700,#371708));").is_ok());
    }

    #[test]
    fn test_item_defined_transformation() {
        assert!(data_entity_item_defined_transformation("('','',#11,#198913);").is_ok());
        assert!(data_entity_item_defined_transformation("('','',#11,#683);").is_ok());
        assert!(data_entity_item_defined_transformation("('','',#11,#1283);").is_ok());
        assert!(data_entity_item_defined_transformation("('','',#11,#146983);").is_ok());
        assert!(data_entity_item_defined_transformation("('','',#11,#166003);").is_ok());
        assert!(data_entity_item_defined_transformation("('','',#11,#333114);").is_ok());
        assert!(data_entity_item_defined_transformation("('','',#11,#24167);").is_ok());
        assert!(data_entity_item_defined_transformation("('','',#11,#387620);").is_ok());
        assert!(data_entity_item_defined_transformation("('','',#11,#146691);").is_ok());
        assert!(data_entity_item_defined_transformation("('','',#11,#1075);").is_ok());
    }

    #[test]
    fn test_line() {
        assert!(data_entity_line("('',#205153,#205154);").is_ok());
        assert!(data_entity_line("('',#233846,#233847);").is_ok());
        assert!(data_entity_line("('',#250073,#250074);").is_ok());
        assert!(data_entity_line("('',#206849,#206850);").is_ok());
        assert!(data_entity_line("('',#48223,#48224);").is_ok());
        assert!(data_entity_line("('',#228178,#228179);").is_ok());
        assert!(data_entity_line("('',#110196,#110197);").is_ok());
        assert!(data_entity_line("('',#36891,#36892);").is_ok());
        assert!(data_entity_line("('',#144070,#144071);").is_ok());
        assert!(data_entity_line("('',#91760,#91761);").is_ok());
    }

    #[test]
    fn test_manifold_solid_brep() {
        assert!(data_entity_manifold_solid_brep("('',#395362);").is_ok());
        assert!(data_entity_manifold_solid_brep("('',#157779);").is_ok());
        assert!(data_entity_manifold_solid_brep("('',#195674);").is_ok());
        assert!(data_entity_manifold_solid_brep("('',#374391);").is_ok());
        assert!(data_entity_manifold_solid_brep("('',#149884);").is_ok());
        assert!(data_entity_manifold_solid_brep("('',#11153);").is_ok());
        assert!(data_entity_manifold_solid_brep("('',#152807);").is_ok());
        assert!(data_entity_manifold_solid_brep("('',#136273);").is_ok());
        assert!(data_entity_manifold_solid_brep("('',#26705);").is_ok());
        assert!(data_entity_manifold_solid_brep("('',#202441);").is_ok());
    }

    #[test]
    fn test_manifold_surface_shape_representation() {
        assert!(data_entity_manifold_surface_shape_representation(
            "('',(#11,#383554),#383590\n  );"
        )
        .is_ok());
        assert!(data_entity_manifold_surface_shape_representation(
            "('',(#11,#385621),#385657\n  );"
        )
        .is_ok());
        assert!(data_entity_manifold_surface_shape_representation(
            "('',(#11,#131064),#131083\n  );"
        )
        .is_ok());
        assert!(
            data_entity_manifold_surface_shape_representation("('',(#11,#5821),#5840);").is_ok()
        );
        assert!(data_entity_manifold_surface_shape_representation(
            "('',(#11,#129314),#129333\n  );"
        )
        .is_ok());
        assert!(data_entity_manifold_surface_shape_representation(
            "('',(#11,#389232),#389268\n  );"
        )
        .is_ok());
    }

    #[test]
    fn test_measure_representation_item() {
        assert!(data_entity_measure_representation_item(
            "('volume measure',VOLUME_MEASURE(\n    109.45690237608),#427286);"
        )
        .is_ok());
        assert!(data_entity_measure_representation_item(
            "('surface area measure',\n  AREA_MEASURE(3.584814638318E+003),#427257);"
        )
        .is_ok());
        assert!(data_entity_measure_representation_item(
            "('surface area measure',\n  AREA_MEASURE(328.74256832),#427279);"
        )
        .is_ok());
        assert!(data_entity_measure_representation_item(
            "('volume measure',VOLUME_MEASURE(\n    1.977283702949E+003),#427264);"
        )
        .is_ok());
    }

    #[test]
    fn test_mechanical_design_geometric_presentation_representation() {
        assert!(data_entity_mechanical_design_geometric_presentation_representation("('',(\n    #416537,#416544,#416552,#416559,#416566,#416573,#416580,#416587,\n    #416594,#416601,#416608,#416615,#416622,#416629,#416636,#416643,\n    #416650,#416657,#416664,#416671,#416678,#416685,#416692,#416699,\n    #416706,#416713,#416720,#416727,#416734,#416741,#416748,#416755,\n    #416762,#416769,#416776,#416783,#416790,#416797,#416804,#416811,\n    #416818,#416825,#416832,#416839,#416846,#416853,#416860,#416867,\n    #416874,#416881,#416888,#416895,#416902,#416909,#416916,#416923,\n    #416930,#416937,#416944,#416951,#416958,#416965,#416972,#416979,\n    #416986,#416993,#417000,#417007,#417014,#417021,#417028,#417035,\n    #417042,#417049,#417056,#417063,#417070,#417077,#417084,#417091,\n    #417098,#417105,#417112,#417119,#417126,#417133,#417140,#417147,\n    #417154,#417161,#417168,#417175,#417182,#417189,#417196,#417203,\n    #417210,#417217,#417224,#417231,#417238,#417245,#417252,#417259,\n    #417266,#417273,#417280,#417287,#417294,#417301,#417308,#417315,\n    #417322,#417329,#417336,#417343,#417350,#417357,#417364,#417371,\n    #417378,#417385,#417392,#417399,#417406,#417413,#417420,#417427,\n    #417434,#417441,#417448,#417455,#417462,#417469,#417476,#417483,\n    #417490,#417497,#417504,#417511,#417518,#417525,#417532,#417539,\n    #417546,#417553,#417560,#417567,#417574,#417581,#417588,#417595,\n    #417602,#417609,#417616,#417623,#417630,#417637,#417644,#417651,\n    #417658,#417665,#417672,#417679,#417686,#417693,#417700,#417707,\n    #417714,#417721,#417728,#417735,#417742,#417749,#417756,#417763,\n    #417770,#417777,#417784,#417791,#417798,#417805,#417812,#417819,\n    #417826,#417833,#417840,#417847,#417854,#417861,#417868,#417875,\n    #417882,#417889,#417896,#417903,#417910,#417917,#417924,#417931,\n    #417938,#417945,#417952,#417959,#417966,#417973,#417980,#417987,\n    #417994,#418001,#418008,#418015,#418022,#418029,#418036,#418043,\n    #418050,#418057,#418064,#418071,#418078,#418085,#418092,#418099,\n    #418106,#418113,#418120,#418127,#418134,#418141,#418148,#418155,\n    #418162,#418169,#418176,#418183,#418190,#418197,#418204,#418211,\n    #418218,#418225,#418232,#418239,#418246,#418253,#418260,#418267,\n    #418274,#418281,#418288,#418295,#418302,#418309,#418316,#418323,\n    #418330,#418337,#418344,#418351,#418358,#418365,#418372,#418379,\n    #418386,#418393,#418400,#418407,#418414,#418421,#418428,#418435,\n    #418442,#418449,#418456,#418463,#418470,#418477,#418484,#418491,\n    #418498,#418505,#418512,#418519,#418526,#418533,#418540,#418547,\n    #418554,#418561,#418568,#418575,#418582,#418589,#418596,#418603,\n    #418610,#418617,#418624,#418631,#418638,#418645,#418652,#418659,\n    #418666,#418673,#418680,#418687,#418694,#418701,#418708,#418715,\n    #418722,#418729,#418736,#418743,#418750,#418757,#418764,#418771,\n    #418778,#418785,#418792,#418799,#418806,#418813,#418820,#418827,\n    #418834,#418841,#418848,#418855,#418862,#418869,#418876,#418883,\n    #418890,#418897,#418904,#418911,#418918,#418925,#418932,#418939,\n    #418946,#418953,#418960,#418967,#418974,#418981,#418988,#418995,\n    #419002,#419009,#419016,#419023,#419030,#419037,#419044,#419051,\n    #419058,#419065,#419072,#419079,#419086,#419093,#419100,#419107,\n    #419114,#419121,#419128,#419135,#419142,#419149,#419156,#419163,\n    #419170,#419177,#419184,#419191,#419198,#419205,#419212,#419219,\n    #419226,#419233,#419240,#419247,#419254,#419261,#419268,#419275,\n    #419282,#419289,#419296,#419303,#419310,#419317,#419324,#419331,\n    #419338,#419345,#419352,#419359,#419366,#419373,#419380,#419387,\n    #419394,#419401,#419408,#419415,#419422,#419429,#419436,#419443,\n    #419450,#419457,#419464,#419471,#419478,#419485,#419492,#419499,\n    #419506,#419513,#419520,#419527,#419534,#419541,#419548,#419555,\n    #419562,#419569,#419576,#419583,#419590,#419597,#419604,#419611,\n    #419618,#419625,#419632,#419639,#419646,#419653,#419660,#419667,\n    #419674,#419681,#419688,#419695,#419702,#419709,#419716,#419723,\n    #419730,#419737,#419744,#419751,#419758,#419765,#419772,#419779,\n    #419786,#419793,#419800,#419807,#419814,#419821,#419828,#419835,\n    #419842,#419849,#419856,#419863,#419870,#419877,#419884,#419891,\n    #419898,#419905,#419912,#419919,#419926,#419933,#419940,#419947,\n    #419954,#419961,#419968,#419975,#419982,#419989,#419996,#420003,\n    #420010,#420017,#420024,#420031,#420038,#420045,#420052,#420059,\n    #420066,#420073,#420080,#420087,#420094,#420101,#420108,#420115,\n    #420122,#420129,#420136,#420143,#420150,#420157,#420164,#420171,\n    #420178,#420185,#420192,#420199,#420206,#420213,#420220,#420227,\n    #420234,#420241,#420248,#420255,#420262,#420269,#420276,#420283,\n    #420290,#420297,#420304,#420311,#420318,#420325,#420332,#420339,\n    #420346,#420353,#420360,#420367,#420374,#420381,#420388,#420395,\n    #420402,#420409,#420416,#420423,#420430,#420437,#420444,#420451,\n    #420458,#420465,#420472,#420479,#420486,#420493,#420500,#420507,\n    #420514,#420521,#420528,#420535,#420542,#420549,#420556,#420563,\n    #420570,#420577,#420584,#420591,#420598,#420605,#420612,#420619,\n    #420626,#420633,#420640,#420647,#420654,#420661,#420668,#420675,\n    #420682,#420689,#420696,#420703,#420710,#420717,#420724,#420731,\n    #420738,#420745,#420752,#420759,#420766,#420773,#420780,#420787,\n    #420794,#420801,#420808,#420815,#420822,#420829,#420836,#420843,\n    #420850,#420857,#420864,#420871,#420878,#420885,#420892,#420899,\n    #420906,#420913,#420920,#420927,#420934,#420941,#420948,#420955,\n    #420962,#420969,#420976,#420983,#420990,#420997,#421004,#421011,\n    #421018,#421025,#421032,#421039,#421046,#421053,#421060,#421067,\n    #421074,#421081,#421088,#421096,#421103,#421111,#421118,#421125,\n    #421133,#421140,#421147,#421154,#421161,#421168,#421175,#421182,\n    #421189,#421196,#421203,#421210,#421217,#421224,#421231,#421238,\n    #421245,#421252,#421259,#421266,#421273,#421280,#421287,#421294,\n    #421301,#421308,#421315,#421322,#421329,#421336,#421343,#421350,\n    #421357,#421364,#421371,#421378,#421385,#421392,#421399,#421406,\n    #421413,#421420,#421427,#421434,#421441,#421448,#421455,#421462,\n    #421469,#421476,#421483,#421490,#421497,#421504,#421511,#421518,\n    #421525,#421532,#421539,#421546,#421553,#421560,#421567,#421574,\n    #421581,#421588,#421595,#421602,#421609,#421616,#421623,#421630,\n    #421637,#421644,#421651,#421658,#421665,#421672,#421679,#421686,\n    #421693,#421700,#421707,#421714,#421721,#421728,#421735,#421742,\n    #421749,#421756,#421763,#421770,#421777,#421784,#421791,#421798,\n    #421805,#421812,#421819,#421826,#421833,#421840,#421847,#421854,\n    #421861,#421868,#421875,#421882,#421889,#421896,#421903,#421910,\n    #421917,#421924,#421931,#421938,#421945,#421952,#421959,#421966,\n    #421973,#421980,#421987,#421994,#422001,#422008,#422015,#422022,\n    #422029,#422036,#422043,#422050,#422057,#422064,#422071,#422078,\n    #422085,#422092,#422099,#422106,#422113,#422120,#422127,#422134,\n    #422141,#422148,#422155,#422162,#422169,#422176,#422183,#422190,\n    #422197,#422204,#422211,#422218,#422225,#422232,#422239,#422246,\n    #422253,#422260,#422267,#422274,#422281,#422288,#422295,#422302,\n    #422309,#422316,#422323,#422330,#422337,#422344,#422351,#422358,\n    #422365,#422372,#422379,#422386,#422393,#422400,#422407,#422414,\n    #422421,#422428,#422435,#422442,#422449,#422456,#422463,#422470,\n    #422477,#422484,#422491,#422498,#422505,#422512,#422519,#422526,\n    #422533,#422540,#422547,#422554,#422561,#422568,#422575,#422582,\n    #422589,#422596,#422603,#422610,#422617,#422624,#422631,#422638,\n    #422645,#422652,#422659,#422666,#422673,#422680,#422687,#422694,\n    #422701,#422708,#422715,#422722,#422729,#422736,#422743,#422750,\n    #422757,#422764,#422771,#422778,#422785,#422792,#422799,#422806,\n    #422813,#422820,#422827,#422834,#422841,#422848,#422855,#422862,\n    #422869,#422876,#422883,#422890,#422897,#422904,#422911,#422918,\n    #422925,#422932,#422939,#422946,#422953,#422960,#422967,#422974,\n    #422981,#422988,#422995,#423002,#423009,#423016,#423023,#423030,\n    #423037,#423044,#423051,#423058,#423065,#423072,#423079,#423086,\n    #423093,#423100,#423107,#423114,#423121,#423128,#423135,#423142,\n    #423149,#423156,#423163,#423170,#423177,#423184,#423191,#423198,\n    #423205,#423212,#423219,#423226,#423233,#423240,#423247,#423254,\n    #423261,#423268,#423275,#423282,#423289,#423296,#423303,#423310,\n    #423317,#423324,#423331,#423338,#423345,#423352,#423359,#423366,\n    #423373,#423380,#423387,#423394,#423401,#423408,#423415,#423422,\n    #423429,#423436,#423443,#423450,#423457,#423464,#423471,#423478,\n    #423485,#423492,#423499,#423506,#423513,#423520,#423527,#423534,\n    #423541,#423548,#423555,#423562,#423569,#423576,#423583,#423590,\n    #423597,#423604,#423611,#423618,#423625,#423632,#423639,#423646,\n    #423653,#423660,#423667,#423674,#423681,#423688,#423695,#423702,\n    #423709,#423716,#423723,#423730,#423737,#423744,#423751,#423758,\n    #423765,#423772,#423779,#423786,#423793,#423800,#423807,#423814,\n    #423821,#423828,#423835,#423842,#423849,#423856,#423863,#423870,\n    #423877,#423884,#423891,#423898,#423905,#423912,#423919,#423926,\n    #423933,#423940,#423947,#423954,#423961,#423968,#423975,#423982,\n    #423989,#423996,#424003,#424010,#424017,#424024,#424031,#424038,\n    #424045,#424052,#424059,#424066,#424073,#424080,#424087,#424094,\n    #424101,#424108,#424115,#424122,#424129,#424136,#424143,#424150,\n    #424157,#424164,#424171,#424178,#424185,#424192,#424199,#424206,\n    #424213,#424220,#424227,#424234,#424241,#424248,#424255,#424262,\n    #424269,#424276,#424283,#424290,#424297,#424304,#424311,#424318,\n    #424325,#424332,#424339,#424346,#424353,#424360,#424367,#424374,\n    #424381,#424388,#424395,#424402,#424409,#424416,#424423,#424430,\n    #424437,#424444,#424451,#424458,#424465,#424472,#424479,#424486,\n    #424493,#424500,#424507,#424514,#424521,#424528,#424535,#424542,\n    #424549,#424556,#424563,#424570,#424577,#424584,#424591,#424598,\n    #424605,#424612,#424619,#424626,#424633,#424640,#424647,#424654,\n    #424661,#424668,#424675,#424682,#424689,#424696,#424703,#424710,\n    #424717,#424724,#424731,#424738,#424745,#424752,#424759,#424766,\n    #424773,#424780,#424787,#424794,#424801,#424808,#424815,#424822,\n    #424829,#424836,#424843,#424850,#424857,#424864,#424871,#424878,\n    #424885,#424892,#424899,#424906,#424913,#424920,#424927,#424934,\n    #424941,#424948,#424955,#424962,#424969,#424976,#424983,#424990,\n    #424997,#425004,#425011,#425018,#425025,#425032,#425039,#425046,\n    #425053,#425060,#425067,#425074,#425081,#425088,#425095,#425102,\n    #425109,#425116,#425123,#425130,#425137,#425144,#425151,#425158,\n    #425165,#425172,#425179,#425186,#425193,#425200,#425207,#425214,\n    #425221,#425228,#425235,#425242,#425249,#425256,#425263,#425270,\n    #425277,#425284,#425291,#425298,#425305,#425312,#425319,#425326,\n    #425333,#425340,#425347,#425354,#425361,#425368,#425375,#425382,\n    #425389,#425396,#425403,#425410,#425417,#425424,#425431,#425438,\n    #425445,#425452,#425459,#425466,#425473,#425480,#425487,#425494,\n    #425501,#425508,#425515,#425522,#425529,#425536,#425543,#425550,\n    #425557,#425564,#425571,#425578,#425585,#425592,#425599,#425606,\n    #425613,#425620,#425627,#425634,#425641,#425648,#425655,#425662,\n    #425669,#425676,#425683,#425690,#425697,#425704,#425711,#425718,\n    #425725,#425732,#425739,#425746,#425753,#425760,#425767,#425774,\n    #425781,#425788,#425795,#425802,#425809,#425816,#425823,#425830,\n    #425837,#425844,#425851,#425858,#425865,#425872,#425879,#425886,\n    #425893,#425900,#425907,#425914,#425921,#425928,#425935,#425942,\n    #425949),#90227);").is_ok());
        assert!(
            data_entity_mechanical_design_geometric_presentation_representation(
                "('',(\n    #411695),#180791);"
            )
            .is_ok()
        );
        assert!(
            data_entity_mechanical_design_geometric_presentation_representation(
                "('',(\n    #416476),#180224);"
            )
            .is_ok()
        );
        assert!(
            data_entity_mechanical_design_geometric_presentation_representation(
                "('',(\n    #412907),#155907);"
            )
            .is_ok()
        );
        assert!(data_entity_mechanical_design_geometric_presentation_representation("('',(\n    #403076,#403083,#403090,#403097,#403104,#403111,#403118,#403125,\n    #403132,#403139,#403146,#403153,#403160,#403167,#403174,#403181,\n    #403188,#403195,#403202,#403209,#403216,#403223,#403230,#403237,\n    #403244,#403251,#403258,#403265,#403272,#403279,#403286,#403293,\n    #403300,#403307,#403314,#403321,#403328,#403335,#403342,#403350,\n    #403357,#403364,#403371,#403378,#403385,#403392,#403399,#403406,\n    #403413),#194225);").is_ok());
        assert!(
            data_entity_mechanical_design_geometric_presentation_representation(
                "('',(\n    #426048),#5307);"
            )
            .is_ok()
        );
        assert!(data_entity_mechanical_design_geometric_presentation_representation("('',(\n    #411764,#411771,#411778,#411785,#411792,#411799,#411806,#411813,\n    #411820,#411827,#411834,#411841,#411848,#411855,#411862,#411869,\n    #411876,#411883,#411890,#411897,#411904,#411911,#411918,#411925,\n    #411932,#411939,#411946,#411953,#411960,#411967,#411974,#411981,\n    #411988,#411995,#412002,#412009,#412016,#412023,#412030,#412037,\n    #412044,#412051,#412058,#412065,#412072,#412079,#412086,#412093,\n    #412100),#387279);").is_ok());
        assert!(data_entity_mechanical_design_geometric_presentation_representation("('',(\n    #401611,#401621,#401630,#401639,#401648,#401657,#401666,#401675,\n    #401684,#401693,#401702,#401711,#401720,#401729,#401738,#401747,\n    #401756,#401765,#401774,#401783,#401792,#401801,#401810,#401819,\n    #401828,#401837,#401846,#401855,#401864,#401873,#401882,#401891,\n    #401900,#401909,#401918,#401927,#401936,#401945,#401954,#401963,\n    #401972,#401981,#401990,#401999,#402008,#402017,#402026,#402035,\n    #402044,#402053,#402062,#402071,#402080,#402089,#402098,#402107,\n    #402116,#402125,#402134,#402143,#402152,#402161,#402170,#402179,\n    #402188,#402197,#402206,#402215,#402224,#402233,#402242,#402251,\n    #402260,#402269,#402278,#402287,#402296,#402305,#402314,#402323,\n    #402332,#402341,#402350,#402359,#402368,#402377,#402386,#402395,\n    #402404,#402413,#402422,#402431,#402440,#402449,#402458,#402467,\n    #402476,#402485,#402494,#402503,#402512,#402521,#402530,#402539,\n    #402548,#402557,#402566,#402575,#402584,#402593,#402602,#402611,\n    #402620,#402629,#402638,#402647,#402656,#402665,#402674,#402683,\n    #402692,#402701,#402710,#402719,#402728,#402737,#402746,#402755,\n    #402764,#402773,#402782,#402791,#402800,#402809,#402818,#402827,\n    #402836,#402845,#402854,#402863,#402872,#402881,#402890,#402899,\n    #402908,#402917,#402926,#402935,#402944,#402953,#402962,#402971,\n    #402980,#402989,#402998,#403007,#403016,#403025,#403034),#145146);").is_ok());
        assert!(
            data_entity_mechanical_design_geometric_presentation_representation(
                "('',(\n    #407083),#399327);"
            )
            .is_ok()
        );
        assert!(
            data_entity_mechanical_design_geometric_presentation_representation(
                "('',(\n    #407053),#384850);"
            )
            .is_ok()
        );
    }

    #[test]
    fn test_next_assembly_usage_occurrence() {
        assert!(data_entity_next_assembly_usage_occurrence(
            "('3943','869','',#198907,#153103\n  ,$);"
        )
        .is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("('2918','','',#5,#137426,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("('4381','','',#5,#335401,$);").is_ok());
        assert!(data_entity_next_assembly_usage_occurrence(
            "('4036','962','',#145221,#200195\n  ,$);"
        )
        .is_ok());
        assert!(data_entity_next_assembly_usage_occurrence(
            "('3275','201','',#165997,#150290\n  ,$);"
        )
        .is_ok());
        assert!(data_entity_next_assembly_usage_occurrence(
            "('4369','','',#333108,#332216,$\n  );"
        )
        .is_ok());
        assert!(
            data_entity_next_assembly_usage_occurrence("('2579','','',#24161,#19978,$);").is_ok()
        );
        assert!(data_entity_next_assembly_usage_occurrence(
            "('4518','15','',#387602,#389450,\n  $);"
        )
        .is_ok());
        assert!(data_entity_next_assembly_usage_occurrence(
            "('3890','816','',#145221,#198151\n  ,$);"
        )
        .is_ok());
        assert!(data_entity_next_assembly_usage_occurrence("('4270','','',#5,#323141,$);").is_ok());
    }

    #[test]
    fn test_open_shell() {
        assert!(data_entity_open_shell("('',(#383556));").is_ok());
        assert!(data_entity_open_shell("('',(#385623));").is_ok());
        assert!(data_entity_open_shell("('',(#131066));").is_ok());
        assert!(data_entity_open_shell("('',(#5823));").is_ok());
        assert!(data_entity_open_shell("('',(#129316));").is_ok());
        assert!(data_entity_open_shell("('',(#389234));").is_ok());
    }

    #[test]
    fn test_oriented_closed_shell() {
        assert!(data_entity_oriented_closed_shell("('',*,#90078,.F.);").is_ok());
        assert!(data_entity_oriented_closed_shell("('',*,#89928,.F.);").is_ok());
    }

    #[test]
    fn test_oriented_edge() {
        assert!(data_entity_oriented_edge("('',*,*,#352147,.F.);").is_ok());
        assert!(data_entity_oriented_edge("('',*,*,#372770,.F.);").is_ok());
        assert!(data_entity_oriented_edge("('',*,*,#36809,.T.);").is_ok());
        assert!(data_entity_oriented_edge("('',*,*,#231406,.F.);").is_ok());
        assert!(data_entity_oriented_edge("('',*,*,#2195,.F.);").is_ok());
        assert!(data_entity_oriented_edge("('',*,*,#128924,.F.);").is_ok());
        assert!(data_entity_oriented_edge("('',*,*,#254526,.T.);").is_ok());
        assert!(data_entity_oriented_edge("('',*,*,#282321,.F.);").is_ok());
        assert!(data_entity_oriented_edge("('',*,*,#216634,.T.);").is_ok());
        assert!(data_entity_oriented_edge("('',*,*,#29894,.T.);").is_ok());
    }

    #[test]
    fn test_over_riding_styled_item() {
        assert!(data_entity_over_riding_styled_item(
            "('overriding color',(#420760),#67182,\n  #416537);"
        )
        .is_ok());
        assert!(data_entity_over_riding_styled_item(
            "('overriding color',(#422093),#74868,\n  #420934);"
        )
        .is_ok());
        assert!(data_entity_over_riding_styled_item(
            "('overriding color',(#403008),#145083,\n  #401621);"
        )
        .is_ok());
        assert!(data_entity_over_riding_styled_item(
            "('overriding color',(#426695),#25323,\n  #426631);"
        )
        .is_ok());
        assert!(data_entity_over_riding_styled_item(
            "('overriding color',(#410405),#375266,\n  #410396);"
        )
        .is_ok());
        assert!(data_entity_over_riding_styled_item(
            "('overriding color',(#412003),#386937,\n  #411897);"
        )
        .is_ok());
        assert!(data_entity_over_riding_styled_item(
            "('overriding color',(#408755),#391252,\n  #408648);"
        )
        .is_ok());
        assert!(data_entity_over_riding_styled_item(
            "('overriding color',(#415312),#38000,\n  #414772);"
        )
        .is_ok());
        assert!(data_entity_over_riding_styled_item(
            "('overriding color',(#422100),#74909,\n  #420934);"
        )
        .is_ok());
        assert!(data_entity_over_riding_styled_item(
            "('overriding color',(#425334),#87487,\n  #420934);"
        )
        .is_ok());
    }

    #[test]
    fn test_plane() {
        assert!(data_entity_plane("('',#34393);").is_ok());
        assert!(data_entity_plane("('',#63225);").is_ok());
        assert!(data_entity_plane("('',#15412);").is_ok());
        assert!(data_entity_plane("('',#306146);").is_ok());
        assert!(data_entity_plane("('',#370704);").is_ok());
        assert!(data_entity_plane("('',#106351);").is_ok());
        assert!(data_entity_plane("('',#82156);").is_ok());
        assert!(data_entity_plane("('',#123902);").is_ok());
        assert!(data_entity_plane("('',#393420);").is_ok());
        assert!(data_entity_plane("('',#322073);").is_ok());
    }

    #[test]
    fn test_presentation_layer_assignment() {
        assert!(data_entity_presentation_layer_assignment("('NONE','visible',(#391469,\n    #391500,#390251,#390406,#390809,#391603,#390763,#391097,#390973,\n    #390211,#390623,#391946,#390561,#390468,#391314,#390880,#390716,\n    #390849,#391345,#389572,#391252,#391707,#391800,#391190,#391924,\n    #390911,#391159,#390313,#391522,#390344,#390499,#391862,#391965,\n    #391066,#391831,#391128,#391769,#391676,#391738,#390654,#391035,\n    #391553,#391605,#390807,#390282,#391376,#391407,#391645,#391438,\n    #390685,#392003,#390530,#390942,#390437,#390209,#391283,#391221,\n    #390375,#391004,#391893,#390738,#390592,#393264));").is_ok());
    }

    #[test]
    fn test_presentation_style_assignment() {
        assert!(data_entity_presentation_style_assignment("((#406513,#406518));").is_ok());
        assert!(data_entity_presentation_style_assignment("((#424054));").is_ok());
        assert!(data_entity_presentation_style_assignment("((#407292));").is_ok());
        assert!(data_entity_presentation_style_assignment("((#400473));").is_ok());
        assert!(data_entity_presentation_style_assignment("((#423753));").is_ok());
        assert!(data_entity_presentation_style_assignment("((#402352,#402357));").is_ok());
        assert!(data_entity_presentation_style_assignment("((#414566));").is_ok());
        assert!(data_entity_presentation_style_assignment("((#412011));").is_ok());
        assert!(data_entity_presentation_style_assignment("((#417135));").is_ok());
        assert!(data_entity_presentation_style_assignment("((#400725));").is_ok());
    }

    #[test]
    fn test_presentation_style_by_context() {
        assert!(data_entity_presentation_style_by_context("((#410648,#410653),#399365);").is_ok());
        assert!(data_entity_presentation_style_by_context("((#406713,#406719),#399367);").is_ok());
        assert!(data_entity_presentation_style_by_context("((#426070,#426076),#399363);").is_ok());
    }

    #[test]
    fn test_product() {
        assert!(data_entity_product("('C66','C66','',(#200814));").is_ok());
        assert!(data_entity_product("('BAT1','BAT1','',(#395319));").is_ok());
        assert!(data_entity_product("('R152','R152','',(#153740));").is_ok());
        assert!(data_entity_product("('TP95','TP95','',(#136981));").is_ok());
        assert!(data_entity_product("('R102','R102','',(#322948));").is_ok());
        assert!(data_entity_product("('C176','C176','',(#197734));").is_ok());
        assert!(data_entity_product("('C42','C42','',(#387560));").is_ok());
        assert!(data_entity_product("('Extruded','Extruded','',(#129000));").is_ok());
        assert!(
            data_entity_product("('RESC310X160X65L45N','RESC310X160X65L45N','',(#27114));").is_ok()
        );
        assert!(data_entity_product("('C250','C250','',(#194450));").is_ok());
    }

    #[test]
    fn test_product_category() {}

    #[test]
    fn test_product_context() {
        assert!(data_entity_product_context("('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("('',#2,'mechanical');").is_ok());
        assert!(data_entity_product_context("('',#2,'mechanical');").is_ok());
    }

    #[test]
    fn test_product_definition() {
        assert!(data_entity_product_definition("('design','',#200812,#200815);").is_ok());
        assert!(data_entity_product_definition("('design','',#395317,#395320);").is_ok());
        assert!(data_entity_product_definition("('design','',#153738,#153741);").is_ok());
        assert!(data_entity_product_definition("('design','',#136979,#136982);").is_ok());
        assert!(data_entity_product_definition("('design','',#322946,#322949);").is_ok());
        assert!(data_entity_product_definition("('design','',#197732,#197735);").is_ok());
        assert!(data_entity_product_definition("('design','',#387558,#387561);").is_ok());
        assert!(data_entity_product_definition("('design','',#128998,#129001);").is_ok());
        assert!(data_entity_product_definition("('design','',#27112,#27115);").is_ok());
        assert!(data_entity_product_definition("('design','',#194448,#194451);").is_ok());
    }

    #[test]
    fn test_product_definition_context() {
        assert!(data_entity_product_definition_context("('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("('part definition',#2,'design');").is_ok());
        assert!(data_entity_product_definition_context("('part definition',#2,'design');").is_ok());
    }

    #[test]
    fn test_product_definition_formation() {
        assert!(data_entity_product_definition_formation("('','',#200813);").is_ok());
        assert!(data_entity_product_definition_formation("('','',#395318);").is_ok());
        assert!(data_entity_product_definition_formation("('','',#153739);").is_ok());
        assert!(data_entity_product_definition_formation("('','',#136980);").is_ok());
        assert!(data_entity_product_definition_formation("('','',#322947);").is_ok());
        assert!(data_entity_product_definition_formation("('','',#197733);").is_ok());
        assert!(data_entity_product_definition_formation("('','',#387559);").is_ok());
        assert!(data_entity_product_definition_formation("('','',#128999);").is_ok());
        assert!(data_entity_product_definition_formation("('','',#27113);").is_ok());
        assert!(data_entity_product_definition_formation("('','',#194449);").is_ok());
    }

    #[test]
    fn test_product_definition_formation_with_specified_source() {}

    #[test]
    fn test_product_definition_shape() {
        assert!(data_entity_product_definition_shape(
            "('Placement','Placement of an item',\n  #167360);"
        )
        .is_ok());
        assert!(data_entity_product_definition_shape(
            "('Placement','Placement of an item',\n  #359400);"
        )
        .is_ok());
        assert!(data_entity_product_definition_shape(
            "('Placement','Placement of an item',\n  #27513);"
        )
        .is_ok());
        assert!(data_entity_product_definition_shape(
            "('Placement','Placement of an item',\n  #165797);"
        )
        .is_ok());
        assert!(data_entity_product_definition_shape("('','',#162783);").is_ok());
        assert!(data_entity_product_definition_shape(
            "('Placement','Placement of an item',\n  #153812);"
        )
        .is_ok());
        assert!(data_entity_product_definition_shape("('','',#194419);").is_ok());
        assert!(data_entity_product_definition_shape("('','',#381707);").is_ok());
        assert!(data_entity_product_definition_shape(
            "('Placement','Placement of an item',\n  #200046);"
        )
        .is_ok());
        assert!(data_entity_product_definition_shape(
            "('Placement','Placement of an item',\n  #24089);"
        )
        .is_ok());
    }

    #[test]
    fn test_product_related_product_category() {
        assert!(data_entity_product_related_product_category("('part',$,(#200869));").is_ok());
        assert!(data_entity_product_related_product_category("('part',$,(#396675));").is_ok());
        assert!(data_entity_product_related_product_category("('part',$,(#153795));").is_ok());
        assert!(data_entity_product_related_product_category("('part',$,(#136980));").is_ok());
        assert!(data_entity_product_related_product_category("('part',$,(#322947));").is_ok());
        assert!(data_entity_product_related_product_category("('part',$,(#197789));").is_ok());
        assert!(data_entity_product_related_product_category("('part',$,(#387559));").is_ok());
        assert!(data_entity_product_related_product_category("('part',$,(#127904));").is_ok());
        assert!(data_entity_product_related_product_category("('part',$,(#24994));").is_ok());
        assert!(data_entity_product_related_product_category("('part',$,(#194505));").is_ok());
    }

    #[test]
    fn test_property_definition() {
        assert!(data_entity_property_definition(
            "('geometric_validation_property','centroid'\n  ,#427273);"
        )
        .is_ok());
        assert!(data_entity_property_definition(
            "('material property','material name',\n  #321263);"
        )
        .is_ok());
        assert!(data_entity_property_definition(
            "('shape with specific properties',\n  'properties for subshape',#427251);"
        )
        .is_ok());
        assert!(data_entity_property_definition(
            "('geometric_validation_property','volume',\n  #427251);"
        )
        .is_ok());
        assert!(data_entity_property_definition(
            "('geometric_validation_property','centroid'\n  ,#427251);"
        )
        .is_ok());
        assert!(data_entity_property_definition(
            "('geometric_validation_property',\n  'surface area',#427273);"
        )
        .is_ok());
        assert!(data_entity_property_definition(
            "('material property','material name',\n  #356697);"
        )
        .is_ok());
        assert!(data_entity_property_definition(
            "('geometric_validation_property',\n  'surface area',#427251);"
        )
        .is_ok());
        assert!(data_entity_property_definition(
            "('shape with specific properties',\n  'properties for subshape',#427273);"
        )
        .is_ok());
        assert!(
            data_entity_property_definition("('material property','density',#321263);").is_ok()
        );
    }

    #[test]
    fn test_property_definition_representation() {
        assert!(data_entity_property_definition_representation("(#427290,#427291);").is_ok());
        assert!(data_entity_property_definition_representation("(#399371,#399369);").is_ok());
        assert!(data_entity_property_definition_representation("(#427254,#427255);").is_ok());
        assert!(data_entity_property_definition_representation("(#427268,#427269);").is_ok());
        assert!(data_entity_property_definition_representation("(#427276,#427277);").is_ok());
        assert!(data_entity_property_definition_representation("(#427283,#427284);").is_ok());
        assert!(data_entity_property_definition_representation("(#399378,#399376);").is_ok());
        assert!(data_entity_property_definition_representation("(#427261,#427262);").is_ok());
        assert!(data_entity_property_definition_representation("(#399373,#399375);").is_ok());
        assert!(data_entity_property_definition_representation("(#399380,#399382);").is_ok());
    }

    #[test]
    fn test_representation() {
        assert!(data_entity_representation("('centroid',(#427292),#359378);").is_ok());
        assert!(data_entity_representation("('material name',(#399370),#321256);").is_ok());
        assert!(data_entity_representation("('surface area',(#427256),#321256);").is_ok());
        assert!(data_entity_representation("('centroid',(#427270),#321256);").is_ok());
        assert!(data_entity_representation("('surface area',(#427278),#359378);").is_ok());
        assert!(data_entity_representation("('volume',(#427285),#359378);").is_ok());
        assert!(data_entity_representation("('material name',(#399377),#356690);").is_ok());
        assert!(data_entity_representation("('volume',(#427263),#321256);").is_ok());
        assert!(data_entity_representation("($  ,(),$\n    );").is_ok());
        assert!(data_entity_representation("($  ,(),$\n    );").is_ok());
    }

    #[test]
    fn test_shape_aspect() {
        assert!(data_entity_shape_aspect("('','',#359384,.F.);").is_ok());
        assert!(data_entity_shape_aspect("('','',#321262,.F.);").is_ok());
    }

    #[test]
    fn test_shape_definition_representation() {
        assert!(data_entity_shape_definition_representation("(#27110,#25007);").is_ok());
        assert!(data_entity_shape_definition_representation("(#191750,#191756);").is_ok());
        assert!(data_entity_shape_definition_representation("(#168012,#168018);").is_ok());
        assert!(data_entity_shape_definition_representation("(#388658,#388664);").is_ok());
        assert!(data_entity_shape_definition_representation("(#43736,#43742);").is_ok());
        assert!(data_entity_shape_definition_representation("(#24879,#24885);").is_ok());
        assert!(data_entity_shape_definition_representation("(#323560,#323566);").is_ok());
        assert!(data_entity_shape_definition_representation("(#166892,#166898);").is_ok());
        assert!(data_entity_shape_definition_representation("(#167956,#167962);").is_ok());
        assert!(data_entity_shape_definition_representation("(#139021,#139027);").is_ok());
    }

    #[test]
    fn test_shape_representation() {
        assert!(data_entity_shape_representation("('',(#11,#43361),#43365);").is_ok());
        assert!(data_entity_shape_representation("('',(#11,#137068),#137072);").is_ok());
        assert!(data_entity_shape_representation("('',(#11,#136928),#136932);").is_ok());
        assert!(data_entity_shape_representation("('',(#11,#138076),#138080);").is_ok());
        assert!(data_entity_shape_representation("('',(#11,#322951),#322955);").is_ok());
        assert!(data_entity_shape_representation(
            "('',(#11,#11152,#11574,#11996,#12418),\n  #12960);"
        )
        .is_ok());
        assert!(data_entity_shape_representation("('',(#11,#199165),#199169);").is_ok());
        assert!(data_entity_shape_representation("('',(#11,#323147),#323151);").is_ok());
        assert!(data_entity_shape_representation("('',(#11,#202161),#202165);").is_ok());
        assert!(data_entity_shape_representation("('',(#11,#138776),#138780);").is_ok());
    }

    #[test]
    fn test_shape_representation_relationship() {}

    #[test]
    fn test_shell_based_surface_model() {
        assert!(data_entity_shell_based_surface_model("('',(#383555));").is_ok());
        assert!(data_entity_shell_based_surface_model("('',(#385622));").is_ok());
        assert!(data_entity_shell_based_surface_model("('',(#131065));").is_ok());
        assert!(data_entity_shell_based_surface_model("('',(#5822));").is_ok());
        assert!(data_entity_shell_based_surface_model("('',(#129315));").is_ok());
        assert!(data_entity_shell_based_surface_model("('',(#389233));").is_ok());
    }

    #[test]
    fn test_spherical_surface() {
        assert!(data_entity_spherical_surface("('',#394324,0.125);").is_ok());
        assert!(data_entity_spherical_surface("('',#171404,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("('',#6207,0.1);").is_ok());
        assert!(data_entity_spherical_surface("('',#39988,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("('',#170006,0.25);").is_ok());
        assert!(data_entity_spherical_surface("('',#39360,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("('',#381307,5.E-002);").is_ok());
        assert!(data_entity_spherical_surface("('',#8086,0.1);").is_ok());
        assert!(data_entity_spherical_surface("('',#325461,1.38E-002);").is_ok());
        assert!(data_entity_spherical_surface("('',#6080,0.1);").is_ok());
    }

    #[test]
    fn test_styled_item() {
        assert!(data_entity_styled_item("('color',(#405958),#188083);").is_ok());
        assert!(data_entity_styled_item("('color',(#426004),#5451);").is_ok());
        assert!(data_entity_styled_item("('color',(#405998),#335417);").is_ok());
        assert!(data_entity_styled_item("('color',(#400141),#359420);").is_ok());
        assert!(data_entity_styled_item("('color',(#411716),#11152);").is_ok());
        assert!(data_entity_styled_item("('color',(#426080),#150353);").is_ok());
        assert!(data_entity_styled_item("('color',(#427231),#163676);").is_ok());
        assert!(data_entity_styled_item("('color',(#426390),#190309);").is_ok());
        assert!(data_entity_styled_item("('color',(#403442),#168410);").is_ok());
        assert!(data_entity_styled_item("('color',(#407474),#380455);").is_ok());
    }

    #[test]
    fn test_surface_of_linear_extrusion() {
        assert!(data_entity_surface_of_linear_extrusion("('',#349510,#349517);").is_ok());
        assert!(data_entity_surface_of_linear_extrusion("('',#349488,#349495);").is_ok());
    }

    #[test]
    fn test_surface_side_style() {
        assert!(data_entity_surface_side_style("('',(#412122));").is_ok());
        assert!(data_entity_surface_side_style("('',(#424063));").is_ok());
        assert!(data_entity_surface_side_style("('',(#417991));").is_ok());
        assert!(data_entity_surface_side_style("('',(#400475));").is_ok());
        assert!(data_entity_surface_side_style("('',(#409793));").is_ok());
        assert!(data_entity_surface_side_style("('',(#402354));").is_ok());
        assert!(data_entity_surface_side_style("('',(#421956));").is_ok());
        assert!(data_entity_surface_side_style("('',(#418271));").is_ok());
        assert!(data_entity_surface_side_style("('',(#417137));").is_ok());
        assert!(data_entity_surface_side_style("('',(#400727));").is_ok());
    }

    #[test]
    fn test_surface_style_fill_area() {
        assert!(data_entity_surface_style_fill_area("(#412123);").is_ok());
        assert!(data_entity_surface_style_fill_area("(#424064);").is_ok());
        assert!(data_entity_surface_style_fill_area("(#417992);").is_ok());
        assert!(data_entity_surface_style_fill_area("(#400476);").is_ok());
        assert!(data_entity_surface_style_fill_area("(#409794);").is_ok());
        assert!(data_entity_surface_style_fill_area("(#402355);").is_ok());
        assert!(data_entity_surface_style_fill_area("(#421957);").is_ok());
        assert!(data_entity_surface_style_fill_area("(#418272);").is_ok());
        assert!(data_entity_surface_style_fill_area("(#417138);").is_ok());
        assert!(data_entity_surface_style_fill_area("(#400728);").is_ok());
    }

    #[test]
    fn test_surface_style_usage() {
        assert!(data_entity_surface_style_usage("(.BOTH.,#412121);").is_ok());
        assert!(data_entity_surface_style_usage("(.BOTH.,#424062);").is_ok());
        assert!(data_entity_surface_style_usage("(.BOTH.,#417990);").is_ok());
        assert!(data_entity_surface_style_usage("(.BOTH.,#400474);").is_ok());
        assert!(data_entity_surface_style_usage("(.BOTH.,#409792);").is_ok());
        assert!(data_entity_surface_style_usage("(.BOTH.,#402353);").is_ok());
        assert!(data_entity_surface_style_usage("(.BOTH.,#421955);").is_ok());
        assert!(data_entity_surface_style_usage("(.BOTH.,#418270);").is_ok());
        assert!(data_entity_surface_style_usage("(.BOTH.,#417136);").is_ok());
        assert!(data_entity_surface_style_usage("(.BOTH.,#400726);").is_ok());
    }

    #[test]
    fn test_toroidal_surface() {
        assert!(data_entity_toroidal_surface("('',#394468,0.113,1.2E-002);").is_ok());
        assert!(data_entity_toroidal_surface("('',#393207,0.113,1.2E-002);").is_ok());
        assert!(data_entity_toroidal_surface("('',#335221,4.9,0.1);").is_ok());
        assert!(data_entity_toroidal_surface("('',#335371,4.225,0.1);").is_ok());
        assert!(data_entity_toroidal_surface("('',#394408,0.113,1.2E-002);").is_ok());
        assert!(data_entity_toroidal_surface("('',#334532,1.35,0.15);").is_ok());
        assert!(data_entity_toroidal_surface("('',#333277,4.225,0.1);").is_ok());
        assert!(data_entity_toroidal_surface("('',#394388,0.113,1.2E-002);").is_ok());
        assert!(data_entity_toroidal_surface("('',#334652,1.35,0.15);").is_ok());
        assert!(data_entity_toroidal_surface("('',#333321,1.15,0.1);").is_ok());
    }

    #[test]
    fn test_trimmed_curve() {
        assert!(data_entity_trimmed_curve("('',#371726,(#371731,PARAMETER_VALUE(0.E+000)),(\n    #371732,PARAMETER_VALUE(3.141592653589)),.T.,.PARAMETER.);").is_ok());
        assert!(data_entity_trimmed_curve("('',#371701,(#371706,PARAMETER_VALUE(0.E+000)),(\n    #371707,PARAMETER_VALUE(3.141592653589)),.T.,.PARAMETER.);").is_ok());
        assert!(data_entity_trimmed_curve("('',#371718,(#371723,PARAMETER_VALUE(0.E+000)),(\n    #371724,PARAMETER_VALUE(3.141592653589)),.T.,.PARAMETER.);").is_ok());
        assert!(data_entity_trimmed_curve("('',#371709,(#371714,PARAMETER_VALUE(0.E+000)),(\n    #371715,PARAMETER_VALUE(3.141592653589)),.T.,.PARAMETER.);").is_ok());
    }

    #[test]
    fn test_uncertainty_measure_with_unit() {
        assert!(data_entity_uncertainty_measure_with_unit(
            "(LENGTH_MEASURE(1.E-007),#200822,\n  'distance_accuracy_value','confusion accuracy');"
        )
        .is_ok());
        assert!(data_entity_uncertainty_measure_with_unit(
            "(LENGTH_MEASURE(1.E-007),#395339,\n  'distance_accuracy_value','confusion accuracy');"
        )
        .is_ok());
        assert!(data_entity_uncertainty_measure_with_unit(
            "(LENGTH_MEASURE(5.E-006),#153748,\n  'distance_accuracy_value','confusion accuracy');"
        )
        .is_ok());
        assert!(data_entity_uncertainty_measure_with_unit(
            "(LENGTH_MEASURE(1.E-007),#136989,\n  'distance_accuracy_value','confusion accuracy');"
        )
        .is_ok());
        assert!(data_entity_uncertainty_measure_with_unit(
            "(LENGTH_MEASURE(5.E-006),#322956,\n  'distance_accuracy_value','confusion accuracy');"
        )
        .is_ok());
        assert!(data_entity_uncertainty_measure_with_unit(
            "(LENGTH_MEASURE(1.E-007),#197742,\n  'distance_accuracy_value','confusion accuracy');"
        )
        .is_ok());
        assert!(data_entity_uncertainty_measure_with_unit(
            "(LENGTH_MEASURE(1.E-007),#387568,\n  'distance_accuracy_value','confusion accuracy');"
        )
        .is_ok());
        assert!(data_entity_uncertainty_measure_with_unit(
            "(LENGTH_MEASURE(1.E-007),#128991,\n  'distance_accuracy_value','confusion accuracy');"
        )
        .is_ok());
        assert!(data_entity_uncertainty_measure_with_unit(
            "(LENGTH_MEASURE(2.E-006),#27105,\n  'distance_accuracy_value','confusion accuracy');"
        )
        .is_ok());
        assert!(data_entity_uncertainty_measure_with_unit(
            "(LENGTH_MEASURE(1.E-007),#194458,\n  'distance_accuracy_value','confusion accuracy');"
        )
        .is_ok());
    }

    #[test]
    fn test_value_representation_item() {}

    #[test]
    fn test_vector() {
        assert!(data_entity_vector("('',#322112,1.);").is_ok());
        assert!(data_entity_vector("('',#270072,1.);").is_ok());
        assert!(data_entity_vector("('',#117143,1.);").is_ok());
        assert!(data_entity_vector("('',#248800,1.);").is_ok());
        assert!(data_entity_vector("('',#339198,1.);").is_ok());
        assert!(data_entity_vector("('',#216792,1.);").is_ok());
        assert!(data_entity_vector("('',#221462,1.);").is_ok());
        assert!(data_entity_vector("('',#292282,1.);").is_ok());
        assert!(data_entity_vector("('',#111916,1.);").is_ok());
        assert!(data_entity_vector("('',#203996,1.);").is_ok());
    }

    #[test]
    fn test_vertex_loop() {}

    #[test]
    fn test_vertex_point() {
        assert!(data_entity_vertex_point("('',#13321);").is_ok());
        assert!(data_entity_vertex_point("('',#209043);").is_ok());
        assert!(data_entity_vertex_point("('',#10124);").is_ok());
        assert!(data_entity_vertex_point("('',#375441);").is_ok());
        assert!(data_entity_vertex_point("('',#90916);").is_ok());
        assert!(data_entity_vertex_point("('',#73268);").is_ok());
        assert!(data_entity_vertex_point("('',#363871);").is_ok());
        assert!(data_entity_vertex_point("('',#173364);").is_ok());
        assert!(data_entity_vertex_point("('',#314641);").is_ok());
        assert!(data_entity_vertex_point("('',#74306);").is_ok());
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
        assert!(
            data_line("#255513 = CARTESIAN_POINT('',(7.575,1.251823672866,-3.616043483317));")
                .is_ok()
        );
        assert!(data_line("#381425 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#395859 = LINE('',#395860,#395861);").is_ok());
        assert!(data_line("#80154 = PLANE('',#80155);").is_ok());
        assert!(data_line(
            "#188350 = CARTESIAN_POINT('',(-0.492273030442,0.270494406184,\n    0.392082765346));"
        )
        .is_ok());
        assert!(data_line("#193529 = EDGE_CURVE('',#193530,#193532,#193534,.T.);").is_ok());
        assert!(data_line("#149798 = DIRECTION('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_line("#152676 = CARTESIAN_POINT('',(0.3,-0.15,0.28));").is_ok());
        assert!(data_line("#134405 = ADVANCED_FACE('',(#134406),#134431,.F.);").is_ok());
        assert!(data_line("#331873 = AXIS2_PLACEMENT_3D('',#331874,#331875,#331876);").is_ok());
        assert!(data_line("#90171 = LINE('',#90172,#90173);").is_ok());
        assert!(
            data_line("#333094 = CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#333095,#333097);").is_ok()
        );
        assert!(data_line("#235265 = CARTESIAN_POINT('',(10.575,-2.447927405784,-7.05));").is_ok());
        assert!(data_line(
            "#68815 = CARTESIAN_POINT('',(11.2999999994,0.7899999999,-7.058485782561)\n  );"
        )
        .is_ok());
        assert!(data_line("#191365 = AXIS2_PLACEMENT_3D('',#191366,#191367,#191368);").is_ok());
        assert!(data_line("#295899 = ORIENTED_EDGE('',*,*,#295797,.T.);").is_ok());
        assert!(data_line("#353585 = VECTOR('',#353586,1.);").is_ok());
        assert!(data_line("#390574 = VERTEX_POINT('',#390575);").is_ok());
        assert!(data_line("#308141 = LINE('',#308142,#308143);").is_ok());
        assert!(data_line("#13026 = EDGE_CURVE('',#13027,#13019,#13029,.T.);").is_ok());
        assert!(data_line("#394503 = CARTESIAN_POINT('',(-98.82047619806,10.644119757988,\n    42.342860302395));").is_ok());
        assert!(data_line("#13726 = ADVANCED_FACE('',(#13727),#13761,.F.);").is_ok());
        assert!(data_line(
            "#199699 = DIRECTION('',(-3.777050848347E-023,8.742273394091E-008,-1.));"
        )
        .is_ok());
        assert!(data_line("#83164 = DIRECTION('',(-0.E+000,-1.,-0.E+000));").is_ok());
        assert!(data_line("#90077 = ORIENTED_CLOSED_SHELL('',*,#90078,.F.);").is_ok());
        assert!(data_line("#406728 = FILL_AREA_STYLE_COLOUR('',#406729);").is_ok());
        assert!(data_line("#368718 = AXIS2_PLACEMENT_3D('',#368719,#368720,#368721);").is_ok());
        assert!(data_line(
            "#263881 = CARTESIAN_POINT('',(-4.875,1.224848688337,-3.335585364953));"
        )
        .is_ok());
        assert!(
            data_line("#3259 = CARTESIAN_POINT('',(9.24999928,103.50000636,0.E+000));").is_ok()
        );
        assert!(data_line("#268049 = VECTOR('',#268050,1.);").is_ok());
        assert!(data_line("#247563 = ORIENTED_EDGE('',*,*,#247548,.T.);").is_ok());
        assert!(
            data_line("#165973 = PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');")
                .is_ok()
        );
        assert!(data_line("#382593 = ADVANCED_FACE('',(#382594),#382612,.T.);").is_ok());
        assert!(data_line("#285325 = LINE('',#285326,#285327);").is_ok());
        assert!(data_line("#256411 = FACE_BOUND('',#256412,.F.);").is_ok());
        assert!(data_line(
            "#90097 = CARTESIAN_POINT('',(16.2499999994,14.99898095209,-9.4499999996)\n  );"
        )
        .is_ok());
        assert!(data_line("#16086 = ORIENTED_EDGE('',*,*,#16087,.T.);").is_ok());
        assert!(data_line("#216011 = VERTEX_POINT('',#216012);").is_ok());
        assert!(data_line("#257199 = ORIENTED_EDGE('',*,*,#257016,.T.);").is_ok());
        assert!(data_line("#355647 = EDGE_CURVE('',#355648,#355552,#355650,.T.);").is_ok());
        assert!(data_line("#332487 = PLANE('',#332488);").is_ok());
        assert!(
            data_line("#340491 = DIRECTION('',(0.707106781187,0.707106781187,-0.E+000));").is_ok()
        );
        assert!(data_line("#4043 = ORIENTED_EDGE('',*,*,#4044,.F.);").is_ok());
        assert!(data_line("#402556 = DRAUGHTING_PRE_DEFINED_CURVE_FONT('continuous');").is_ok());
        assert!(data_line("#301777 = LINE('',#301778,#301779);").is_ok());
        assert!(data_line("#254801 = DIRECTION('',(0.E+000,0.E+000,-1.));").is_ok());
        assert!(data_line("#170114 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#170111,\n  'distance_accuracy_value','confusion accuracy');").is_ok());
        assert!(
            data_line("#192959 = PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');")
                .is_ok()
        );
        assert!(data_line("#99033 = DIRECTION('',(0.E+000,0.E+000,1.));").is_ok());
        assert!(data_line("#359078 = LINE('',#359079,#359080);").is_ok());
        assert!(data_line("#65500 = DIRECTION('',(-0.E+000,-1.,-0.E+000));").is_ok());
        assert!(data_line("#7100 = ADVANCED_FACE('',(#7101),#7128,.T.);").is_ok());
    }
}
