
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

mod flat_array;
use self::flat_array::FlatArray;

mod selected_cube;
use self::selected_cube::SelectedCube;

const CUBES_PER_SIDE: usize = 3;

fn main() {
    let mut window= Window::new("Autonomy");

    window.set_light(Light::StickToCamera);

    let mut cubes = FlatArray::new();
    let mut mesh = Rc::new(RefCell::new(cubes.dumb_mesh()));
    let mut mesh_clone = mesh.clone();
    let mut mesh_scene_node = window.add_mesh(mesh, Vector3::new(1.,1.,1.));
    mesh_scene_node.set_color(1.0, 0.0, 0.0);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    let mut selected_cube = SelectedCube::new();
    let mut selected_cube_node = SelectedCube::add_selected_cube_node(&mut window);
    selected_cube.move_selected_cube_node(&mut selected_cube_node);

    while window.render() {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::E, Action::Press, _) => {
                    cubes.toggle(&selected_cube);
                    *mesh_clone.borrow_mut() = cubes.dumb_mesh();
                },

                WindowEvent::Key(Key::A, Action::Press, _) => {
                    if selected_cube.x < CUBES_PER_SIDE-1 {
                        selected_cube.x += 1;
                        selected_cube.move_selected_cube_node(&mut selected_cube_node);
                    }
                },
                WindowEvent::Key(Key::D, Action::Press, _) => {
                    if selected_cube.x > 0 {
                        selected_cube.x -= 1;
                        selected_cube.move_selected_cube_node(&mut selected_cube_node);
                    }
                },
                WindowEvent::Key(Key::W, Action::Press, _) => {
                    if selected_cube.z < CUBES_PER_SIDE-1 {
                        selected_cube.z += 1;
                        selected_cube.move_selected_cube_node(&mut selected_cube_node);
                    }
                },
                WindowEvent::Key(Key::S, Action::Press, _) => {
                    if selected_cube.z > 0 {
                        selected_cube.z -= 1;
                        selected_cube.move_selected_cube_node(&mut selected_cube_node);
                    }
                },
                WindowEvent::Key(Key::R, Action::Press, _) => {
                    if selected_cube.y < CUBES_PER_SIDE-1 {
                        selected_cube.y += 1;
                        selected_cube.move_selected_cube_node(&mut selected_cube_node);
                    }
                },
                WindowEvent::Key(Key::F, Action::Press, _) => {
                    if selected_cube.y > 0 {
                        selected_cube.y -= 1;
                        selected_cube.move_selected_cube_node(&mut selected_cube_node);
                    }
                },


                _ => {},
            }
        }


//        c.prepend_to_local_rotation(&rot);
    }
}





