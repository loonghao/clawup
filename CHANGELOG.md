# Changelog

## [0.1.10](https://github.com/loonghao/clawup/compare/v0.1.9...v0.1.10) (2026-03-23)


### 🚀 Features

* implement export and diff commands with JSON/TOML support ([84651d4](https://github.com/loonghao/clawup/commit/84651d42a113e50e69a7b549c9b43bb94dc4163e))

## [0.1.9](https://github.com/loonghao/clawup/compare/v0.1.8...v0.1.9) (2026-03-22)


### 🐛 Bug Fixes

* prevent docs/chore commits from triggering releases and add upgrade detection to install scripts ([5844727](https://github.com/loonghao/clawup/commit/5844727128630842e38b4308b263574358ef93d0))

## [0.1.8](https://github.com/loonghao/clawup/compare/v0.1.7...v0.1.8) (2026-03-22)


### 🚀 Features

* add SHA256 checksum verification to install scripts and release workflow ([689b65b](https://github.com/loonghao/clawup/commit/689b65b07a89fb5dadef4fae1f6a02fcad0d3445))

## [0.1.7](https://github.com/loonghao/clawup/compare/v0.1.6...v0.1.7) (2026-03-22)


### 🐛 Bug Fixes

* upgrade GitHub Actions to v6 and improve install scripts ([50f9b6b](https://github.com/loonghao/clawup/commit/50f9b6bef92a833d9ff87ef64e8394d2dfbc75c9))

## [0.1.6](https://github.com/loonghao/clawup/compare/v0.1.5...v0.1.6) (2026-03-22)


### 🐛 Bug Fixes

* **ci:** rewrite release workflow for reliable cross-platform builds ([512dc1e](https://github.com/loonghao/clawup/commit/512dc1ed02f6937b51a37d81baf30e74fa1923fc))

## [0.1.5](https://github.com/loonghao/clawup/compare/v0.1.4...v0.1.5) (2026-03-22)


### 🚀 Features

* add install scripts + fix release workflow dispatch ([#9](https://github.com/loonghao/clawup/issues/9)) ([2ded0e1](https://github.com/loonghao/clawup/commit/2ded0e1b3b491f2ae7efcf928ed90146c28d209c))


### 🐛 Bug Fixes

* **ci:** use macos-14 runner and disable python wheels in release workflow ([2cf3bf0](https://github.com/loonghao/clawup/commit/2cf3bf08cffe32a8318649c20950a1c87bf3d0c7))

## [0.1.4](https://github.com/loonghao/clawup/compare/v0.1.3...v0.1.4) (2026-03-22)


### 🐛 Bug Fixes

* **ci:** dispatch release workflow after release-please creates a release ([9f41d89](https://github.com/loonghao/clawup/commit/9f41d89e44521225c23577f75d0a0047a4f4edcb))

## [0.1.3](https://github.com/loonghao/clawup/compare/v0.1.2...v0.1.3) (2026-03-22)


### 🚀 Features

* add clawup-schema, clawup-core, clawup-ops crates and AGENTS.md ([211b326](https://github.com/loonghao/clawup/commit/211b32644ca7c0e77e7c4e0fb46cf0966a9c49e4))
* initial project scaffold for clawup CLI ([3a46ed4](https://github.com/loonghao/clawup/commit/3a46ed41992bbd5766d0fe30552f54f0398ad603))
* **schema:** add complete OpenClaw configuration coverage ([a67e21b](https://github.com/loonghao/clawup/commit/a67e21b855e8ad5514c7ada1c8d55aa8c53108e0))


### 🐛 Bug Fixes

* **ci:** disable empty release-consistency matrix to fix CI failure ([7f3d4d5](https://github.com/loonghao/clawup/commit/7f3d4d5b267e114bbf00ed092e44114b72d2b0aa))
* **ci:** exclude component name from release tag to trigger release workflow ([94947aa](https://github.com/loonghao/clawup/commit/94947aa7f3782e0a400c7ccc177ce7d939ee4dd7))
* **deps:** enable vendored-openssl for git2 to fix musl cross-compilation ([307c724](https://github.com/loonghao/clawup/commit/307c7246f0bd8399e713ac7f6df5e1c1e40272e2))
* **lint:** suppress dead_code warnings for legacy modules ([536234d](https://github.com/loonghao/clawup/commit/536234d853eaaa236c93b8e65fb8295c4cb8a835))
* resolve release-please workspace version error and fix docs ([bd4b735](https://github.com/loonghao/clawup/commit/bd4b7355317436ed5089dd63cf6c4bc22840b904))


### 🔧 Miscellaneous

* **main:** release clawup 0.1.1 ([7f0e864](https://github.com/loonghao/clawup/commit/7f0e8648b7dc8b26da30252508c73ed97399ac4e))
* **main:** release clawup 0.1.2 ([9eab56b](https://github.com/loonghao/clawup/commit/9eab56b80af797d35ed1e40b124183206acf14b7))

## [0.1.2](https://github.com/loonghao/clawup/compare/clawup-v0.1.1...clawup-v0.1.2) (2026-03-21)


### 🚀 Features

* **schema:** add complete OpenClaw configuration coverage ([a67e21b](https://github.com/loonghao/clawup/commit/a67e21b855e8ad5514c7ada1c8d55aa8c53108e0))


### 🐛 Bug Fixes

* **deps:** enable vendored-openssl for git2 to fix musl cross-compilation ([307c724](https://github.com/loonghao/clawup/commit/307c7246f0bd8399e713ac7f6df5e1c1e40272e2))
* **lint:** suppress dead_code warnings for legacy modules ([536234d](https://github.com/loonghao/clawup/commit/536234d853eaaa236c93b8e65fb8295c4cb8a835))

## [0.1.1](https://github.com/loonghao/clawup/compare/clawup-v0.1.0...clawup-v0.1.1) (2026-03-21)


### 🚀 Features

* add clawup-schema, clawup-core, clawup-ops crates and AGENTS.md ([211b326](https://github.com/loonghao/clawup/commit/211b32644ca7c0e77e7c4e0fb46cf0966a9c49e4))
* initial project scaffold for clawup CLI ([3a46ed4](https://github.com/loonghao/clawup/commit/3a46ed41992bbd5766d0fe30552f54f0398ad603))


### 🐛 Bug Fixes

* resolve release-please workspace version error and fix docs ([bd4b735](https://github.com/loonghao/clawup/commit/bd4b7355317436ed5089dd63cf6c4bc22840b904))
