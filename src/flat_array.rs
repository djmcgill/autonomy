use super::CUBES_PER_SIDE;
use kiss3d::resource::Mesh;
use nalgebra::*;
use num::Integer;
use super::selected_cube::SelectedCube;

pub struct FlatArray([bool; CUBES_PER_SIDE*CUBES_PER_SIDE*CUBES_PER_SIDE]);
impl FlatArray {
    pub fn new() -> Self {
        FlatArray([true; CUBES_PER_SIDE*CUBES_PER_SIDE*CUBES_PER_SIDE])
    }

    pub fn dumb_mesh(&self) -> Mesh {
        let mut coords = Vec::with_capacity(8*CUBES_PER_SIDE*CUBES_PER_SIDE*CUBES_PER_SIDE);
        let mut faces = Vec::with_capacity(12*CUBES_PER_SIDE*CUBES_PER_SIDE*CUBES_PER_SIDE);

        for (item, ix) in self.0.into_iter().cloned().zip(0usize..) {
            if item {
                let (ix_z, ix_yx) = ix.div_rem(&(CUBES_PER_SIDE * CUBES_PER_SIDE));
                let (ix_y, ix_x) = ix_yx.div_rem(&CUBES_PER_SIDE);
                let x_diff = ix_x as f32; // cubes have size 1
                let y_diff = ix_y as f32;
                let z_diff = ix_z as f32;

                let ix_diff = coords.len() as u16;

                coords.push(Point3::new(0.0 + x_diff, 0.0 + y_diff, 1.0 + z_diff));
                coords.push(Point3::new(0.0 + x_diff, 0.0 + y_diff, 0.0 + z_diff));
                coords.push(Point3::new(1.0 + x_diff, 0.0 + y_diff, 0.0 + z_diff));
                coords.push(Point3::new(1.0 + x_diff, 0.0 + y_diff, 1.0 + z_diff));
                coords.push(Point3::new(0.0 + x_diff, 1.0 + y_diff, 1.0 + z_diff));
                coords.push(Point3::new(0.0 + x_diff, 1.0 + y_diff, 0.0 + z_diff));
                coords.push(Point3::new(1.0 + x_diff, 1.0 + y_diff, 0.0 + z_diff));
                coords.push(Point3::new(1.0 + x_diff, 1.0 + y_diff, 1.0 + z_diff));

                faces.push(Point3::new(6 + ix_diff, 2 + ix_diff, 1 + ix_diff));
                faces.push(Point3::new(1 + ix_diff, 5 + ix_diff, 6 + ix_diff));
                faces.push(Point3::new(7 + ix_diff, 3 + ix_diff, 2 + ix_diff));
                faces.push(Point3::new(2 + ix_diff, 6 + ix_diff, 7 + ix_diff));
                faces.push(Point3::new(7 + ix_diff, 4 + ix_diff, 0 + ix_diff));
                faces.push(Point3::new(0 + ix_diff, 3 + ix_diff, 7 + ix_diff));
                faces.push(Point3::new(4 + ix_diff, 7 + ix_diff, 6 + ix_diff));
                faces.push(Point3::new(6 + ix_diff, 5 + ix_diff, 4 + ix_diff));
                faces.push(Point3::new(0 + ix_diff, 1 + ix_diff, 2 + ix_diff));
                faces.push(Point3::new(2 + ix_diff, 3 + ix_diff, 0 + ix_diff));
                faces.push(Point3::new(4 + ix_diff, 5 + ix_diff, 1 + ix_diff));
                faces.push(Point3::new(1 + ix_diff, 0 + ix_diff, 4 + ix_diff));
            }
        }

        if !(coords.is_empty() || faces.is_empty()) {
            // TODO: calculate normals and uvs upfront
            Mesh::new(coords, faces, None, None, true)
        } else {
            // This is a dumb work around for a crash in kiss3d if you make a mesh with an empty
            // vec for either coords or faces
            let coords = vec![
                Point3::new(-0.5, -0.5, 0.5),
                Point3::new(-0.5, -0.5, -0.5),
                Point3::new(-0.5, 0.5, 0.5),
            ];
            let faces = vec![
                Point3::new(0, 1, 2)
            ];
            Mesh::new(coords, faces, None, None, true)
        }

    }

    pub fn toggle(&mut self, selected_cube: &SelectedCube) {
        let SelectedCube{x, y, z} = selected_cube;
        let selected_cube_ix = x + y*CUBES_PER_SIDE + z*CUBES_PER_SIDE*CUBES_PER_SIDE;
        self.0[selected_cube_ix] = !self.0[selected_cube_ix];
    }
}
