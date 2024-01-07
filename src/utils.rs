use std::{fs::DirEntry, path::PathBuf};

use rayon::prelude::*;
use slpprocess::{Game, GameMetadata, GameStub};

use crate::app::Filters;

/// Returns a vec containin the start and end indexes (standard range format start..end) for the slices
/// containing games from a single set.
pub fn get_sets(games: &[GameStub]) -> Vec<(usize, usize)> {
    if games.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::new();
    /* probably a better way to do this, but this is what came to mind.
        We essentially just check for membership via match ID and reset state any time membership
        changes
    */

    // TODO handle match_id.is_none()
    let mut id = games[0].match_id();
    let mut start_ind = 0;
    for i in 1..games.len() {
        if games[i].match_id() != id {
            result.push((start_ind, i));
            start_ind = i;
            id = games[i].match_id();
        }
    }

    // the last set won't have a new match id to trigger its addition, so we need to manually make sure it makes it in
    result.push((start_ind, games.len()));

    result
}

pub fn parse_entries(files: Vec<PathBuf>) -> Vec<GameStub> {
    // dbg!(files.len());
    files
        .par_iter()
        .filter_map(|file| Game::stub(&file).ok())
        .collect()
}

pub fn filter_game(game: &GameStub, filter: &Filters) -> bool {
    for player in &game.players {
        if (filter.character.is_none() || filter.character.is_some_and(|c| c == player.character))
            && (filter.display_name.is_empty()
                || player.display_name.as_ref().is_some_and(|dn| {
                    dn.to_ascii_lowercase()
                        .starts_with(&filter.display_name.to_ascii_lowercase())
                }))
            && (filter.code.is_empty()
                || player.connect_code.as_ref().is_some_and(|cc| {
                    cc.to_ascii_lowercase()
                        .starts_with(&filter.code.to_ascii_lowercase())
                }))
        {
            return true;
        }
    }

    false
}
