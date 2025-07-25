# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.34.8](https://github.com/conda/rattler/compare/rattler-v0.34.7...rattler-v0.34.8) - 2025-07-21

### Other

- update Cargo.toml dependencies

## [0.34.7](https://github.com/conda/rattler/compare/rattler-v0.34.6...rattler-v0.34.7) - 2025-07-14

### Fixed

- *(clobber registry)* directory and file clobbering ([#1497](https://github.com/conda/rattler/pull/1497))

## [0.34.6](https://github.com/conda/rattler/compare/rattler-v0.34.5...rattler-v0.34.6) - 2025-07-09

### Other

- updated the following local packages: rattler_conda_types, rattler_networking, rattler_package_streaming, rattler_cache, rattler_shell, rattler_menuinst

## [0.34.5](https://github.com/conda/rattler/compare/rattler-v0.34.4...rattler-v0.34.5) - 2025-07-01

### Fixed

- *(ci)* run pre-commit-run for all files ([#1481](https://github.com/conda/rattler/pull/1481))

## [0.34.4](https://github.com/conda/rattler/compare/rattler-v0.34.3...rattler-v0.34.4) - 2025-06-26

### Fixed

- recursively remove empty directories after uninstall ([#1475](https://github.com/conda/rattler/pull/1475))

## [0.34.3](https://github.com/conda/rattler/compare/rattler-v0.34.2...rattler-v0.34.3) - 2025-06-25

### Other

- updated the following local packages: rattler_conda_types, rattler_networking, rattler_shell, rattler_package_streaming, rattler_cache, rattler_menuinst

## [0.34.2](https://github.com/conda/rattler/compare/rattler-v0.34.1...rattler-v0.34.2) - 2025-06-24

### Other

- updated the following local packages: rattler_shell, rattler_menuinst

## [0.34.1](https://github.com/conda/rattler/compare/rattler-v0.34.0...rattler-v0.34.1) - 2025-06-23

### Other

- update npm name ([#1368](https://github.com/conda/rattler/pull/1368))
- update readme ([#1364](https://github.com/conda/rattler/pull/1364))

## [0.34.0](https://github.com/conda/rattler/compare/rattler-v0.33.7...rattler-v0.34.0) - 2025-05-23

### Fixed

- consistent usage of rustls-tls / native-tls feature ([#1324](https://github.com/conda/rattler/pull/1324))

## [0.33.7](https://github.com/conda/rattler/compare/rattler-v0.33.6...rattler-v0.33.7) - 2025-05-16

### Other

- update Cargo.toml dependencies

## [0.33.6](https://github.com/conda/rattler/compare/rattler-v0.33.5...rattler-v0.33.6) - 2025-05-03

### Other

- lock workspace member dependencies ([#1279](https://github.com/conda/rattler/pull/1279))

## [0.33.5](https://github.com/conda/rattler/compare/rattler-v0.33.4...rattler-v0.33.5) - 2025-04-10

### Other

- change `InstallOptions::target_prefix` to `Option<PathBuf>` ([#1242](https://github.com/conda/rattler/pull/1242))

## [0.33.4](https://github.com/conda/rattler/compare/rattler-v0.33.3...rattler-v0.33.4) - 2025-04-04

### Added

- Set link options in installer ([#1178](https://github.com/conda/rattler/pull/1178))

### Other

- add the remove_from_backup function and update the prefix ([#1155](https://github.com/conda/rattler/pull/1155))

## [0.33.3](https://github.com/conda/rattler/compare/rattler-v0.33.2...rattler-v0.33.3) - 2025-03-18

### Fixed

- use `create_dir_all` for `.trash` directory on Windows ([#1174](https://github.com/conda/rattler/pull/1174))

## [0.33.2](https://github.com/conda/rattler/compare/rattler-v0.33.1...rattler-v0.33.2) - 2025-03-18

### Other

- updated the following local packages: rattler_menuinst

## [0.33.1](https://github.com/conda/rattler/compare/rattler-v0.33.0...rattler-v0.33.1) - 2025-03-14

### Other

- updated the following local packages: rattler_conda_types, rattler_networking, rattler_menuinst

## [0.33.0](https://github.com/conda/rattler/compare/rattler-v0.32.4...rattler-v0.33.0) - 2025-03-10

### Added

- add reinstallation method to installer and transaction ([#1128](https://github.com/conda/rattler/pull/1128))

## [0.32.4](https://github.com/conda/rattler/compare/rattler-v0.32.3...rattler-v0.32.4) - 2025-03-04

### Other

- update Cargo.toml dependencies

## [0.32.3](https://github.com/conda/rattler/compare/rattler-v0.32.2...rattler-v0.32.3) - 2025-02-28

### Other

- update Cargo.toml dependencies

## [0.32.2](https://github.com/conda/rattler/compare/rattler-v0.32.1...rattler-v0.32.2) - 2025-02-27

### Other

- updated the following local packages: rattler_conda_types, rattler_networking, rattler_package_streaming, rattler_shell, rattler_menuinst

## [0.32.1](https://github.com/conda/rattler/compare/rattler-v0.32.0...rattler-v0.32.1) - 2025-02-25

### Added

- add `rattler_menuinst` crate (#840)

## [0.32.0](https://github.com/conda/rattler/compare/rattler-v0.31.1...rattler-v0.32.0) - 2025-02-18

### Fixed

- use new PackageRecord when issuing reinstallation in `Transaction::from_current_and_desired` (#1070)
- clobber issue where path was not correctly searched for in clobbered paths (#1066)

### Other

- update dependencies (#1069)

## [0.31.1](https://github.com/conda/rattler/compare/rattler-v0.31.0...rattler-v0.31.1) - 2025-02-06

### Other

- bump rust 1.84.1 (#1053)

## [0.31.0](https://github.com/conda/rattler/compare/rattler-v0.30.0...rattler-v0.31.0) - 2025-02-06

### Fixed

- create parent directories for file storage (#1045)

## [0.30.0](https://github.com/conda/rattler/compare/rattler-v0.29.0...rattler-v0.30.0) - 2025-02-03

### Added

- add S3 support (#1008)

## [0.29.0](https://github.com/conda/rattler/compare/rattler-v0.28.12...rattler-v0.29.0) - 2025-01-23

### Other

- Improve AuthenticationStorage (#1026)

## [0.28.12](https://github.com/conda/rattler/compare/rattler-v0.28.11...rattler-v0.28.12) - 2025-01-09

### Other

- updated the following local packages: rattler_conda_types

## [0.28.11](https://github.com/conda/rattler/compare/rattler-v0.28.10...rattler-v0.28.11) - 2025-01-09

### Other

- updated the following local packages: rattler_conda_types

## [0.28.10](https://github.com/conda/rattler/compare/rattler-v0.28.9...rattler-v0.28.10) - 2025-01-08

### Other

- update dependencies (#1009)

## [0.28.9](https://github.com/conda/rattler/compare/rattler-v0.28.8...rattler-v0.28.9) - 2024-12-20

### Other

- reflink directories at once on macOS (#995)

## [0.28.8](https://github.com/conda/rattler/compare/rattler-v0.28.7...rattler-v0.28.8) - 2024-12-17

### Added

- speed up `PrefixRecord` loading (#984)
- improve performance when linking files using `rayon` (#985)

## [0.28.7](https://github.com/conda/rattler/compare/rattler-v0.28.6...rattler-v0.28.7) - 2024-12-13

### Fixed

- check reflink support before linking (#979)

## [0.28.6](https://github.com/conda/rattler/compare/rattler-v0.28.5...rattler-v0.28.6) - 2024-12-12

### Other
- updated the following local packages: rattler_cache, rattler_conda_types

## [0.28.5](https://github.com/conda/rattler/compare/rattler-v0.28.4...rattler-v0.28.5) - 2024-12-05

### Other

- updated the following local packages: rattler_networking

## [0.28.4](https://github.com/conda/rattler/compare/rattler-v0.28.3...rattler-v0.28.4) - 2024-11-30

### Added

- use `fs-err` also for tokio ([#958](https://github.com/conda/rattler/pull/958))
- Move files to .trash folder if they are in use ([#953](https://github.com/conda/rattler/pull/953))
- merge pixi-build branch ([#950](https://github.com/conda/rattler/pull/950))

## [0.28.3](https://github.com/conda/rattler/compare/rattler-v0.28.2...rattler-v0.28.3) - 2024-11-18

### Other

- updated the following local packages: rattler_networking

## [0.28.2](https://github.com/conda/rattler/compare/rattler-v0.28.1...rattler-v0.28.2) - 2024-11-18

### Other

- updated the following local packages: rattler_conda_types

## [0.28.1](https://github.com/conda/rattler/compare/rattler-v0.28.0...rattler-v0.28.1) - 2024-11-14

### Other

- updated the following local packages: rattler_cache, rattler_conda_types, rattler_package_streaming

## [0.28.0](https://github.com/conda/rattler/compare/rattler-v0.27.16...rattler-v0.28.0) - 2024-11-04

### Added

- use python_site_packages_path field when available for installing noarch: python packages, CEP-17 ([#909](https://github.com/conda/rattler/pull/909))

### Fixed

- typo in `set_io_concurrentcy_limit` ([#914](https://github.com/conda/rattler/pull/914))

## [0.27.16](https://github.com/conda/rattler/compare/rattler-v0.27.15...rattler-v0.27.16) - 2024-10-21

### Fixed

- removal / linking when package name changes ([#905](https://github.com/conda/rattler/pull/905))

## [0.27.15](https://github.com/conda/rattler/compare/rattler-v0.27.14...rattler-v0.27.15) - 2024-10-07

### Fixed

- self-clobber when updating/downgrading packages ([#893](https://github.com/conda/rattler/pull/893))

## [0.27.14](https://github.com/conda/rattler/compare/rattler-v0.27.13...rattler-v0.27.14) - 2024-10-03

### Fixed

- fix-up shebangs with spaces ([#887](https://github.com/conda/rattler/pull/887))

## [0.27.13](https://github.com/conda/rattler/compare/rattler-v0.27.12...rattler-v0.27.13) - 2024-10-01

### Other

- update Cargo.toml dependencies

## [0.27.12](https://github.com/conda/rattler/compare/rattler-v0.27.11...rattler-v0.27.12) - 2024-09-23

### Other

- updated the following local packages: rattler_conda_types

## [0.27.11](https://github.com/conda/rattler/compare/rattler-v0.27.10...rattler-v0.27.11) - 2024-09-09

### Other

- updated the following local packages: rattler_conda_types

## [0.27.10](https://github.com/conda/rattler/compare/rattler-v0.27.9...rattler-v0.27.10) - 2024-09-05

### Fixed
- remaining typos ([#854](https://github.com/conda/rattler/pull/854))
- typos ([#849](https://github.com/conda/rattler/pull/849))

## [0.27.9](https://github.com/conda/rattler/compare/rattler-v0.27.8...rattler-v0.27.9) - 2024-09-03

### Other
- updated the following local packages: rattler_cache

## [0.27.8](https://github.com/conda/rattler/compare/rattler-v0.27.7...rattler-v0.27.8) - 2024-09-03

### Other
- make PackageCache multi-process safe ([#837](https://github.com/conda/rattler/pull/837))

## [0.27.7](https://github.com/conda/rattler/compare/rattler-v0.27.6...rattler-v0.27.7) - 2024-09-02

### Other
- updated the following local packages: rattler_conda_types, rattler_package_streaming

## [0.27.6](https://github.com/conda/rattler/compare/rattler-v0.27.5...rattler-v0.27.6) - 2024-08-16

### Other
- updated the following local packages: rattler_networking

## [0.27.5](https://github.com/conda/rattler/compare/rattler-v0.27.4...rattler-v0.27.5) - 2024-08-15

### Fixed
- move more links to the conda org from conda-incubator ([#816](https://github.com/conda/rattler/pull/816))

### Other
- change links from conda-incubator to conda ([#813](https://github.com/conda/rattler/pull/813))
- update banner ([#808](https://github.com/conda/rattler/pull/808))
- add banner to docs

## [0.27.4](https://github.com/baszalmstra/rattler/compare/rattler-v0.27.3...rattler-v0.27.4) - 2024-08-06

### Other
- updated the following local packages: rattler_conda_types

## [0.27.3](https://github.com/baszalmstra/rattler/compare/rattler-v0.27.2...rattler-v0.27.3) - 2024-08-02

### Other
- mark some crates 1.0 ([#789](https://github.com/baszalmstra/rattler/pull/789))

## [0.27.2](https://github.com/conda/rattler/compare/rattler-v0.27.1...rattler-v0.27.2) - 2024-07-23

### Other
- updated the following local packages: rattler_conda_types

## [0.27.1](https://github.com/conda/rattler/compare/rattler-v0.27.0...rattler-v0.27.1) - 2024-07-23

### Other
- updated the following local packages: rattler_conda_types

## [0.27.0](https://github.com/conda/rattler/compare/rattler-v0.26.5...rattler-v0.27.0) - 2024-07-15

### Fixed
- unclobber issue when packages are named differently ([#776](https://github.com/conda/rattler/pull/776))

### Other
- bump dependencies and remove unused ones ([#771](https://github.com/conda/rattler/pull/771))

## [0.26.5](https://github.com/conda/rattler/compare/rattler-v0.26.4...rattler-v0.26.5) - 2024-07-08

### Added
- add direct url repodata building ([#725](https://github.com/conda/rattler/pull/725))

### Fixed
- errors should not contain trailing punctuation ([#763](https://github.com/conda/rattler/pull/763))
- run clippy on all targets ([#762](https://github.com/conda/rattler/pull/762))

## [0.26.4](https://github.com/conda/rattler/compare/rattler-v0.26.3...rattler-v0.26.4) - 2024-06-06

### Other
- updated the following local packages: rattler_shell

## [0.26.3](https://github.com/baszalmstra/rattler/compare/rattler-v0.26.2...rattler-v0.26.3) - 2024-06-04

### Other
- remove lfs ([#512](https://github.com/baszalmstra/rattler/pull/512))
- move the cache tooling into its own crate for reuse downstream ([#721](https://github.com/baszalmstra/rattler/pull/721))

## [0.26.2](https://github.com/conda/rattler/compare/rattler-v0.26.1...rattler-v0.26.2) - 2024-06-03

### Other
- updated the following local packages: rattler_conda_types, rattler_package_streaming

## [0.26.1](https://github.com/conda/rattler/compare/rattler-v0.26.0...rattler-v0.26.1) - 2024-05-28

### Other
- updated the following local packages: rattler_conda_types

## [0.26.0](https://github.com/conda/rattler/compare/rattler-v0.25.0...rattler-v0.26.0) - 2024-05-27

### Fixed
- improve progress bar duration display ([#680](https://github.com/conda/rattler/pull/680))

### Other
- introducing the installer ([#664](https://github.com/conda/rattler/pull/664))
- create directories up front ([#533](https://github.com/conda/rattler/pull/533))

## [0.25.0](https://github.com/conda/rattler/compare/rattler-v0.24.1...rattler-v0.25.0) - 2024-05-14

### Added
- exclude repodata records based on timestamp ([#654](https://github.com/conda/rattler/pull/654))

### Other
- use semaphore for install driver ([#653](https://github.com/conda/rattler/pull/653))

## [0.24.1](https://github.com/conda/rattler/compare/rattler-v0.24.0...rattler-v0.24.1) - 2024-05-13

### Other
- updated the following local packages: rattler_conda_types, rattler_digest, rattler_package_streaming, rattler_networking

## [0.24.0](https://github.com/conda/rattler/compare/rattler-v0.23.2...rattler-v0.24.0) - 2024-05-06

### Fixed
- use the output of `readlink` as hash for softlinks ([#643](https://github.com/conda/rattler/pull/643))
- sha computation of symlinks was failing sometimes ([#641](https://github.com/conda/rattler/pull/641))

## [0.23.2](https://github.com/conda/rattler/compare/rattler-v0.23.1...rattler-v0.23.2) - 2024-04-30

### Other
- updated the following local packages: rattler_networking

## [0.23.1](https://github.com/conda/rattler/compare/rattler-v0.23.0...rattler-v0.23.1) - 2024-04-25

### Other
- updated the following local packages: rattler_networking

## [0.23.0](https://github.com/conda/rattler/compare/rattler-v0.22.0...rattler-v0.23.0) - 2024-04-25

### Added
- Expose paths_data as PathEntry in py-rattler ([#620](https://github.com/conda/rattler/pull/620))
- add support for extracting prefix placeholder data to PathsEntry ([#614](https://github.com/conda/rattler/pull/614))

### Fixed
- compare `UrlOrPath` ([#618](https://github.com/conda/rattler/pull/618))

## [0.22.0](https://github.com/conda/rattler/compare/rattler-v0.21.0...rattler-v0.22.0) - 2024-04-19

### Added
- make root dir configurable in channel config ([#602](https://github.com/conda/rattler/pull/602))

### Fixed
- unicode activation issues on windows ([#604](https://github.com/conda/rattler/pull/604))
- no shebang on windows to make spaces in prefix work ([#611](https://github.com/conda/rattler/pull/611))
- use correct platform to decide the windows launcher ([#608](https://github.com/conda/rattler/pull/608))

### Other
- update dependencies incl. reqwest ([#606](https://github.com/conda/rattler/pull/606))

## [0.21.0](https://github.com/baszalmstra/rattler/compare/rattler-v0.20.1...rattler-v0.21.0) - 2024-04-05

### Fixed
- replace long shebangs with `/usr/bin/env` ([#594](https://github.com/baszalmstra/rattler/pull/594))
- run post-link scripts ([#574](https://github.com/baszalmstra/rattler/pull/574))

## [0.20.1](https://github.com/conda/rattler/compare/rattler-v0.20.0...rattler-v0.20.1) - 2024-04-02

### Fixed
- copy windows dll without replacements ([#590](https://github.com/conda/rattler/pull/590))

## [0.20.0](https://github.com/conda/rattler/compare/rattler-v0.19.6...rattler-v0.20.0) - 2024-04-02

### Fixed
- do not do cstring replacement on windows ([#589](https://github.com/conda/rattler/pull/589))

## [0.19.6](https://github.com/conda/rattler/compare/rattler-v0.19.5...rattler-v0.19.6) - 2024-03-30

### Other
- remove unused dependencies ([#585](https://github.com/conda/rattler/pull/585))

## [0.19.5](https://github.com/conda/rattler/compare/rattler-v0.19.4...rattler-v0.19.5) - 2024-03-21

### Fixed
- typo ([#576](https://github.com/conda/rattler/pull/576))

## [0.19.4](https://github.com/conda/rattler/compare/rattler-v0.19.3...rattler-v0.19.4) - 2024-03-19

### Fixed
- multi-prefix replacement in binary files ([#570](https://github.com/conda/rattler/pull/570))

## [0.19.3](https://github.com/conda/rattler/compare/rattler-v0.19.2...rattler-v0.19.3) - 2024-03-14

### Added
- add mirror handling and OCI mirror type ([#553](https://github.com/conda/rattler/pull/553))

### Other
- add pixi badge ([#563](https://github.com/conda/rattler/pull/563))

## [0.19.2](https://github.com/conda/rattler/compare/rattler-v0.19.1...rattler-v0.19.2) - 2024-03-08

### Other
- update Cargo.toml dependencies

## [0.19.1](https://github.com/conda/rattler/compare/rattler-v0.19.0...rattler-v0.19.1) - 2024-03-06

### Added
- generalised CLI authentication ([#537](https://github.com/conda/rattler/pull/537))

### Fixed
- removal of multiple packages that clobber each other ([#556](https://github.com/conda/rattler/pull/556))
- dont use workspace dependencies for local crates ([#546](https://github.com/conda/rattler/pull/546))

### Other
- every crate should have its own version ([#557](https://github.com/conda/rattler/pull/557))

## [0.19.0](https://github.com/baszalmstra/rattler/compare/rattler-v0.18.0...rattler-v0.19.0) - 2024-02-26
