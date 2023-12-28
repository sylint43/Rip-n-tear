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

use std::{ffi::OsString, iter::once, num::NonZeroU8, path::PathBuf};

use clap::ValueEnum;

pub struct DsdaArgs {
    pub(crate) iwad: PathBuf,
    pub(crate) warp: Option<NonZeroU8>,
    pub(crate) renderer: Option<Renderer>,
    pub(crate) skill: Option<Skill>,
    pub(crate) complevel: Option<Complevel>,
    pub(crate) pistolstart: bool,
    pub(crate) files: Vec<PathBuf>,
    pub(crate) extra: Vec<OsString>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Renderer {
    #[value(name = "sw", help = "Software rendering")]
    Software,
    #[value(name = "gl", help = "Hardware rendering")]
    OpenGL,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Skill {
    #[value(help = "I'm too young to die")]
    Baby,
    #[value(help = "Hey, not too rough")]
    Easy,
    #[value(help = "Hurt me plenty")]
    Medium,
    #[value(help = "Ultra-Violence")]
    Hard,
    Nightmare,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Complevel {
    #[value(name = "2", help = "Doom v1.9")]
    Doom19,
    #[value(name = "3", help = "Ultimate Doom")]
    UDoom,
    #[value(name = "4", help = "Final Doom")]
    FinalDoom,
    #[value(name = "9", help = "Boom")]
    Boom,
    #[value(name = "11", help = "MBF")]
    Mbf,
    #[value(name = "21", help = "MBF21")]
    Mbf21,
}

impl From<Skill> for OsString {
    fn from(value: Skill) -> Self {
        match value {
            Skill::Baby => "1".into(),
            Skill::Easy => "2".into(),
            Skill::Medium => "3".into(),
            Skill::Hard => "4".into(),
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

impl From<Complevel> for OsString {
    fn from(value: Complevel) -> Self {
        match value {
            Complevel::Doom19 => "2".into(),
            Complevel::UDoom => "3".into(),
            Complevel::FinalDoom => "4".into(),
            Complevel::Boom => "9".into(),
            Complevel::Mbf => "11".into(),
            Complevel::Mbf21 => "21".into(),
        }
    }
}

impl DsdaArgs {
    #[must_use]
    pub fn generate_arguments(self) -> Vec<OsString> {
        let Self {
            iwad,
            warp,
            renderer,
            skill,
            complevel,
            pistolstart,
            files,
            extra,
        } = self;

        let iwad = vec!["-iwad".into(), iwad.into_os_string()];
        let warp = warp.map_or(vec![], |lvl| vec!["-warp".into(), lvl.to_string().into()]);
        let renderer = renderer.map_or(vec![], |renderer| vec!["-vidmode".into(), renderer.into()]);
        let skill = skill.map_or(vec![], |skill| vec!["-skill".into(), skill.into()]);
        let complevel = complevel.map_or(vec![], |complevel| {
            vec!["-complevel".into(), complevel.into()]
        });
        let pistolstart = if pistolstart {
            vec!["-pistolstart".into()]
        } else {
            vec![]
        };
        let files = if files.is_empty() {
            vec![]
        } else {
            once("-file".into())
                .chain(files.into_iter().map(PathBuf::into_os_string))
                .collect()
        };

        iwad.into_iter()
            .chain(warp)
            .chain(renderer)
            .chain(skill)
            .chain(complevel)
            .chain(pistolstart)
            .chain(files)
            .chain(extra)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_generate_valid_dsda_cli_arguments() {
        let args = DsdaArgs {
            iwad: PathBuf::from("doom2.wad"),
            warp: Some(NonZeroU8::new(1).unwrap()),
            renderer: Some(Renderer::Software),
            skill: Some(Skill::Hard),
            complevel: Some(Complevel::Doom19),
            pistolstart: true,
            files: vec![PathBuf::from("test.wad"), PathBuf::from("test2.wad")],
            extra: vec!["-extra".into()],
        };
        let expected = [
            "-iwad",
            "doom2.wad",
            "-warp",
            "1",
            "-vidmode",
            "sw",
            "-skill",
            "4",
            "-complevel",
            "2",
            "-pistolstart",
            "-file",
            "test.wad",
            "test2.wad",
            "-extra",
        ];

        let actual = args.generate_arguments();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_should_return_default_arguments() {
        let args = DsdaArgs {
            iwad: PathBuf::from("doom2.wad"),
            warp: None,
            renderer: None,
            skill: None,
            complevel: None,
            pistolstart: false,
            files: vec![],
            extra: vec![],
        };
        let expected = ["-iwad", "doom2.wad"];

        let actual = args.generate_arguments();

        assert_eq!(actual, expected);
    }
}
