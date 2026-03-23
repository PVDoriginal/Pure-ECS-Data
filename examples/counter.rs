use bevy::prelude::*;
use pure_ecs_data::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PureDataPlugin));
    app.add_patch(counter);
    app.run();
}

patch!(
    counter;

    trigger = Trigger {bang, 1.5, "lol"};

    bang = Bang # Space;
    f = F;

    bang -> f;

    add1 = Sum<2> [100];
    f -> add1;

    print = Print;

    add1 -> f[1], print;
);
