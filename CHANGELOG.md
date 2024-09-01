# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]


## [0.2.1] - 2024-09-01

### Added

* `Aabb::from_top_left_and_size`


## [0.2.0] - 2023-12-09


### Breaking change

* change `Collides` and `Penetrate` traits to take `Borrow<S>` instead of `&S`

Now users can decide whether to pass shapes by reference
or with ownership, which gives more flexibility and notably makes possible to use iterators
that build shapes on the fly.


## [0.1.2] - 2023-12-09

### Added

* `aabb`: `Aabb::min` and `Aabb::max` getters

### Documentation

* Improve formatting of the feature list on docs.rs


## [0.1.1] - 2023-12-01

### Documentation

* Better document required feature flags on docs.rs


## [0.1.0] - 2023-12-01

* `Collides` trait to check for collision
* `Penetration` trait get penetration vector
* Axis-Aligned Bounding Box (`Aabb`)
* Support `no_std` target (require to disable default features)

[Unreleased]: https://github.com/jcornaz/beancount_parser_2/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/jcornaz/beancount_parser_2/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jcornaz/collision2d/compare/...v0.1.0
