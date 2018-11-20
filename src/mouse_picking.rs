use kiss3d::{
    camera::Camera,
    window::Window,
};
use nalgebra::*;
use alga::linear::Transformation;

use super::CUBES_PER_SIDE;

pub fn find_collision(camera: &dyn Camera, window: &Window) -> Option<Point3<f32>> {
    window.cursor_pos().and_then(|cursor_pos| {

//        let w = window.width();
//        let h = window.height();
//        let relative_cursor_pos = (cursor_pos.0 / w as f64, cursor_pos.1 / h as f64);
//        let relative_cursor_pos: (f64, f64) = (0.5, 0.5);

        // Ray origin O, ray direction, D
        let (ray_start, ray_dir) =
            camera.unproject(
              &Point2::new(cursor_pos.0 as f32, cursor_pos.1 as f32),
              &Vector2::new(window.width() as f32, window.height() as f32)
            );

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
        let nearest_collisions = &mut normals_and_points.into_iter().map(|&(normal, point)| {
            let dot_dir_normal = ray_dir.dot(&normal);
            if dot_dir_normal == 0.0 { panic!("mouse ray perpendicular"); }

            let ray_start_vec = Vector3::new(ray_start.x, ray_start.y, ray_start.z);

            let dot_start_normal = (point - ray_start_vec).dot(&normal);
            let t: f32 = dot_start_normal / dot_dir_normal;
            let collision = ray_start + t * ray_dir;
//                println!("normal: {},{},{}, dot_dir_normal: {}, t: {}, collision: {}", normal.x, normal.y, normal.z, dot_dir_normal, t, collision);
            if collision.x > -0.1 && collision.x < 3.1 &&
                collision.y > -0.1 && collision.y < 3.1 &&
                collision.z > -0.1 && collision.z < 3.1 {
//                    println!("hit! {}", collision);
            }
            println!("COL: {}, {}", collision, t);
            (collision, t)
        });

        let option_nearest_collision = nearest_collisions.filter(|&(collision, _)| {
            collision.x > -0.1 && collision.x < 3.1 &&
                collision.y > -0.1 && collision.y < 3.1 &&
                collision.z > -0.1 && collision.z < 3.1
        }).min_by(|&(_, t1), &(_, t2)| partial_cmp(&t1.abs(), &t2.abs()).unwrap());
//            println!("nearest_collision: {:?}", option_nearest_collision);
        option_nearest_collision.map(|(col, _)| col)
    })
}
