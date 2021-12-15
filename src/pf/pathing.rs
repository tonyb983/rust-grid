use pathfinding::prelude::{astar, bfs, dfs, dijkstra, fringe};

use crate::{
    data::{GridPos, MapGrid},
    util::math::absdiff,
};

/// Static struct holding pathfinding functions that work with [`MapGrid`](`crate::data::MapGrid`).
pub struct Pathfinding;

impl Pathfinding {
    fn default_heuristic(first: (usize, usize), second: (usize, usize)) -> usize {
        absdiff(first.0, second.0) + absdiff(first.1, second.1)
    }

    fn default_success(current: (usize, usize), goal: (usize, usize)) -> bool {
        current == goal
    }

    /// Attempts to find a path from `start` to `goal` using the implementation of ***dijkstra's*** algorithm from
    /// the [`pathfinding`] library. If a path cannot be found, `None` is returned, otherwise a [`Vec<GridPos>`]
    /// is returned containing each point in the resulting path.
    #[must_use]
    pub fn dijkstra<P1: Into<(usize, usize)>, P2: Into<(usize, usize)>>(
        grid: &MapGrid,
        start: P1,
        goal: P2,
    ) -> Option<Vec<GridPos>> {
        let startu = start.into();
        let goalu = goal.into();
        dijkstra(
            &startu,
            |&p| {
                grid.neighbors_with_state(p, false, false)
                    .into_iter()
                    .map(|pi| (pi, 1usize))
                    .collect::<Vec<((usize, usize), usize)>>()
            },
            |&p| Self::default_success(p, goalu),
        )
        .map(|(path, _)| path.into_iter().map(std::convert::Into::into).collect())
    }

    /// Attempts to find a path from `start` to `goal` using the ***A-Star*** algorithm from the [`pathfinding`] library.
    /// If a path cannot be found, `None` is returned, otherwise a [`Vec<GridPos>`] is returned containing each point
    /// in the resulting path.
    #[must_use]
    pub fn a_star<P1: Into<(usize, usize)>, P2: Into<(usize, usize)>>(
        grid: &MapGrid,
        start: P1,
        goal: P2,
    ) -> Option<Vec<GridPos>> {
        let startu = start.into();
        let goalu = goal.into();
        astar(
            &startu,
            |&p| {
                grid.neighbors_with_state(p, false, false)
                    .into_iter()
                    .map(|pi| (pi, 1usize))
                    .collect::<Vec<((usize, usize), usize)>>()
            },
            |&xy| Self::default_heuristic(xy, goalu),
            |&p| Self::default_success(p, goalu),
        )
        .map(|(path, _)| path.into_iter().map(std::convert::Into::into).collect())
    }

    /// Attempts to find a path from `start` to `goal` using the ***BFS*** algorithm from the [`pathfinding`] library.
    /// If a path cannot be found, `None` is returned, otherwise a [`Vec<GridPos>`] is returned containing each point
    /// in the resulting path.
    #[must_use]
    pub fn bfs<P1: Into<(usize, usize)>, P2: Into<(usize, usize)>>(
        grid: &MapGrid,
        start: P1,
        goal: P2,
    ) -> Option<Vec<GridPos>> {
        let startu = start.into();
        let goalu = goal.into();
        bfs(
            &startu,
            |&p| grid.neighbors_with_state(p, false, false),
            |&p| Self::default_success(p, goalu),
        )
        .map(|path| path.into_iter().map(std::convert::Into::into).collect())
    }

    /// Attempts to find a path from `start` to `goal` using the ***DFS*** algorithm from the [`pathfinding`] library.
    /// If a path cannot be found, `None` is returned, otherwise a [`Vec<GridPos>`] is returned containing each point
    /// in the resulting path.
    #[must_use]
    pub fn dfs<P1: Into<(usize, usize)>, P2: Into<(usize, usize)>>(
        grid: &MapGrid,
        start: P1,
        goal: P2,
    ) -> Option<Vec<GridPos>> {
        let startu: (usize, usize) = start.into();
        let goalu = goal.into();
        dfs(
            startu,
            |&p| grid.neighbors_with_state(p, false, false),
            |&p| Self::default_success(p, goalu),
        )
        .map(|path| path.into_iter().map(std::convert::Into::into).collect())
    }

    /// Attempts to find a path from `start` to `goal` using the ***fringe*** algorithm from the [`pathfinding`] library.
    /// If a path cannot be found, `None` is returned, otherwise a [`Vec<GridPos>`] is returned containing each point
    /// in the resulting path.
    #[must_use]
    pub fn fringe<P1: Into<(usize, usize)>, P2: Into<(usize, usize)>>(
        grid: &MapGrid,
        start: P1,
        goal: P2,
    ) -> Option<Vec<GridPos>> {
        let startu: (usize, usize) = start.into();
        let goalu: (usize, usize) = goal.into();
        fringe(
            &startu,
            |p| {
                grid.neighbors_with_state(*p, false, false)
                    .into_iter()
                    .map(|pi| (pi, 1usize))
                    .collect::<Vec<((usize, usize), usize)>>()
            },
            |&p| Self::default_heuristic(p, goalu),
            |&p| Self::default_success(p, goalu),
        )
        .map(|(path, _)| path.into_iter().map(std::convert::Into::into).collect())
    }
}
