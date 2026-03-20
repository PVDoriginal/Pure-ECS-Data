use bevy::prelude::*;
use pure_ecs_data::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PureDataPlugin));
    app.add_patch(counter);
    app.run();
}

use pure_ecs_data::node::nodes::Add;

patch!(
    counter;

    a, b = Print;

    c = Number { 32 };
    lol = Number { 5.3 };

    d = Trigger<4> { bang, bang, 3.0, "lol" };

    e = Add<3>;

    bang = Bang | Space, KeyA;
    bang2 = Bang |# Space, KeyB;
);

// fn counter() -> Patch {
//     let mut patch = Patch::default();

//     let bang = patch.create_node(Bang).with_input(keys_once!(Space)).id();
//     let f = patch.create_node(F::default()).id();

//     patch.connect(outlet!(bang, 0), inlet!(f, 0));

//     let add = patch
//         .create_node(Add::<2>)
//         .with_data([Num::Int(1).into()])
//         .id();

//     let print = patch.create_node(Print).id();

//     patch.connect(outlet!(f, 0), inlet!(add, 0));

//     patch.connect(outlet!(add, 0), inlet!(f, 1));

//     patch.connect(outlet!(add, 0), inlet!(print, 0));

//     patch
// }
