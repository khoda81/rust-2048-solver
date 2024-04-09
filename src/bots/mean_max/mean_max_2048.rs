use crate::bots::heuristic::TwentyFortyEightHeuristic;
use crate::game::twenty_forty_eight::TwentyFortyEight;
use std::num::NonZeroUsize;

impl<const ROWS: usize, const COLS: usize>
    super::MeanMax<TwentyFortyEight<COLS, ROWS>, TwentyFortyEightHeuristic<COLS, ROWS>>
{
    const DEFAULT_CACHE_SIZE: usize = 10_000_000;

    pub fn new() -> Self {
        Self::new_with_cache_size(Self::DEFAULT_CACHE_SIZE.try_into().unwrap())
    }

    pub fn new_with_cache_size(capacity: NonZeroUsize) -> Self {
        Self {
            evaluation_cache: lru::LruCache::new(capacity),
            deadline: None,
            depth_limit: super::max_depth::MaxDepth::Unlimited,
            heuristic: TwentyFortyEightHeuristic::new(),
            logger: super::logger::Logger::new(),
        }
    }
}

impl<const ROWS: usize, const COLS: usize> Default
    for super::MeanMax<TwentyFortyEight<COLS, ROWS>, TwentyFortyEightHeuristic<COLS, ROWS>>
{
    fn default() -> Self {
        Self::new()
    }
}
