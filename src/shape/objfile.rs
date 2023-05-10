//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// include an .obj file
//

use crate::intersection::{Intersection, Ray};
use crate::vector3d::Vector3D;
use crate::shape::Shape;
use crate::shape::Composite;
use crate::shape::Triangle;
use crate::material::Material;

use serde::{Serialize, Deserialize};

use std::io::BufRead;

#[derive(Serialize, Deserialize)]
struct SerOBJFile {
    path: String,
    material: Material
}

#[derive(Serialize, Deserialize)]
#[serde(try_from = "SerOBJFile")]
pub struct OBJFile {
    pub wrapped: Composite
}

impl TryFrom<SerOBJFile> for OBJFile {
    type Error = std::io::Error;

    fn try_from(ser: SerOBJFile) -> Result<Self, Self::Error> {
        let file = std::fs::File::open(ser.path)?;
        let reader = std::io::BufReader::new(file);
        let mut composite = Composite { shapes: vec![] };
        let mut vertices = vec![];
        for line in reader.lines() {
            let line = line?;
            let mut words = line.split_whitespace();
            match words.next() {
                Some("v") => {
                    let Some(Ok(x)) = words.next().map(str::parse::<f64>) else {
                        continue
                    };
                    let Some(Ok(y)) = words.next().map(str::parse::<f64>) else {
                        continue
                    };
                    let Some(Ok(z)) = words.next().map(str::parse::<f64>) else {
                        continue
                    };
                    vertices.push(Vector3D { x, y, z, w: 1.0 });
                }
                Some("f") => {
                    fn parse_vertex(s: &str) -> Result<usize, std::io::Error> {
                        s.split('/').next().unwrap().parse::<usize>().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                    }
                    let Some(Ok(a)) = words.next().map(parse_vertex) else {
                        continue
                    };
                    let Some(Ok(b)) = words.next().map(parse_vertex) else {
                        continue
                    };
                    let Some(Ok(c)) = words.next().map(parse_vertex) else {
                        continue
                    };
                    composite.shapes.push(Box::new(Triangle::new(
                        vertices[a - 1],
                        vertices[b - 1],
                        vertices[c - 1],
                        ser.material
                    )));
                }
                _ => {}
            }
        }
        Ok(OBJFile { wrapped: composite })
    }
}

#[typetag::serde]
impl Shape for OBJFile {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.wrapped.intersect(ray)
    }
}
