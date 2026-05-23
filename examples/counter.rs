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

    osc = Osc~ [200];

    dac = Dac~; 

    a = Number {0.5} # KeyA; 
    b = Number {2.0} # KeyB; 

    mult = Mult~ [0.5]; 

    a -> mult; 
    b -> mult; 

    osc => mult; 

    mult => dac; 

);
