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

use std::process::Command;

use clap::Parser;
use rnt::{cli::Cli, dsda_doom::DsdaArgs};

fn main() {
    let cli = Cli::parse();
    let args = DsdaArgs::from(cli).generate_arguments();

    Command::new("dsda-doom")
        .args(args)
        .spawn()
        .expect("Failed to launch dsda-doom");
}
