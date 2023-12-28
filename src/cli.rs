// Copyright (C) 2023 Sylvia Waldron
//
// This file is part of rnt.
//
// rnt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rnt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rnt.  If not, see <http://www.gnu.org/licenses/>.

use std::{ffi::OsString, num::NonZeroU8, path::PathBuf};

use clap::Parser;

use crate::engine::{
    dsda_doom::{Complevel, DsdaArgs, DsdaDoom, Renderer, Skill},
    Engine,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to IWad to use
    #[arg(long)]
    iwad: PathBuf,
    /// Warp to level at start
    #[arg(long, short, value_name = "LEVEL")]
    warp: Option<NonZeroU8>,
    /// Set graphics renderer
    #[arg(long = "vid", short = 'v', value_enum)]
    renderer: Option<Renderer>,
    /// Set skill level
    #[arg(long, short, value_enum)]
    skill: Option<Skill>,
    /// Set compability level
    #[arg(long, short, value_enum)]
    complevel: Option<Complevel>,
    /// Pistolstart after every level
    #[arg(long, short)]
    pistolstart: bool,
    /// Paths to PWads to use
    #[arg(value_name = "PWADS")]
    files: Vec<PathBuf>,
    /// Extra command line aruguments
    #[arg(long, short, value_name = "EXTRA ARGS")]
    extra: Vec<OsString>,
}

impl Cli {
    #[must_use]
    pub fn to_engine(self) -> impl Engine {
        DsdaDoom::new(DsdaArgs {
            iwad: self.iwad,
            warp: self.warp,
            renderer: self.renderer,
            skill: self.skill,
            complevel: self.complevel,
            pistolstart: self.pistolstart,
            files: self.files,
            extra: self.extra,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
