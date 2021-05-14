#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, Ord)]
pub struct Id(pub usize);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LengthMeasure(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ParameterValue(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CountMeasure(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VolumeMeasure(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PositiveLengthMeasure(pub f64);

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
    SurfOfLinearExtrusion,
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
    VertexLoop(&'a str, Id),
    VertexPoint(&'a str, Id),
}

impl DataEntity<'_> {
    pub fn upstream(&self) -> Vec<Id> {
        use DataEntity::*;
        match self {
            Null | ComplexBucketType => vec![],
            AdvancedBrepShapeRepresentation(_, _, x2) => vec![x2.clone()],
            AdvancedFace(_, _, x2, _) => vec![x2.clone()],
            ApplicationContext(_) => vec![],
            ApplicationProtocolDefinition(_, _, _, x3) => vec![x3.clone()],
            Axis2Placement3d(_, x1, x2, x3) => vec![x1.clone(), x2.clone(), x3.clone()],
            BSplineCurveWithKnots(_, _, _, _, _, _, _, _, _) => vec![],
            BSplineSurfaceWithKnots(_, _, _, _, _, _, _, _, _, _, _, _, _) => vec![],
            BrepWithVoids(_, x1, _) => vec![x1.clone()],
            CartesianPoint(_, _) => vec![],
            Circle(_, x1, _) => vec![x1.clone()],
            ClosedShell(_, _) => vec![],
            ColourRgb(_, _, _, _) => vec![],
            ConicalSurface(_, x1, _, _) => vec![x1.clone()],
            ContextDependentShapeRepresentation(x0, x1) => vec![x0.clone(), x1.clone()],
            CurveStyle(_, x1, _, x3) => vec![x1.clone(), x3.clone()],
            CylindricalSurface(_, x1, _) => vec![x1.clone()],
            DerivedUnit(_) => vec![],
            DerivedUnitElement(x0, _) => vec![x0.clone()],
            DescriptiveRepresentationItem(_, _) => vec![],
            Direction(_, _) => vec![],
            DraughtingPreDefinedColour(_) => vec![],
            DraughtingPreDefinedCurveFont(_) => vec![],
            EdgeCurve(_, x1, x2, x3, _) => vec![x1.clone(), x2.clone(), x3.clone()],
            EdgeLoop(_, _) => vec![],
            Ellipse(_, x1, _, _) => vec![x1.clone()],
            FaceBound(_, x1, _) => vec![x1.clone()],
            FillAreaStyle(_, _) => vec![],
            FillAreaStyleColour(_, x1) => vec![x1.clone()],
            GeometricCurveSet(_, _) => vec![],
            ItemDefinedTransformation(_, _, x2, x3) => vec![x2.clone(), x3.clone()],
            Line(_, x1, x2) => vec![x1.clone(), x2.clone()],
            ManifoldSolidBrep(_, x1) => vec![x1.clone()],
            ManifoldSurfaceShapeRepresentation(_, _, x2) => vec![x2.clone()],
            MeasureRepresentationItem(_, _, x2) => vec![x2.clone()],
            MechanicalDesignGeometricPresentationRepresentation(_, _, x2) => vec![x2.clone()],
            NextAssemblyUsageOccurrence(_, _, _, x3, x4, _) => vec![x3.clone(), x4.clone()],
            OpenShell(_, _) => vec![],
            OrientedClosedShell(_, x2, _) => vec![x2.clone()],
            OrientedEdge(_, x3, _) => vec![x3.clone()],
            OverRidingStyledItem(_, _, x2, x3) => vec![x2.clone(), x3.clone()],
            Plane(_, x1) => vec![x1.clone()],
            PresentationLayerAssignment(_, _, _) => vec![],
            PresentationStyleAssignment(_) => vec![],
            PresentationStyleByContext(_, x1) => vec![x1.clone()],
            Product(_, _, _, _) => vec![],
            ProductCategory(_, _) => vec![],
            ProductContext(_, x1, _) => vec![x1.clone()],
            ProductDefinition(_, _, x2, x3) => vec![x2.clone(), x3.clone()],
            ProductDefinitionContext(_, x1, _) => vec![x1.clone()],
            ProductDefinitionFormation(_, _, x2) => vec![x2.clone()],
            ProductDefinitionFormationWithSpecifiedSource(_, _, x2, _) => vec![x2.clone()],
            ProductDefinitionShape(_, _, x2) => vec![x2.clone()],
            ProductRelatedProductCategory(_, _, _) => vec![],
            PropertyDefinition(_, _, x2) => vec![x2.clone()],
            PropertyDefinitionRepresentation(x0, x1) => vec![x0.clone(), x1.clone()],
            Representation(_, _, _) => vec![],
            ShapeAspect(_, _, x2, _) => vec![x2.clone()],
            ShapeDefinitionRepresentation(x0, x1) => vec![x0.clone(), x1.clone()],
            ShapeRepresentation(_, _, x2) => vec![x2.clone()],
            ShapeRepresentationRelationship(_, _, x2, x3) => vec![x2.clone(), x3.clone()],
            ShellBasedSurfaceModel(_, _) => vec![],
            SphericalSurface(_, x1, _) => vec![x1.clone()],
            StyledItem(_, _, x2) => vec![x2.clone()],
            SurfaceOfLinearExtrusion(_, x1, x2) => vec![x1.clone(), x2.clone()],
            SurfaceSideStyle(_, _) => vec![],
            SurfaceStyleFillArea(x0) => vec![x0.clone()],
            SurfaceStyleUsage(_, x1) => vec![x1.clone()],
            ToroidalSurface(_, x1, _, _) => vec![x1.clone()],
            TrimmedCurve(_, x1, _, _, _, _) => vec![x1.clone()],
            UncertaintyMeasureWithUnit(_, x1, _, _) => vec![x1.clone()],
            ValueRepresentationItem(_, _) => vec![],
            Vector(_, x1, _) => vec![x1.clone()],
            VertexLoop(_, x1) => vec![x1.clone()],
            VertexPoint(_, x1) => vec![x1.clone()],
        }
    }
}
