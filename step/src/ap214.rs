use std::marker::PhantomData;

pub struct Id<T>(usize, PhantomData<*const T>);
impl<T> Id<T> {
    pub const fn new(u: usize) -> Self {
        Self(u, PhantomData)
    }
}
pub struct Set<T, const L: usize, const U: usize>(Vec<T>);
pub struct List<T, const L: usize, const U: usize>(Vec<T>);
pub struct UniqueList<T, const L: usize, const U: usize>(Vec<T>);

////////////////////////////////////////////////////////////////////////////////
// Types (primitive)
pub struct DescriptiveMeasure(String);
pub struct Identifier(String);
pub struct Label(String);
pub struct LengthMeasure(f64);
pub struct Text(String);
pub struct NonNegativeLengthMeasure(LengthMeasure);
pub struct PlaneAngleMeasure(f64);
pub struct PositiveLengthMeasure(NonNegativeLengthMeasure);
pub struct PositiveRatioMeasure(RatioMeasure);
pub struct RatioMeasure(f64);

pub enum Axis2Placement {
    Axis2Placement3D(Id<Axis2Placement3D>),
    Axis2Placement2D(Id<Axis2Placement2D>),
}
pub enum FillStyleSelect {
    FillAreaStyleColour(Id<FillAreaStyleColour>),
    // Rest of options are skipped
}
pub enum PresentationStyleSelect {
    SurfaceStyleUsage(Id<SurfaceStyleUsage>),
    // Rest of options are skipped
}
pub enum Source {
    Made,
    Bought,
    NotKnown,
}
pub enum SurfaceSide {
    Positive,
    Negative,
    Both,
}
pub enum SurfaceSideStyleSelect {
    SurfaceSideStyle(Id<SurfaceSideStyle>),
}
pub enum SurfaceStyleElementSelect {
    SurfaceStyleFillArea(Id<SurfaceStyleFillArea>),
    // Rest of options are skipped
}
pub enum RepresentedDefinition {
    PropertyDefinition(Id<PropertyDefinition>),
    // Rest of options are skipped
}
pub enum MeasureValue {
    CountMeasure(f64),
    // Rest of options are skipped
}
pub enum CharacterizedDefinition {
    // Nothing here?
}
pub enum Unit {
    NamedUnit(Id<NamedUnit>),
    // Nothing else here
}

////////////////////////////////////////////////////////////////////////////////
// Entities
pub struct AdvancedBrepShapeRepresentation {
    pub _0: ShapeRepresentation,
}
pub struct AdvancedFace {
    pub _0: FaceSurface,
}
pub struct ApplicationContext {
    pub application: Label,
}
pub struct ApplicationContextElement {
    pub name: Label,
    pub frame_of_reference: Id<ApplicationContext>,
}
pub struct ApplicationProtocolDefinition {
    pub name: Label,
    pub description: Option<Text>,
    pub relating_context: Id<ApplicationContext>,
    pub related_context: Id<ApplicationContext>,
}
pub struct Axis2Placement2D {
    pub _0: Placement,
    pub ref_direction: Option<Direction>,
}
pub struct Axis2Placement3D {
    pub _0: Placement,
    pub ref_direction: Option<Direction>,
}
pub struct CartesianPoint {
    pub _0: Point,
    pub coordinates: List<LengthMeasure, 1, 3>,
}
pub struct Circle {
    pub _0: Conic,
    pub radius: PositiveLengthMeasure,
}
pub struct ClosedShell {
    pub _0: ConnectedFaceSet,
}
pub struct Colour {}
pub struct ColourRgb {
    pub _0: ColourSpecification,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}
pub struct ColourSpecification {
    pub _0: Colour,
    pub name: Label,
}
pub struct Conic {
    pub _0: Curve,
    pub position: Axis2Placement,
}
pub struct ConnectedFaceSet {
    pub _0: TopologicalRepresentationItem,
    pub cfs_faces: Set<Id<Face>, 1, {usize::MAX}>,
}
pub struct Curve {
    pub _0: GeometricRepresentationItem,
}
pub struct CylindricalSurface {
    pub _0: ElementarySurface,
    pub radius: PositiveLengthMeasure,
}
pub struct Direction {
    pub _0: GeometricRepresentationItem,
    pub direction_ratios: List<f64, 2, 3>,
}
pub struct DimensionalExponents {
    pub length_exponent: f64,
    pub mass_exponent: f64,
    pub time_exponent: f64,
    pub electric_current_exponent: f64,
    pub thermodynamic_temperature_exponent: f64,
    pub amount_of_substance_exponent: f64,
    pub luminous_intensity_exponent: f64,
}
pub struct Edge {
    pub _0: TopologicalRepresentationItem,
    pub edge_start: Id<Vertex>,
    pub edge_end: Id<Vertex>,
}
pub struct EdgeCurve {
    pub _0: Edge,
    pub _1: GeometricRepresentationItem,
    pub edge_geometry: Id<Curve>,
    pub same_sense: bool,
}
pub struct EdgeLoop {
    pub _0: Loop,
    pub _1: Path,
}
pub struct ElementarySurface {
    pub _0: Surface,
    pub position: Axis2Placement3D,
}
pub struct Face {
    pub _0: TopologicalRepresentationItem,
    pub bounds: Set<Id<FaceBound>, 1, {usize::MAX}>,
}
pub struct FaceBound {
    pub _0: TopologicalRepresentationItem,
    pub bound: Id<Loop>,
    pub orientation: bool,
}
pub struct FaceSurface {
    pub _0: Face,
    pub _1: GeometricRepresentationItem,
    pub face_geometry: Id<Surface>,
    pub same_sense: bool,
}
pub struct FillAreaStyle {
    pub _0: FoundedItem,
    pub name: Label,
    pub fill_stypes: Set<FillStyleSelect, 1, {usize::MAX}>,
}
pub struct FillAreaStyleColour {
    pub name: Label,
    pub fill_color: Id<Colour>,
}
pub struct FoundedItem {}
pub struct GeometricRepresentationItem {
    pub _0: RepresentationItem,
}
pub struct Line {
    pub _0: Curve,
    pub pnt: Id<CartesianPoint>,
    pub dir: Id<Vector>,
}
pub struct Loop {
    pub _0: TopologicalRepresentationItem,
}
pub struct ManifoldSolidBrep {
    pub _0: SolidModel,
    pub outer: ClosedShell,
}
pub struct MeasureWithUnit {
    pub value_component: MeasureValue,
    pub unit_component: Unit,
}
pub struct MechanicalDesignGeometricPresentationArea {
    pub _0: PresentationArea,
}
pub struct NamedUnit {
    pub dimensions: Id<DimensionalExponents>,
}
pub struct OrientedEdge {
    pub _0: Edge,
    pub edge_element: Id<Edge>,
    pub orientation: bool,
}
pub struct Path {
    pub _0: GeometricRepresentationItem,
    pub edge_list: UniqueList<Id<OrientedEdge>, 1, {usize::MAX}>,
}
pub struct Placement {
    pub _0: GeometricRepresentationItem,
    pub location: Id<CartesianPoint>,
}
pub struct Plane {
    pub _0: ElementarySurface,
}
pub struct PresentationArea {
    pub _0: PresentationRepresentation,
}
pub struct PresentationRepresentation {
    pub _0: Representation,
}
pub struct PresentationStyleAssignment {
    pub _0: FoundedItem,
    pub styles: Set<Id<PresentationStyleSelect>, 1, {usize::MAX}>,
}
pub struct Product {
    pub id: Identifier,
    pub name: Label,
    pub description: Option<Text>,
    pub frame_of_reference: Set<Id<ProductContext>, 1, {usize::MAX}>,
}
pub struct ProductCategory {
    pub name: Label,
    pub description: Option<Text>,
}
pub struct ProductContext {
    pub _0: ApplicationContextElement,
    pub discipline_type: Label,
}
pub struct ProductDefinition {
    pub id: Identifier,
    pub description: Option<Text>,
    pub formation: Id<ProductDefinitionFormation>,
    pub frame_of_reference: Id<ProductDefinitionContext>,
}
pub struct ProductDefinitionContext {
    pub _0: ApplicationContextElement,
    pub life_cycle_stage: Label,
}
pub struct ProductDefinitionFormation {
    pub id: Identifier,
    pub description: Option<Text>,
    pub of_product: Product,
}
pub struct ProductDefinitionFormationWithSpecifiedSource {
    pub _0: ProductDefinitionFormation,
    pub make_or_buy: Source,
}
pub struct ProductDefinitionShape {
    pub _0: PropertyDefinition,
}
pub struct ProductRelatedProductCategory {
    pub _0: ProductCategory,
    pub products: Set<Id<Product>, 1, {usize::MAX}>,
}
pub struct PropertyDefinition {
    pub name: Label,
    pub description: Option<Text>,
    pub definition: Id<CharacterizedDefinition>,
}
pub struct PropertyDefinitionRepresentation {
    pub definition: RepresentedDefinition,
    pub used_representation: Id<Representation>,
}
pub struct Point {
    pub _0: GeometricRepresentationItem,
}
pub struct Representation {
    pub name: Label,
    pub items: Set<Id<RepresentationItem>, 1, {usize::MAX}>,
    pub context_of_items: Id<RepresentationContext>,
}
pub struct RepresentationContext {
    pub context_identifier: Identifier,
    pub context_type: Text,
}
pub struct RepresentationItem{
    pub name: Label,
}
pub struct RepresentationRelationship {
    pub name: Label,
    pub description: Option<Text>,
    pub rep_1: Representation,
    pub rep_2: Representation,
}
pub struct ShapeDefinitionRepresentation {
    pub _0: PropertyDefinitionRepresentation,
}
pub struct ShapeRepresentation {
    pub _0: Representation,
}
pub struct ShapeRepresentationRelationship {
    pub _0: RepresentationRelationship,
}
pub struct SolidModel {
    pub _0: GeometricRepresentationItem,
}
pub struct StyledItem {
    pub _0: RepresentationItem,
    pub styles: Set<Id<PresentationStyleAssignment>, 1, {usize::MAX}>,
    pub item: RepresentationItem,
}
pub struct Surface {
    pub _0: GeometricRepresentationItem,
}
pub struct SurfaceStyleUsage {
    pub _0: FoundedItem,
    pub side: SurfaceSide,
    pub style: SurfaceSideStyleSelect,
}
pub struct SurfaceSideStyle {
    pub _0: FoundedItem,
    pub name: Label,
    pub styles: Set<Id<SurfaceStyleElementSelect>, 1, 7>,
}
pub struct SurfaceStyleFillArea {
    pub _0: FoundedItem,
    pub fill_area: FillAreaStyle,
}
pub struct TopologicalRepresentationItem {
    pub _0: RepresentationItem,
}
pub struct UncertaintyMeasureWithUnit {
    pub _0: MeasureWithUnit,
    pub name: Label,
    pub description: Option<Text>,
}
pub struct ValueRepresentationItem {
    pub _0: RepresentationItem,
    pub value_component: MeasureValue,
}
pub struct Vector {
    pub _0: GeometricRepresentationItem,
    pub orientation: Direction,
    pub magnitude: LengthMeasure,
}
pub struct Vertex {
    pub _0: TopologicalRepresentationItem,
}
pub struct VertexPoint {
    pub _0: Vertex,
    pub _1: GeometricRepresentationItem,
    pub vertex_geometry: Id<Point>,
}

////////////////////////////////////////////////////////////////////////////////

pub enum Entity {
    AdvancedBrepShapeRepresentation(AdvancedBrepShapeRepresentation),
    AdvancedFace(AdvancedFace),
    ApplicationContext(ApplicationContext),
    ApplicationContextElement(ApplicationContextElement),
    ApplicationProtocolDefinition(ApplicationProtocolDefinition),
    Axis2Placement2D(Axis2Placement2D),
    Axis2Placement3D(Axis2Placement3D),
    CartesianPoint(CartesianPoint),
    Circle(Circle),
    ClosedShell(ClosedShell),
    Colour(Colour),
    ColourRgb(ColourRgb),
    ColourSpecification(ColourSpecification),
    Conic(Conic),
    ConnectedFaceSet(ConnectedFaceSet),
    Curve(Curve),
    CylindricalSurface(CylindricalSurface),
    Direction(Direction),
    DimensionalExponents(DimensionalExponents),
    Edge(Edge),
    EdgeCurve(EdgeCurve),
    EdgeLoop(EdgeLoop),
    ElementarySurface(ElementarySurface),
    Face(Face),
    FaceBound(FaceBound),
    FaceSurface(FaceSurface),
    FillAreaStyle(FillAreaStyle),
    FillAreaStyleColour(FillAreaStyleColour),
    FoundedItem(FoundedItem),
    GeometricRepresentationItem(GeometricRepresentationItem),
    Line(Line),
    Loop(Loop),
    ManifoldSolidBrep(ManifoldSolidBrep),
    MeasureWithUnit(MeasureWithUnit),
    MechanicalDesignGeometricPresentationArea(MechanicalDesignGeometricPresentationArea),
    NamedUnit(NamedUnit),
    OrientedEdge(OrientedEdge),
    Path(Path),
    Placement(Placement),
    Plane(Plane),
    PresentationArea(PresentationArea),
    PresentationRepresentation(PresentationRepresentation),
    PresentationStyleAssignment(PresentationStyleAssignment),
    Product(Product),
    ProductCategory(ProductCategory),
    ProductContext(ProductContext),
    ProductDefinition(ProductDefinition),
    ProductDefinitionContext(ProductDefinitionContext),
    ProductDefinitionFormation(ProductDefinitionFormation),
    ProductDefinitionFormationWithSpecifiedSource(ProductDefinitionFormationWithSpecifiedSource),
    ProductDefinitionShape(ProductDefinitionShape),
    ProductRelatedProductCategory(ProductRelatedProductCategory),
    PropertyDefinition(PropertyDefinition),
    PropertyDefinitionRepresentation(PropertyDefinitionRepresentation),
    Point(Point),
    Representation(Representation),
    RepresentationContext(RepresentationContext),
    RepresentationItem(RepresentationItem),
    RepresentationRelationship(RepresentationRelationship),
    ShapeDefinitionRepresentation(ShapeDefinitionRepresentation),
    ShapeRepresentation(ShapeRepresentation),
    ShapeRepresentationRelationship(ShapeRepresentationRelationship),
    SolidModel(SolidModel),
    StyledItem(StyledItem),
    Surface(Surface),
    SurfaceStyleUsage(SurfaceStyleUsage),
    SurfaceSideStyle(SurfaceSideStyle),
    SurfaceStyleFillArea(SurfaceStyleFillArea),
    TopologicalRepresentationItem(TopologicalRepresentationItem),
    UncertaintyMeasureWithUnit(UncertaintyMeasureWithUnit),
    ValueRepresentationItem(ValueRepresentationItem),
    Vector(Vector),
    Vertex(Vertex),
    VertexPoint(VertexPoint),
}
