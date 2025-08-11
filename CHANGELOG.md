# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.13](https://github.com/ddanielsantos/fman/compare/v0.1.12...v0.1.13) - 2025-08-11

### Fixed

- *(deps)* update rust crate clap to v4.5.41 ([#57](https://github.com/ddanielsantos/fman/pull/57))
- *(deps)* update rust crate clap to v4.5.40 ([#51](https://github.com/ddanielsantos/fman/pull/51))
- *(deps)* update rust crate color-eyre to v0.6.5 ([#53](https://github.com/ddanielsantos/fman/pull/53))
- *(deps)* update rust crate tui-input to 0.14.0 ([#52](https://github.com/ddanielsantos/fman/pull/52))
- inline local_rfc_3339 ([#55](https://github.com/ddanielsantos/fman/pull/55))
- *(deps)* update rust crate clap to v4.5.28 ([#45](https://github.com/ddanielsantos/fman/pull/45))
- *(deps)* update rust crate directories to v6 ([#48](https://github.com/ddanielsantos/fman/pull/48))

### Other

- *(deps)* update actions/checkout action to v5 ([#59](https://github.com/ddanielsantos/fman/pull/59))
- Revert "fix(deps): update rust crate clap to v4.5.28 ([#45](https://github.com/ddanielsantos/fman/pull/45))" ([#50](https://github.com/ddanielsantos/fman/pull/50))

## [0.1.12](https://github.com/ddanielsantos/fman/compare/v0.1.11...v0.1.12) - 2024-12-16

### Added

- handle long directories (#46)

## [0.1.11](https://github.com/ddanielsantos/fman/compare/v0.1.10...v0.1.11) - 2024-12-05

### Added

- add scroll to input ([#43](https://github.com/ddanielsantos/fman/pull/43))

### Fixed

- *(deps)* update rust crate clap to v4.5.22 ([#44](https://github.com/ddanielsantos/fman/pull/44))
- *(deps)* update rust crate tracing-subscriber to v0.3.19 ([#42](https://github.com/ddanielsantos/fman/pull/42))
- *(deps)* update rust crate tracing to v0.1.41 ([#39](https://github.com/ddanielsantos/fman/pull/39))
- add missing parameter from cross-compile action ([#40](https://github.com/ddanielsantos/fman/pull/40))

## [0.1.10](https://github.com/ddanielsantos/fman/compare/v0.1.9...v0.1.10) - 2024-11-21

### Added

- improve create item ui ([#38](https://github.com/ddanielsantos/fman/pull/38))
- command picker ([#36](https://github.com/ddanielsantos/fman/pull/36))

### Fixed

- *(deps)* update rust crate clap to v4.5.21 ([#35](https://github.com/ddanielsantos/fman/pull/35))
- *(deps)* update rust crate ratatui to 0.29.0 ([#33](https://github.com/ddanielsantos/fman/pull/33))

### Other

- renovate.json ([#31](https://github.com/ddanielsantos/fman/pull/31))

## [0.1.9](https://github.com/ddanielsantos/fman/compare/v0.1.8...v0.1.9) - 2024-11-07

### Other

- unify steps

## [0.1.8](https://github.com/ddanielsantos/fman/compare/v0.1.7...v0.1.8) - 2024-11-07

### Other

- tag released prs (4th try)
- tag released prs (3rd try)

## [0.1.7](https://github.com/ddanielsantos/fman/compare/v0.1.6...v0.1.7) - 2024-11-07

### Other

- tag released prs (2nd try)

## [0.1.6](https://github.com/ddanielsantos/fman/compare/v0.1.5...v0.1.6) - 2024-11-07

### Other

- add tags to released prs

## [0.1.5](https://github.com/ddanielsantos/fman/compare/v0.1.4...v0.1.5) - 2024-11-05

### Other

- update README.md
- move file system related code to own module
- add links
- add readme
- use cross for build action
- checkout repo before building
- add matrix.os
- remove verbose flag
- ignore
- remove step
- add test action
- ignore
- ignore
- Create test.yml

## [0.1.4](https://github.com/ddanielsantos/fman/compare/v0.1.3...v0.1.4) - 2024-10-30

### Added

- add deletion logic
- add handling for showing or not hidden files

### Fixed

- change dir before deleting current path
- delete deepest items first
- reset state after each directory change

### Other

- use common for instead
- wrap current_dir()
- move code up
- use path ref
- remove clones
- apply clippy sugestions
- remove args for now

## [0.1.3](https://github.com/ddanielsantos/fman/compare/v0.1.2...v0.1.3) - 2024-10-26

### Added

- loggin
- impl move_to_parent
- init move_to_parent

### Fixed

- move_to_parent working
- timestamp

### Other

- strip ansi
- simplify package
- improve log file path
- add deps
- move to func
- prepare

## [0.1.2](https://github.com/ddanielsantos/fman/compare/v0.1.1...v0.1.2) - 2024-10-22

### Other

- update repo url

## [0.1.1](https://github.com/ddanielsantos/fm/compare/v0.1.0...v0.1.1) - 2024-10-17

### Other

- add label to released prs ([#7](https://github.com/ddanielsantos/fm/pull/7))
- release v0.1.0 ([#4](https://github.com/ddanielsantos/fm/pull/4))

## [0.1.0](https://github.com/ddanielsantos/fm/releases/tag/v0.1.0) - 2024-10-17

### Added

- enter directory
- stateful list
- current dir content
- args
- init

### Fixed

- optional path

### Other

- add crate info ([#5](https://github.com/ddanielsantos/fm/pull/5))
- rename package ([#3](https://github.com/ddanielsantos/fm/pull/3))
- add release-pr ([#2](https://github.com/ddanielsantos/fm/pull/2))
- rename
- living dangerously for now
- unneeded abstraction
