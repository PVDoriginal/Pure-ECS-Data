use bevy::prelude::*;
use pure_ecs_data::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PureDataPlugin));
    app.add_patch(piano_test);
    app.run();
}

patch!(
    piano_test;

    keyboard_a = Bang # KeyA;
    keyboard_s = Bang # KeyS;
    keyboard_d = Bang # KeyD;
    keyboard_f = Bang # KeyF;
    keyboard_g = Bang # KeyG;
    keyboard_h = Bang # KeyH;
    keyboard_j = Bang # KeyJ;




    midi_do = Number [60];
    midi_re  = Number [62];
    midi_mi  = Number [64];
    midi_fa  = Number [65];
    midi_sol = Number [67];
    midi_la  = Number [69];
    midi_si  = Number [71];

    keyboard_a  -> midi_do;
    keyboard_s  -> midi_re;
    keyboard_d  -> midi_mi;
    keyboard_f  -> midi_fa;
    keyboard_g  -> midi_sol;
    keyboard_h  -> midi_la;
    keyboard_j  -> midi_si;

    freq = MtoF;

    midi_do  -> freq;
    midi_re  -> freq;
    midi_mi  -> freq;
    midi_fa  -> freq;
    midi_sol -> freq;
    midi_la  -> freq;
    midi_si  -> freq;

    print_freq = Print {"Freq: "};
    osc = Osc~;

    freq -> osc, print_freq;

    mult_envelope = Mult~ [1];
    mult_global_volume = Mult~ [0.1];

    osc => mult_envelope;

    dac = Dac~;

    mult_envelope => mult_global_volume;
    mult_global_volume => dac[0], dac[1];
);
