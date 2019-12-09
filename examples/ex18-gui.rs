rltk::add_wasm_support!();

use rltk::{Console, GameState, Rltk, element};
use rltk::gui::*;

struct State {
    pub ui : Option<TextUI>
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if let Some(ui) = &mut self.ui {

            let fps_id = *ui.get_id("fps").unwrap();
            if let Some(fps) = element!(ui, fps_id, StatusBarText) {
                fps.set_text(format!("FPS: {}", ctx.fps));
            }
            let ft_id = *ui.get_id("frametime").unwrap();
            if let Some(ft) = element!(ui, ft_id, StatusBarText) {
                ft.set_text(format!("Frame Time: {}", ctx.frame_time_ms));
            }

            ui.render(ctx);
        } else {
            let mut ui = TextUI::new(Theme::turbo_vision());
            ui
                .add(ctx, WidgetType::ScreenBackground, "background", "")
                .set_base("background")
                .add(ctx, WidgetType::StatusBar, "statusbar", "background")
                .add(ctx, WidgetType::StatusText{text : "FPS: 00".to_string()}, "fps", "statusbar")
                .add(ctx, WidgetType::StatusText{text : "Frame Time: 00".to_string()}, "frametime", "statusbar");
            self.ui = Some(ui);
        }
    }    
}

impl State {
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "Hello GUI", "resources");

    let gs: State = State { ui : None };
    rltk::main_loop(context, gs);
}
