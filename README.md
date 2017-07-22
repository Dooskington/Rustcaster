# Rustcaster

This is a fairly lightweight raycaster, which supports ceilings, floors, and sprites. It uses SDL2 for windowing, rendering, and image loading.

![Rustcaster](http://declanhopkins.com/static/images/screenshots/rustcaster.png)

Most of the actual raycasting code is contained within the internal `rustcaster-lib` library. By supplying the `render` function with a map, camera values, and some other information, it will render the view into a buffer that can be displayed on the screen. The map and textures are loaded from the `res` directory.

## Building and Running

If you have the SDL2 libraries on your computer, then you should be good to go with a simple `cargo run`. It is reccomended that you specify the `--release` flag, as the unoptimized code for this project runs quite slow.

If you are on Windows, you will need the SDL2 and SDL2 image .dlls in the working directory of the executable.