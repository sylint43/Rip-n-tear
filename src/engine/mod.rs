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

use std::{io, process::ExitStatus};

pub mod dsda_doom;

pub trait Engine {
    fn run(self) -> io::Result<ExitStatus>;
}
