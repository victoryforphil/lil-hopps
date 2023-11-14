use eframe::egui_glow;
use egui::{Ui, Frame};
use three_d::*;

use super::DockableWidget;

use crate::{simulation::state::SimulationState, viz::context::VizContext};

pub struct ThreeVizWidget {}

impl ThreeVizWidget {
    pub fn new() -> Self {
        Self {}
    }
    fn custom_painting(&mut self, ui: &mut egui::Ui, state: Option<SimulationState>) {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                let gl = painter.gl().clone();
                
                let context = three_d::Context::from_gl_context(gl).unwrap();

                let target = vec3(0.0, 2.0, 0.0);
                let scene_radius = 6.0;
                let viewport = Viewport::new_at_origo(1280, 720);
                let mut camera = Camera::new_perspective(
                    viewport,
                    vec3(5.0, 2.0, 2.5),
                    vec3(0.0, 0.0, -0.5),
                    vec3(0.0, 1.0, 0.0),
                    degrees(45.0),
                    0.1,
                    1000.0,
                );
                let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

                let mut sphere = Gm::new(
                    Mesh::new(&context, &CpuMesh::sphere(16)),
                    PhysicalMaterial::new_transparent(
                        &context,
                        &CpuMaterial {
                            albedo: Srgba {
                                r: 255,
                                g: 0,
                                b: 0,
                                a: 200,
                            },
                            ..Default::default()
                        },
                    ),
                );
                sphere.set_transformation(
                    Mat4::from_translation(vec3(0.0, 1.3, 0.0)) * Mat4::from_scale(0.2),
                );
                

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

        if let Some(state) = sim_state {
            let uav_pose = state.uav_state.uav_state.pose;
            let uav_motors = state.uav_state.uav_state.motors;

            ui.label("UAV Pose");
            ui.label("X: ".to_string() + &uav_pose.position.x.to_string());
            ui.label("Y: ".to_string() + &uav_pose.position.y.to_string());
            ui.label("Z: ".to_string() + &uav_pose.position.z.to_string());
            ui.separator();
            ui.label("UAV Motors");
            ui.label("M1: ".to_string() + &uav_motors[0].to_string());
            ui.label("M2: ".to_string() + &uav_motors[1].to_string());
            ui.label("M3: ".to_string() + &uav_motors[2].to_string());
            ui.label("M4: ".to_string() + &uav_motors[3].to_string());
        }
    }
}
