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

use std::{ffi::OsString, path::PathBuf};

use clap::Parser;

use crate::dsda_doom::{Complevel, DsdaArgs, Renderer, Skill};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long)]
    iwad: PathBuf,
    #[arg(long, short)]
    warp: Option<u8>,
    #[arg(long = "vid", short = 'v', value_enum)]
    renderer: Option<Renderer>,
    #[arg(long, short, value_enum)]
    skill: Option<Skill>,
    #[arg(long, short, value_enum)]
    complevel: Option<Complevel>,
    #[arg(long, short)]
    pistolstart: bool,
    #[arg(long, short)]
    files: Vec<PathBuf>,
    #[arg(long, short)]
    extra: Vec<OsString>,
}

impl From<Cli> for DsdaArgs {
    fn from(value: Cli) -> Self {
        DsdaArgs {
            iwad: value.iwad,
            warp: value.warp,
            renderer: value.renderer,
            skill: value.skill,
            complevel: value.complevel,
            pistolstart: value.pistolstart,
            files: value.files,
            extra: value.extra,
        }
    }
}
