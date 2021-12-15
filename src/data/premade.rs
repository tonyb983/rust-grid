use std::path::Path;

use crate::{
    data::{GridPos, MapGrid},
    logging::{error, trace},
};

/// Static struct holding methods to access the premade grids / mazes.
pub struct Grids;

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

/// Enum over the premade grids that are held in const strings.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GridStrings {
    /// Invalid entry for parsing and whatnot.
    Invalid = 0,
    /// Gets the premade grid held in string #1.
    One = 1,
    /// Gets the premade grid held in string #2.
    Two = 2,
    /// Gets the premade grid held in string #3.
    Three = 3,
    /// Gets the premade grid held in string #4.
    Four = 4,
    /// Gets the premade grid held in string #5.
    Five = 5,
    /// Gets the premade grid held in string #6.
    Six = 6,
}

impl GridStrings {
    /// Gets the total number of premade grid strings.
    #[must_use]
    pub fn count() -> usize {
        6
    }

    /// Convert a [`GridStrings`] to a [`MapGrid`].
    #[must_use]
    pub fn get_maze(&self) -> Option<MapGrid> {
        match self {
            GridStrings::One => Some(Grids::maze1()),
            GridStrings::Two => Some(Grids::maze2()),
            GridStrings::Three => Some(Grids::maze3()),
            GridStrings::Four => Some(Grids::maze4()),
            GridStrings::Five => Some(Grids::maze5()),
            GridStrings::Six => Some(Grids::maze6()),
            GridStrings::Invalid => None,
        }
    }

    /// Gets the designated start and goal points for the grid indicated by [`GridStrings`].
    #[must_use]
    pub fn get_start_end(&self) -> Option<(GridPos, GridPos)> {
        match self {
            GridStrings::One => Some(Grids::maze1_start_end()),
            GridStrings::Two => Some(Grids::maze2_start_end()),
            GridStrings::Three => Some(Grids::maze3_start_end()),
            GridStrings::Four => Some(Grids::maze4_start_end()),
            GridStrings::Five => Some(Grids::maze5_start_end()),
            GridStrings::Six => Some(Grids::maze6_start_end()),
            GridStrings::Invalid => None,
        }
    }

    /// Get all [`GridStrings`].
    #[must_use] 
    pub const fn all() -> [GridStrings; 6] {
        [
            GridStrings::One,
            GridStrings::Two,
            GridStrings::Three,
            GridStrings::Four,
            GridStrings::Five,
            GridStrings::Six,
        ]
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

/// Enum over the premade grids that are held in separate "maze files".
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GridFiles {
    /// Invalid entry for parsing and whatnot.
    Invalid = 0,
    /// Gets the premade grid held in file #1.
    One = 1,
    /// Gets the premade grid held in file #2.
    Two = 2,
    /// Gets the premade grid held in file #3.
    Three = 3,
    /// Gets the premade grid held in file #4.
    Four = 4,
}

impl GridFiles {
    /// Gets the total number of premade grid files.
    #[must_use]
    pub fn count() -> usize {
        4
    }

    /// Convert a [`GridFiles`] to a [`MapGrid`], along with the start and goal positions.
    #[must_use]
    pub fn load_maze(&self) -> Option<(MapGrid, GridPos, GridPos)> {
        match self {
            GridFiles::One => Grids::file_maze1(),
            GridFiles::Two => Grids::file_maze2(),
            GridFiles::Three => Grids::file_maze3(),
            GridFiles::Four => Grids::file_maze4(),
            GridFiles::Invalid => None,
        }
    }

    /// Get all [`GridFiles`].
    #[must_use] 
    pub const fn all() -> [GridFiles; 4] {
        [
            GridFiles::One,
            GridFiles::Two,
            GridFiles::Three,
            GridFiles::Four,
        ]
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

impl Grids {
    /// convenience function to get the correct maze string from the given [`GridStrings`].
    #[must_use]
    pub fn maze_string(grid_string: &GridStrings) -> Option<MapGrid> {
        grid_string.get_maze()
    }

    /// convenience function to get the correct maze string from the given [`GridFiles`].
    #[must_use]
    pub fn maze_file(grid_file: &GridFiles) -> Option<(MapGrid, GridPos, GridPos)> {
        grid_file.load_maze()
    }

    /// ## Maze 1 
    /// Size = **(50 x 20)**
    /// 
    /// Start = **(0,0)**
    /// 
    /// End = **(49, 19)**
    /// ```
    /// .#################################################  
    /// .................................................#  
    /// ################################################.#  
    /// #................................................#  
    /// #.################################################  
    /// #.#........................................###...#  
    /// #.##.#####################################.###.#.#  
    /// #.##.##..................................#.###.#.#  
    /// #.##.##.################################.#.###.#.#  
    /// #.##.##.#.##############################.#.###.#.#  
    /// #.##.##.#................................#.###.#.#  
    /// #.##.##.##################################.###.#.#  
    /// #....##....................................###.#.#  
    /// #.##.#########################################.#.#  
    /// #.##......................................####.#.#  
    /// #.############################################.#.#  
    /// #.############################################.#.#  
    /// #.############################################.#.#  
    /// #..............................................#.#  
    /// ################################################..
    /// ```
    #[must_use]
    pub fn maze1() -> MapGrid {
        trace!("Grids::maze()");
        MapGrid::parse_string(MAZE, '#', '.').expect("Unable to parse premade maze.")
    }

    /// Gets the suggested start and end points of maze 1.
    #[must_use]
    pub fn maze1_start_end() -> (GridPos, GridPos) {
        ((0, 0).into(), (49, 19).into())
    }

    /// ## Maze 2  
    /// Size = **(80 x 15)**
    /// 
    /// Start = **(1,13)**
    /// 
    /// End = **(67, 8)**
    /// ```ignore
    /// ################################################################################
    /// #........##........##..........................#####...........................#
    /// #........##........##............######........#####.....##################....#
    /// #...##...##...##...##...##.......######........#####.....##...............#....#
    /// #...##...##...##...##...##.......######........#####.....##.#############.#....#
    /// #...##...##...##...##...##.......######........#####.....##.#.......#####.#....#
    /// #...##...##...##...##...##.......######........#####.....##.#.#####.#####.#....#
    /// #...##...##...##...##...##.......######........#####.....##.#.#####.#####.#....#
    /// #...##...##...##...##...##.......######..................##.#.#####.#####.#....#
    /// #...##...##...##...##...##.......######...######.........##.#.###########.#....#
    /// #...##...##...##...##...##.......######...######.........##.#.............#....#
    /// #...##...##...##...##...##................######.........##.###############....#
    /// #...##........##........##................######.........##....................#
    /// #...##........##........##................######.........##....................#
    /// ################################################################################
    /// ```
    #[must_use]
    pub fn maze2() -> MapGrid {
        trace!("Grids::maze2()");
        MapGrid::parse_string(MAZE2, '#', '.').expect("Unable to parse premade maze2.")
    }

    /// Gets the suggested start and end points of maze 2.
    #[must_use]
    pub fn maze2_start_end() -> (GridPos, GridPos) {
        ((1, 13).into(), (67, 8).into())
    }

    /// ## Maze 3
    /// 
    /// Size = **(73 x 23)**
    /// 
    /// Start = **(1,22)**
    /// 
    /// End = **(71, 0)**
    /// ```ignore
    /// #####################################################################...#
    /// #...#...............#...............#...........#...................#...#
    /// #...#...#########...#...#####...#########...#####...#####...#####...#...#
    /// #...............#.......#...#...........#...........#...#...#.......#...#
    /// #########...#...#########...#########...#####...#...#...#...#########...#
    /// #.......#...#...............#...........#...#...#...#...#...........#...#
    /// #...#...#############...#...#...#########...#####...#...#########...#...#
    /// #...#...............#...#...#.......#...........#...........#.......#...#
    /// #...#############...#####...#####...#...#####...#########...#...#####...#
    /// #...........#.......#...#.......#...#.......#...........#...#...........#
    /// #...#####...#####...#...#####...#...#########...#...#...#...#############
    /// #.......#.......#...#...#.......#.......#.......#...#...#.......#.......#
    /// #############...#...#...#...#########...#...#####...#...#####...#####...#
    /// #...........#...#...........#.......#...#.......#...#.......#...........#
    /// #...#####...#...#########...#####...#...#####...#####...#############...#
    /// #...#.......#...........#...........#.......#...#...#...............#...#
    /// #...#...#########...#...#####...#########...#...#...#############...#...#
    /// #...#...........#...#...#...#...#...........#...............#...#.......#
    /// #...#########...#...#...#...#####...#########...#########...#...#########
    /// #...#.......#...#...#...........#...........#...#.......#...............#
    /// #...#...#####...#####...#####...#########...#####...#...#########...#...#
    /// #...#...................#...........#...............#...............#...#
    /// #...#####################################################################
    /// ```
    #[must_use]
    pub fn maze3() -> MapGrid {
        trace!("Grids::maze3()");
        MapGrid::parse_string(MAZE3, '#', '.').expect("Unable to parse premade maze3.")
    }

    /// Gets the suggested start and end points of maze 3.
    #[must_use]
    pub fn maze3_start_end() -> (GridPos, GridPos) {
        ((1, 22).into(), (71, 0).into())
    }

    /// ## Maze 4
    /// Size = **(33 x 18)**
    /// 
    /// Start = **(0,0)**
    /// 
    /// End = **(32, 17)**
    /// ```ignore
    /// .################################
    /// ..###############...##.........##
    /// #.#................####.##.###..#
    /// #.#.##############..##..##.######
    /// #..........#######.####.##....###
    /// ##.##.##.#####.......##.#####.###
    /// ##.##.##.##.##.##.##.##.#####.###
    /// ##.##.##....##...#......#.....###
    /// ##.##.########.##.##.##.#.###.###
    /// ##.##.#.##...........##.#...#.###
    /// #####...##.#########.##.#####.###
    /// #.###.####.#########.##.......###
    /// #.###.............##.##.#####.###
    /// #.######.####.###.##.##.##.##.###
    /// #.#......###...##.##.##.##.....##
    /// #.#.#########.###.##.##.#########
    /// #....................##......#...
    /// ############################...#.
    /// ```
    #[must_use]
    pub fn maze4() -> MapGrid {
        trace!("Grids::maze4()");
        MapGrid::parse_string(MAZE4, '#', '.').expect("Unable to parse premade maze4.")
    }

    /// Gets the suggested start and end points of maze 4.
    #[must_use]
    pub fn maze4_start_end() -> (GridPos, GridPos) {
        ((0, 0).into(), (32, 17).into())
    }

    /// ## Maze 5
    /// Size = **(21 x 20)**
    /// 
    /// Start = **(10,19)**
    /// 
    /// End = **(10, 1)**
    /// ```ignore
    /// #####################
    /// #......#............#
    /// #....##############.#
    /// #....###.....######.#
    /// #....##.......#####.#
    /// #....#.........####.#
    /// #....#.........####.#
    /// #....#.........####.#
    /// #...................#
    /// #########...#########
    /// #########...#########
    /// #########...#########
    /// #########...#########
    /// #...................#
    /// #....###########....#
    /// #....###########....#
    /// #...................#
    /// #...................#
    /// #...................#
    /// #####################
    /// ```
    #[must_use]
    pub fn maze5() -> MapGrid {
        trace!("Grids::maze5()");
        MapGrid::parse_string(MAZE5, '#', '.').expect("Unable to parse premade maze5.")
    }

    /// Gets the suggested start and end points of maze 5.
    #[must_use]
    pub fn maze5_start_end() -> (GridPos, GridPos) {
        ((10, 19).into(), (10, 1).into())
    }

    /// ## Maze 6
    /// Size = **(37 x 20)**
    /// 
    /// Start = **(1,18)**
    /// 
    /// End = **(35, 7)**
    /// ```ignore
    /// #####################################
    /// #...................................#
    /// #......############...........#.....#
    /// #....####...#########.........##....#
    /// #...###.......########.......###....#
    /// #..###.........#######......####....#
    /// #..###..........#####.......#####...#
    /// #...##...........###.......#######..#
    /// #........................############
    /// #...####....#####.........###########
    /// #..######....#####..................#
    /// #..######.....#####.................#
    /// #...####.......########.........#...#
    /// #........#.##.....#######.....###...#
    /// #....#####.#####.....####....###....#
    /// #########.....####....####....###...#
    /// ####.............###...#####....##..#
    /// #.........######...#.....#####...##.#
    /// #........########................####
    /// #####################################
    /// ```
    #[must_use]
    pub fn maze6() -> MapGrid {
        trace!("Grids::maze6()");
        MapGrid::parse_string(MAZE6, '#', '.').expect("Unable to parse premade maze6.")
    }

    /// Gets the suggested start and end points of maze 6.
    #[must_use]
    pub fn maze6_start_end() -> (GridPos, GridPos) {
        trace!("Grids::maze6_start_end()");
        ((1, 18).into(), (35, 7).into())
    }

    /// ## `Vertigo`
    /// Size = **(67,46)**
    /// 
    /// Start = **(1,1)**
    /// 
    /// Goal = **(65,44)**
    /// ```ignore
    /// ###################################################################
    /// #S..#...#...#...#.##...#...#......................................#
    /// #.#...#...#...#...##.#.#.#.#.#.##################################.#
    /// #.##################.#...#...#.##...................##......#...#.#
    /// #..###.............#.##############.######.#######.###.####.#.#.#.#
    /// ##.###.#####..######...........###..######.#######..##....#.#.#.#.#
    /// #..###.####....###############.######...........#########.#.#.#.#.#
    /// #.##...###..##..####...........#####..####.####..#####....#.#.#.#.#
    /// #..#.####..#..#..###.##############..#####.#####..####.####.#.#.#.#
    /// ##.#.###..#....#..##...........###..#...........#..###....#.#.#.#.#
    /// #..#.##............###########.##..##.#.##.##.#.##..#####.#.#.#.#.#
    /// #.##.#######..######...........#..###.#..#.#..#.###..##...#.#.#.#.#
    /// #..#.##.####..####.#.###########.##.#.####.####.#.##..#.###.#.#.#.#
    /// ##.#.##............#............##.................##.#.###.#.#.#.#
    /// #..#.#######..#################..##.#.#.##.##.#.#.##..#...#.#.#.#.#
    /// #.##.#######..#####...#...#...##..###.#.##.##.#.###..####.#.#.#.#.#
    /// #..#.#####.....####.#.#.#.#.#.#.#..##.#..#.#..#.##..##....#.#.#.#.#
    /// ##.#.####..#.#..###.#.#.#.#.#.#.##..#.####.####.#..###.####.#.#.#.#
    /// #..#.###....#....##.#.#.#...#.......#....#.#....#.####....#.#.#.#.#
    /// #.##.##...#####...#.#.#.##################.##############.#.#.#.#.#
    /// #..#.##..#..#..#..#.#.#.##..............##.#..............#...#...#
    /// ##.#.###...#.#...##.#.#.##..##########..##.#.######################
    /// #..#.####........##.#.#.##..#.#.##.#.#..##.#......................#
    /// #.##.#####.....####.#.#.##..#.#.##.#.#..##.######################.#
    /// #..#.##.####.####.#.#.#.##..##########..##.#......................#
    /// ##.#.##...........#.#.#.##..............##.#.######################
    /// #..#.#######.######.#.#.##################.#......................#
    /// #.##.#######.######.#.#....................######################.#
    /// #..#.##...........#.#.####################.#......................#
    /// ##.#.##.####.####.#.#.#..#...............#.#.######################
    /// #..#.#######.######.#.##.#.###########.#.#.#......................#
    /// #.##.#........#...#.#.#..#...........#.#.#.######################.#
    /// #..#.#.######.#.#.#.#.#.############.#.#.#.#......................#
    /// ##.#.#.#....#.#.#.#.#.#..##.......##.#.#.#.#.######################
    /// #..#.#.#.##.#.#.#.#.#.##.##.#####.##.#.#.#.#......................#
    /// #.##.#.#.#..#.#.#.#.#.#..##.##....##.#.#.#.######################.#
    /// #..#.#.#.####.###.#.#.#.###.##.#####.#.#.#.#......................#
    /// ##.#.#.#..........#.#.#..##.##.......#.#.#.#.######################
    /// #..#.#.############.#.##.##.##########.#.#.#......................#
    /// #.##.#..............#.......#........#.#...######################.#
    /// #..#.###########################..###########...#...#...#...#...#.#
    /// ##.#........................................#.#...#...#...#...#...#
    /// #..########################################.#.#####################
    /// #.#...#...#...#...#...#...#...#...#...#...#.#.#...#...#...#...#...#
    /// #...#...#...#...#...#...#...#...#...#...#...#...#...#...#...#...#G#
    /// ###################################################################
    /// ``` 
    #[must_use]
    pub fn file_maze1() -> Option<(MapGrid, GridPos, GridPos)> {
        trace!("Grids::file_maze1()");
        let res = MapGrid::parse_map_file(Path::new(MAZE_FILE1));
        if res.is_err() {
            error!("Error(s) parsing Maze1.txt: {:?}", res.as_ref().err());
        }
        res.ok()
    }

    /// ## `Archon`
    /// Size = **(63, 23)**
    /// 
    /// Start = **(1,1)**
    /// 
    /// Goal = **(59,3)**
    /// ```ignore
    /// ###############################################################
    /// #S#...........................................................#
    /// #.#.#########################################################.#
    /// #.#.........................#..#...........................G#.#
    /// #.#########################.#..#.############################.#
    /// #.#.........................#..#............................#.#
    /// #.#.#########################..############################.#.#
    /// #.#.........................#..#............................#.#
    /// #.#########################.#..#.############################.#
    /// #.#.........................#..#............................#.#
    /// #.#.#########################..############################.#.#
    /// #.#.........................#..#............................#.#
    /// #.#########################.#..#.############################.#
    /// #.#.........................#..#............................#.#
    /// #.#.#########################..############################.#.#
    /// #.#.........................#..#............................#.#
    /// #.#########################.#..#.############################.#
    /// #.#.........................#..#............................#.#
    /// #.#.#########################..############################.#.#
    /// #.#.........................#..#............................#.#
    /// #.#########################.#..#.############################.#
    /// #...........................#..#..............................#
    /// ###############################################################
    /// ```
    #[must_use]
    pub fn file_maze2() -> Option<(MapGrid, GridPos, GridPos)> {
        trace!("Grids::file_maze2()");
        let res = MapGrid::parse_map_file(Path::new(MAZE_FILE2));
        if res.is_err() {
            error!("Error(s) parsing Maze2.txt: {:?}", res.as_ref().err());
        }
        res.ok()
    }

    /// ## `RedditEasy`
    /// Size = **(15,15)**
    ///
    /// Start = **(1,1)**
    ///
    /// Goal = **(13,13)**
    /// ```ignore
    /// ###############
    /// #S........#...#
    /// ###.###.###.#.#
    /// #...#...#...#.#
    /// #.#####.#####.#
    /// #.....#...#...#
    /// #.###.#.###.###
    /// #.#...#.#...#.#
    /// #.#.###.#.###.#
    /// #.#.#.#.#.#...#
    /// ###.#.#.#.#.#.#
    /// #...#...#.#.#.#
    /// #.#######.#.#.#
    /// #...........#G#
    /// ###############
    /// ```
    #[must_use]
    pub fn file_maze3() -> Option<(MapGrid, GridPos, GridPos)> {
        trace!("Grids::file_maze3()");
        let res = MapGrid::parse_map_file(Path::new(MAZE_FILE3));
        if res.is_err() {
            error!("Error(s) parsing Maze3.txt: {:?}", res.as_ref().err());
        }
        res.ok()
    }

    /// ## `RedditHard`
    /// Size = **(41,41)**
    ///
    /// Start = **(3,3)**
    ///
    /// Goal = **(37,39)**
    /// ```ignore
    /// #########################################
    /// #...#.......#.....#...........#.........#
    /// #.#.#.###.#.#.###.#.#######.###.#######.#
    /// #.#S#...#.#...#...#.#.....#...........#.#
    /// #.#####.#.#########.#.#.#############.#.#
    /// #.#.....#.#.........#.#.......#...#...#.#
    /// #.#.#####.#.#########.#####.#.#.#.#.###.#
    /// #.#.....#...#.....#.....#...#.#.#.#.#.#.#
    /// #.#####.#########.#.#####.###.#.#.#.#.#.#
    /// #...#...........#...#.....#...#.#.#...#.#
    /// #.###.#########.#.###.#####.###.#.#####.#
    /// #...#...#.....#.#.#...#.......#.#.......#
    /// #.#.###.#.###.#.###.###.#######.#######.#
    /// #.#.....#.#...#.....#...#.#.....#.....#.#
    /// #.#######.#.###########.#.#.#####.#.###.#
    /// #.....#.#.#...#.......#...#.#...#.#.....#
    /// #####.#.#####.#.#####.###.#.###.#.#######
    /// #...#.#.....#.#...#...#...#.#...#.....#.#
    /// #.###.###.###.###.#.###.###.#.#######.#.#
    /// #...#.....#...#...#.#...#...#.#.....#...#
    /// ###.#####.#.###.###.###.#.###.#.###.###.#
    /// #.......#.#...#.#.#...#.#.#...#.#.#.....#
    /// #.#######.###.#.#.###.###.#.###.#.#######
    /// #.......#...#...#...#.#...#.....#.......#
    /// #.#####.###.#####.#.#.#.#####.###.###.###
    /// #...#.#.#...#.....#.#.#.....#.#.....#...#
    /// ###.#.#.#.###.#.#####.#.###.#.#.#######.#
    /// #.#...#...#...#.#.....#...#.#.#.#.....#.#
    /// #.###.#####.###.#.#####.###.#.#.#.###.#.#
    /// #...#.......#...#.#.#...#...#.#.#...#...#
    /// #.#.#########.###.#.#.###.###.#.###.#####
    /// #.#.....#...#.#.#.#...#...#.#.#...#.....#
    /// #.#####.#.#.#.#.#.###.#.###.#.#########.#
    /// #.#...#.#.#.#.#.#...#.#...#.............#
    /// #.#.#.#.#.#.#.#.###.###.#.#############.#
    /// #.#.#.....#.#.#...#...#.#.......#.......#
    /// #.#########.#.#.#.###.###.#####.#.#######
    /// #.....#.....#.#.#...#...#.#.....#.#.....#
    /// #.###.#######.###.#.###.###.#####.#.###.#
    /// #...#.............#...#.....#.......#G..#
    /// #########################################
    /// ```
    #[must_use]
    pub fn file_maze4() -> Option<(MapGrid, GridPos, GridPos)> {
        trace!("Grids::file_maze4()");
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