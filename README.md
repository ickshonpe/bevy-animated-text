# bevy-animated-text

Basic animated text plugin for Bevy.

In order to work nicely with Bevy's exisiting text implementation, it's limited to only animating the positions of the glyphs.

Support for scalings, rotations and colors is possible but would need a much more complicated plugin,
probably requiring alternatives to some of the existing systems, components and bundles, and/or limiting users to a single glyph per `TextSection`.

### Examples
* ```cargo run --example hello_world```
* ```cargo run --example text2d```
* ```cargo run --example ui```
The `text2d` example is the most complete.
