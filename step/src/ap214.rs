use crate::ap214_autogen::{DataEntity, Id};

////////////////////////////////////////////////////////////////////////////////

impl DataEntity {
    pub fn upstream(&self) -> Vec<Id> {
        use DataEntity::*;
        match self {
            AdvancedBrepShapeRepresentation(_s, v, a) => {
                let mut v = v.clone(); v.push(*a); v
            },
            AdvancedFace(_s, v, a, _b) => {
                let mut v = v.clone(); v.push(*a); v
            },
            ApplicationContext(_) => vec![],
            ApplicationProtocolDefinition(_, _, _, i) => vec![*i],
            Axis2Placement3d(_, a, b, c) => vec![*a, *b, *c],
            CartesianPoint(_, _) => vec![],
            Circle(_, a, _) => vec![*a],
            ClosedShell(_, v) => v.clone(),
            ColourRgb(_, _, _, _) => vec![],
            CylindricalSurface(_, a, _) => vec![*a],
            Direction(_, _) => vec![],
            EdgeCurve(_, a, b, c, _) => vec![*a, *b, *c],
            EdgeLoop(_, v) => v.clone(),
            FaceBound(_, a, _) => vec![*a],
            FillAreaStyle(_, v) => v.clone(),
            FillAreaStyleColour(_, a) => vec![*a],
            Line(_, a, b) => vec![*a, *b],
            ManifoldSolidBrep(_, a) => vec![*a],
            MechanicalDesignGeometricPresentationRepresentation(_, v, a) => {
                let mut v = v.clone(); v.push(*a); v
            },
            OrientedEdge(_, a, _) => vec![*a],
            Plane(_, a) => vec![*a],
            PresentationStyleAssignment(v) => v.clone(),
            Product(_, _, _, v) => v.clone(),
            ProductCategory(_, _) => vec![],
            ProductContext(_, a, _) => vec![*a],
            ProductDefinition(_, _, a, b) => vec![*a, *b],
            ProductDefinitionContext(_, a, _) => vec![*a],
            ProductDefinitionFormationWithSpecifiedSource(_, _, a, _) => vec![*a],
            ProductDefinitionShape(_, _, a) => vec![*a],
            ProductRelatedProductCategory(_, _, v) => v.clone(),
            PropertyDefinition(_, _, a) => vec![*a],
            PropertyDefinitionRepresentation(a, b) => vec![*a, *b],
            Representation(_, v, a) => {
                let mut v = v.clone();
                if let Some(a) = a {
                    v.push(*a);
                }
                v
            },
            ShapeDefinitionRepresentation(a, b) => vec![*a, *b],
            ShapeRepresentation(_, v, a) => {
                let mut v = v.clone();
                v.push(*a);
                v
            },
            ShapeRepresentationRelationship(_, _, a, b) => vec![*a, *b],
            StyledItem(_, v, a) => {
                let mut v = v.clone();
                v.push(*a);
                v
            },
            SurfaceSideStyle(_, v) => v.clone(),
            SurfaceStyleFillArea(a) => vec![*a],
            SurfaceStyleUsage(_, a) => vec![*a],
            UncertaintyMeasureWithUnit(_, a, _, _) => vec![*a],
            ValueRepresentationItem(_, _) => vec![],
            Vector(_, a, _) => vec![*a],
            VertexPoint(_, a) => vec![*a],

            _ => vec![], // TODO
        }
    }
}

pub struct StepFile(pub Vec<DataEntity>);

impl StepFile {
    pub fn to_dot(&self) -> String {
        let mut out = "digraph {\n".to_owned();
        for (i, e) in self.0.iter().enumerate() {
            let d = format!("{:?}", e);
            let name = d.split("(").next().unwrap();

            out += &format!("  e{} [ label = \"#{}: {}\" ];\n", i, i, name);
            for j in e.upstream() {
                out += &format!("  e{} -> e{};\n", i, j.0);
            }
        }
        out += "}";
        out
    }
    pub fn save_dot(&self, filename: &str) -> std::io::Result<()> {
        std::fs::write(filename, self.to_dot())
    }
}
