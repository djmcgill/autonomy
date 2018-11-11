
use std::rc::Rc;
use std::cell::RefCell;
use nalgebra::*;
use kiss3d::{
    camera::ArcBall,
    event::{Action, Key, WindowEvent},
    window::Window,
    scene::SceneNode,
    light::Light,
};


mod flat_array;
use self::flat_array::FlatArray;

mod selected_cube;
use self::selected_cube::SelectedCube;

mod mouse_picking;
use self::mouse_picking::*;

const CUBES_PER_SIDE: usize = 3;

fn main() {
    // Initialisation
    let mut window = Window::new("Autonomy");
    let mut camera = ArcBall::new(
        Point3::new(0., 0., -10.),
//        Point3::new(1.5, 7., -10.),
        Point3::new(0., 0., 0.),
    );
    window.set_light(Light::StickToCamera);

    // Code that handles the voxels
    let mut cubes = FlatArray::new();
    let mesh = Rc::new(RefCell::new(cubes.dumb_mesh()));
    let mut mesh_scene_node = window.add_mesh(mesh.clone(), Vector3::new(1.,1.,1.));
    mesh_scene_node.set_color(1.0, 0.0, 0.0);

    // Code that handles the mouse ray sphere
    let mut sphere_hit: SceneNode = window.add_sphere(0.1);
    sphere_hit.set_color(1.0, 1.0, 0.0);

    // Code that handles the cube selector
    let mut selected_cube = SelectedCube::new();
    let mut selected_cube_node = SelectedCube::add_selected_cube_node(&mut window);
    selected_cube.move_selected_cube_node(&mut selected_cube_node);

    // Event loop
    while window.render_with_camera(&mut camera) {
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::E, Action::Press, _) => cubes.toggle(&selected_cube, mesh.clone()),
                WindowEvent::Key(Key::A, Action::Press, _) => selected_cube.west(&mut selected_cube_node),
                WindowEvent::Key(Key::D, Action::Press, _) => selected_cube.east(&mut selected_cube_node),
                WindowEvent::Key(Key::W, Action::Press, _) => selected_cube.north(&mut selected_cube_node),
                WindowEvent::Key(Key::S, Action::Press, _) => selected_cube.south(&mut selected_cube_node),
                WindowEvent::Key(Key::R, Action::Press, _) => selected_cube.up(&mut selected_cube_node),
                WindowEvent::Key(Key::F, Action::Press, _) => selected_cube.down(&mut selected_cube_node),
                _ => {},
            }
        }
        let option_nearest_collision = find_collision(&camera, &window);
        for nearest_collision in option_nearest_collision {
            sphere_hit.set_local_translation(Translation3::new(nearest_collision.x, nearest_collision.y, nearest_collision.z));
        }
    }
}





