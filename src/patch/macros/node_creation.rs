#[macro_export]
macro_rules! create_node {
    ($patch:ident $($name:ident)* | $node:ty | $({$($node_args:tt)*})? $(| $($inputs_n:ident)+)? $(|# $($inputs_f:ident)+)?) => {

        initialize_node!(node | $node | $({$($node_args)*})?);


        let mut input: Option<fn(ButtonInput<KeyCode>) -> bool> = None;

        $(
            input = Some(|keys| {
                $(keys.pressed(KeyCode::$inputs_n) && )* true
            });
        )?

        $(
            input = Some(|keys| {
                inputs_f!(keys $($inputs_f)*)
            });
        )?


        $(
        let mut $name = $patch.create_node(node.clone()).with_input_maybe(input);
        )*

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

pub use create_node;
pub use filter_args;
pub use initialize_node;
pub use inputs_f;
