use tiny_skia::{Color, Paint, Pixmap, Rect, Transform};

use crate::{data::grid::MapGrid, util::tri::TriState};

lazy_static::lazy_static!(
    static ref WHITE_COLOR: Color = Color::from_rgba(1.0, 1.0, 1.0, 1.0).expect("Failed to create color white.");
    static ref BLACK_COLOR: Color = Color::from_rgba(0.0, 0.0, 0.0, 1.0).expect("Failed to create color black.");
    static ref RED_COLOR: Color = Color::from_rgba(1.0, 0.0, 0.0, 1.0).expect("Failed to create color red.");
    static ref WHITE_PAINT: Paint<'static> = {
        let mut p = Paint::default();
        p.set_color(*WHITE_COLOR);
        p.anti_alias = true;

        p
    };
    static ref BLACK_PAINT: Paint<'static> = {
        let mut p = Paint::default();
        p.set_color(*BLACK_COLOR);
        p.anti_alias = true;

        p
    };
    static ref RED_PAINT: Paint<'static> = {
        let mut p = Paint::default();
        p.set_color(*RED_COLOR);
        p.anti_alias = true;

        p
    };
);

enum Group {
    Fg,
    Bg,
    Error,
}

impl Group {
    fn color(&self) -> &'static Color {
        match self {
            Group::Fg => &*WHITE_COLOR,
            Group::Bg => &*BLACK_COLOR,
            Group::Error => &*RED_COLOR,
        }
    }

    fn paint(&self) -> &'static Paint<'static> {
        match self {
            Group::Fg => &WHITE_PAINT,
            Group::Bg => &BLACK_PAINT,
            Group::Error => &RED_PAINT,
        }
    }
}

pub struct Artist;

impl Artist {
    #[allow(clippy::cast_precision_loss)]
    pub fn draw_mapgrid<S: std::fmt::Display>(
        grid: &MapGrid,
        file_name: S,
        block_size: u32,
        fg_color: (u8, u8, u8, u8),
        bg_color: (u8, u8, u8, u8),
    ) -> Result<(), String> {
        let cells = grid.dump_all_cells();
        let bsf = block_size as f32;
        let (w, h): (u32, u32) = {
            let (x, y) = grid.size().into();
            (
                x.try_into().expect("w too big for u32"),
                y.try_into().expect("h too big for u32"),
            )
        };

        let mut squares = Vec::new();
        for ((xy), cell) in cells {
            let (x, y) = xy.into();
            let (xf, yf) = (x as f32, y as f32);
            let grp = match cell.state() {
                TriState::True => Group::Fg,
                TriState::False => Group::Bg,
                TriState::Invalid => Group::Error,
            };
            if let Some(rect) = Rect::from_xywh(xf * bsf, yf * bsf, bsf, bsf) {
                squares.push((rect, grp));
            };
        }
        assert_eq!(squares.len(), (w * h) as usize, "Not all cells were drawn.");

        let mut pixmap = if let Some(p) = Pixmap::new(w * block_size, h * block_size) {
            p
        } else {
            return Err("Could not create pixmap!".to_string());
        };

        pixmap.fill(*BLACK_COLOR);

        for (rect, grp) in squares {
            pixmap.fill_rect(rect, grp.paint(), Transform::identity(), None);
        }

        pixmap
            .save_png(format!("output/{}.png", file_name))
            .map_err(|e| format!("Failed to save pixmap: {}", e))
    }
}
