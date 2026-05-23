# Pure ECS Data

A lazy implementation of [PD (Pure Data)](https://puredata.info/) built on top of [Bevy](https://bevy.org/)'s ECS engine and Rust's macro system. 

## How to use 

To get started, add the crate as a dependency in your project's `Cargo.toml` file: 
```Toml 
pure_ecs_data = {git = "https://github.com/PVDoriginal/Pure-ECS-Data"}
```
(You'll need to grab the git repo directly since the crate isn't oficially published at this moment)

Here's a minimal example of using this crate: 

```Rs
use bevy::prelude::*;
use pure_ecs_data::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PureDataPlugin));
    app.add_patch(hello_world);
    app.run();
}

patch!(
  hello_world;

  bang = Bang |# Space;
  print = Print {"Hello World!"};

  bang -> print; 
);
```

You can either run your program with `cargo run`, or with `BEVY_ASSET_ROOT="." dx serve --hot-patch --features "bevy/hotpatching"`, which will make it hotpatch-able (the patch can be modified while the prorgam is running). 

## The `patch!` macro 

The `patch!` macro is a sequential list of instructions, separated by `;`. 

The first line of the macro should always be the patch's name, which can then be added into the Bevy app via `app.add_patch()`. 

The rest of the lines can be put into two clear categories: `Constructors` and `Connections`. 

### Constructors 

A constructor is an instruction which will create a new PD Node and bind it to the given name. 
For instance, `a = Bang` creates a new `Bang` node which you can now refer to as `a`. 

There are a few modifiers which can be used when constructing a node: 

- A `|` can be written at the end, followed by a list of keys, specifying the node will be activated each frame that all of the keys are pressed. Alternatively, `#` acts similarly but makes it so the node is 
activated only on frames where all the keys are currently pressed, and the last key in the sequence has been *just* pressed. For instance, `bang = Bang | Enter` will create a `Bang` which is active while `Enter` is held down, while `bang = Bang # Enter` will create one that only activated when the `Enter` key was just pressed. 

- A set of parameters inside a `[]` can be passed, signifying the default values placed on the inlets of that node, usually from right to left. A default inlet value can also be assigned after construction by writing `name[x] <- value`, where `name` is the name of the node, `x` is the index of the inlet, and `value` is the value you wish to place there. For instance, `Sum<2>` is a node with 2 inlets, the values of which are summed together when a value is received on the left-most inlet (like in PD!). 
`sum = Sum<2> [2]` will create a node which adds 2 to each value passed into its first inlet.  

- Finally, for a few nodes, immediately following the node's type, you can pass a set of arguments inside a `{}`. These are values that are actually stored inside the node, which are used for nodes such as `Number` and `Comment`. 

### Connections 

A connection is something of the form `a[x] -> b[y]`, meaning that the x-th control outlet of a goes into the y-th control inlet of b. This can be changed to signal inlets / outlets instead by wriing `=>` instead of `->`. 

You can also write `a[x] -> b[y], c[z], d[w]`, and `a[x], b[y], c[z] -> d[w]`, connecting one inlet or outlet to multiple inlets or outlets at once. 

Also, the outlet/inlet index can be omitted, being automatically assumed to be 0. For instance, `a -> b[x]` will connect the first inlet of a into the x-th inlet of b.
