#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, Ord)]
pub struct Id(pub usize);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AreaMeasure(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CountMeasure(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LengthMeasure(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ParameterValue(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PositiveLengthMeasure(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VolumeMeasure(pub f64);

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
    RepresentationRelationshipWithTransformation(&'a str, &'a str, Id, Id, Id),
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
            AdvancedBrepShapeRepresentation(_, x1, x2) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r.push(x2.clone());
                r
            }
            AdvancedFace(_, x1, x2, _) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r.push(x2.clone());
                r
            }
            ApplicationContext(_) => {
                vec![]
            }
            ApplicationProtocolDefinition(_, _, _, x3) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x3.clone());
                r
            }
            Axis2Placement3d(_, x1, x2, x3) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r.push(x2.clone());
                r.push(x3.clone());
                r
            }
            BSplineCurveWithKnots(_, _, x2, _, _, _, _, _, _) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x2 {
                    r.push(n0.clone());
                }
                r
            }
            BSplineSurfaceWithKnots(_, _, _, x3, _, _, _, _, _, _, _, _, _) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x3 {
                    for n1 in n0 {
                        r.push(n1.clone());
                    }
                }
                r
            }
            BrepWithVoids(_, x1, x2) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                for n0 in x2 {
                    r.push(n0.clone());
                }
                r
            }
            CartesianPoint(_, _) => {
                vec![]
            }
            Circle(_, x1, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            ClosedShell(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r
            }
            ColourRgb(_, _, _, _) => {
                vec![]
            }
            ConicalSurface(_, x1, _, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            ContextDependentShapeRepresentation(x0, x1) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x0.clone());
                r.push(x1.clone());
                r
            }
            CurveStyle(_, x1, _, x3) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r.push(x3.clone());
                r
            }
            CylindricalSurface(_, x1, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            DerivedUnit(x0) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x0 {
                    r.push(n0.clone());
                }
                r
            }
            DerivedUnitElement(x0, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x0.clone());
                r
            }
            DescriptiveRepresentationItem(_, _) => {
                vec![]
            }
            Direction(_, _) => {
                vec![]
            }
            DraughtingPreDefinedColour(_) => {
                vec![]
            }
            DraughtingPreDefinedCurveFont(_) => {
                vec![]
            }
            EdgeCurve(_, x1, x2, x3, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r.push(x2.clone());
                r.push(x3.clone());
                r
            }
            EdgeLoop(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r
            }
            Ellipse(_, x1, _, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            FaceBound(_, x1, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            FillAreaStyle(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r
            }
            FillAreaStyleColour(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            GeometricCurveSet(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r
            }
            ItemDefinedTransformation(_, _, x2, x3) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x2.clone());
                r.push(x3.clone());
                r
            }
            Line(_, x1, x2) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r.push(x2.clone());
                r
            }
            ManifoldSolidBrep(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            ManifoldSurfaceShapeRepresentation(_, x1, x2) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r.push(x2.clone());
                r
            }
            MeasureRepresentationItem(_, _, x2) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x2.clone());
                r
            }
            MechanicalDesignGeometricPresentationRepresentation(_, x1, x2) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r.push(x2.clone());
                r
            }
            NextAssemblyUsageOccurrence(_, _, _, x3, x4, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x3.clone());
                r.push(x4.clone());
                r
            }
            OpenShell(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r
            }
            OrientedClosedShell(_, x2, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x2.clone());
                r
            }
            OrientedEdge(_, x3, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x3.clone());
                r
            }
            OverRidingStyledItem(_, x1, x2, x3) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r.push(x2.clone());
                r.push(x3.clone());
                r
            }
            Plane(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            PresentationLayerAssignment(_, _, x2) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x2 {
                    r.push(n0.clone());
                }
                r
            }
            PresentationStyleAssignment(x0) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x0 {
                    r.push(n0.clone());
                }
                r
            }
            PresentationStyleByContext(x0, x1) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x0 {
                    r.push(n0.clone());
                }
                r.push(x1.clone());
                r
            }
            Product(_, _, _, x3) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x3 {
                    r.push(n0.clone());
                }
                r
            }
            ProductCategory(_, _) => {
                vec![]
            }
            ProductContext(_, x1, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            ProductDefinition(_, _, x2, x3) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x2.clone());
                r.push(x3.clone());
                r
            }
            ProductDefinitionContext(_, x1, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            ProductDefinitionFormation(_, _, x2) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x2.clone());
                r
            }
            ProductDefinitionFormationWithSpecifiedSource(_, _, x2, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x2.clone());
                r
            }
            ProductDefinitionShape(_, _, x2) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x2.clone());
                r
            }
            ProductRelatedProductCategory(_, _, x2) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x2 {
                    r.push(n0.clone());
                }
                r
            }
            PropertyDefinition(_, _, x2) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x2.clone());
                r
            }
            PropertyDefinitionRepresentation(x0, x1) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x0.clone());
                r.push(x1.clone());
                r
            }
            Representation(_, x1, x2) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                if let Some(i) = x2 {
                    r.push(i.clone());
                };
                r
            }
            RepresentationRelationshipWithTransformation(_, _, x2, x3, x4) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x2.clone());
                r.push(x3.clone());
                r.push(x4.clone());
                r
            }
            ShapeAspect(_, _, x2, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x2.clone());
                r
            }
            ShapeDefinitionRepresentation(x0, x1) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x0.clone());
                r.push(x1.clone());
                r
            }
            ShapeRepresentation(_, x1, x2) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r.push(x2.clone());
                r
            }
            ShapeRepresentationRelationship(_, _, x2, x3) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x2.clone());
                r.push(x3.clone());
                r
            }
            ShellBasedSurfaceModel(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r
            }
            SphericalSurface(_, x1, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            StyledItem(_, x1, x2) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r.push(x2.clone());
                r
            }
            SurfaceOfLinearExtrusion(_, x1, x2) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r.push(x2.clone());
                r
            }
            SurfaceSideStyle(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                for n0 in x1 {
                    r.push(n0.clone());
                }
                r
            }
            SurfaceStyleFillArea(x0) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x0.clone());
                r
            }
            SurfaceStyleUsage(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            ToroidalSurface(_, x1, _, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            TrimmedCurve(_, x1, x2, x3, _, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r.push(x2.0.clone());
                r.push(x3.0.clone());
                r
            }
            UncertaintyMeasureWithUnit(_, x1, _, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            ValueRepresentationItem(_, _) => {
                vec![]
            }
            Vector(_, x1, _) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            VertexLoop(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
            VertexPoint(_, x1) => {
                let mut r: Vec<Id> = Vec::new();
                r.push(x1.clone());
                r
            }
        }
    }
}
