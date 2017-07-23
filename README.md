# Rustcaster

This is a fairly lightweight raycaster, which supports ceilings, floors, and sprites. It uses SDL2 for windowing, rendering, and image loading. It's the first actual program I've written in Rust, so it's likely that it isn't idiomatic. Feel free to leave any feedback!

![Rustcaster](http://declanhopkins.com/static/images/screenshots/rustcaster.png)

Most of the actual raycasting code is contained within the internal `rustcaster-lib` library. By supplying the `render` function with a map, camera values, and some other information, it will render the view into a buffer that can be displayed on the screen. The map and textures are loaded from the `res` directory.

### Building and Running

If you have the SDL2 libraries on your computer, then you should be good to go with a simple `cargo run`. It is recommended that you specify the `--release` flag, as the unoptimized code for this project runs quite slow.

If you are on Windows, you will need the SDL2 and SDL2 image .dlls in the working directory of the executable.

### Resources

I do now own the example textures used, as they are from the game Blood, and are owned by Monolith Productions. Find more [here](http://www.bghq.com/textures.php?game=blood).
