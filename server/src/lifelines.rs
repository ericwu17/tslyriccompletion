use rand::prelude::*;
use serde::Serialize;

/// An enum representing one of three possible lifelines.
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lifeline {
    ShowTitleAlbum,
    ShowPrevLines,
    Skip,
}

impl Lifeline {
    pub fn random_lifeline() -> Self {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..3) {
            0 => Lifeline::ShowTitleAlbum,
            1 => Lifeline::ShowPrevLines,
            _ => Lifeline::Skip,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Lifeline::ShowPrevLines => "ShowPrevLines".to_owned(),
            Lifeline::ShowTitleAlbum => "ShowTitleAlbum".to_owned(),
            Lifeline::Skip => "Skip".to_owned(),
        }
    }
}

/// A struct representing how many of each lifeline a player has available.
#[derive(Serialize, Clone, Debug)]
pub struct LifelineInventory {
    show_title_album: i32,
    show_prev_lines: i32,
    skip: i32,
}

impl LifelineInventory {
    pub fn new() -> LifelineInventory {
        LifelineInventory {
            show_title_album: 1,
            show_prev_lines: 1,
            skip: 1,
        }
    }

    fn count_mut(&mut self, lifeline: Lifeline) -> &mut i32 {
        match lifeline {
            Lifeline::ShowTitleAlbum => &mut self.show_title_album,
            Lifeline::ShowPrevLines => &mut self.show_prev_lines,
            Lifeline::Skip => &mut self.skip,
        }
    }

    /// Uses up one of `lifeline`. Returns false (and does nothing) if none are left.
    pub fn consume_lifeline(&mut self, lifeline: Lifeline) -> bool {
        let count = self.count_mut(lifeline);
        if *count > 0 {
            *count -= 1;
            true
        } else {
            false
        }
    }

    pub fn add_lifeline(&mut self, lifeline: &Lifeline) {
        *self.count_mut(*lifeline) += 1;
    }
}

impl Default for LifelineInventory {
    fn default() -> Self {
        Self::new()
    }
}
