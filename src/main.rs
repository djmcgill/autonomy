
use std::rc::Rc;
use std::slice::Iter;
use std::cell::RefCell;
use nalgebra::*;
use kiss3d::event::*;
use kiss3d::window::*;
use kiss3d::scene::*;
use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use num::Integer;

const CUBES_PER_SIDE: usize = 3;

fn main() {
    let mut window= Window::new("Kiss3d: cube");

    window.set_light(Light::StickToCamera);

    let mut cubes = [true; CUBES_PER_SIDE*CUBES_PER_SIDE*CUBES_PER_SIDE];
    let mut mesh = Rc::new(RefCell::new(dumb_mesh(
        cubes.into_iter().cloned()
    )));
    let mut mesh_clone = mesh.clone();
    let mut mesh_scene_node = window.add_mesh(mesh, Vector3::new(1.,1.,1.));
    mesh_scene_node.set_color(1.0, 0.0, 0.0);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    let mut selected_cube = (0usize, 0usize, 0usize);
    let mut selected_cube_node = add_selected_cube_node(&mut window);
    move_selected_cube_node(&mut selected_cube_node, selected_cube);

    while window.render() {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::E, Action::Press, _) => {
                    let selected_cube_ix =
                        selected_cube.0 +
                            selected_cube.1*CUBES_PER_SIDE +
                            selected_cube.2*CUBES_PER_SIDE*CUBES_PER_SIDE;

                    cubes[selected_cube_ix] = !cubes[selected_cube_ix];
                    *mesh_clone.borrow_mut() = dumb_mesh(cubes.into_iter().cloned());
                },

                WindowEvent::Key(Key::A, Action::Press, _) => {
                    if selected_cube.0 < CUBES_PER_SIDE-1 {
                        selected_cube.0 += 1;
                        move_selected_cube_node(&mut selected_cube_node, selected_cube);
                    }
                },
                WindowEvent::Key(Key::D, Action::Press, _) => {
                    if selected_cube.0 > 0 {
                        selected_cube.0 -= 1;
                        move_selected_cube_node(&mut selected_cube_node, selected_cube);
                    }
                }
                WindowEvent::Key(Key::W, Action::Press, _) => {
                    if selected_cube.2 < CUBES_PER_SIDE-1 {
                        selected_cube.2 += 1;
                        move_selected_cube_node(&mut selected_cube_node, selected_cube);
                    }
                },
                WindowEvent::Key(Key::S, Action::Press, _) => {
                    if selected_cube.2 > 0 {
                        selected_cube.2 -= 1;
                        move_selected_cube_node(&mut selected_cube_node, selected_cube);
                    }
                }
                WindowEvent::Key(Key::R, Action::Press, _) => {
                    if selected_cube.1 < CUBES_PER_SIDE-1 {
                        selected_cube.1 += 1;
                        move_selected_cube_node(&mut selected_cube_node, selected_cube);
                    }
                },
                WindowEvent::Key(Key::F, Action::Press, _) => {
                    if selected_cube.1 > 0 {
                        selected_cube.1 -= 1;
                        move_selected_cube_node(&mut selected_cube_node, selected_cube);
                    }
                }


                _ => {},
            }
        }


//        c.prepend_to_local_rotation(&rot);
    }
}

fn move_selected_cube_node(selected_cube_node: &mut SceneNode, selected_xyz: (usize, usize, usize)) {
    selected_cube_node.set_local_translation(
        Translation3::new(
            selected_xyz.0 as f32,
            selected_xyz.1 as f32,
            selected_xyz.2 as f32
        )
    )
}

fn add_selected_cube_node(window: &mut Window) -> SceneNode {
    let selection_extra = 0.1f32;

    let mut selected_cube_node = window.add_group();

    let mut selected_edge_1 = selected_cube_node.add_cube(1.+2.*selection_extra, selection_extra, selection_extra);
    selected_edge_1.append_translation(&Translation3::new(0., -0.5, -0.5));
    let mut selected_edge_2 = selected_cube_node.add_cube(1.+2.*selection_extra, selection_extra, selection_extra);
    selected_edge_2.append_translation(&Translation3::new(0., -0.5, 0.5));
    let mut selected_edge_3 = selected_cube_node.add_cube(1.+2.*selection_extra, selection_extra, selection_extra);
    selected_edge_3.append_translation(&Translation3::new(0., 0.5, -0.5));
    let mut selected_edge_4 = selected_cube_node.add_cube(1.+2.*selection_extra, selection_extra, selection_extra);
    selected_edge_4.append_translation(&Translation3::new(0., 0.5, 0.5));

    let mut selected_edge_5 = selected_cube_node.add_cube(selection_extra, 1.+2.*selection_extra, selection_extra);
    selected_edge_5.append_translation(&Translation3::new(-0.5, 0., -0.5));
    let mut selected_edge_6 = selected_cube_node.add_cube(selection_extra, 1.+2.*selection_extra, selection_extra);
    selected_edge_6.append_translation(&Translation3::new(0.5, 0., -0.5));
    let mut selected_edge_7 = selected_cube_node.add_cube(selection_extra, 1.+2.*selection_extra, selection_extra);
    selected_edge_7.append_translation(&Translation3::new(-0.5, 0., 0.5));
    let mut selected_edge_8 = selected_cube_node.add_cube(selection_extra, 1.+2.*selection_extra, selection_extra);
    selected_edge_8.append_translation(&Translation3::new(0.5, 0., 0.5));

    let mut selected_edge_9 = selected_cube_node.add_cube(selection_extra, selection_extra, 1.+2.*selection_extra);
    selected_edge_9.append_translation(&Translation3::new(-0.5, -0.5, 0.));
    let mut selected_edge_10 = selected_cube_node.add_cube(selection_extra, selection_extra, 1.+2.*selection_extra);
    selected_edge_10.append_translation(&Translation3::new(-0.5, 0.5, 0.));
    let mut selected_edge_11 = selected_cube_node.add_cube(selection_extra, selection_extra, 1.+2.*selection_extra);
    selected_edge_11.append_translation(&Translation3::new(0.5, -0.5, 0.));
    let mut selected_edge_12 = selected_cube_node.add_cube(selection_extra, selection_extra, 1.+2.*selection_extra);
    selected_edge_12.append_translation(&Translation3::new(0.5, 0.5, 0.));
    selected_cube_node
}


fn dumb_mesh<I: Iterator<Item=bool>>(iter: I) -> Mesh {
    let mut coords = Vec::with_capacity(8*CUBES_PER_SIDE*CUBES_PER_SIDE*CUBES_PER_SIDE);
    let mut faces = Vec::with_capacity(12*CUBES_PER_SIDE*CUBES_PER_SIDE*CUBES_PER_SIDE);

    for (item, ix) in iter.zip(0usize..) {
        if item {
            let (ix_z, ix_yx) = ix.div_rem(&(CUBES_PER_SIDE * CUBES_PER_SIDE));
            let (ix_y, ix_x) = ix_yx.div_rem(&CUBES_PER_SIDE);
            let x_diff = ix_x as f32; // cubes have size 1
            let y_diff = ix_y as f32;
            let z_diff = ix_z as f32;

            let ix_diff = coords.len() as u16;

            coords.push(Point3::new(-0.5 + x_diff, -0.5 + y_diff, 0.5 + z_diff));
            coords.push(Point3::new(-0.5 + x_diff, -0.5 + y_diff, -0.5 + z_diff));
            coords.push(Point3::new(0.5 + x_diff, -0.5 + y_diff, -0.5 + z_diff));
            coords.push(Point3::new(0.5 + x_diff, -0.5 + y_diff, 0.5 + z_diff));
            coords.push(Point3::new(-0.5 + x_diff, 0.5 + y_diff, 0.5 + z_diff));
            coords.push(Point3::new(-0.5 + x_diff, 0.5 + y_diff, -0.5 + z_diff));
            coords.push(Point3::new(0.5 + x_diff, 0.5 + y_diff, -0.5 + z_diff));
            coords.push(Point3::new(0.5 + x_diff, 0.5 + y_diff, 0.5 + z_diff));

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

    if !coords.is_empty() && !faces.is_empty() {
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
