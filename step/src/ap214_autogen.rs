#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Ord)]
pub struct Id(pub usize);

#[derive(Debug, PartialEq)]
pub struct ParameterValue(pub f64);

#[derive(Debug, PartialEq)]
pub struct CountMeasure(pub f64);

#[derive(Debug, PartialEq)]
pub struct AreaMeasure(pub f64);

#[derive(Debug, PartialEq)]
pub struct LengthMeasure(pub f64);

#[derive(Debug, PartialEq)]
pub struct PositiveLengthMeasure(pub f64);

#[derive(Debug, PartialEq)]
pub struct VolumeMeasure(pub f64);

pub enum AreaMeasureOrVolumeMeasure {
    AreaMeasure(AreaMeasure),
    VolumeMeasure(VolumeMeasure),
}

pub enum SurfaceSide {
    Positive,
    Negative,
    Both,
}

pub enum Source {
    Made,
    Bought,
    NotKnown,
}

pub enum BSplineEnum1 {
    Unspecified,
    WeDontSupportOneElmentEnumsYet,
}

pub enum BSplineEnum2 {
    PiecewiseBezierKnots,
    Unspecified,
    QuasiUniformKnots,
}

pub enum TrimmedCurveEnum {
    Parameter,
    WeDontSupportOneElmentEnumsYet,
}

pub enum DataEntity<'a> {
    Null,
    ComplexBucketType,
    AdvancedBrepShapeRepresentation(&'a str, Vec<Id>, Id),
    AdvancedFace(&'a str, Vec<Id>, Id, bool),
    ApplicationContext(&'a str),
    ApplicationProtocolDefinition(&'a str, &'a str, usize, Id),
    Axis2Placement3d(&'a str, Id, Id, Id),
    BSplineCurveWithKnots(
        &'a str,
        usize,
        Vec<Id>,
        BSplineEnum1,
        bool,
        bool,
        Vec<usize>,
        Vec<f64>,
        BSplineEnum2,
    ),
    BSplineSurfaceWithKnots(
        &'a str,
        usize,
        usize,
        Vec<Vec<Id>>,
        BSplineEnum1,
        bool,
        bool,
        bool,
        Vec<usize>,
        Vec<usize>,
        Vec<f64>,
        Vec<f64>,
        BSplineEnum2,
    ),
    BrepWithVoids(&'a str, Id, Vec<Id>),
    CartesianPoint(&'a str, Vec<f64>),
    Circle(&'a str, Id, f64),
    ClosedShell(&'a str, Vec<Id>),
    ColourRgb(&'a str, f64, f64, f64),
    ConicalSurface(&'a str, Id, f64, f64),
    ContextDependentShapeRepresentation(Id, Id),
    CurveStyle(&'a str, Id, PositiveLengthMeasure, Id),
    CylindricalSurface(&'a str, Id, f64),
    DerivedUnit(Vec<Id>),
    DerivedUnitElement(Id, f64),
    DescriptiveRepresentationItem(&'a str, &'a str),
    Direction(&'a str, Vec<f64>),
    DraughtingPreDefinedColour(&'a str),
    DraughtingPreDefinedCurveFont(&'a str),
    EdgeCurve(&'a str, Id, Id, Id, bool),
    EdgeLoop(&'a str, Vec<Id>),
    Ellipse(&'a str, Id, f64, f64),
    FaceBound(&'a str, Id, bool),
    FillAreaStyle(&'a str, Vec<Id>),
    FillAreaStyleColour(&'a str, Id),
    GeometricCurveSet(&'a str, Vec<Id>),
    ItemDefinedTransformation(&'a str, &'a str, Id, Id),
    Line(&'a str, Id, Id),
    ManifoldSolidBrep(&'a str, Id),
    ManifoldSurfaceShapeRepresentation(&'a str, Vec<Id>, Id),
    MeasureRepresentationItem(&'a str, AreaMeasureOrVolumeMeasure, Id),
    MechanicalDesignGeometricPresentationRepresentation(&'a str, Vec<Id>, Id),
    NextAssemblyUsageOccurrence(&'a str, &'a str, &'a str, Id, Id, Option<&'a str>),
    OpenShell(&'a str, Vec<Id>),
    OrientedClosedShell(&'a str, Id, bool),
    OrientedEdge(&'a str, Id, bool),
    OverRidingStyledItem(&'a str, Vec<Id>, Id, Id),
    Plane(&'a str, Id),
    PresentationLayerAssignment(&'a str, &'a str, Vec<Id>),
    PresentationStyleAssignment(Vec<Id>),
    PresentationStyleByContext(Vec<Id>, Id),
    Product(&'a str, &'a str, &'a str, Vec<Id>),
    ProductCategory(&'a str, &'a str),
    ProductContext(&'a str, Id, &'a str),
    ProductDefinition(&'a str, &'a str, Id, Id),
    ProductDefinitionContext(&'a str, Id, &'a str),
    ProductDefinitionFormation(&'a str, &'a str, Id),
    ProductDefinitionFormationWithSpecifiedSource(&'a str, &'a str, Id, Source),
    ProductDefinitionShape(&'a str, &'a str, Id),
    ProductRelatedProductCategory(&'a str, Option<&'a str>, Vec<Id>),
    PropertyDefinition(&'a str, &'a str, Id),
    PropertyDefinitionRepresentation(Id, Id),
    Representation(Option<&'a str>, Vec<Id>, Option<Id>),
    ShapeAspect(&'a str, &'a str, Id, bool),
    ShapeDefinitionRepresentation(Id, Id),
    ShapeRepresentation(&'a str, Vec<Id>, Id),
    ShapeRepresentationRelationship(&'a str, &'a str, Id, Id),
    ShellBasedSurfaceModel(&'a str, Vec<Id>),
    SphericalSurface(&'a str, Id, f64),
    StyledItem(&'a str, Vec<Id>, Id),
    SurfaceOfLinearExtrusion(&'a str, Id, Id),
    SurfaceSideStyle(&'a str, Vec<Id>),
    SurfaceStyleFillArea(Id),
    SurfaceStyleUsage(SurfaceSide, Id),
    ToroidalSurface(&'a str, Id, f64, f64),
    TrimmedCurve(
        &'a str,
        Id,
        (Id, ParameterValue),
        (Id, ParameterValue),
        bool,
        TrimmedCurveEnum,
    ),
    UncertaintyMeasureWithUnit(LengthMeasure, Id, &'a str, &'a str),
    ValueRepresentationItem(&'a str, CountMeasure),
    Vector(&'a str, Id, f64),
    VertexPoint(&'a str, Id),
}
