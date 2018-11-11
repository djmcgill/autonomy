use kiss3d::{
    camera::Camera,
    window::Window,
};
use nalgebra::*;
use alga::linear::Transformation;

use super::CUBES_PER_SIDE;

pub fn find_collision(camera: &dyn Camera, window: &Window) -> Option<Point3<f32>> {
    window.cursor_pos().and_then(|cursor_pos| {

        let w = window.width();
        let h = window.height();
        let relative_cursor_pos = (cursor_pos.0 / w as f64, cursor_pos.1 / h as f64);

        // Ray origin O, ray direction, D
        let (ray_start, ray_dir) = mouse_ray(camera, relative_cursor_pos);

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

            let dot_start_normal = (point - ray_start_vec).dot(&normal);
            let t = dot_start_normal / dot_dir_normal;
            let collision = ray_start + t * ray_dir;
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
        option_nearest_collision.map(|(col, _)| col)
    })
}

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
