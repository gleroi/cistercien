use ggez;
use ggez::event;
use ggez::graphics;

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let screen = graphics::screen_coordinates(ctx);
        self.pos_x = self.pos_x % screen.w + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let screen = graphics::screen_coordinates(ctx);
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [self.pos_x, 380.0],
            100.0,
            2.0,
            graphics::Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default().dest([0.0, 0.0]))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
