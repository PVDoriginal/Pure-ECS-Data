#[macro_export]
macro_rules! create_node {
    ($patch:ident $($name:ident)* | $node:ty | $({$($node_args:tt)*})? $([$($inlet_data:tt)+])? $(| $($inputs_n:ident)+)? $(|# $($inputs_f:ident)+)?) => {

        initialize_node!(node | $node | $({$($node_args)*})?);

        let mut input: Option<Input> = None;

        $(
            input = Some(
                Input {
                    keys: [$(KeyCode::$inputs_n)*].to_vec(),
                    once: false,
                    input: |keys| {
                        $(keys.pressed(KeyCode::$inputs_n) && )* true
                    }
                }
            )
        )?

        $(
            input = Some(
                Input {
                    keys: [$(KeyCode::$inputs_f)*].to_vec(),
                    once: true,
                    input: |keys: ButtonInput<KeyCode>| {
                        inputs_f!(keys $($inputs_f)*)
                    }
                }

            );
        )?


        $(
        let $name = $patch.create_node(stringify!($name).into(), node.clone()).with_input_maybe(input).id();
        )*

        let nodes = [$($name.clone())*];

        $(
            for node in nodes {
                bind_inlets!($patch node <- $($inlet_data)+);
            }
        )?
    };
}

#[macro_export]
macro_rules! inputs_f {
    ($keys:ident $input:ident) => {
        $keys.just_pressed(KeyCode::$input) && true
    };
    ($keys:ident $input:ident $($tail:ident)+) => {
        $keys.pressed(KeyCode::$input) && inputs_f!($keys $($tail)*)
    }
}

#[macro_export]
macro_rules! filter_args {
    () => {};
    (bang) => {
        Data::Bang
    };
    ($head:expr) => {
        Data::from($head)
    };
}

#[macro_export]
macro_rules! initialize_node {
    ($name: ident | $node:ty | ) => {
        let $name = <$node>::default();
    };
    ($
        name: ident | $node:ty | {$head:tt, $($tail:tt),*}) => {
        let args = [filter_args!($head), $(filter_args!($tail)),*];
        let $name = <$node>::from(args);
    };
    ($name: ident | $node:ty | {$head:tt}) => {
        let $name = <$node>::from(filter_args!($head));
    };
}

#[macro_export]
macro_rules! bind_inlets {
    ($patch:ident $inlet:ident [$index:expr] <- $data:tt) => {
        $patch.bind_data_inlet(get_inlet!($inlet[$index]), filter_args!($data));
    };
    ($patch:ident $inlet:ident <- $($data:tt)*) => {
        $patch.bind_data($inlet, [$(filter_args!($data)),*]);
    };
}

pub use bind_inlets;
pub use create_node;
pub use filter_args;
pub use initialize_node;
pub use inputs_f;
