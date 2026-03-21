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

    bang = Bang # Space;
    print = Print;
    plus = Sum<2> [1];

    f = F;

    bang -> f;

    f -> plus;
    plus -> f[1], print;
);
