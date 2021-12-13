use std::{fs::File, io::Read, path::Path};

use lazy_static::lazy_static;
use log::{error, trace};

use crate::data::{MapGrid, GridPos};

pub struct PremadeGrids;

const MAZE: &str = ".#################################################\n.................................................#\n################################################.#\n#................................................#\n#.################################################\n#.#........................................###...#\n#.##.#####################################.###.#.#\n#.##.##..................................#.###.#.#\n#.##.##.################################.#.###.#.#\n#.##.##.#.##############################.#.###.#.#\n#.##.##.#................................#.###.#.#\n#.##.##.##################################.###.#.#\n#....##....................................###.#.#\n#.##.#########################################.#.#\n#.##......................................####.#.#\n#.############################################.#.#\n#.############################################.#.#\n#.############################################.#.#\n#..............................................#.#\n################################################..";
const MAZE2: &str = "################################################################################\n#........##........##..........................#####...........................#\n#........##........##............######........#####.....##################....#\n#...##...##...##...##...##.......######........#####.....##...............#....#\n#...##...##...##...##...##.......######........#####.....##.#############.#....#\n#...##...##...##...##...##.......######........#####.....##.#.......#####.#....#\n#...##...##...##...##...##.......######........#####.....##.#.#####.#####.#....#\n#...##...##...##...##...##.......######........#####.....##.#.#####.#####.#....#\n#...##...##...##...##...##.......######..................##.#.#####.#####.#....#\n#...##...##...##...##...##.......######...######.........##.#.###########.#....#\n#...##...##...##...##...##.......######...######.........##.#.............#....#\n#...##...##...##...##...##................######.........##.###############....#\n#...##........##........##................######.........##....................#\n#...##........##........##................######.........##....................#\n################################################################################";
const MAZE3: &str = "#####################################################################...#\n#...#...............#...............#...........#...................#...#\n#...#...#########...#...#####...#########...#####...#####...#####...#...#\n#...............#.......#...#...........#...........#...#...#.......#...#\n#########...#...#########...#########...#####...#...#...#...#########...#\n#.......#...#...............#...........#...#...#...#...#...........#...#\n#...#...#############...#...#...#########...#####...#...#########...#...#\n#...#...............#...#...#.......#...........#...........#.......#...#\n#...#############...#####...#####...#...#####...#########...#...#####...#\n#...........#.......#...#.......#...#.......#...........#...#...........#\n#...#####...#####...#...#####...#...#########...#...#...#...#############\n#.......#.......#...#...#.......#.......#.......#...#...#.......#.......#\n#############...#...#...#...#########...#...#####...#...#####...#####...#\n#...........#...#...........#.......#...#.......#...#.......#...........#\n#...#####...#...#########...#####...#...#####...#####...#############...#\n#...#.......#...........#...........#.......#...#...#...............#...#\n#...#...#########...#...#####...#########...#...#...#############...#...#\n#...#...........#...#...#...#...#...........#...............#...#.......#\n#...#########...#...#...#...#####...#########...#########...#...#########\n#...#.......#...#...#...........#...........#...#.......#...............#\n#...#...#####...#####...#####...#########...#####...#...#########...#...#\n#...#...................#...........#...............#...............#...#\n#...#####################################################################";
const MAZE4: &str = ".################################\n..###############...##.........##\n#.#................####.##.###..#\n#.#.##############..##..##.######\n#..........#######.####.##....###\n##.##.##.#####.......##.#####.###\n##.##.##.##.##.##.##.##.#####.###\n##.##.##....##...#......#.....###\n##.##.########.##.##.##.#.###.###\n##.##.#.##...........##.#...#.###\n#####...##.#########.##.#####.###\n#.###.####.#########.##.......###\n#.###.............##.##.#####.###\n#.######.####.###.##.##.##.##.###\n#.#......###...##.##.##.##.....##\n#.#.#########.###.##.##.#########\n#....................##......#...\n############################...#.";
const MAZE5: &str = "#####################\n#......#............#\n#....##############.#\n#....###.....######.#\n#....##.......#####.#\n#....#.........####.#\n#....#.........####.#\n#....#.........####.#\n#...................#\n#########...#########\n#########...#########\n#########...#########\n#########...#########\n#...................#\n#....###########....#\n#....###########....#\n#...................#\n#...................#\n#...................#\n#####################";
const MAZE6: &str = "#####################################\n#...................................#\n#......############...........#.....#\n#....####...#########.........##....#\n#...###.......########.......###....#\n#..###.........#######......####....#\n#..###..........#####.......#####...#\n#...##...........###.......#######..#\n#........................############\n#...####....#####.........###########\n#..######....#####..................#\n#..######.....#####.................#\n#...####.......########.........#...#\n#........#.##.....#######.....###...#\n#....#####.#####.....####....###....#\n#########.....####....####....###...#\n####.............###...#####....##..#\n#.........######...#.....#####...##.#\n#........########................####\n#####################################";

const MAZE_FILE1: &str = ".\\res\\mazes\\Maze1.txt";
const MAZE_FILE2: &str = ".\\res\\mazes\\Maze2.txt";
const MAZE_FILE3: &str = ".\\res\\mazes\\Maze3.txt";
const MAZE_FILE4: &str = ".\\res\\mazes\\Maze4.txt";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GridStrings {
    Invalid = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl GridStrings {
    pub fn count() -> usize {
        6
    }

    pub fn get_maze(&self) -> Option<MapGrid> {
        match self {
            GridStrings::One => Some(PremadeGrids::maze1()),
            GridStrings::Two => Some(PremadeGrids::maze2()),
            GridStrings::Three => Some(PremadeGrids::maze3()),
            GridStrings::Four => Some(PremadeGrids::maze4()),
            GridStrings::Five => Some(PremadeGrids::maze5()),
            GridStrings::Six => Some(PremadeGrids::maze6()),
            GridStrings::Invalid => None,
        }
    }

    pub fn get_start_end(&self) -> Option<(GridPos, GridPos)> {
        match self {
            GridStrings::One => Some(PremadeGrids::maze1_start_end()),
            GridStrings::Two => Some(PremadeGrids::maze2_start_end()),
            GridStrings::Three => Some(PremadeGrids::maze3_start_end()),
            GridStrings::Four => Some(PremadeGrids::maze4_start_end()),
            GridStrings::Five => Some(PremadeGrids::maze5_start_end()),
            GridStrings::Six => Some(PremadeGrids::maze6_start_end()),
            GridStrings::Invalid => None,
        }
    }
}

impl From<usize> for GridStrings {
    fn from(value: usize) -> Self {
        match value {
            1 => GridStrings::One,
            2 => GridStrings::Two,
            3 => GridStrings::Three,
            4 => GridStrings::Four,
            5 => GridStrings::Five,
            6 => GridStrings::Six,
            _ => GridStrings::Invalid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GridFiles {
    Invalid = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

impl GridFiles {
    pub fn count() -> usize {
        4
    }

    pub fn load_maze(&self) -> Option<(MapGrid, GridPos, GridPos)> {
        match self {
            GridFiles::One => PremadeGrids::file_maze1(),
            GridFiles::Two => PremadeGrids::file_maze2(),
            GridFiles::Three => PremadeGrids::file_maze3(),
            GridFiles::Four => PremadeGrids::file_maze4(),
            GridFiles::Invalid => None,
        }
    }
}

impl From<usize> for GridFiles {
    fn from(value: usize) -> Self {
        match value {
            1 => GridFiles::One,
            2 => GridFiles::Two,
            3 => GridFiles::Three,
            4 => GridFiles::Four,
            _ => GridFiles::Invalid,
        }
    }
}

impl PremadeGrids {
    pub fn maze_string(grid_string: &GridStrings) -> Option<MapGrid> {
        grid_string.get_maze()
    }

    pub fn maze_file(grid_file: &GridFiles) -> Option<(MapGrid, GridPos, GridPos)> {
        grid_file.load_maze()
    }

    /// Create the first premade maze.
    /// 
    /// #### Size: (50, 20)
    /// 
    /// #### Start: (0, 0)
    /// 
    /// #### Goal: (49, 19)
    pub fn maze1() -> MapGrid {
        trace!("PremadeGrids::maze()");
        MapGrid::parse_string(MAZE, '#', '.').expect("Unable to parse premade maze.")
    }

    /// Gets the suggested start and end points of maze 1.
    pub fn maze1_start_end() -> (GridPos, GridPos) {
        ((0,0).into(), (49, 19).into())
    }

    /// Create the second premade maze.
    /// 
    /// #### Size: (80, 15)
    /// 
    /// #### Start: (1, 13)
    /// 
    /// #### Goal: (67, 8)
    pub fn maze2() -> MapGrid {
        trace!("PremadeGrids::maze2()");
        MapGrid::parse_string(MAZE2, '#', '.').expect("Unable to parse premade maze2.")
    }

    /// Gets the suggested start and end points of maze 2.
    pub fn maze2_start_end() -> (GridPos, GridPos) {
        ((1,13).into(), (67, 8).into())
    }

    /// Create the third premade maze.
    /// 
    /// #### Size: (73, 23)
    /// 
    /// #### Start: (1, 22)
    /// 
    /// #### Goal: (71, 0)
    pub fn maze3() -> MapGrid {
        trace!("PremadeGrids::maze3()");
        MapGrid::parse_string(MAZE3, '#', '.').expect("Unable to parse premade maze3.")
    }

    /// Gets the suggested start and end points of maze 3.
    pub fn maze3_start_end() -> (GridPos, GridPos) {
        ((1,22).into(), (71, 0).into())
    }

    /// Create the fourth premade maze.
    /// 
    /// #### Size: (33, 18)
    /// 
    /// #### Start: (0, 0)
    /// 
    /// #### Goal: (32, 17)
    pub fn maze4() -> MapGrid {
        trace!("PremadeGrids::maze4()");
        MapGrid::parse_string(MAZE4, '#', '.').expect("Unable to parse premade maze4.")
    }

    /// Gets the suggested start and end points of maze 4.
    pub fn maze4_start_end() -> (GridPos, GridPos) {
        ((0,0).into(), (32, 17).into())
    }

    /// Create the fifth premade maze.
    /// 
    /// #### Size: (21, 20)
    /// 
    /// #### Start: (10, 19)
    /// 
    /// #### Goal: (10, 1)
    pub fn maze5() -> MapGrid {
        trace!("PremadeGrids::maze5()");
        MapGrid::parse_string(MAZE5, '#', '.').expect("Unable to parse premade maze5.")
    }

    /// Gets the suggested start and end points of maze 5.
    pub fn maze5_start_end() -> (GridPos, GridPos) {
        ((10,19).into(), (10, 1).into())
    }

    /// Create the sixth premade maze.
    /// 
    /// #### Size: (21, 20)
    /// 
    /// #### Start: (10, 19)
    /// 
    /// #### Goal: (10, 1)
    pub fn maze6() -> MapGrid {
        trace!("PremadeGrids::maze6()");
        MapGrid::parse_string(MAZE6, '#', '.').expect("Unable to parse premade maze6.")
    }

    /// Gets the suggested start and end points of maze 6.
    pub fn maze6_start_end() -> (GridPos, GridPos) {
        trace!("PremadeGrids::maze6_start_end()");
        ((1,18).into(), (35, 7).into())
    }

    pub fn file_maze1() -> Option<(MapGrid, GridPos, GridPos)> {
        trace!("PremadeGrids::file_maze1()");
        let res = MapGrid::parse_map_file(Path::new(MAZE_FILE1));
        if res.is_err() {
            error!("Error(s) parsing Maze1.txt: {:?}", res.as_ref().err());
        }
        res.ok()
    }

    pub fn file_maze2() -> Option<(MapGrid, GridPos, GridPos)> {
        trace!("PremadeGrids::file_maze2()");
        let res = MapGrid::parse_map_file(Path::new(MAZE_FILE2));
        if res.is_err() {
            error!("Error(s) parsing Maze2.txt: {:?}", res.as_ref().err());
        }
        res.ok()
    }

    pub fn file_maze3() -> Option<(MapGrid, GridPos, GridPos)> {
        trace!("PremadeGrids::file_maze3()");
        let res = MapGrid::parse_map_file(Path::new(MAZE_FILE3));
        if res.is_err() {
            error!("Error(s) parsing Maze3.txt: {:?}", res.as_ref().err());
        }
        res.ok()
    }

    pub fn file_maze4() -> Option<(MapGrid, GridPos, GridPos)> {
        trace!("PremadeGrids::file_maze4()");
        let res = MapGrid::parse_map_file(Path::new(MAZE_FILE4));
        if res.is_err() {
            error!("Error(s) parsing Maze4.txt: {:?}", res.as_ref().err());
        }
        res.ok()
    }
}

// Maze 1 Size = (50 x 20) Start = (0,0) End = (49, 19)
// .#################################################
// .................................................#
// ################################################.#
// #................................................#
// #.################################################
// #.#........................................###...#
// #.##.#####################################.###.#.#
// #.##.##..................................#.###.#.#
// #.##.##.################################.#.###.#.#
// #.##.##.#.##############################.#.###.#.#
// #.##.##.#................................#.###.#.#
// #.##.##.##################################.###.#.#
// #....##....................................###.#.#
// #.##.#########################################.#.#
// #.##......................................####.#.#
// #.############################################.#.#
// #.############################################.#.#
// #.############################################.#.#
// #..............................................#.#
// ################################################..

// Maze 2 Size = (80 x 15) Start = (1,13) End = (67, 8)
// ################################################################################
// #........##........##..........................#####...........................#
// #........##........##............######........#####.....##################....#
// #...##...##...##...##...##.......######........#####.....##...............#....#
// #...##...##...##...##...##.......######........#####.....##.#############.#....#
// #...##...##...##...##...##.......######........#####.....##.#.......#####.#....#
// #...##...##...##...##...##.......######........#####.....##.#.#####.#####.#....#
// #...##...##...##...##...##.......######........#####.....##.#.#####.#####.#....#
// #...##...##...##...##...##.......######..................##.#.#####.#####.#....#
// #...##...##...##...##...##.......######...######.........##.#.###########.#....#
// #...##...##...##...##...##.......######...######.........##.#.............#....#
// #...##...##...##...##...##................######.........##.###############....#
// #...##........##........##................######.........##....................#
// #...##........##........##................######.........##....................#
// ################################################################################

// Maze 3 Size = (73 x 23) Start = (1,22) End = (71, 0)
// #####################################################################...#
// #...#...............#...............#...........#...................#...#
// #...#...#########...#...#####...#########...#####...#####...#####...#...#
// #...............#.......#...#...........#...........#...#...#.......#...#
// #########...#...#########...#########...#####...#...#...#...#########...#
// #.......#...#...............#...........#...#...#...#...#...........#...#
// #...#...#############...#...#...#########...#####...#...#########...#...#
// #...#...............#...#...#.......#...........#...........#.......#...#
// #...#############...#####...#####...#...#####...#########...#...#####...#
// #...........#.......#...#.......#...#.......#...........#...#...........#
// #...#####...#####...#...#####...#...#########...#...#...#...#############
// #.......#.......#...#...#.......#.......#.......#...#...#.......#.......#
// #############...#...#...#...#########...#...#####...#...#####...#####...#
// #...........#...#...........#.......#...#.......#...#.......#...........#
// #...#####...#...#########...#####...#...#####...#####...#############...#
// #...#.......#...........#...........#.......#...#...#...............#...#
// #...#...#########...#...#####...#########...#...#...#############...#...#
// #...#...........#...#...#...#...#...........#...............#...#.......#
// #...#########...#...#...#...#####...#########...#########...#...#########
// #...#.......#...#...#...........#...........#...#.......#...............#
// #...#...#####...#####...#####...#########...#####...#...#########...#...#
// #...#...................#...........#...............#...............#...#
// #...#####################################################################

// Maze 4 Size = (33 x 18) Start = (0,0) End = (32, 17)
// .################################
// ..###############...##.........##
// #.#................####.##.###..#
// #.#.##############..##..##.######
// #..........#######.####.##....###
// ##.##.##.#####.......##.#####.###
// ##.##.##.##.##.##.##.##.#####.###
// ##.##.##....##...#......#.....###
// ##.##.########.##.##.##.#.###.###
// ##.##.#.##...........##.#...#.###
// #####...##.#########.##.#####.###
// #.###.####.#########.##.......###
// #.###.............##.##.#####.###
// #.######.####.###.##.##.##.##.###
// #.#......###...##.##.##.##.....##
// #.#.#########.###.##.##.#########
// #....................##......#...
// ############################...#.

// Maze 5 Size = (21 x 20) Start = (10,19) End = (10, 1)
// #####################
// #......#............#
// #....##############.#
// #....###.....######.#
// #....##.......#####.#
// #....#.........####.#
// #....#.........####.#
// #....#.........####.#
// #...................#
// #########...#########
// #########...#########
// #########...#########
// #########...#########
// #...................#
// #....###########....#
// #....###########....#
// #...................#
// #...................#
// #...................#
// #####################

// Maze 6 Size = (37 x 20) Start = (1,18) End = (35, 7)
// #####################################
// #...................................#
// #......############...........#.....#
// #....####...#########.........##....#
// #...###.......########.......###....#
// #..###.........#######......####....#
// #..###..........#####.......#####...#
// #...##...........###.......#######..#
// #........................############
// #...####....#####.........###########
// #..######....#####..................#
// #..######.....#####.................#
// #...####.......########.........#...#
// #........#.##.....#######.....###...#
// #....#####.#####.....####....###....#
// #########.....####....####....###...#
// ####.............###...#####....##..#
// #.........######...#.....#####...##.#
// #........########................####
// #####################################
