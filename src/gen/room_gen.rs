use std::{
    collections::HashMap,
    ops::{Div, Range},
};

use euclid::num::Round;
use log::{error, info, trace, warn};

use crate::{
    data::{GridPos, GridSize, MapGrid},
    gen::rooms::{Room, RoomSize},
    util::geo::get_curve_between,
};

/// Classification categories for maps, determined by the number of rows, columns,
/// and total number of cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GridClassification {
    /// A "TooSmall Map" is one that should only have one room. Size is 3-8x3-4 (9-36 cells).
    TooSmall = 1,
    /// A "Tiny Map" fills approx. 10% of the screen, 10x5 (50 cells), with wiggle room \[9-20\]x\[5-6\] (45-120 cells).
    Tiny = 2,
    /// A "Small Map" fills approx. 25% of the screen, 41x9 (369 cells), with wiggle room \[21-57\]x\[7-13\] (147-741 cells).
    Small = 3,
    /// A "Medium Map" fills approx. 50% of the screen, 82x18 (1476 cells), with wiggle room \[58-99\]x\[14-22\] (x812-2178 cells).
    Medium = 4,
    /// A "Large Map" fills approx. 75% of the screen, 123x28 (3444 cells), with wiggle room \[100-134\]x\[23-32\] (2400-4299 cells).
    Large = 5,
    /// A "**HUGE** Map" fills (almost) 100% of the screen, 165x37 (6105 cells), with wiggle room \[135-165\]x\[33-37\] (4300 - 6105 cells).
    Huge = 6,
    /// An "**Oversized** Map" fills more than the entire screen, 166+x37+ (6106+ cells).
    Oversized = 7,
    /// Unknown or Unitialized result, or a parsing error.
    Unknown = 0,
}

impl From<GridClassification> for usize {
    fn from(g: GridClassification) -> Self {
        match g {
            GridClassification::TooSmall => 1,
            GridClassification::Tiny => 2,
            GridClassification::Small => 3,
            GridClassification::Medium => 4,
            GridClassification::Large => 5,
            GridClassification::Huge => 6,
            GridClassification::Oversized => 7,
            GridClassification::Unknown => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassificationResult {
    rows: usize,
    rows_class: GridClassification,
    cols: usize,
    cols_class: GridClassification,
}

impl ClassificationResult {
    pub fn new<Pos: Into<GridSize>>(size: Pos) -> Self {
        let (x, y) = size.into().into();
        Self {
            rows: x,
            rows_class: GridClassification::Unknown,
            cols: y,
            cols_class: GridClassification::Unknown,
        }
    }

    pub fn classify(grid_size: GridSize) -> Self {
        let (cols, rows) = grid_size.into();
        let rows_class = GridClassification::classify_rows(rows);
        let cols_class = GridClassification::classify_cols(cols);
        Self {
            rows,
            rows_class,
            cols,
            cols_class,
        }
    }

    pub fn cells(&self) -> usize {
        self.rows * self.cols
    }

    pub fn cells_class(&self) -> GridClassification {
        let cells = self.cells();
        GridClassification::classify_cells(cells)
    }
}

impl GridClassification {
    pub fn classify_rows(rows: usize) -> Self {
        match rows {
            3 | 4 => Self::TooSmall,
            5 | 6 => Self::Tiny,
            7..=13 => Self::Small,
            14..=22 => Self::Medium,
            23..=32 => Self::Large,
            33..=39 => Self::Huge,
            40..=usize::MAX => Self::Oversized,
            _ => Self::Unknown,
        }
    }

    pub fn row_range(self) -> Range<usize> {
        match self {
            Self::TooSmall => 3..5,
            Self::Tiny => 5..7,
            Self::Small => 7..14,
            Self::Medium => 14..23,
            Self::Large => 23..33,
            Self::Huge => 33..40,
            Self::Oversized => 40..200,
            Self::Unknown => 0..0,
        }
    }

    pub fn row_max(self) -> usize {
        match self {
            Self::TooSmall => 5,
            Self::Tiny => 7,
            Self::Small => 14,
            Self::Medium => 23,
            Self::Large => 33,
            Self::Huge => 40,
            Self::Oversized => 200,
            Self::Unknown => 0,
        }
    }

    pub fn row_min(self) -> usize {
        match self {
            Self::TooSmall => 3,
            Self::Tiny => 5,
            Self::Small => 7,
            Self::Medium => 14,
            Self::Large => 23,
            Self::Huge => 33,
            Self::Oversized => 40,
            Self::Unknown => 0,
        }
    }

    pub fn classify_cols(cols: usize) -> Self {
        match cols {
            3..=8 => Self::TooSmall,
            9..=20 => Self::Tiny,
            21..=57 => Self::Small,
            58..=99 => Self::Medium,
            100..=134 => Self::Large,
            135..=165 => Self::Huge,
            166..=usize::MAX => Self::Oversized,
            _ => Self::Unknown,
        }
    }

    pub fn col_range(self) -> Range<usize> {
        match self {
            Self::TooSmall => 3..9,
            Self::Tiny => 9..21,
            Self::Small => 21..58,
            Self::Medium => 58..100,
            Self::Large => 100..135,
            Self::Huge => 135..166,
            Self::Oversized => 166..400,
            Self::Unknown => 0..0,
        }
    }

    pub fn col_max(self) -> usize {
        match self {
            Self::TooSmall => 9,
            Self::Tiny => 21,
            Self::Small => 58,
            Self::Medium => 100,
            Self::Large => 135,
            Self::Huge => 166,
            Self::Oversized => 400,
            Self::Unknown => 0,
        }
    }

    pub fn col_min(self) -> usize {
        match self {
            Self::TooSmall => 3,
            Self::Tiny => 9,
            Self::Small => 21,
            Self::Medium => 58,
            Self::Large => 100,
            Self::Huge => 135,
            Self::Oversized => 166,
            Self::Unknown => 0,
        }
    }

    pub fn classify_cells(cells: usize) -> Self {
        match cells {
            3..=44 => Self::TooSmall,
            45..=129 => Self::Tiny,
            130..=799 => Self::Small,
            800..=2399 => Self::Medium,
            2400..=4299 => Self::Large,
            4300..=6105 => Self::Huge,
            _ => Self::Oversized,
        }
    }

    pub fn cell_range(self) -> Range<usize> {
        match self {
            Self::TooSmall => 3..45,
            Self::Tiny => 45..130,
            Self::Small => 130..800,
            Self::Medium => 800..2400,
            Self::Large => 2400..4300,
            Self::Huge => 4300..6106,
            Self::Oversized => 6106..150_000,
            Self::Unknown => 0..0,
        }
    }

    pub fn cell_min(self) -> usize {
        match self {
            Self::TooSmall => 3,
            Self::Tiny => 45,
            Self::Small => 130,
            Self::Medium => 800,
            Self::Large => 2400,
            Self::Huge => 4300,
            Self::Oversized => 6106,
            Self::Unknown => 0,
        }
    }

    pub fn cell_max(self) -> usize {
        match self {
            Self::TooSmall => 45,
            Self::Tiny => 130,
            Self::Small => 800,
            Self::Medium => 2400,
            Self::Large => 4300,
            Self::Huge => 6106,
            Self::Oversized => 150_000,
            Self::Unknown => 0,
        }
    }
}

impl From<usize> for GridClassification {
    fn from(u: usize) -> Self {
        match u {
            1 => GridClassification::TooSmall,
            2 => GridClassification::Tiny,
            3 => GridClassification::Small,
            4 => GridClassification::Medium,
            5 => GridClassification::Large,
            6 => GridClassification::Huge,
            7 => GridClassification::Oversized,
            _ => GridClassification::Unknown,
        }
    }
}

pub struct RoomBasedGen;

impl RoomBasedGen {
    pub fn basic(size: GridSize) -> MapGrid {
        trace!("RoomGen::basic({:?})", size);
        let (map_width, map_height) = size.into();
        let max_rooms = 100usize;
        let width_range = 3usize..=20usize;
        let height_range = 3usize..=13usize;
        let mut rooms = Vec::new();

        warn!(
            "map_width: {}, map_height: {}, max_rooms = {}, width_range = {:?}, height_range = {:?}",
            map_width,
            map_height,
            max_rooms,
            width_range,
            height_range
        );

        for i in 0..max_rooms {
            warn!("Room generation iteration {}/{}", i + 1, max_rooms);
            let mut x = fastrand::usize(0..map_width);
            let mut y = fastrand::usize(0..map_height);
            let mut w = fastrand::usize(width_range.clone());
            let mut h = fastrand::usize(height_range.clone());
            warn!(
                "  Initial generated numbers:\nx = {}, y = {}, w = {}, h = {}",
                x, y, w, h
            );

            while x + w >= map_width {
                x -= 1;
            }

            while y + h >= map_height {
                y -= 1;
            }
            warn!(
                "  Fixed generated numbers:\nx = {}, y = {}, w = {}, h = {}",
                x, y, w, h
            );

            let room = Room::new((x, y), w, h);
            warn!("  Created room: {:?}", room);
            let mut collides = false;
            for r in &rooms {
                if room.intersects(r) {
                    warn!("  Collision detected, scrapping room.");
                    collides = true;
                    break;
                }
            }

            if !collides {
                rooms.push(room);
            }
        }

        let mut map = MapGrid::empty(size);
        warn!("Using {} rooms for generated map.", rooms.len());
        for room in rooms {
            Self::outline_room_on_grid(&room, &mut map);
        }

        map
    }

    #[allow(clippy::too_many_lines)]
    pub fn tiered(size: GridSize) -> MapGrid {
        trace!("RoomGen::tiered({:?})", size);
        let (map_width, map_height) = size.into();
        let (big_room_x, big_room_width) = {
            let size_start = (map_width / 7).max(5);
            let size_end = (map_width / 3).max(8);
            let x_start = 0;
            let x_end = map_width - size_start;
            (x_start..=x_end, size_start..=size_end)
        };
        let (big_room_y, big_room_height) = {
            let size_start = (map_height / 7).max(5);
            let size_end = (map_height / 3).max(8);
            let y_start = 0;
            let y_end = map_height - size_start;
            (y_start..=y_end, size_start..=size_end)
        };

        let (mid_room_x, mid_room_width) = {
            let size_start = (map_width / 9).max(5);
            let size_end = (map_width / 5).max(7);
            let x_start = 0;
            let x_end = map_width - size_start;
            (x_start..=x_end, size_start..=size_end)
        };
        let (mid_room_y, mid_room_height) = {
            let size_start = (map_height / 9).max(5);
            let size_end = (map_height / 5).max(7);
            let y_start = 0;
            let y_end = map_height - size_start;
            (y_start..=y_end, size_start..=size_end)
        };
        let (small_room_x, small_room_width) = {
            let size_start = 5;
            let size_end = (map_width / 7).max(6);
            let x_start = 0;
            let x_end = map_width - size_start;
            (x_start..=x_end, size_start..=size_end)
        };
        let (small_room_y, small_room_height) = {
            let size_start = 5;
            let size_end = (map_height / 7).max(6);
            let y_start = 0;
            let y_end = map_height - size_start;
            (y_start..=y_end, size_start..=size_end)
        };
        warn!("RoomGen::tiered - Generated Ranges:\nmap size = {:?}", size);
        warn!(
            "RoomGen::tiered - big_room_start = {:?} big_room_size = {:?}",
            (&big_room_x, &big_room_y),
            (&big_room_width, &big_room_height)
        );
        warn!(
            "RoomGen::tiered - mid_room_start = {:?} mid_room_size = {:?}",
            (&mid_room_x, &mid_room_y),
            (&mid_room_width, &mid_room_height)
        );
        warn!(
            "RoomGen::tiered - small_room_start = {:?} small_room_size = {:?}",
            (&small_room_x, &small_room_y),
            (&small_room_width, &small_room_height)
        );

        let mut rooms = Vec::new();

        let big_room_target = fastrand::usize(2..=4);
        let mid_room_target = fastrand::usize(3..=6);
        let small_room_target = fastrand::usize(4..=10);
        warn!(
            "RoomGen::tiered - target numbers: big = {} mid = {} small = {}",
            big_room_target, mid_room_target, small_room_target
        );

        let (mut total, mut iters) = (0usize, 0usize);
        'big_room_iter: while rooms.len() < big_room_target {
            iters += 1;
            total += 1;
            info!(
                "RoomGen::tiered - big room iteration {} (total iterations = {})",
                iters, total
            );

            let mut x = fastrand::usize(big_room_x.clone());
            let mut y = fastrand::usize(big_room_y.clone());
            let mut w = fastrand::usize(big_room_width.clone());
            let mut h = fastrand::usize(big_room_height.clone());
            info!(
                "RoomGen::tiered - big room start = {:?} size = {:?}",
                (&x, &y),
                (&w, &h)
            );

            if x + w >= map_width || y + h >= map_height {
                info!("RoomGen::tiered - big room out of bounds, scrapping room.");
                continue;
            }

            if x + w < 3 || y + h < 3 {
                info!("RoomGen::tiered - big room too small, scrapping room.");
                continue;
            }

            let room = Room::new((x, y), w, h);

            for r in &rooms {
                if room.intersects_with_buffer(r, 3) {
                    info!(
                        "RoomGen::tiered - big room collides with existing rooms, scrapping room."
                    );
                    continue 'big_room_iter;
                }
            }

            info!("RoomGen::tiered - big room acceptable, adding to list.");
            rooms.push(room);

            assert!(
                iters <= 10000,
                "Over 10000 iterations attempted during big room generation!"
            );
        }

        iters = 0;
        'mid_room_iter: while rooms.len() < mid_room_target + big_room_target {
            iters += 1;
            total += 1;
            info!(
                "RoomGen::tiered - mid room iteration {} (total iterations = {})",
                iters, total
            );

            let mut x = fastrand::usize(mid_room_x.clone());
            let mut y = fastrand::usize(mid_room_y.clone());
            let mut w = fastrand::usize(mid_room_width.clone());
            let mut h = fastrand::usize(mid_room_height.clone());
            info!(
                "RoomGen::tiered - mid room start = {:?} size = {:?}",
                (&x, &y),
                (&w, &h)
            );

            if x + w > map_width || y + h > map_height {
                info!("RoomGen::tiered - mid room out of bounds, scrapping room.");
                continue 'mid_room_iter;
            }

            if x + w < 3 || y + h < 3 {
                info!("RoomGen::tiered - mid room too small, scrapping room.");
                continue;
            }

            let room = Room::new((x, y), w, h);

            for r in &rooms {
                if room.intersects_with_buffer(r, 3) {
                    info!(
                        "RoomGen::tiered - mid room collides with existing rooms, scrapping room."
                    );
                    continue 'mid_room_iter;
                }
            }

            info!("RoomGen::tiered - mid room acceptable, adding to list.");
            rooms.push(room);

            assert!(
                iters <= 10000,
                "Over 10000 iterations attempted during mid room generation!"
            );
        }

        iters = 0;
        'small_room_iter: while rooms.len() < small_room_target + mid_room_target + big_room_target
        {
            iters += 1;
            total += 1;
            info!(
                "RoomGen::tiered - small room iteration {} (total iterations = {})",
                iters, total
            );

            let mut x = fastrand::usize(small_room_x.clone());
            let mut y = fastrand::usize(small_room_y.clone());
            let mut w = fastrand::usize(small_room_width.clone());
            let mut h = fastrand::usize(small_room_height.clone());
            info!(
                "RoomGen::tiered - small room start = {:?} size = {:?}",
                (&x, &y),
                (&w, &h)
            );

            if x + w > map_width || y + h > map_height {
                info!("RoomGen::tiered - mid room out of bounds, scrapping room.");
                continue 'small_room_iter;
            }

            if x + w < 3 || y + h < 3 {
                info!("RoomGen::tiered - small room too small, scrapping room.");
                continue;
            }

            let room = Room::new((x, y), w, h);

            for r in &rooms {
                if room.intersects_with_buffer(r, 3) {
                    info!("RoomGen::tiered - small room collides with existing rooms, scrapping room.");
                    continue 'small_room_iter;
                }
            }

            info!("RoomGen::tiered - small room acceptable, adding to list.");
            rooms.push(room);

            assert!(
                iters <= 10000,
                "Over 10000 iterations attempted during small room generation!"
            );
        }

        let mut grid = MapGrid::empty(size);
        for room in &rooms {
            Self::fill_room_on_grid(room, &mut grid);
        }

        Self::connect_all_rooms(&mut grid, &mut rooms);

        grid
    }

    fn connect_all_rooms(grid: &mut MapGrid, rooms: &mut [Room]) {
        fastrand::shuffle(rooms);
        let room_count = rooms.len();
        for room in rooms.windows(2) {
            let mut connections = 0;
            let (r1, r2) = (room[0], room[1]);
            if fastrand::u8(0..5) > 1 {
                connections += 1;
                Self::connect_rooms(grid, &r1, &r2);
            }

            for sub in room {
                let mut sub_conn = connections;
                for i in 0..=(fastrand::u8(0..3)) {
                    sub_conn += 1;
                    let random_room = &rooms[fastrand::usize(0..room_count)];
                    Self::connect_rooms(grid, sub, random_room);
                }
                if sub_conn < 1 {
                    let random_room = &rooms[fastrand::usize(0..room_count)];
                    Self::connect_rooms(grid, sub, random_room);
                }
            }
        }
    }

    fn connect_rooms(grid: &mut MapGrid, first: &Room, second: &Room) {
        let c1 = first.square().center();
        let c2 = second.square().center();

        if fastrand::u8(0..3) == 2 {
            /// 33% chance of connecting with curve
            Self::curved_path(grid, c1, c2);
        } else if fastrand::bool() {
            /// Otherwise 50-50 shot of connecting from upper left vs lower right mid point
            Self::horizontal_path(grid, c1.x, c2.x, c1.y);
            Self::vertical_path(grid, c1.y, c2.y, c2.x);
        } else {
            Self::vertical_path(grid, c1.y, c2.y, c2.x);
            Self::horizontal_path(grid, c1.x, c2.x, c1.y);
        }
    }

    fn horizontal_path(grid: &mut MapGrid, first: usize, second: usize, y: usize) {
        let start = first.min(second);
        let end = first.max(second);
        for col in start..=end {
            grid.set_cell_state(col, y, true);
        }
    }

    fn vertical_path(grid: &mut MapGrid, first: usize, second: usize, x: usize) {
        let start = first.min(second);
        let end = first.max(second);
        for row in start..=end {
            grid.set_cell_state(x, row, true);
        }
    }

    fn curved_path(grid: &mut MapGrid, first: GridPos, second: GridPos) {
        let path = get_curve_between(first, second);
        for pos in path {
            grid.set_cell_state(pos.0, pos.1, true);
        }
    }

    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::too_many_lines,
        clippy::cast_possible_truncation
    )]
    pub fn tiered_heuristic(size: GridSize) -> MapGrid {
        struct RoomDims {
            count: Range<usize>,
            pos: (Range<usize>, Range<usize>),
            size: (Range<usize>, Range<usize>),
        }
        trace!("RoomGen::tiered({:?})", size);
        let (map_width, map_height) = size.into();
        let map_cell_count = map_width * map_height;

        let grid_class = Self::classify_grid(size);
        warn!("Grid classification: {:?}", grid_class);

        let mut rooms = Vec::new();

        let mut ranges: HashMap<RoomSize, RoomDims> = HashMap::new();
        for rs in RoomSize::all_sizes() {
            let (pos, size) = Self::get_room_sizes(rs, grid_class);
            let mut count = 0;
            let mut total = 0;
            for x in size.0.clone() {
                total += x;
                count += 1;
            }
            let x_ave = total / count;
            count = 0;
            total = 0;
            for y in size.1.clone() {
                total += y;
                count += 1;
            }
            let y_ave = total / count;
            let cells = x_ave * y_ave;
            let max = ((map_cell_count as f64 / cells as f64) * 0.15).round() as usize;
            let dims = RoomDims {
                count: (max / 2)..max,
                pos: pos.into(),
                size: size.into(),
            };
            ranges.insert(rs, dims);
        }

        let huge_room_target = fastrand::usize(ranges.get(&RoomSize::Huge).unwrap().count.clone());
        let huge_room_pos = ranges.get(&RoomSize::Huge).unwrap().pos.clone();
        let huge_room_size = ranges.get(&RoomSize::Huge).unwrap().size.clone();
        warn!("huge_room target = {} pos = {:?} size = {:?}", huge_room_target, huge_room_pos, huge_room_size);

        let big_room_target = fastrand::usize(ranges.get(&RoomSize::Big).unwrap().count.clone());
        let big_room_pos = ranges.get(&RoomSize::Big).unwrap().pos.clone();
        let big_room_size = ranges.get(&RoomSize::Big).unwrap().size.clone();
        warn!("big_room target = {} pos = {:?} size = {:?}", big_room_target, big_room_pos, big_room_size);

        let mid_room_target = fastrand::usize(ranges.get(&RoomSize::Medium).unwrap().count.clone());
        let mid_room_pos = ranges.get(&RoomSize::Medium).unwrap().pos.clone();
        let mid_room_size = ranges.get(&RoomSize::Medium).unwrap().size.clone();
        warn!("mid_room target = {} pos = {:?} size = {:?}", mid_room_target, mid_room_pos, mid_room_size);

        let small_room_target =
            fastrand::usize(ranges.get(&RoomSize::Small).unwrap().count.clone());
        let small_room_pos = ranges.get(&RoomSize::Small).unwrap().pos.clone();
        let small_room_size = ranges.get(&RoomSize::Small).unwrap().size.clone();
        warn!("small_room target = {} pos = {:?} size = {:?}", small_room_target, small_room_pos, small_room_size);

        let (mut total, mut iters) = (0usize, 0usize);
        'huge_room_iter: while rooms.len() < huge_room_target {
            iters += 1;
            total += 1;
            info!(
                "RoomGen::tiered - huge room iteration {} (total iterations = {})",
                iters, total
            );

            let (size_x, size_y) = huge_room_size.clone();
            let (pos_x, pos_y) = huge_room_pos.clone();
            let mut x = fastrand::usize(pos_x);
            let mut y = fastrand::usize(pos_y);
            let mut w = fastrand::usize(size_x);
            let mut h = fastrand::usize(size_y);
            info!(
                "RoomGen::tiered - huge room start = {:?} size = {:?}",
                (&x, &y),
                (&w, &h)
            );

            if x + w >= map_width || y + h >= map_height {
                info!("RoomGen::tiered - huge room out of bounds, scrapping room.");
                continue;
            }

            if x + w < 3 || y + h < 3 {
                info!("RoomGen::tiered - huge room too small, scrapping room.");
                continue;
            }

            let room = Room::new((x, y), w, h);

            for r in &rooms {
                if room.intersects(r) {
                    info!(
                        "RoomGen::tiered - huge room collides with existing rooms, scrapping room."
                    );
                    continue 'huge_room_iter;
                }
            }

            info!("RoomGen::tiered - huge room acceptable, adding to list.");
            rooms.push(room);

            assert!(
                iters <= 10000,
                "Over 10000 iterations attempted during huge room generation!"
            );
        }

        iters = 0;
        'big_room_iter: while rooms.len() < big_room_target + huge_room_target {
            iters += 1;
            total += 1;
            info!(
                "RoomGen::tiered - big room iteration {} (total iterations = {})",
                iters, total
            );

            let (size_x, size_y) = big_room_size.clone();
            let (pos_x, pos_y) = big_room_pos.clone();
            let mut x = fastrand::usize(pos_x);
            let mut y = fastrand::usize(pos_y);
            let mut w = fastrand::usize(size_x);
            let mut h = fastrand::usize(size_y);
            info!(
                "RoomGen::tiered - big room start = {:?} size = {:?}",
                (&x, &y),
                (&w, &h)
            );

            if x + w >= map_width || y + h >= map_height {
                info!("RoomGen::tiered - big room out of bounds, scrapping room.");
                continue;
            }

            if x + w < 3 || y + h < 3 {
                info!("RoomGen::tiered - big room too small, scrapping room.");
                continue;
            }

            let room = Room::new((x, y), w, h);

            for r in &rooms {
                if room.intersects(r) {
                    info!(
                        "RoomGen::tiered - big room collides with existing rooms, scrapping room."
                    );
                    continue 'big_room_iter;
                }
            }

            info!("RoomGen::tiered - big room acceptable, adding to list.");
            rooms.push(room);

            assert!(
                iters <= 10000,
                "Over 10000 iterations attempted during big room generation!"
            );
        }

        iters = 0;
        'mid_room_iter: while rooms.len() < mid_room_target + big_room_target + huge_room_target {
            iters += 1;
            total += 1;
            info!(
                "RoomGen::tiered - mid room iteration {} (total iterations = {})",
                iters, total
            );

            let (size_x, size_y) = mid_room_size.clone();
            let (pos_x, pos_y) = mid_room_pos.clone();
            let mut x = fastrand::usize(pos_x);
            let mut y = fastrand::usize(pos_y);
            let mut w = fastrand::usize(size_x);
            let mut h = fastrand::usize(size_y);
            info!(
                "RoomGen::tiered - mid room start = {:?} size = {:?}",
                (&x, &y),
                (&w, &h)
            );

            if x + w > map_width || y + h > map_height {
                info!("RoomGen::tiered - mid room out of bounds, scrapping room.");
                continue 'mid_room_iter;
            }

            if x + w < 3 || y + h < 3 {
                info!("RoomGen::tiered - mid room too small, scrapping room.");
                continue;
            }

            let room = Room::new((x, y), w, h);

            for r in &rooms {
                if room.intersects(r) {
                    info!(
                        "RoomGen::tiered - mid room collides with existing rooms, scrapping room."
                    );
                    continue 'mid_room_iter;
                }
            }

            info!("RoomGen::tiered - mid room acceptable, adding to list.");
            rooms.push(room);

            assert!(
                iters <= 10000,
                "Over 10000 iterations attempted during mid room generation!"
            );
        }

        iters = 0;
        'small_room_iter: while rooms.len()
            < small_room_target + mid_room_target + big_room_target + huge_room_target
        {
            iters += 1;
            total += 1;
            info!(
                "RoomGen::tiered - small room iteration {} (total iterations = {})",
                iters, total
            );

            let (size_x, size_y) = small_room_size.clone();
            let (pos_x, pos_y) = small_room_pos.clone();
            let mut x = fastrand::usize(pos_x);
            let mut y = fastrand::usize(pos_y);
            let mut w = fastrand::usize(size_x);
            let mut h = fastrand::usize(size_y);
            info!(
                "RoomGen::tiered - small room start = {:?} size = {:?}",
                (&x, &y),
                (&w, &h)
            );

            if x + w > map_width || y + h > map_height {
                info!("RoomGen::tiered - mid room out of bounds, scrapping room.");
                continue 'small_room_iter;
            }

            if x + w < 3 || y + h < 3 {
                info!("RoomGen::tiered - small room too small, scrapping room.");
                continue;
            }

            let room = Room::new((x, y), w, h);

            for r in &rooms {
                if room.intersects(r) {
                    info!("RoomGen::tiered - small room collides with existing rooms, scrapping room.");
                    continue 'small_room_iter;
                }
            }

            info!("RoomGen::tiered - small room acceptable, adding to list.");
            rooms.push(room);

            assert!(
                iters <= 10000,
                "Over 10000 iterations attempted during small room generation!"
            );
        }

        let mut grid = MapGrid::empty(size);
        for room in &rooms {
            Self::fill_room_on_grid(room, &mut grid);
        }

        Self::connect_all_rooms(&mut grid, &mut rooms);

        grid
    }

    fn classify_grid(size: GridSize) -> ClassificationResult {
        ClassificationResult::classify(size)
    }

    /// Return value is ((<X-PosRange>, <Y-PosRange>), (X-SizeRange, Y-SizeRange)).
    fn get_room_sizes(size: RoomSize, class: ClassificationResult) -> (PosRange, SizeRange) {
        let (map_width, map_height) = (class.cols, class.rows);

        match size {
            RoomSize::Small => {
                let (x_pos, x_size) = {
                    let size_start = 5;
                    let size_end = (map_width / 7).max(6) + 1;
                    let x_start = 0;
                    let x_end = map_width - size_start + 1;
                    (x_start..x_end, size_start..size_end)
                };
                let (y_pos, y_size) = {
                    let size_start = 5;
                    let size_end = (map_height / 7).max(6) + 1;
                    let y_start = 0;
                    let y_end = map_height - size_start + 1;
                    (y_start..y_end, size_start..size_end)
                };

                (PosRange(x_pos, y_pos), SizeRange(x_size, y_size))
            }
            RoomSize::Medium => {
                let (x_pos, x_size) = {
                    let size_start = (map_width / 9).max(5);
                    let size_end = (map_width / 5).max(7) + 1;
                    let x_start = 0;
                    let x_end = map_width - size_start + 1;
                    (x_start..x_end, size_start..size_end)
                };

                let (y_pos, y_size) = {
                    let size_start = (map_height / 9).max(5);
                    let size_end = (map_height / 5).max(7) + 1;
                    let y_start = 0;
                    let y_end = map_height - size_start + 1;
                    (y_start..y_end, size_start..size_end)
                };

                (PosRange(x_pos, y_pos), SizeRange(x_size, y_size))
            }
            RoomSize::Big => {
                let (x_pos, x_size) = {
                    let size_start = (map_width / 7).max(5);
                    let size_end = (map_width / 3).max(8) + 1;
                    let x_start = 0;
                    let x_end = map_width - size_start + 1;
                    (x_start..x_end, size_start..size_end)
                };

                let (y_pos, y_size) = {
                    let size_start = (map_height / 7).max(5);
                    let size_end = (map_height / 3).max(8) + 1;
                    let y_start = 0;
                    let y_end = map_height - size_start + 1;
                    (y_start..y_end, size_start..size_end)
                };

                (PosRange(x_pos, y_pos), SizeRange(x_size, y_size))
            }
            RoomSize::Huge => {
                let (x_pos, x_size) = {
                    let size_start = (map_width / 5).max(8);
                    let size_end = (map_width / 2).max(15) + 1;
                    let x_start = 0;
                    let x_end = map_width - size_start + 1;
                    (x_start..x_end, size_start..size_end)
                };

                let (y_pos, y_size) = {
                    let size_start = (map_height / 5).max(8);
                    let size_end = (map_height / 2).max(15) + 1;
                    let y_start = 0;
                    let y_end = map_height - size_start + 1;
                    (y_start..y_end, size_start..size_end)
                };

                (PosRange(x_pos, y_pos), SizeRange(x_size, y_size))
            }
        }
    }

    fn outline_room_on_grid(room: &Room, grid: &mut MapGrid) {
        for (x, y) in room.get_edges() {
            grid.set_cell_state(x, y, true);
        }
    }

    fn fill_room_on_grid(room: &Room, grid: &mut MapGrid) {
        for y in room.square().y_range() {
            for x in room.square().x_range() {
                grid.set_cell_state(x, y, true);
            }
        }
    }
}

struct PosRange(Range<usize>, Range<usize>);
struct SizeRange(Range<usize>, Range<usize>);

impl From<PosRange> for (Range<usize>, Range<usize>) {
    fn from(val: PosRange) -> Self {
        (val.0, val.1)
    }
}

impl From<SizeRange> for (Range<usize>, Range<usize>) {
    fn from(val: SizeRange) -> Self {
        (val.0, val.1)
    }
}
