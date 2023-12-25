

use eframe::{egui_glow, glow};
use egui::{Ui};
use three_d::*;

use super::DockableWidget;

use crate::{simulation::state::SimulationState, viz::{context::VizContext, frame_input}, types::pose::Pose};

pub struct ThreeVizWidget {}

impl ThreeVizWidget {
    pub fn new() -> Self {
        Self {}
    }
    fn custom_painting(&mut self, ui: &mut egui::Ui, state: Option<SimulationState>) {
        let (rect, _response) =
            ui.allocate_exact_size(egui::Vec2::splat(800.), egui::Sense::drag());

            let callback = egui::PaintCallback {
                rect,
                callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |info, painter| {
                    with_three_d(painter.gl(), |three_d| {
                        three_d.frame(
                            frame_input::FrameInput::new(&three_d.context, &info, painter),
                            state.clone(),
                        );
                    });
                })),
            };
            ui.painter().add(callback);
       
    }
}

impl DockableWidget for ThreeVizWidget {
    fn draw(&mut self, ui: &mut Ui, context: VizContext) {
        let sim_state = context.sim_state.clone();
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            self.custom_painting(ui, sim_state.clone());
        });
    }
}

fn with_three_d<R>(gl: &std::sync::Arc<glow::Context>, f: impl FnOnce(&mut ThreeDApp) -> R) -> R {
    use std::cell::RefCell;
    thread_local! {
        pub static THREE_D: RefCell<Option<ThreeDApp>> = RefCell::new(None);
    }

    THREE_D.with(|three_d| {
        let mut three_d = three_d.borrow_mut();
        let three_d = three_d.get_or_insert_with(|| ThreeDApp::new(gl.clone()));
        f(three_d)
    })
}



pub struct ThreeDApp {
    context: Context,
    camera: Camera,
    floor_mesh: Gm<Mesh, PhysicalMaterial>,
    uav_meshes: Gm<Mesh, PhysicalMaterial>,
}

impl ThreeDApp {
    pub fn new(gl: std::sync::Arc<glow::Context>) -> Self {
        let context = Context::from_gl_context(gl).unwrap();
        // Create a camera
        let camera = Camera::new_perspective(
            Viewport::new_at_origo(800, 600),
            vec3(2.5, 3.0, 5.0),
            vec3(0.0, 0., 0.0),
            vec3(0.0, 0.0, 1.0),
            degrees(80.0),
            0.1,
            1000.0,
        );

        let mut floor = Gm::new(
            Mesh::new(&context, &CpuMesh::square()),
            PhysicalMaterial::new_opaque(
                &context,
                &CpuMaterial {
                    albedo: Srgba::new(150, 150, 150, 255),
                    metallic: 0.1,
                    roughness: 0.3,
                    ..Default::default()
                },
            ),
        );

        let uav = Gm::new(
            Mesh::new(&context, &CpuMesh::cube()),
            PhysicalMaterial::new_opaque(
                &context,
                &CpuMaterial {
                    albedo: Srgba::new(200, 200, 0, 255),
                    ..Default::default()
                },
            ),
        );

        floor.set_transformation(
            Mat4::from_translation(vec3(0.0, 0.0, 0.0))
                * Mat4::from_nonuniform_scale(10., 10.0 , 0.1),
        );
        Self {
            context,
            camera,
            floor_mesh: floor,
            uav_meshes: uav,
        }
    }

    pub fn frame(
        &mut self,
        frame_input: frame_input::FrameInput,
        state: Option<SimulationState>,
    ) -> Option<glow::Framebuffer> {
        // Ensure the viewport matches the current window viewport which changes if the window is resized
        self.camera.set_viewport(frame_input.viewport);
       
        
        let ambient = AmbientLight::new(&self.context, 0.4, Srgba::WHITE);
        let mut directional0 = DirectionalLight::new(&self.context, 1.0, Srgba::WHITE, &vec3(-1.0, -1.0, -1.0));
        
        let mut objects: Vec<&dyn Object> = vec![];

        let state = state;
        let uav_pose = match state.clone() {
            Some(state) => state.uav_state.uav_state.pose,
            None => Pose::zero(),
        };
      
        let uav_rot = uav_pose.orientation.euler_angles();
        let cam_up = self.camera.up().clone();
        let cam_pos = self.camera.position().clone();
        let taget = Vector3::new(uav_pose.position.x, uav_pose.position.y, uav_pose.position.z);
        self.camera.set_view(cam_pos, taget, cam_up);
       
        self.uav_meshes.set_transformation(
            Mat4::from_translation(vec3(uav_pose.position.x, uav_pose.position.y, uav_pose.position.z))
                * Mat4::from_angle_x(radians(uav_rot.0))
                * Mat4::from_angle_y(radians(uav_rot.1))
                * Mat4::from_angle_z(radians(uav_rot.2))
                * Mat4::from_nonuniform_scale(0.5, 0.5, 0.05),
        );

      
      
        directional0.generate_shadow_map(1024, &self.uav_meshes);
        
        objects.push(&self.uav_meshes as &dyn Object); // Add the light to the objects to be rendered
        objects.push(&self.floor_mesh as &dyn Object); // Add the light to the objects to be rendered
        
     
        frame_input.screen.clear_partially(frame_input.scissor_box, ClearState::depth(1.0));
        frame_input
            .screen
            // Clear the color and depth of the screen render target
           
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render_partially(
                frame_input.scissor_box,
                &self.camera,
                &objects,
                &[&ambient,&directional0],
            );
        
        frame_input.screen.into_framebuffer() // Take back the screen fbo, we will continue to use it.
    }
}