use lazy_static::lazy_static;
use tiny_skia::{Color, Paint, Pixmap, Rect, Transform};

use crate::{data::grid::MapGrid, util::tri::TriState};

lazy_static! {
    /// ### Const reference to the color white.
    static ref WHITE_COLOR: Color = Color::from_rgba(1.0, 1.0, 1.0, 1.0).expect("Failed to create color white.");
    /// ### Const reference to the color black.
    static ref BLACK_COLOR: Color = Color::from_rgba(0.0, 0.0, 0.0, 1.0).expect("Failed to create color black.");
    /// ### Const reference to the color red.
    static ref RED_COLOR: Color = Color::from_rgba(1.0, 0.0, 0.0, 1.0).expect("Failed to create color red.");
    /// ### Const reference to white paint.
    static ref WHITE_PAINT: Paint<'static> = {
        let mut p = Paint::default();
        p.set_color(*WHITE_COLOR);
        p.anti_alias = true;

        p
    };
    /// ### Const reference to white paint.
    static ref BLACK_PAINT: Paint<'static> = {
        let mut p = Paint::default();
        p.set_color(*BLACK_COLOR);
        p.anti_alias = true;

        p
    };
    /// ### Const reference to white paint.
    static ref RED_PAINT: Paint<'static> = {
        let mut p = Paint::default();
        p.set_color(*RED_COLOR);
        p.anti_alias = true;

        p
    };
}

enum Group {
    Fg,
    Bg,
    Error,
}

impl Group {
    /// Gets the color for this group.
    #[allow(dead_code)]
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

/// Static struct holding drawing functions.
pub struct Artist;

impl Artist {
    /// Draws a [`MapGrid`](`crate::data::MapGrid`) to a png file.
    /// 
    /// ### Arguments
    /// - `grid` - The [`MapGrid`](`crate::data::MapGrid`) to draw.
    /// - `file_name` - The name of the output file. This name will be prefixed with `output/` and suffixed with `.png`.
    /// - `block_size` - The size of each block in the grid, default would be 50.
    /// - `fg_color` - The color of the "foreground" aka any blocks that are `on`. This parameter is currently unused, using default colors isntead.
    /// - `bg_color` - The color of the "background" aka any blocks that are `off`. This parameter is currently unused, using default colors isntead.
    /// 
    /// ### Errors
    /// - Function errors if the [`PixMap`](`tiny_skia::pixmap::PixMap`) cannot be created.
    /// - Function errors if the png cannot be saved.
    /// 
    /// ### Panics
    /// - Function panics if the current size of the grid is too big to fit into a u32, necessary for the `tiny_skia` library.
    /// 
    /// ### Example(s)
    #[allow(clippy::cast_precision_loss, unused_variables)]
    pub fn draw_mapgrid<S: std::fmt::Display>(
        grid: &MapGrid,
        file_name: S,
        block_size: u32,
        fg_color: (u8, u8, u8, u8),
        bg_color: (u8, u8, u8, u8),
    ) -> Result<(), String> {
        let bsf = block_size as f32;
        let (w, h): (u32, u32) = {
            let (x, y) = grid.size().into();
            (
                x.try_into().expect("w too big for u32"),
                y.try_into().expect("h too big for u32"),
            )
        };

        let mut squares = Vec::new();
        for ((x, y), cell) in grid.iter_pos() {
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

    /// Calls [`draw_mapgrid`](`crate::draw::artist::Artist::draw_mapgrid`) with default values, drawing the
    /// [`MapGrid`](`crate::data::MapGrid`) to a png file.
    /// 
    /// ### Arguments
    /// - `grid` - The [`MapGrid`](`crate::data::MapGrid`) to draw.
    /// - `file_name` - The name of the output file. This name will be prefixed with `output/` and suffixed with `.png`.
    /// 
    /// ### Errors
    /// - Function errors if the [`PixMap`](`tiny_skia::pixmap::PixMap`) cannot be created.
    /// - Function errors if the png cannot be saved.
    /// 
    /// ### Panics
    /// - Function panics if the current size of the grid is too big to fit into a u32, necessary for the `tiny_skia` library.
    pub fn draw_mapgrid_default<S: std::fmt::Display>(grid: &MapGrid, out_file: S) -> Result<(), String> {
        Artist::draw_mapgrid(
            grid,
            out_file,
            50,
            (255, 255, 255, 255),
            (0, 0, 0, 255),
        )
    }
}
