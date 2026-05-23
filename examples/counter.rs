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

    f = F [60];

    midi = MtoF;

    select = Select<3>;

    bang -> f;

    add1 = Sum<2> [1.];
    f -> add1, midi;

    print = Print;
    print1 = Print {"midi"};
    print_nothing = Print;
    print_msg = Print {"test"};

    bang -> print_nothing, print_msg;

    add1 -> f[1], print;
    midi -> print1; // <- maybe also try to print a specific print message?

);
