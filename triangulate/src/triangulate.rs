use std::collections::{HashMap, HashSet};
use nalgebra_glm as glm;
use glm::{DVec2, DVec3, DVec4, DMat4, U32Vec3};

use step2::{step_file::StepFile, id::Id, ap214::Entity, ap214::*};
use crate::{mesh::Mesh, stats::Stats};

fn triangulate(s: &StepFile) -> (Mesh, Stats) {
    // Build the tree of transforms, from the root up
    let mut roots = HashSet::new();
    let mut leaves = HashSet::new();

    // from root to REPRESENTATION_RELATIONSHIP id
    let mut lookup: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, e) in s.0.iter().enumerate() {
        let r = match e {
            Entity::RepresentationRelationshipWithTransformation(r) => {
                leaves.insert(r.rep_1.0);
                r.rep_2.0
            },
            Entity::ShapeRepresentationRelationship(r) => {
                r.rep_2.0
            },
            Entity::ShapeDefinitionRepresentation(r) => {
                r.used_representation.0
            },
            _ => continue,
        };
        roots.insert(r);
        lookup.entry(r).or_insert_with(Vec::new).push(i);
    }
    // Pick out the roots of the transformation DAG
    let mut todo: Vec<(usize, DMat4)> = roots.difference(&leaves)
        .map(|i| (*i, DMat4::identity()))
        .collect();

    // We'll store leaves of the graph along with their transform here
    let mut to_mesh: Vec<(usize, DMat4)> = Vec::new();

    // Iterate through the transformation DAG
    while let Some((id, mat)) = todo.pop() {
        for v in &lookup[&id] {
            match &s.0[*v] {
                Entity::RepresentationRelationshipWithTransformation(r) => {
                    assert!(r.rep_2.0 == id);
                    let t = &r.transformation_operator;
                    use Transformation::ItemDefinedTransformation;
                    let next_mat = if let ItemDefinedTransformation(i) = t {
                        item_defined_transformation(s, *i)
                    } else {
                        panic!("Invalid transformation {:?}", t);
                    };
                    if roots.contains(&r.rep_1.0) {
                        todo.push((r.rep_1.0, mat * next_mat));
                    } else {
                        to_mesh.push((r.rep_1.0, mat * next_mat));
                    }
                },
                Entity::ShapeRepresentationRelationship(r) =>
                    to_mesh.push((r.rep_2.0, mat)),
                Entity::ShapeDefinitionRepresentation(r) =>
                    to_mesh.push((r.used_representation.0, mat)),
                e => panic!("Invalid entity {:?}", e),
            }
        }
    }

    /*
    let t = to_mesh.par_iter()
        .map(|(id, mat)| {
            let mut out = self.shape_representation_(*id);
            for v in 0..out.verts.len() {
                 let p = out.verts[v].pos;
                 out.verts[v].pos = (mat * DVec4::new(p.x, p.y, p.z, 1.0)).xyz();
                 let n = out.verts[v].norm;
                 out.verts[v].norm = (mat * DVec4::new(n.x, n.y, n.z, 0.0)).xyz();
             }
            out
        })
        .reduce(Triangulation::default, Triangulation::combine);
    self.vertices = t.verts;
    self.triangles = t.index;
    println!("num_shells: {}", t.num_shells);
    println!("num_faces: {}", t.num_faces);
    println!("num_errors: {}", t.num_errors);
    println!("num_panics: {}", t.num_panics);
    */
    unimplemented!()
}

fn item_defined_transformation(s: &StepFile, t: Id<ItemDefinedTransformation_>) -> DMat4 {
    DMat4::identity()
}
