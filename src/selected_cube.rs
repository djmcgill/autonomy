use nalgebra::*;
use kiss3d::window::*;
use kiss3d::scene::*;

pub struct SelectedCube{pub x: usize, pub y: usize, pub z: usize}
impl SelectedCube {
    pub fn new() -> Self {
        SelectedCube{x: 0, y: 0, z: 0}
    }

    pub fn move_selected_cube_node(&self, selected_cube_node: &mut SceneNode) {
        selected_cube_node.set_local_translation(
            Translation3::new(
                self.x as f32,
                self.y as f32,
                self.z as f32
            )
        )
    }

    pub fn add_selected_cube_node(window: &mut Window) -> SceneNode {
        let selection_extra = 0.1f32;

        let mut selected_cube_node = window.add_group();

        let mut selected_edge_1 = selected_cube_node.add_cube(1.+2.*selection_extra, selection_extra, selection_extra);
        selected_edge_1.append_translation(&Translation3::new(0.5, 0.0, 0.0));
        let mut selected_edge_2 = selected_cube_node.add_cube(1.+2.*selection_extra, selection_extra, selection_extra);
        selected_edge_2.append_translation(&Translation3::new(0.5, 0.0, 1.0));
        let mut selected_edge_3 = selected_cube_node.add_cube(1.+2.*selection_extra, selection_extra, selection_extra);
        selected_edge_3.append_translation(&Translation3::new(0.5, 1.0, 0.0));
        let mut selected_edge_4 = selected_cube_node.add_cube(1.+2.*selection_extra, selection_extra, selection_extra);
        selected_edge_4.append_translation(&Translation3::new(0.5, 1.0, 1.0));

        let mut selected_edge_5 = selected_cube_node.add_cube(selection_extra, 1.+2.*selection_extra, selection_extra);
        selected_edge_5.append_translation(&Translation3::new(0.0, 0.5, 0.0));
        let mut selected_edge_6 = selected_cube_node.add_cube(selection_extra, 1.+2.*selection_extra, selection_extra);
        selected_edge_6.append_translation(&Translation3::new(1.0, 0.5, 0.0));
        let mut selected_edge_7 = selected_cube_node.add_cube(selection_extra, 1.+2.*selection_extra, selection_extra);
        selected_edge_7.append_translation(&Translation3::new(0.0, 0.5, 1.0));
        let mut selected_edge_8 = selected_cube_node.add_cube(selection_extra, 1.+2.*selection_extra, selection_extra);
        selected_edge_8.append_translation(&Translation3::new(1.0, 0.5, 1.0));

        let mut selected_edge_9 = selected_cube_node.add_cube(selection_extra, selection_extra, 1.+2.*selection_extra);
        selected_edge_9.append_translation(&Translation3::new(0.0, 0.0, 0.5));
        let mut selected_edge_10 = selected_cube_node.add_cube(selection_extra, selection_extra, 1.+2.*selection_extra);
        selected_edge_10.append_translation(&Translation3::new(0.0, 1.0, 0.5));
        let mut selected_edge_11 = selected_cube_node.add_cube(selection_extra, selection_extra, 1.+2.*selection_extra);
        selected_edge_11.append_translation(&Translation3::new(1.0, 0.0, 0.5));
        let mut selected_edge_12 = selected_cube_node.add_cube(selection_extra, selection_extra, 1.+2.*selection_extra);
        selected_edge_12.append_translation(&Translation3::new(1.0, 1.0, 0.5));
        selected_cube_node
    }
}
