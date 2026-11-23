//! The startup screen: the credits on black from launch until the game
//! shows its first picture, with "Compiling shaders, please hold on: N
//! remaining" while the startup pipelines are built. The same screen as
//! WitherBlitz's NSMB-Wii-Recompilation (`settings_overlay.cpp`,
//! `DrawStartupScreen`): the same text, places, sizes and colours, in the
//! game's own fonts from the player's disc, else in egui's.

use egui::{Color32, FontId, Pos2, Shape};

use super::layout::{self, DrawList, Rgba, View};

const TITLE: &str = "Made by WitherBlitz";
const CREDIT: &str = "Code from WiiCompiled used and modified by an LLM to make this port possible.";
/// At the bottom of the screen.
const REPORT: &str = "Report any bugs you find to my github page!";

/// The screen's shapes. `aspect`: the game picture's (the layout units
/// follow its height, as the game's own screens do); `compiling`: startup
/// pipelines still being built, for the shader line.
pub fn draw(ctx: &egui::Context, aspect: f32, compiling: Option<u32>) -> Vec<Shape> {
    let screen = ctx.content_rect();
    let mut list = DrawList::default();
    list.fill_screen(ctx, screen, Rgba::from_rgba8(0x0000_00FF));
    let shader_line = compiling.map(|n| format!("Compiling shaders, please hold on: {n} remaining"));
    let title_font = layout::load_font(ctx, "mj2d01_marioFont_64_IA4.brfnt");
    let body_font = layout::load_font(ctx, "mj2d00_MessageFont_32_I4.brfnt");
    if title_font.ready() && body_font.ready() {
        // The picture's height: the window's, or less in a narrower window.
        let (w, mut h) = (screen.width(), screen.height());
        if aspect > 0.0 && w / h.max(1.0) <= aspect {
            h = w / aspect;
        }
        let view = View { center_x: screen.center().x, center_y: screen.center().y, scale: h / 456.0 };
        let max_width = 560.0f32.min(screen.width() / view.scale - 48.0);
        let lines = layout::wrap_text(&body_font, CREDIT, 26.0, max_width);
        let line_height = layout::line_height(&body_font, 26.0) + 4.0;
        let block_top = 40.0 + lines.len() as f32 * line_height / 2.0;
        layout::draw_text(&mut list, &view, &title_font, TITLE, 0.0, block_top + 38.0, 46.0, 0xFFFF_00FF, 0xFF8C_00FF, 1.0, 1, 1);
        let mut y = block_top - 16.0;
        for line in &lines {
            layout::draw_text(&mut list, &view, &body_font, line, 2.0, y - 2.0, 26.0, 0x3020_0AFF, 0x3020_0AFF, 0.6, 1, 1);
            layout::draw_text(&mut list, &view, &body_font, line, 0.0, y, 26.0, 0xFFFF_FFFF, 0xFFFF_FFFF, 1.0, 1, 1);
            y -= line_height;
        }
        if let Some(text) = &shader_line {
            layout::draw_text(&mut list, &view, &body_font, text, 0.0, y - 22.0, 20.0, 0xB4BE_CAFF, 0xB4BE_CAFF, 1.0, 1, 1);
        }
        let bottom = -(screen.height() / 2.0) / view.scale + 30.0;
        layout::draw_text(&mut list, &view, &body_font, REPORT, 2.0, bottom - 2.0, 22.0, 0x3020_0AFF, 0x3020_0AFF, 0.6, 1, 1);
        layout::draw_text(&mut list, &view, &body_font, REPORT, 0.0, bottom, 22.0, 0xFFD2_4AFF, 0xFFD2_4AFF, 1.0, 1, 1);
        return list.into_shapes();
    }
    // Without the game's fonts: plain text, laid out as the recompilation's
    // overlay font does it (13 px, the title 1.25 times that, the shader line
    // 0.9 times).
    let mut shapes = list.into_shapes();
    let (sw, sh) = (screen.width(), screen.height());
    ctx.fonts_mut(|f| {
        let title = f.layout_no_wrap(TITLE.into(), FontId::proportional(13.0 * 1.25), Color32::WHITE);
        let top = screen.top() + (sh * 0.3).max(0.0);
        shapes.push(Shape::galley(Pos2::new(screen.left() + ((sw - title.size().x) * 0.5).max(0.0), top), title.clone(), Color32::WHITE));
        let wrap = (sw - 48.0).min(640.0);
        let credit = f.layout(CREDIT.into(), FontId::proportional(13.0), Color32::WHITE, wrap);
        let credit_top = top + title.size().y * 2.0;
        shapes.push(Shape::galley(Pos2::new(screen.left() + (sw - wrap) * 0.5, credit_top), credit.clone(), Color32::WHITE));
        let report = f.layout_no_wrap(REPORT.into(), FontId::proportional(13.0), Color32::WHITE);
        shapes.push(Shape::galley(Pos2::new(screen.left() + ((sw - report.size().x) * 0.5).max(0.0), screen.bottom() - report.size().y - 24.0), report, Color32::WHITE));
        if let Some(text) = shader_line {
            let line = f.layout_no_wrap(text, FontId::proportional(13.0 * 0.9), Color32::WHITE);
            shapes.push(Shape::galley(Pos2::new(screen.left() + ((sw - line.size().x) * 0.5).max(0.0), credit_top + credit.size().y), line, Color32::WHITE));
        }
    });
    shapes
}
