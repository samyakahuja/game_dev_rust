# GameDev in Rust

## Libraries

Instead of using a bigger framework, like `ggez`, going to write a lot
of abstractions, that these game engines provide, ourselves.

|  Crate  | Summary                              |
|:-------:|--------------------------------------|
| `sdl2`  | rendering, window management, events |
| `specs` | entity-component system              |
| `rand`  | random-number generation             |
| `rayon` | data parallelism                     |

## Requirements

Install these three sdl2 related libraries using your system's package manager.

- `sdl2`
- `sdl2_image`
- `sdl2_ttf`

The assets are by [Jason Perry](http://finalbossblues.com/timefantasy/category/freebies/)

## Notes

> See the [Specs Book](https://specs.amethyst.rs/docs/tutorials/) for detailed explanations.

- `MovementComponent` component stores the sprites for each frame of
  animation so that we don't have to compute all the frames every time we
  render. It might not be memory efficient but it works.
- Events cannot be components because we don't know what entities they are
  associated with until we process them. Instead we send events to the system
  through a resource.
- `Keyboard` is added as a dependency for `Physics` and `Animator`, since we
  want the player's velocity to be set before any physics or animation happens.
- Canvas scale on resize
    + [resizable](https://docs.rs/sdl2/0.32.2/sdl2/video/struct.WindowBuilder.html#method.resizable)
    + [set_scale](https://docs.rs/sdl2/0.32.2/sdl2/render/struct.Canvas.html#method.set_scale)
    + [WindowEvent::Resized](https://docs.rs/sdl2/0.32.2/sdl2/event/enum.WindowEvent.html#variant.Resized)
