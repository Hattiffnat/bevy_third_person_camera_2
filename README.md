[![crates.io](https://img.shields.io/crates/v/bevy_third_person_camera_2)](https://crates.io/crates/bevy_third_person_camera_2)
[![docs.rs](https://docs.rs/bevy_third_person_camera_2/badge.svg)](https://docs.rs/bevy_third_person_camera_2)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-main-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

# Description

This is a simple and flexible plugin for adding a third-person camera to Bevy.

## Main features

1. Support for multiple cameras and targets (multiple cameras can be aimed at a single target).

2. Event-based controls ([all events](./src/events.rs))

## [Example](./examples/follow_cube.rs)

# Installation

```sh
cargo add bevy_third_person_camera_2 --rename third_person_camera
```
