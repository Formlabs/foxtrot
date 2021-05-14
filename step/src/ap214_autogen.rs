#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, Ord)]
pub struct Id(pub usize);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PositiveLengthMeasure(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LengthMeasure(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ParameterValue(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VolumeMeasure(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CountMeasure(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AreaMeasure(pub f64);

#[derive(Debug, Copy, Clone)]
pub enum AreaMeasureOrVolumeMeasure {
    AreaMeasure(AreaMeasure),
    VolumeMeasure(VolumeMeasure),
}

#[derive(Debug, Copy, Clone)]
pub enum SurfaceSide {
    Positive,
    Negative,
    Both,
}

#[derive(Debug, Copy, Clone)]
pub enum Source {
    Made,
    Bought,
    NotKnown,
}

#[derive(Debug, Copy, Clone)]
pub enum BSplineEnum1 {
    Unspecified,
    WeDontSupportOneElmentEnumsYet,
}

#[derive(Debug, Copy, Clone)]
pub enum BSplineEnum2 {
    PiecewiseBezierKnots,
    Unspecified,
    QuasiUniformKnots,
}

#[derive(Debug, Copy, Clone)]
pub enum TrimmedCurveEnum {
    Parameter,
    WeDontSupportOneElmentEnumsYet,
}

#[derive(Debug)]
pub enum DataEntity {
    Null,
    ComplexBucketType,
    AdvancedBrepShapeRepresentation(String, Vec<Id>, Id),
    AdvancedFace(String, Vec<Id>, Id, bool),
    ApplicationContext(String),
    ApplicationProtocolDefinition(String, String, usize, Id),
    Axis2Placement3d(String, Id, Id, Id),
    BSplineCurveWithKnots(
        String,
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
        String,
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
    BrepWithVoids(String, Id, Vec<Id>),
    CartesianPoint(String, Vec<f64>),
    Circle(String, Id, f64),
    ClosedShell(String, Vec<Id>),
    ColourRgb(String, f64, f64, f64),
    ConicalSurface(String, Id, f64, f64),
    ContextDependentShapeRepresentation(Id, Id),
    CurveStyle(String, Id, PositiveLengthMeasure, Id),
    CylindricalSurface(String, Id, f64),
    DerivedUnit(Vec<Id>),
    DerivedUnitElement(Id, f64),
    DescriptiveRepresentationItem(String, String),
    Direction(String, Vec<f64>),
    DraughtingPreDefinedColour(String),
    DraughtingPreDefinedCurveFont(String),
    EdgeCurve(String, Id, Id, Id, bool),
    EdgeLoop(String, Vec<Id>),
    Ellipse(String, Id, f64, f64),
    FaceBound(String, Id, bool),
    FillAreaStyle(String, Vec<Id>),
    FillAreaStyleColour(String, Id),
    GeometricCurveSet(String, Vec<Id>),
    ItemDefinedTransformation(String, String, Id, Id),
    Line(String, Id, Id),
    ManifoldSolidBrep(String, Id),
    ManifoldSurfaceShapeRepresentation(String, Vec<Id>, Id),
    MeasureRepresentationItem(String, AreaMeasureOrVolumeMeasure, Id),
    MechanicalDesignGeometricPresentationRepresentation(String, Vec<Id>, Id),
    NextAssemblyUsageOccurrence(String, String, String, Id, Id, Option<String>),
    OpenShell(String, Vec<Id>),
    OrientedClosedShell(String, Id, bool),
    OrientedEdge(String, Id, bool),
    OverRidingStyledItem(String, Vec<Id>, Id, Id),
    Plane(String, Id),
    PresentationLayerAssignment(String, String, Vec<Id>),
    PresentationStyleAssignment(Vec<Id>),
    PresentationStyleByContext(Vec<Id>, Id),
    Product(String, String, String, Vec<Id>),
    ProductCategory(String, String),
    ProductContext(String, Id, String),
    ProductDefinition(String, String, Id, Id),
    ProductDefinitionContext(String, Id, String),
    ProductDefinitionFormation(String, String, Id),
    ProductDefinitionFormationWithSpecifiedSource(String, String, Id, Source),
    ProductDefinitionShape(String, String, Id),
    ProductRelatedProductCategory(String, Option<String>, Vec<Id>),
    PropertyDefinition(String, String, Id),
    PropertyDefinitionRepresentation(Id, Id),
    Representation(Option<String>, Vec<Id>, Option<Id>),
    ShapeAspect(String, String, Id, bool),
    ShapeDefinitionRepresentation(Id, Id),
    ShapeRepresentation(String, Vec<Id>, Id),
    ShapeRepresentationRelationship(String, String, Id, Id),
    ShellBasedSurfaceModel(String, Vec<Id>),
    SphericalSurface(String, Id, f64),
    StyledItem(String, Vec<Id>, Id),
    SurfaceOfLinearExtrusion(String, Id, Id),
    SurfaceSideStyle(String, Vec<Id>),
    SurfaceStyleFillArea(Id),
    SurfaceStyleUsage(SurfaceSide, Id),
    ToroidalSurface(String, Id, f64, f64),
    TrimmedCurve(
        String,
        Id,
        (Id, ParameterValue),
        (Id, ParameterValue),
        bool,
        TrimmedCurveEnum,
    ),
    UncertaintyMeasureWithUnit(LengthMeasure, Id, String, String),
    ValueRepresentationItem(String, CountMeasure),
    Vector(String, Id, f64),
    VertexPoint(String, Id),
}
