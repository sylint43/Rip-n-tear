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

use std::{ffi::OsString, iter::once, path::PathBuf};

struct DsdaArgs {
    warp: Option<u8>,
    renderer: Option<Renderer>,
    skill: Option<Skill>,
    pistolstart: bool,
    files: Vec<PathBuf>,
    extra: Vec<OsString>,
}

enum Renderer {
    Software,
    OpenGL,
}

enum Skill {
    Easy,
    Medium,
    Hard,
    VeryHard,
    Nightmare,
}

impl From<Skill> for OsString {
    fn from(value: Skill) -> Self {
        match value {
            Skill::Easy => "1".into(),
            Skill::Medium => "2".into(),
            Skill::Hard => "3".into(),
            Skill::VeryHard => "4".into(),
            Skill::Nightmare => "5".into(),
        }
    }
}

impl From<Renderer> for OsString {
    fn from(value: Renderer) -> Self {
        match value {
            Renderer::Software => "sw".into(),
            Renderer::OpenGL => "gl".into(),
        }
    }
}

fn generate_arguments(args: DsdaArgs) -> Vec<OsString> {
    let DsdaArgs {
        warp,
        renderer,
        skill,
        pistolstart,
        files,
        extra,
    } = args;

    let warp = warp.map_or(vec![], |lvl| vec!["-warp".into(), lvl.to_string().into()]);
    let renderer = renderer.map_or(vec![], |renderer| vec!["-vid".into(), renderer.into()]);
    let skill = skill.map_or(vec![], |skill| vec!["-skill".into(), skill.into()]);
    let pistolstart = if pistolstart {
        vec!["-pistolstart".into()]
    } else {
        vec![]
    };
    let files = if !files.is_empty() {
        once("-file".into())
            .chain(files.into_iter().map(|path| path.into_os_string()))
            .collect()
    } else {
        vec![]
    };

    warp.into_iter()
        .chain(renderer)
        .chain(skill)
        .chain(pistolstart)
        .chain(files)
        .chain(extra)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_generate_valid_dsda_cli_arguments() {
        let args = DsdaArgs {
            warp: Some(1),
            renderer: Some(Renderer::Software),
            skill: Some(Skill::VeryHard),
            pistolstart: true,
            files: vec![PathBuf::from("test.wad"), PathBuf::from("test2.wad")],
            extra: vec!["-extra".into()],
        };
        let expected = [
            "-warp",
            "1",
            "-vid",
            "sw",
            "-skill",
            "4",
            "-pistolstart",
            "-file",
            "test.wad",
            "test2.wad",
            "-extra",
        ];

        let actual = generate_arguments(args);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_should_return_default_arguments() {
        let args = DsdaArgs {
            warp: None,
            renderer: None,
            skill: None,
            pistolstart: false,
            files: vec![],
            extra: vec![],
        };
        let expected: [&str; 0] = [];

        let actual = generate_arguments(args);

        assert_eq!(actual, expected);
    }
}
