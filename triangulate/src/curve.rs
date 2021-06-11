use nalgebra_glm as glm;
use glm::{DVec3, DVec4, DMat4};

use nurbs::{AbstractCurve, NDBSplineCurve, SampledCurve};
use crate::surface::Surface;

#[derive(Debug)]
pub enum Curve {
    // TODO: move this to a standalone struct?
    Ellipse {
        eplane_from_world: DMat4,
        world_from_eplane: DMat4,
        closed: bool,
        dir: bool
    },
    Line,
    BSplineCurveWithKnots(SampledCurve<3>),
    NURBSCurve(SampledCurve<4>),
}

impl Curve {
    pub fn new_ellipse(location: DVec3, axis: DVec3, ref_direction: DVec3,
                       radius1: f64, radius2: f64, closed: bool, dir: bool)
        -> Self
    {
        // Build a rotation matrix to go from flat (XY) to 3D space
        let world_from_eplane = Surface::make_affine_transform(axis,
            radius1 * ref_direction,
            radius2 * axis.cross(&ref_direction),
            location);
        let eplane_from_world = world_from_eplane
            .try_inverse()
            .expect("Could not invert");
        Self::Ellipse {
            world_from_eplane,
            eplane_from_world,
            closed, dir
        }
    }

    pub fn new_circle(location: DVec3, axis: DVec3, ref_direction: DVec3,
                      radius: f64, closed: bool, dir: bool) -> Self {
        Self::new_ellipse(location, axis, ref_direction,
                          radius, radius, closed, dir)
    }

    pub fn new_line() -> Self {
        Self::Line
    }

    fn curve_points<const N: usize>(u: DVec3, v: DVec3, curve: &SampledCurve<N>) -> Vec<DVec3>
        where NDBSplineCurve<N>: AbstractCurve
    {
        let t_start = curve.u_from_point(u);
        let t_end = curve.u_from_point(v);
        let mut c = curve.as_polyline(t_start, t_end, 8);
        c[0] = u;
        *c.last_mut().unwrap() = v;
        c
    }

    pub fn build(&self, u: DVec3, v: DVec3) -> Vec<DVec3> {
        match self {
            Self::Line => vec![u, v],
            Self::BSplineCurveWithKnots(curve) => Self::curve_points(u, v, curve),
            Self::NURBSCurve(curve) => Self::curve_points(u, v, curve),
            Self::Ellipse {
                eplane_from_world, world_from_eplane, closed, dir
            } => {
                // Project from 3D into the "ellipse plane".  In the "eplane",
                // the ellipse lies on the unit circle.
                let u_eplane = eplane_from_world *
                               DVec4::new(u.x, u.y, u.z, 1.0);
                let v_eplane = eplane_from_world *
                               DVec4::new(v.x, v.y, v.z, 1.0);

                // Pick the starting angle in the circle's flat plane
                let u_ang = u_eplane.y.atan2(u_eplane.x);
                let mut v_ang = v_eplane.y.atan2(v_eplane.x);
                const PI2: f64 = 2.0 * std::f64::consts::PI;
                if *closed {
                    if *dir {
                        v_ang = u_ang + PI2;
                    } else {
                        v_ang = u_ang - PI2;
                    }
                } else if *dir && v_ang <= u_ang {
                    v_ang += PI2;
                } else if !*dir && v_ang >= u_ang {
                    v_ang -= PI2;
                }

                const N: usize = 64;
                let count = 4.max(
                    (N as f64 * (u_ang - v_ang).abs() /
                    (2.0 * std::f64::consts::PI)).round() as usize);

                let mut out_world = vec![u];
                // Walk around the circle, using the true positions for start
                // and end points to improve numerical accuracy.
                for i in 1..(count - 1) {
                    let frac = (i as f64) / ((count - 1) as f64);
                    let ang = u_ang * (1.0 - frac) + v_ang * frac;
                    let pos_eplane = DVec4::new(ang.cos(), ang.sin(), 0.0, 1.0);

                    // Project back into 3D
                    let p = world_from_eplane * pos_eplane;
                    out_world.push(glm::vec4_to_vec3(&p));
                }
                out_world.push(v);
                out_world
            }
        }
    }
}
