use crate::board::{Board, Direction};
use std::{
    fmt,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SearchResult<A> {
    pub depth: u32,
    pub value: f64,
    pub action: A,
}

#[derive(Debug)]
pub enum SearchError {
    TimeOut,
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimeOut => write!(f, "reached deadline before finishing computation"),
        }
    }
}

pub struct DFS<const ROWS: usize, const COLS: usize> {
    pub player_cache: lru::LruCache<Board<ROWS, COLS>, SearchResult<Direction>>,
    pub deadline: Instant,
}

impl std::error::Error for SearchError {}

impl<const ROWS: usize, const COLS: usize> Default for DFS<ROWS, COLS> {
    fn default() -> Self {
        DFS {
            player_cache: lru::LruCache::new(1000000.try_into().unwrap()),
            deadline: Instant::now(),
        }
    }
}

impl<const ROWS: usize, const COLS: usize> DFS<ROWS, COLS> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_heuristic_sample(board: &Board<ROWS, COLS>, depth: u32, eval: f64) {
        // TODO
    }

    pub fn heuristic(board: &Board<ROWS, COLS>) -> f64 {
        2_usize.pow((board.count_empty() + 1) as u32) as f64
    }

    fn act_by_heuristic(board: &Board<ROWS, COLS>) -> SearchResult<Direction> {
        SearchResult {
            depth: 0,
            value: Self::heuristic(board),
            // action without any search
            action: Direction::Up,
        }
    }

    pub fn evaluate_for_player(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: u32,
    ) -> Result<SearchResult<Direction>, SearchError> {
        let mut result = SearchResult {
            depth: u32::MAX,
            value: 0.,
            // action on terminal states
            action: Direction::Up,
        };

        if board.is_lost() {
            return Ok(result);
        }

        if depth == 0 {
            return Ok(Self::act_by_heuristic(board));
        }

        if let Some(result) = self.player_cache.get(board) {
            if result.depth >= depth {
                return Ok(*result);
            }
        }

        for action in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let mut new_board = board.clone();
            let moved = new_board.swipe(action);

            if !moved {
                continue;
            }

            let value = self.evaluate_for_opponent(&new_board, depth - 1)?;

            if result.value < value {
                result = SearchResult {
                    depth,
                    value,
                    action,
                }
            }
        }

        self.player_cache.put(board.clone(), result);
        Ok(result)
    }

    pub fn evaluate_for_opponent(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: u32,
    ) -> Result<f64, SearchError> {
        if Instant::now() >= self.deadline {
            return Err(SearchError::TimeOut);
        }

        let mut numerator = 0.;
        let mut denominator = 0.;
        let spawns = board.spawns();

        for (new_board, weight) in spawns {
            let evaluation = self.evaluate_for_player(&new_board, depth)?.value;

            numerator += weight * evaluation;
            denominator += weight;
        }

        Ok(numerator / denominator)
    }

    pub fn evaluate_until(
        &mut self,
        board: &Board<ROWS, COLS>,
        deadline: Instant,
    ) -> SearchResult<Direction> {
        // pessimistic deadline to end early instead of late
        self.deadline = deadline - Duration::from_micros(100);

        let mut result = Self::act_by_heuristic(board);

        while let Ok(new_result) = self.evaluate_for_player(board, result.depth + 1) {
            result = new_result;
        }

        println!("{result:.2?}");
        result
    }

    pub fn act(&mut self, board: &Board<ROWS, COLS>, deadline: Instant) -> Direction {
        self.evaluate_until(board, deadline).action
    }
}
