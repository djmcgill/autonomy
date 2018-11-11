
use std::rc::Rc;
use std::slice::Iter;
use std::cell::RefCell;
use nalgebra::*;
use kiss3d::camera::*;
use kiss3d::event::*;
use kiss3d::window::*;
use kiss3d::scene::*;
use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use num::Integer;
use alga::linear::Transformation;

mod flat_array;
use self::flat_array::FlatArray;

mod selected_cube;
use self::selected_cube::SelectedCube;

const CUBES_PER_SIDE: usize = 3;

// where relative_cursor_pos is between (0, 0) top left and (1, 1) bottom right
fn mouse_ray(camera: &dyn Camera, relative_cursor_pos: (f64, f64)) -> (Point3<f32>, Vector3<f32>) {
    let cursor_x = (1.-relative_cursor_pos.0 as f32) * 2. - 1.; // FIXME: something is wrong here
    let cursor_y = (1.-relative_cursor_pos.1 as f32) * 2. - 1.;
    let cursor_vec = Vector4::new(cursor_x, cursor_y, -1.0, 1.0);

    let inv_proj = camera.inverse_transformation();
    let inv_view = camera.view_transform().inverse();

    let ray_eye: Vector4<f32> = inv_proj * cursor_vec;
    let corrected_ray_eye = Vector3::new(ray_eye.x, ray_eye.y, -1.0);
    let ray_world = inv_view.transform_vector(&corrected_ray_eye);

    let camera_pos = camera.eye();
    println!("ray_world: {}", ray_world);
    (camera_pos, ray_world)
}

fn main() {
    let mut window= Window::new("Autonomy");

    let mut camera = ArcBall::new(
        Point3::new(0., 0., -10.),
//        Point3::new(1.5, 7., -10.),
        Point3::new(0., 0., 0.),
    );

    window.set_light(Light::StickToCamera);

    let mut cubes = FlatArray::new();
    let mut mesh = Rc::new(RefCell::new(cubes.dumb_mesh()));
    let mut mesh_clone = mesh.clone();
    let mut mesh_scene_node = window.add_mesh(mesh, Vector3::new(1.,1.,1.));
    mesh_scene_node.set_color(1.0, 0.0, 0.0);

    let mut sphere_hit: SceneNode = window.add_sphere(0.1);
    sphere_hit.set_color(1.0, 1.0, 0.0);
    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    let mut selected_cube = SelectedCube::new();
    let mut selected_cube_node = SelectedCube::add_selected_cube_node(&mut window);
    selected_cube.move_selected_cube_node(&mut selected_cube_node);

    while window.render_with_camera(&mut camera) {
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
        for cursor_pos in window.cursor_pos() {
            let w = window.width();
            let h = window.height();
            let relative_cursor_pos = (cursor_pos.0/w as f64, cursor_pos.1/h as f64);

            // Ray origin O, ray direction, D
            let (ray_start, ray_dir) = mouse_ray(&camera, relative_cursor_pos);

            // Definitions of planes of the whole bounding cube - unit cube from (0,0,0) to (1,1,1)
            let normals_and_points = [
                (Vector3::new(0.0f32, 0.0, 1.0), Vector3::new(0.0, 0.0, CUBES_PER_SIDE as f32)),
                (Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, CUBES_PER_SIDE as f32, 0.0)),
                (Vector3::new(1.0, 0.0, 0.0), Vector3::new(CUBES_PER_SIDE as f32, 0.0, 0.0)),
                (Vector3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 0.0)),
                (Vector3::new(0.0, -1.0, 0.0), Vector3::new(0.0, 0.0, 0.0)),
                (Vector3::new(-1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)),
            ];
            // Each point P on plane is defined by dot(P-point, normal) == 0
            // Each point Q on ray line is defined by O + t*D for some t
            // so we need to find t such that dot(ray_start + t*ray_dir -point, normal) == 0
            // dot(t*ray_dir, normal) + dot(ray_start-point) = 0
            // t*dot(ray_dir, normal) + dot(ray_start-point) = 0
            // t*dot(ray_dir, normal) = - dot(ray_start-point)
            // t*dot(ray_dir, normal) = dot(point-ray_start)
            // t = dot(point-ray_start, normal)/dot(ray_dir, normal)
//            println!("START");
            let option_nearest_collision = &normals_and_points.into_iter().map(|&(normal, point)| {
                let dot_dir_normal = ray_dir.dot(&normal);
                if dot_dir_normal == 0.0 { panic!("mouse ray perpendicular"); }

                let ray_start_vec = Vector3::new(ray_start.x, ray_start.y, ray_start.z);

                let dot_start_normal = (point-ray_start_vec).dot(&normal);
                let t = dot_start_normal / dot_dir_normal;
                let collision = ray_start + t*ray_dir;
//                println!("normal: {},{},{}, dot_dir_normal: {}, t: {}, collision: {}", normal.x, normal.y, normal.z, dot_dir_normal, t, collision);
                if collision.x > -0.1 && collision.x < 3.1 &&
                    collision.y > -0.1 && collision.y < 3.1 &&
                    collision.z > -0.1 && collision.z < 3.1 {
//                    println!("hit! {}", collision);
                }
                (collision, t.abs())
            }).filter(|&(collision, _)| {
                collision.x > -0.1 && collision.x < 3.1 &&
                    collision.y > -0.1 && collision.y < 3.1 &&
                    collision.z > -0.1 && collision.z < 3.1
            }).min_by(|&(_, t1), &(_, t2)| partial_cmp(&t1, &t2).unwrap());
//            println!("nearest_collision: {:?}", option_nearest_collision);
            for &(nearest_collision, _) in option_nearest_collision {
                sphere_hit.set_local_translation(Translation3::new(nearest_collision.x, nearest_collision.y, nearest_collision.z));
            }

        }

//        c.prepend_to_local_rotation(&rot);
    }
}





