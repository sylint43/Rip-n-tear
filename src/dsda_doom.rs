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

use std::{fmt::Display, iter::once, path::PathBuf};

struct DsdaArgs {
    warp: Option<u8>,
    renderer: Option<Renderer>,
    skill: Option<Skill>,
    pistolstart: bool,
    files: Vec<PathBuf>,
    extra: Vec<String>,
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

impl Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Skill::Easy => 1.fmt(f),
            Skill::Medium => 2.fmt(f),
            Skill::Hard => 3.fmt(f),
            Skill::VeryHard => 4.fmt(f),
            Skill::Nightmare => 5.fmt(f),
        }
    }
}

impl Display for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Renderer::Software => "sw".fmt(f),
            Renderer::OpenGL => "gl".fmt(f),
        }
    }
}

fn generate_arguments(args: DsdaArgs) -> Vec<String> {
    let DsdaArgs {
        warp,
        renderer,
        skill,
        pistolstart,
        files,
        extra,
    } = args;

    let warp = warp.map_or(vec![], |lvl| vec!["-warp".to_string(), lvl.to_string()]);
    let renderer = renderer.map_or(vec![], |renderer| {
        vec!["-vid".to_string(), renderer.to_string()]
    });
    let skill = skill.map_or(vec![], |skill| {
        vec!["-skill".to_string(), skill.to_string()]
    });
    let pistolstart = if pistolstart {
        vec!["-pistolstart".to_string()]
    } else {
        vec![]
    };
    let files = if !files.is_empty() {
        once("-file".to_string())
            .chain(
                files
                    .into_iter()
                    .map(|path| path.to_string_lossy().to_string()),
            )
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
            extra: vec!["-extra".to_string()],
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
