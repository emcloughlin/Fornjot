use crate::{
    kernel::{
        shape::Shape,
        topology::{
            edges::{Cycle, Edge},
            faces::Face,
        },
    },
    math::Transform,
};

/// Create a new shape that is a transformed version of an existing one
///
/// # Implementation note
///
/// This code isn't really correct, only transforming the faces of the original
/// shape and not taking care of anything else, but this is more a reflection of
/// the state of `Shape`, with its redundant data.
///
/// Addressing the shortcomings in this method probably doesn't make sense,
/// except as a side effect of addressing the shortcomings of `Shape`.
pub fn transform_shape(original: &Shape, transform: &Transform) -> Shape {
    let mut transformed = Shape::new();

    for face in &original.faces.0 {
        let face = transform_face(face, transform, &mut transformed);
        transformed.faces.0.push(face);
    }

    transformed
}

/// Create a new face that is a transformed version of an existing one
pub fn transform_face(
    original: &Face,
    transform: &Transform,
    shape: &mut Shape,
) -> Face {
    match original.clone() {
        Face::Face { cycles, surface } => {
            let mut cycles_trans = Vec::new();

            for cycle in cycles {
                let mut edges = Vec::new();

                for edge in cycle.edges {
                    let vertices = edge.vertices.clone().map(|vertices| {
                        vertices.map(|vertex| {
                            let point =
                                transform.transform_point(&vertex.point());

                            shape.vertices().add(point)
                        })
                    });

                    let edge = Edge {
                        curve: edge.curve.transform(transform),
                        vertices,
                    };
                    let edge = shape.edges().add(edge);

                    edges.push(edge);
                }

                cycles_trans.push(Cycle { edges });
            }

            Face::Face {
                cycles: cycles_trans,
                surface: surface.transform(transform),
            }
        }
        Face::Triangles(mut triangles) => {
            for triangle in &mut triangles {
                *triangle = transform.transform_triangle(triangle);
            }

            Face::Triangles(triangles)
        }
    }
}