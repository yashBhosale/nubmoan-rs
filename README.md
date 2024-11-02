# nubmoan-rs

Source code for nubmoan, the program that makes your ThinkPad TrackPoint (the red nub) moan when you press it. Written in rust, only works on linux systems that use libinput and alsa. Inspired by [https://github.com/wttdotm/nubmoan/](wttdotm/nubmoan)


Change line 65 to match your own absolute file path.


Requirements:
- lib-alsa-devel
- libinput-devel

Compile with `cargo build`

TODO:
- The original project allows you to use multiple sound files. I would like to set that up.
