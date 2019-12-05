use cgmath::{EuclideanSpace, Point2, Vector2};
use ggez::graphics::{DrawMode, DrawParam, Image, Mesh, MeshBuilder, WHITE};
use ggez::{graphics, Context, GameResult};

use crate::engine;
use crate::engine::camera_handler;
use crate::engine::camera_handler::CameraHandler;
use crate::engine::shape_render::ShapeRenderer;

pub struct RenderContext<'a> {
    pub cam: &'a mut camera_handler::CameraHandler,
    ctx: &'a mut Context,
}

#[allow(dead_code)]
impl<'a> RenderContext<'a> {
    pub fn new(
        cam: &'a mut engine::camera_handler::CameraHandler,
        ctx: &'a mut Context,
    ) -> RenderContext<'a> {
        RenderContext { ctx, cam }
    }

    pub fn clear(&mut self) {
        graphics::clear(self.ctx, graphics::Color::from_rgb(0, 0, 0));
        graphics::set_window_title(
            self.ctx,
            format!("{} FPS", ggez::timer::fps(self.ctx) as i32).as_ref(),
        );
    }

    pub fn draw_text(&mut self, text: &graphics::Text, mut pos: Vector2<f32>) -> GameResult<()> {
        pos.y += text.height(self.ctx) as f32;
        let trans = graphics::DrawParam::new()
            .dest(Point2::from_vec(pos))
            .scale([1., -1.]);
        graphics::draw(self.ctx, text, trans)
    }

    pub fn draw_image<P>(&mut self, image: &Image, mut pos: Vector2<f32>) -> GameResult<()> {
        pos.y += image.height() as f32;
        let trans = graphics::DrawParam::new()
            .dest(Point2::from_vec(pos))
            .scale([1., -1.]);
        graphics::draw(self.ctx, image, trans)
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh, dp: DrawParam) -> GameResult<()> {
        graphics::draw(self.ctx, mesh, dp)
    }

    pub fn shape_render(&mut self, f: fn(&mut ShapeRenderer)) -> GameResult<()> {
        let mut rect = self.cam.get_screen_box();
        rect.scale(1.1, 1.1);
        rect.x -= 50.;
        rect.y -= 50.;
        rect.w += 100.;
        rect.h += 100.;
        let mut sr = ShapeRenderer {
            color: WHITE,
            mode: DrawMode::fill(),
            meshbuilder: MeshBuilder::new(),
            empty: true,
            screen_box: rect,
        };

        f(&mut sr);

        if sr.empty {
            return Ok(());
        }
        let mesh = sr.meshbuilder.build(self.ctx)?;
        graphics::draw(self.ctx, &mesh, DrawParam::new().dest([0.0, 0.0]))
    }
}