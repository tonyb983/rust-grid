//! # Dungen
//!
//! My experimentation with all things procedurally generated, pathfinding, 2D grid / matrix operations, etc.
//!
//! Eventual goal is to have a library that creates "dungeons" for "games", whatever those
//! words mean to me at the moment.

#![feature(let_else, slice_group_by)]

use std::{
    char, env,
    ops::Sub,
    time::{Duration, Instant},
};

use pad::PadStr;
use pathfinding::prelude as pflib;

use dungen::{
    data::{
        size, GridPos, MapGrid, PremadeGridFiles as GridFiles, PremadeGridStrings as GridStrings,
        PremadeGrids,
    },
    draw::Artist,
    gen::{
        cell_auto::{Algorithm as CaAlgorithm, CellularAutomata},
        room_based::RoomBased,
    },
    pf::pathing::Pathfinding,
    term_menu::{run_long, run_select, run_simple, run_strnum},
    util::{math::get_curve_between, random::init_rng},
};

const FUNCTION: usize = 27usize;

fn main() {
    let args = init();

    let input = if args.is_empty() {
        FUNCTION
    } else if args[0].starts_with("h") {
        println!("{}", help());

        return;
    } else {
        match args[0].parse() {
            Ok(n) => n,
            Err(err) => {
                println!("Error parsing input: {}", err);

                FUNCTION
            }
        }
    };

    match input {
        0 => simple_artist_run(),
        2 => tiny_skia_first(),
        3 => run_ca_first_initial_fill_comparison(),
        4 => run_ca_first_param_comparison(),
        5 => check_ca_firsts(),
        6 => pf_grid(),
        7 => pathfinding_comparison(),
        8 => file_loading(),
        9 => compare_maps_and_algs(false),
        10 => basic_room_generator(),
        11 => tiered_room_generator(),
        12 => generate_various_sizes(),
        13 => curve_and_cell_auto_test(),
        14 => curves(),
        15 => json_serial_test(),
        16 => msgpack_serial_test(),
        18 => run_grid_tests(),
        19 => multiple_serial_compare(),
        20 => compare_algorithms_internal(),
        21 => print_all_maze_strings(),
        22 => run_simple(),
        23 => run_select(),
        24 => run_long(),
        25 => run_strnum(),
        26 => dungen::ansi_col::run_basic(),
        27 => dungen::ansi_col::run_build_compare(),
        _ => println!("No function associated with {}", FUNCTION),
    }
}

fn init() -> Vec<String> {
    println!("Main Starting");
    println!("Initializing logger (env_logger)");
    env_logger::init();
    println!("Initializing rng (fastrand)");
    init_rng();
    println!("Getting input args.");
    let args: Vec<String> = env::args().skip(1).collect();
    println!("Args: {:?}", args);

    args
}

fn help() -> &'static str {
    "
    Usage:
        dungen --bin runner [function]

    Functions:
        0 - Simple Artist
        2 - Tiny Skia First
        3 - Compare cellular automata first fill
        4 - Compare cellular automata first param
        5 - Compare cellular automata firsts
        6 - PF Grid
        7 - Pathfinding Comparison
        8 - File Loading
        9 - Compare maps and algorithms
        10 - Basic Room Generator
        11 - Tiered Room Generator
        12 - Generate various sizes
        13 - Curve and Cell Auto Test
        14 - Curves
        15 - JSON Serial Test
        16 - MsgPack Serial Test
        18 - Grid Tests
        19 - Multiple Serial Comparison
        20 - Compare algorithms internal
        21 - Print all maze strings
        22 - Run simple
        23 - Run select
        24 - Run long
        25 - Run strnum
        26 - ANSI Col Test
        27 - ANSI Col Build Comparison
    "
}

fn print_all_maze_strings() {
    for string in GridStrings::all() {
        println!("{}", string.get_maze().expect("Unable to get maze!"));
    }
}

fn compare_algorithms_internal() {
    let mut results = Vec::new();

    for string in GridStrings::all() {
        println!("Processing {:?}", string);

        let grid = string.get_maze().expect("Could not get maze from string");
        let (start, goal) = string
            .get_start_end()
            .expect("Unable to get start and goal.");

        println!("Running dijkstra");
        let (path, time) = timed_result(|| {
            Pathfinding::dijkstra(&grid, start, goal).expect("Unable to find path!")
        });

        results.push((format!("{:?}", &string), "dijkstra", time, path));

        println!("Running astar");
        let (path, time) =
            timed_result(|| Pathfinding::a_star(&grid, start, goal).expect("Unable to find path!"));

        results.push((format!("{:?}", &string), "astar", time, path));

        println!("Running bfs");
        let (path, time) =
            timed_result(|| Pathfinding::bfs(&grid, start, goal).expect("Unable to find path!"));

        results.push((format!("{:?}", &string), "bfs", time, path));

        println!("Running fringe");
        let (path, time) =
            timed_result(|| Pathfinding::fringe(&grid, start, goal).expect("Unable to find path!"));

        results.push((format!("{:?}", &string), "fringe", time, path));
    }

    for group in results.group_by(|a, b| a.0 == b.0) {
        let mut first = false;

        let mut fastest = ("", &Duration::MAX);

        for (name, alg, dur, _) in group {
            if !first {
                println!("{}", name);
                first = true;
            }
            if dur < fastest.1 {
                fastest = (alg, dur);
            }
            println!("\t{:<10} {:>10}", alg, format!("{:?}", dur));
        }

        println!("\nFastest: {:?} ({:?})\n", fastest.0, fastest.1);
    }
}

fn multiple_serial_compare() {
    let m1 = PremadeGrids::maze1();
    let m2 = PremadeGrids::maze2();
    let m3 = PremadeGrids::maze3();
    let m4 = PremadeGrids::maze4();
    let m5 = PremadeGrids::maze5();
    let m6 = PremadeGrids::maze6();

    let mut results = Vec::with_capacity(6);
    let maps = vec![m1, m2, m3, m4, m5, m6];

    for map in &maps {
        let (title, se, de) = serial_time_comparison(map, false);
        let json_b_size = map.to_json_bytes().expect("to_json_bytes failed").len();
        let json_size = map
            .to_json_string(false)
            .expect("to_json_string failed")
            .len();
        let m_size = map.to_msgpack().expect("to_msgpack failed").len();
        let sizes = vec![json_size, json_b_size, m_size];
        results.push((title, se, de, sizes));
    }

    for (title, se, de, sizes) in &results {
        println!("Results for {}", title);
        println!("\tSerialization Results:");
        for &(s, d) in se {
            println!("\t\t{:<20} {:?}", s, d);
        }
        println!("\tDeserialization Results:");
        for &(s, d) in de {
            println!("\t\t{:<20} {:?}", s, d);
        }
        println!("\tSerialization Sizes:");
        println!(
            "\t\t{:<20} {}\n\t\t{:<20} {}\n\t\t{:<20} {}",
            "JSON:", sizes[0], "JSON Bytes:", sizes[1], "MsgPack:", sizes[2]
        );
        println!();
    }
}

#[allow(clippy::type_complexity)]
fn serial_time_comparison(
    original: &MapGrid,
    print: bool,
) -> (String, Vec<(&str, Duration)>, Vec<(&str, Duration)>) {
    if print {
        println!("Testing serialization and deserialization times.");
    }
    // let original = PremadeGrids::maze3();
    let mut ser_results = Vec::new();
    let mut de_results = Vec::new();

    ser_results.push(timed_result(|| {
        let _unused = original.to_json().expect("MapGrid::to_json failed");
        "to_json"
    }));

    ser_results.push(timed_result(|| {
        let _unused = original
            .to_json_bytes()
            .expect("MapGrid::to_json_bytes failed");
        "to_json_bytes"
    }));

    ser_results.push(timed_result(|| {
        let _unused = original
            .to_json_string(false)
            .expect("MapGrid::to_json_string(false) failed");
        "to_json_string(false)"
    }));

    ser_results.push(timed_result(|| {
        let _unused = original
            .to_json_string(true)
            .expect("MapGrid::to_json_string(true) failed");
        "to_json_string(true)"
    }));

    ser_results.push(timed_result(|| {
        let _unused = original.to_msgpack().expect("MapGrid::to_msgpack failed");
        "to_msgpack"
    }));

    let jv = original.to_json().expect("MapGrid::to_json failed");
    let jb = original
        .to_json_bytes()
        .expect("MapGrid::to_json_bytes failed");
    let js = original
        .to_json_string(false)
        .expect("MapGrid::to_json_string(false) failed");
    // let jsp = original
    //     .to_json_string(true)
    //     .expect("MapGrid::to_json_string(true) failed");
    let mb = original.to_msgpack().expect("MapGrid::to_msgpack failed");

    // let mut j_file = tempfile::tempfile().expect("tempfile::tempfile failed");
    // write!(j_file, "{}", &js).expect("write to tempfile failed");
    // let mut m_file = tempfile::tempfile().expect("tempfile::tempfile failed");
    // m_file.write(&mb);

    de_results.push(timed_result(|| {
        let _unused = MapGrid::from_json(jv.clone()).expect("MapGrid::from_json failed");
        "from_json"
    }));

    de_results.push(timed_result(|| {
        let _unused =
            MapGrid::from_json_bytes(jb.clone()).expect("MapGrid::from_json_bytes failed");
        "from_json_bytes"
    }));

    de_results.push(timed_result(|| {
        let _unused = MapGrid::from_json_str(js.clone()).expect("MapGrid::from_json_str failed");
        "from_json_str"
    }));

    // de_results.push(timed_result(|| {
    //     MapGrid::from_json_reader(std::io::Read::by_ref(&mut j_file)).expect("MapGrid::from_json_reader failed");
    //     "from_json_reader"
    // }));

    de_results.push(timed_result(|| {
        let _unused = MapGrid::from_msgpack_ref(&mb).expect("MapGrid::from_msgpack_ref failed");
        "from_msgpack_ref"
    }));

    // de_results.push(timed_result(|| {
    //     MapGrid::from_msgpack_reader(&m_file).expect("MapGrid::from_msgpack_reader failed");
    //     "from_msgpack_reader"
    // }));

    ser_results.sort_by(|a, b| a.1.cmp(&b.1));
    de_results.sort_by(|a, b| a.1.cmp(&b.1));

    if print {
        println!("Serialization Results:");
        for (s, d) in &ser_results {
            println!("\t{:<15} {:?}", s, d);
        }
        println!();
        println!("Deserialization Results:");
        for (s, d) in &de_results {
            println!("\t{:<15} {:?}", s, d);
        }
    }

    let name = original
        .name_copy()
        .unwrap_or_else(|| "MapGrid".to_string());
    let (x, y) = original.size().into();

    (
        format!("{:?} - {}x{} ({} cells)", name, x, y, x * y),
        ser_results,
        de_results,
    )
}

fn msgpack_serial_test() {
    let original = PremadeGrids::maze3();
    println!("Got Maze #3:\n{}", original);

    let Ok(bytes) = original.to_msgpack() else {
        panic!("to_msgpack failed!");
    };

    println!("Got bytes: {:?}", bytes);
    match MapGrid::from_msgpack_ref(&bytes) {
        Ok(grid) => {
            println!("Parsed Grid:\n{}", &grid);
            println!(
                "Result {} match the Original!",
                if grid == original { "DOES" } else { "DOES NOT" }
            );
        }
        Err(err) => println!("Errors parsing bytes into msgpack: {}", err),
    };
}

fn json_serial_test() {
    let original = PremadeGrids::maze3();
    println!("Got Maze #3:\n{}", original);

    let json_s = original.to_json_string(true).unwrap_or_else(|err| {
        panic!("Failed to serialize to json string (pretty): {}", err);
    });
    println!("Got JSON String:\n{}", json_s);

    let json_v = original.to_json().unwrap_or_else(|err| {
        panic!("Failed to serialize to json value: {}", err);
    });
    println!("Got JSON Value:\n{:?}", json_v);

    let json_b = original.to_json_bytes().unwrap_or_else(|err| {
        panic!("Failed to serialize to json bytes: {}", err);
    });
    println!("Got JSON Bytes:\n{:?}", json_b);

    let back_s = MapGrid::from_json_str(json_s).unwrap_or_else(|err| {
        panic!("Failed to deserialize from json string: {}", err);
    });
    println!("Got back from JSON String:\n{}", back_s);

    let back_b = MapGrid::from_json_bytes(json_b).unwrap_or_else(|err| {
        panic!("Failed to deserialize from json bytes: {}", err);
    });
    println!("Got back from JSON Bytes:\n{}", back_b);

    let back_v = MapGrid::from_json(json_v).unwrap_or_else(|err| {
        panic!("Failed to deserialize from json value: {}", err);
    });
    println!("Got back from JSON Value:\n{}", back_v);

    println!();
    println!(
        "String result {} match the Original!",
        if back_s == original {
            "DOES"
        } else {
            "DOES NOT"
        }
    );
    println!(
        "Byte result {} match the Original!",
        if back_b == original {
            "DOES"
        } else {
            "DOES NOT"
        }
    );
    println!(
        "JsonValue result {} match the Original!",
        if back_v == original {
            "DOES"
        } else {
            "DOES NOT"
        }
    );
}

fn curve_and_cell_auto_test() {
    let map = MapGrid::reverse(&RoomBased::tiered_heuristic(size(
        fastrand::usize(75..=160),
        fastrand::usize(20..=37),
    )));
    println!("Created Map:\n{}", &map);
    let one = CellularAutomata::execute_on(&map, 1, CaAlgorithm::default_first());
    let two = CellularAutomata::execute_on(&one, 1, CaAlgorithm::default_first());
    println!("One Pass\n{}\nTwoPasses\n{}", &one, &two);
    let alt = CellularAutomata::execute_on(
        &map,
        1,
        CaAlgorithm::flex(|_, n, s| {
            let t = n + if s { 1 } else { 0 };
            t >= 4 && t != 6
        }),
    );
    println!("Alt\n{}", &alt);
}

fn curves() {
    let mut grid = MapGrid::empty((60, 30));
    let first = grid.random_cell_pos();
    let second = grid.random_cell_pos();

    let path = get_curve_between(first, second);

    println!("Generating curve from {:?} to {:?}", first, second);
    println!("Got points ({}): {:?}", path.len(), path);
    println!("Drawing points...");
    grid.set_cell_state(first.x, first.y, true);
    grid.set_cell_state(second.x, second.y, true);
    for point in path {
        grid.set_cell_state(point.0, point.1, true);
    }

    println!("Final Graph:\n{}", grid);
}

fn generate_various_sizes() {
    println!(
        "Grid 5x5 ({} Cells)\n{:?}",
        5 * 5,
        MapGrid::reverse(&MapGrid::empty((5, 5)))
    );
    println!(
        "Grid 10x10 ({} Cells)\n{:?}",
        10 * 10,
        MapGrid::reverse(&MapGrid::empty((10, 10)))
    );
    println!(
        "Grid 15x15 ({} Cells)\n{:?}",
        15 * 15,
        MapGrid::reverse(&MapGrid::empty((15, 15)))
    );
    println!(
        "Grid 20x20 ({} Cells)\n{:?}",
        20 * 20,
        MapGrid::reverse(&MapGrid::empty((20, 20)))
    );
    println!(
        "Grid 40x20 ({} Cells)\n{:?}",
        40 * 20,
        MapGrid::reverse(&MapGrid::empty((40, 20)))
    );
    println!(
        "Grid 74x37 (Full Screen Height - {} Cells)\n{:?}",
        40 * 20,
        MapGrid::reverse(&MapGrid::empty((40, 20)))
    );
    println!(
        "Grid 84x42 (Half Screen Width - {} Cells)\n{:?}",
        74 * 42,
        MapGrid::reverse(&MapGrid::empty((74, 42)))
    );
    println!(
        "Grid 120x60 ({} Cells)\n{:?}",
        120 * 60,
        MapGrid::reverse(&MapGrid::empty((120, 60)))
    );
    println!(
        "Grid 165x82 (Full Screen Width - {} Cells)\n{:?}",
        165 * 75,
        MapGrid::reverse(&MapGrid::empty((165, 75)))
    );
    println!(
        "Grid 165x38 (Full Screen Size - {} Cells)\n{:?}",
        165 * 39,
        MapGrid::reverse(&MapGrid::empty((165, 38)))
    );
}

fn basic_room_generator() {
    let grid = RoomBased::basic((60, 30).into());
    println!("Created grid:\n{}", grid);
}

fn tiered_room_generator() {
    let x = fastrand::usize(50..=100);
    let y = fastrand::usize(40..=70);
    let grid = RoomBased::tiered((x, y).into());
    println!("Created {:?} Grid:\n{}", (x, y), grid);
}

fn compare_maps_and_algs(print: bool) {
    let res1 = compare_map_strings(print);
    let res2 = compare_map_files(print);

    for (map_name, results) in res1.iter().chain(res2.iter()) {
        println!("{}", map_name);
        let fastest = results.first().unwrap().0;
        for (dur, alg) in results {
            let diff = dur.sub(fastest);
            let perc = ((diff.as_secs_f32() / fastest.as_secs_f32()) * 100.0).round();
            println!(
                "  {:<10} {:>10} (+{:<10} {:>5}%)",
                alg,
                format!("{:?}", dur),
                format!("{:?}", diff),
                perc
            );
        }
    }
}

fn compare_map_strings(print: bool) -> Vec<(String, Vec<(Duration, String)>)> {
    let mut results = Vec::new();
    for i in 0..GridStrings::count() {
        let grid_string = GridStrings::from(i + 1);
        match grid_string.get_maze() {
            Some(grid) => {
                let (start, goal) = grid_string.get_start_end().unwrap_or_else(|| {
                    panic!("Unable to get start and end from {:?}", grid_string)
                });
                let mut r = compare_algorithms(&grid, start, goal, print);
                r.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                let name = format!(
                    "{:^44}",
                    format!(
                        "{} {:?}",
                        grid.name_copy().unwrap_or(format!("{:?}", grid_string)),
                        grid.size()
                    )
                );
                results.push((name, r));
            }
            None => {
                println!("Unable to get maze from {:?}", grid_string);
            }
        }
    }

    results
}

fn compare_map_files(print: bool) -> Vec<(String, Vec<(Duration, String)>)> {
    let mut results = Vec::new();
    for i in 0..GridFiles::count() {
        let file = GridFiles::from(i + 1);
        match file.load_maze() {
            Some((map, start, goal)) => {
                let mut r = compare_algorithms(&map, start, goal, print);
                r.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                let name = format!(
                    "{:^44}",
                    format!(
                        "{} {:?}",
                        map.name_copy().unwrap_or(format!("{:?}", file)),
                        map.size()
                    )
                );
                results.push((name, r));
            }
            None => {
                println!("Error parsing map file {:?}", &file);
            }
        }
    }

    results
}

fn file_loading() {
    let map_file = std::path::Path::new("./res/mazes/Maze1.txt");
    assert!(map_file.exists(), "Map file 1 not found!");

    match MapGrid::parse_map_file(map_file) {
        Ok(grid) => println!(
            "Created MapGrid from File 1\nStart {:?} -> Goal {:?}\n{}",
            grid.1, grid.2, grid.0
        ),
        Err(ss) => println!("Error(s) parsing map file:\n{}", ss.join("\n")),
    }

    let map_file = std::path::Path::new("./res/mazes/Maze2.txt");
    assert!(map_file.exists(), "Map file 2 not found!");

    match MapGrid::parse_map_file(map_file) {
        Ok(grid) => println!(
            "Created MapGrid from File 2\nStart {:?} -> Goal {:?}\n{}",
            grid.1, grid.2, grid.0
        ),
        Err(ss) => println!("Error(s) parsing map file 2:\n{}", ss.join("\n")),
    }

    let map_file = std::path::Path::new("./res/mazes/Maze3.txt");
    assert!(map_file.exists(), "Map file 3 not found!");

    match MapGrid::parse_map_file(map_file) {
        Ok(grid) => println!(
            "Created MapGrid from File 3\nStart {:?} -> Goal {:?}\n{}",
            grid.1, grid.2, grid.0
        ),
        Err(ss) => println!("Error(s) parsing map file 3:\n{}", ss.join("\n")),
    }

    let map_file = std::path::Path::new("./res/mazes/Maze4.txt");
    assert!(map_file.exists(), "Map file 4 not found!");

    match MapGrid::parse_map_file(map_file) {
        Ok(grid) => println!(
            "Created MapGrid from File 4\nStart {:?} -> Goal {:?}\n{}",
            grid.1, grid.2, grid.0
        ),
        Err(ss) => println!("Error(s) parsing map file 4:\n{}", ss.join("\n")),
    }
}

fn pathfinding_comparison() {
    let grid1 = PremadeGrids::maze1();
    let (grid1_start, grid1_end) = PremadeGrids::maze1_start_end();
    let mut times1 = compare_algorithms(&grid1, grid1_start, grid1_end, true);
    let grid2 = PremadeGrids::maze2();
    let (grid2_start, grid2_end) = PremadeGrids::maze2_start_end();
    let mut times2 = compare_algorithms(&grid2, grid2_start, grid2_end, true);
    let grid3 = PremadeGrids::maze3();
    let (grid3_start, grid3_end) = PremadeGrids::maze3_start_end();
    let mut times3 = compare_algorithms(&grid3, grid3_start, grid3_end, true);

    times1.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    times2.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    times3.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    println!("Times for Maze 1");
    for time in times1 {
        println!("{:<10}{:?}", time.1, time.0);
    }
    println!();

    println!("Times for Maze 2");
    for time in times2 {
        println!("{:<10}{:?}", time.1, time.0);
    }
    println!();

    println!("Times for Maze 3");
    for time in times3 {
        println!("{:<10}{:?}", time.1, time.0);
    }
}

#[allow(clippy::too_many_lines)]
fn compare_algorithms(
    grid: &MapGrid,
    start: GridPos,
    goal: GridPos,
    print: bool,
) -> Vec<(Duration, String)> {
    type TimedDijkstra = (Option<(Vec<(usize, usize)>, usize)>, Duration);
    let mut times = Vec::new();
    let grid_size = grid.size();
    if print {
        println!(
            "Running grid through pathfinding comparison from {:?} -> {:?}:\n{}\n",
            start, goal, grid
        );
    }

    if print {
        println!("Calling dijkstra for {:?} to {:?}.", start, goal);
    }

    let goal_tup: (usize, usize) = goal.into();
    let (results, dur): TimedDijkstra = timed_result(|| {
        pflib::dijkstra(
            &start.into(),
            |p| {
                grid.neighbors_with_state(*p, false, false)
                    .into_iter()
                    .map(|pi| (pi, 1usize))
                    .collect::<Vec<((usize, usize), usize)>>()
            },
            |p| *p == goal.into(),
        )
    });

    if let Some((path, _)) = results {
        if print {
            println!("Path found by dijkstra in {:?}", dur);
        }
        times.push((dur, "dijkstra".to_string()));
        if print {
            println!("Creating MapGrid showing path...");
            let path_grid = map_path_to_grid(grid_size.into(), &path);
            print_grid_side_by_side("Maze", grid, "Dijkstra", &path_grid);
        }
    } else if print {
        println!("No path found by dijkstra.");
    }

    if print {
        println!("Calling astar for {:?} to {:?}.", start, goal);
    }
    let (results, dur): TimedDijkstra = timed_result(|| {
        pflib::astar(
            &start.into(),
            |p| {
                grid.neighbors_with_state(*p, false, false)
                    .into_iter()
                    .map(|pi| (pi, 1usize))
                    .collect::<Vec<((usize, usize), usize)>>()
            },
            |&(x, y)| pflib::absdiff(x, goal_tup.0) + pflib::absdiff(y, goal_tup.1),
            |&p| p == goal.into(),
        )
    });

    if let Some((path, _)) = results {
        if print {
            println!("Path found by astar in {:?}", dur);
        }
        times.push((dur, "astar".to_string()));
        if print {
            println!("Creating MapGrid showing path...");
            let path_grid = map_path_to_grid(grid_size.into(), &path);
            print_grid_side_by_side("grid", grid, "Astar", &path_grid);
        }
    } else if print {
        println!("No path found by astar.");
    }

    if print {
        println!("Calling BFS for {:?} to {:?}.", start, goal);
    }
    let (r2, dur) = timed_result(|| {
        pflib::bfs(
            &start.into(),
            |p| grid.neighbors_with_state(*p, false, false),
            |&p| p == goal.into(),
        )
    });

    if let Some(path) = r2 {
        if print {
            println!("Path found by BFS in {:?}", dur);
        }
        times.push((dur, "bfs".to_string()));
        if print {
            println!("Creating MapGrid showing path...");
            let path_grid = map_path_to_grid(grid_size.into(), &path);
            print_grid_side_by_side("grid", grid, "BFS", &path_grid);
        }
    } else if print {
        println!("No path found by BFS.");
    }

    if print {
        println!("Calling fringe for {:?} to {:?}.", start, goal);
    }
    let (results, dur): TimedDijkstra = timed_result(|| {
        pflib::fringe(
            &start.into(),
            |p| {
                grid.neighbors_with_state(*p, false, false)
                    .into_iter()
                    .map(|pi| (pi, 1usize))
                    .collect::<Vec<((usize, usize), usize)>>()
            },
            |&(x, y)| pflib::absdiff(x, goal_tup.0) + pflib::absdiff(y, goal_tup.1),
            |&p| p == goal_tup,
        )
    });

    if let Some((path, _)) = results {
        if print {
            println!("Path found by fringe in {:?}", dur);
        }
        times.push((dur, "fringe".to_string()));
        if print {
            println!("Creating MapGrid showing path...");
            let path_grid = map_path_to_grid(grid_size.into(), &path);
            print_grid_side_by_side("grid", grid, "Fringe", &path_grid);
        }
    } else if print {
        println!("No path found by fringe.");
    }

    if print {
        println!("Calling yen for {:?} to {:?}.", start, goal);
    }
    let (r3, dur) = timed_result(|| {
        pflib::yen(
            &start.into(),
            |&p| {
                grid.neighbors_with_state(p, false, false)
                    .into_iter()
                    .map(|pi| (pi, 1usize))
                    .collect::<Vec<((usize, usize), usize)>>()
            },
            |&p| p == goal_tup,
            1,
        )
    });

    if r3.is_empty() {
        if print {
            println!("No path found by yen.");
        }
    } else {
        if print {
            println!("Path found by yen in {:?}", dur);
        }
        times.push((dur, "yen".to_string()));
        if print {
            println!("Creating MapGrid showing path...");
            let path_grid = map_path_to_grid(grid_size.into(), &r3[0].0);
            print_grid_side_by_side("grid", grid, "yen", &path_grid);
        }
    }

    times
}

fn map_path_to_grid(size: (usize, usize), points: &[(usize, usize)]) -> MapGrid {
    let mut grid = MapGrid::empty(size);

    for p in points {
        grid.set_cell_state(p.0, p.1, true);
    }

    grid
}

fn pf_grid() {
    segment("PF Grid", || {
        let mut pfg = pathfinding::grid::Grid::new(20, 11);
        pfg.enable_diagonal_mode();
        println!("Created PF Grid:\n{:?}", pfg);
        for x in 3..(pfg.width - 3) {
            for y in 3..(pfg.height - 3) {
                pfg.add_vertex((x, y));
            }
        }

        //pfg.distance((1, 1), (9, 9));
        println!("Modified PF Grid:\n{:?}", pfg);

        let mid = (pfg.width / 2, pfg.height / 2);
        for xy in pfg.neighbours(mid) {
            pfg.remove_vertex(xy);
        }

        println!("Modified PF Grid:\n{:?}", pfg);

        println!("Creating MapGrid from PF Grid");
        let grid = MapGrid::from(pfg);
        println!("Created MapGrid:\n{}", grid);

        println!("Creating PF Grid from MapGrid");
        let back = grid.to_pf_grid();
        println!("Created PF Grid:\n{:?}", back);
    });

    segment("MapGrid Iterators", || {
        let mut grid = MapGrid::empty((20, 10));

        let mut count = 0;
        for cell in grid.iter() {
            if cell.is_on() {
                count += 1;
            }
        }

        println!("{} Cells are ON", count);

        for cell in grid.iter_mut() {
            cell.toggle();
        }

        count = 0;
        for cell in grid.iter() {
            if cell.is_on() {
                count += 1;
            }
        }

        println!("{} Cells are ON", count);

        let size = grid.size();
        for (pos, cell) in grid.iter_pos_mut() {
            if pos.0 == 0
                || pos.1 == 0
                || pos.0 == size.width - 1
                || pos.1 == size.height - 1
                || (pos.0 / 2 == pos.1)
                || ((size.width - pos.0) / 2 == pos.1)
            {
                cell.toggle();
            }
        }

        for (pos, cell) in grid.iter_pos() {
            if cell.is_on() {
                println!("Cell at {:?} is ON", pos);
            }
        }

        println!("{}", grid);
    });
}

fn check_ca_firsts() {
    segment("Modifying First Alg", || {
        let original = MapGrid::random_fill_percent((60, 30), 0.45);

        let first = CellularAutomata::execute_on(&original, 5, CaAlgorithm::default_first());
        let second = CellularAutomata::execute_on(
            &original,
            5,
            CaAlgorithm::flex(|_, n, s| {
                let t = n + if s { 1 } else { 0 };
                t >= 5
            }),
        );
        let third = CellularAutomata::execute_on(
            &original,
            5,
            CaAlgorithm::flex(|_, n, s| {
                let t = n + if s { 1 } else { 0 };
                !(1..5).contains(&t)
            }),
        );

        let fourth = CellularAutomata::execute_on(
            &original,
            5,
            CaAlgorithm::flex2(|_, n, n2, s| {
                let t = n + if s { 1 } else { 0 };
                t >= 5 || n2 < 1
            }),
        );

        let fifth1 = CellularAutomata::execute_on(
            &original,
            4,
            CaAlgorithm::flex2(|_, n, n2, s| {
                let t = n + if s { 1 } else { 0 };
                t >= 5 || n2 < 1 || t < 1
            }),
        );

        let fifth = CellularAutomata::execute_on(
            &fifth1,
            3,
            CaAlgorithm::flex(|_, n, s| {
                let t = n + if s { 1 } else { 0 };
                t >= 5
            }),
        );

        print_grid_side_by_side_with_fill("First", &first, "Second", &second);
        print_double_div('=', 60);
        print_grid_side_by_side_with_fill("Second", &second, "Third", &third);
        print_double_div('=', 60);
        print_grid_side_by_side_with_fill("Third", &third, "Fourth", &fourth);
        print_double_div('=', 60);
        print_grid_side_by_side_with_fill("Fifth (Int.)", &fifth1, "Fifth (Final)", &fifth);
        print_double_div('=', 60);
    });
}

fn run_ca_first_param_comparison() {
    segment("CA First Alg - Default vs Alt 4/4", || {
        let original = MapGrid::random_fill_percent((60, 30), 0.45);
        let (_, def_history) =
            CellularAutomata::execute_with_history(&original, 5, CaAlgorithm::default_first());

        let (_, alt_history) =
            CellularAutomata::execute_with_history(&original, 5, CaAlgorithm::first(4, 4));

        println!("Starting Grid");
        print_grid(&original);

        let def_history_len = def_history.len();
        let alt_history_len = alt_history.len();
        for (i, (def_step, alt_step)) in def_history.iter().zip(alt_history.iter()).enumerate() {
            print_grid_side_by_side_with_fill(
                format!("Default Step {}/{}", i + 1, def_history_len),
                def_step,
                format!("Alternate 4/4 Step {}/{}", i + 1, alt_history_len),
                alt_step,
            );
            print_double_div('-', 60);
        }
    });
}

#[allow(clippy::cast_lossless)]
fn run_ca_first_initial_fill_comparison() {
    segment("CA First Alg - Default 45% - 55%", || {
        for i in (0..10).step_by(2) {
            let fill1 = (45.0 + i as f64) / 100.0;
            let fill2 = (45.0 + 1.0 + i as f64) / 100.0;
            let original1 = MapGrid::random_fill_percent((60, 30), fill1);
            let def_final =
                CellularAutomata::execute_on(&original1, 5, CaAlgorithm::default_first());

            let original2 = MapGrid::random_fill_percent((60, 30), fill2);
            let alt_final =
                CellularAutomata::execute_on(&original2, 5, CaAlgorithm::default_first());

            print_grid_side_by_side_with_fill(
                format!("{}% Filled Final", (fill1 * 100.0).round()),
                &def_final,
                format!("{}% Filled Final", (fill2 * 100.0).round()),
                &alt_final,
            );
            print_double_div('=', 60);
        }
    });
}

#[allow(dead_code)]
fn print_div(sep: char, size: usize) {
    println!("|{div}|", div = sep.to_string().repeat(size));
}

fn print_double_div(sep: char, size: usize) {
    println!("|{div}|{div}|", div = sep.to_string().repeat(size));
}

fn print_grid(grid: &MapGrid) {
    println!("{}", grid);
}

fn print_grid_side_by_side<S1: AsRef<str>, S2: AsRef<str>>(
    first_title: S1,
    first_grid: &MapGrid,
    second_title: S2,
    second_grid: &MapGrid,
) {
    let f_strings = first_grid.to_strings();
    let s_strings = second_grid.to_strings();

    let width = if first_grid.cols() > second_grid.cols() {
        first_grid.cols()
    } else {
        second_grid.cols()
    };

    println!(
        "|{:^w$}|{:^w$}|\n|{}|{}|",
        first_title.as_ref(),
        //.pad_to_width_with_alignment(width, pad::Alignment::MiddleRight),
        second_title.as_ref(),
        // .pad_to_width_with_alignment(width, pad::Alignment::Middle),
        "-".repeat(width),
        "-".repeat(width),
        w = width,
    );
    for (frst, scd) in f_strings.iter().zip(s_strings.iter()) {
        println!("|{}|{}|", frst, scd);
    }
}

fn print_grid_side_by_side_with_fill<S1: AsRef<str>, S2: AsRef<str>>(
    first_title: S1,
    first_grid: &MapGrid,
    second_title: S2,
    second_grid: &MapGrid,
) {
    print_grid_side_by_side(first_title, first_grid, second_title, second_grid);

    let width = if first_grid.cols() > second_grid.cols() {
        first_grid.cols()
    } else {
        second_grid.cols()
    };

    println!(
        "|{}|{}|",
        format!(
            "{}% Filled",
            (first_grid.cell_state_ratio().0 * 100.0).round()
        )
        .pad_to_width_with_alignment(width, pad::Alignment::MiddleRight),
        format!(
            "{}% Filled",
            (second_grid.cell_state_ratio().0 * 100.0).round()
        )
        .pad_to_width_with_alignment(width, pad::Alignment::Middle)
    );
}

fn segment<S: AsRef<str>, F: FnMut()>(name: S, mut f: F) {
    let t = name.as_ref();
    let ts = t.chars().count();
    println!(
        "\n|  {title}\n|{title_line}\n",
        title = name.as_ref(),
        title_line = "-".repeat(ts + 4)
    );
    f();

    println!();
}

fn timed<S: AsRef<str>, F: FnMut()>(name: S, mut f: F) {
    println!("Timing {}...", name.as_ref());
    let start = Instant::now();
    f();
    let end = start.elapsed();
    println!("Execution took {:?}", end);
}

fn timed_result<R, F: FnMut() -> R>(mut f: F) -> (R, Duration) {
    let start = Instant::now();
    let res = f();
    let end = start.elapsed();

    (res, end)
}

fn simple_artist_run() {
    let grid = MapGrid::parse_string("###\n#.#\n###", '#', '.').expect("Failed to parse grid");
    println!("Created Grid:\n{}", grid);
    timed("Drawing first grid", || {
        if let Err(err) =
            Artist::draw_mapgrid(&grid, "simple_artist_run1", 50, (1, 1, 1, 1), (0, 0, 0, 1))
        {
            println!("Failed to draw grid: {}", err);
        }
    });

    let grid = MapGrid::parse_string("#.#.#\n.#.#.\n#.#.#\n.#.#.\n#.#.#", '#', '.')
        .expect("Failed to parse grid");
    println!("Created Grid:\n{}", grid);
    timed("Drawing second grid", || {
        if let Err(err) =
            Artist::draw_mapgrid(&grid, "simple_artist_run2", 50, (1, 1, 1, 1), (0, 0, 0, 1))
        {
            println!("Failed to draw grid: {}", err);
        }
    });

    let grid = MapGrid::random_fill_percent((60, 30), 0.5);
    println!("Created Grid:\n{}", grid);
    timed("Drawing third grid", || {
        if let Err(err) =
            Artist::draw_mapgrid(&grid, "simple_artist_run3", 50, (1, 1, 1, 1), (0, 0, 0, 1))
        {
            println!("Failed to draw grid: {}", err);
        }
    });
}

fn tiny_skia_first() {
    let mut pixmap = tiny_skia::Pixmap::new(100, 100).expect("Could not create pixmap");
    let black = tiny_skia::Color::from_rgba8(255, 255, 255, 255);
    let white = tiny_skia::Color::from_rgba8(0, 0, 0, 255);
    let white_paint = {
        let mut p = tiny_skia::Paint::default();
        p.set_color(white);

        p
    };

    pixmap.fill(black);
    pixmap.fill_rect(
        tiny_skia::Rect::from_xywh(10.0, 10.0, 80.0, 80.0).expect("Could not create rect"),
        &white_paint,
        tiny_skia::Transform::identity(),
        None,
    );
    pixmap.save_png("output/tiny_skia_first.png").unwrap();
}

#[allow(clippy::too_many_lines)]
fn run_grid_tests() {
    segment("Testing manual creation...", || {
        let mut grid = MapGrid::empty((3, 3));

        grid.set_cell_state(1, 0, true);
        grid.set_cell_state(1, 1, true);
        grid.set_cell_state(1, 2, true);

        println!("Created Grid:\n{}", grid);

        println!(
            "Active Neighbors for (1,1) = {}",
            grid.active_neighbor_count((1, 1), true)
        );
        println!(
            "Active Neighbors for (0,1) = {}",
            grid.active_neighbor_count((0, 1), true)
        );
    });

    segment("Testing parsing...", || {
        let grid = match MapGrid::parse_string(".#.\n.#.\n.#.", '#', '.') {
            Ok(g) => g,
            Err(errs) => {
                println!("Errors parsing grid:");
                for err in errs {
                    println!("\t{}", err);
                }
                return;
            }
        };
        println!("Created Grid:\n{}", grid);

        println!(
            "Active Neighbors for (1,1) = {}",
            grid.active_neighbor_count((1, 1), true)
        );
        println!(
            "Active Neighbors for (0,1) = {}",
            grid.active_neighbor_count((0, 1), true)
        );
    });

    segment("Testing bad parse...", || {
        let grid = match MapGrid::parse_string(".#.\n.#..\n.#.@", '#', '.') {
            Ok(g) => g,
            Err(errs) => {
                println!("Errors parsing grid:");
                for err in errs {
                    println!("\t{}", err);
                }
                return;
            }
        };
        println!("Created Grid:\n{}", grid);

        println!(
            "Active Neighbors for (1,1) = {}",
            grid.active_neighbor_count((1, 1), true)
        );
        println!(
            "Active Neighbors for (0,1) = {}",
            grid.active_neighbor_count((0, 1), true)
        );
    });

    segment("Testing grid too small", || {
        let grid = match MapGrid::parse_string(".#\n#.", '#', '.') {
            Ok(g) => g,
            Err(errs) => {
                println!("Errors parsing grid:");
                for err in errs {
                    println!("\t{}", err);
                }
                return;
            }
        };
        println!("Created Grid:\n{}", grid);

        println!(
            "Active Neighbors for (1,1) = {}",
            grid.active_neighbor_count((1, 1), true)
        );
        println!(
            "Active Neighbors for (0,1) = {}",
            grid.active_neighbor_count((0, 1), true)
        );
    });

    segment("Testing alternate chars", || {
        let grid = match MapGrid::parse_string("0101\n1010\n0101\n1010", '1', '0') {
            Ok(g) => g,
            Err(errs) => {
                println!("Errors parsing grid:");
                for err in errs {
                    println!("\t{}", err);
                }
                return;
            }
        };
        println!("Created Grid:\n{}", grid);

        for i in 0..grid.rows() {
            println!(
                "Active Neighbors for ({0},{0}) = {1}",
                i,
                grid.active_neighbor_count((i, i), true)
            );
        }
    });

    segment("Testing random grid", || {
        let grid = MapGrid::random((8, 4));
        println!("Created Grid:\n{}", grid);

        for i in 0..grid.rows() {
            println!(
                "Active Neighbors for ({},{}) = {}",
                i * 2,
                i,
                grid.active_neighbor_count((i * 2, i), true)
            );
        }
    });
}
