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

struct DsdaArgs {
    warp: Option<u8>,
    renderer: Option<Renderer>,
    skill: Option<Skill>,
    pistolstart: bool,
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

impl From<Skill> for String {
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

impl From<Renderer> for String {
    fn from(value: Renderer) -> Self {
        match value {
            Renderer::Software => "sw".to_string(),
            Renderer::OpenGL => "gl".to_string(),
        }
    }
}

fn generate_arguments(args: DsdaArgs) -> Vec<String> {
    let DsdaArgs {
        warp,
        renderer,
        skill,
        pistolstart,
    } = args;

    let warp = warp.map_or(vec![], |lvl| vec!["-warp".to_string(), lvl.to_string()]);
    let renderer = renderer.map_or(vec![], |renderer| vec!["-vid".to_string(), renderer.into()]);
    let skill = skill.map_or(vec![], |skill| vec!["-skill".to_string(), skill.into()]);
    let pistolstart = if pistolstart {
        vec!["-pistolstart".to_string()]
    } else {
        vec![]
    };

    warp.into_iter()
        .chain(renderer)
        .chain(skill)
        .chain(pistolstart)
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
        };
        let expected = ["-warp", "1", "-vid", "sw", "-skill", "4", "-pistolstart"];

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
        };
        let expected: [&str; 0] = [];

        let actual = generate_arguments(args);

        assert_eq!(actual, expected);
    }
}
