# Rustcaster

This is a fairly lightweight raycaster, which supports ceilings, floors, and sprites. It uses SDL2 for windowing, rendering, and image loading.

![Rustcaster](http://declanhopkins.com/static/images/screenshots/rustcaster.png)

Most of the actual raycasting code is contained within the internal `rustcaster-lib` library. By supplying the `render` function with a map, camera values, and some other information, it will render the view into a buffer that can be displayed on the screen. The map and textures are loaded from the `res` directory.