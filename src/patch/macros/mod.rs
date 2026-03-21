pub mod connection;
pub mod nodes;

#[macro_export]
macro_rules! patch_instruction {
    ($patch:ident) => {};
    ($patch:ident $($var_name:ident),* = $node_type:ty $({$($node_args:tt)*})? $([$($inlet_data:tt),+])? $(| $($inputs_n:ident),+)? $(|# $($inputs_f:ident),+)?; $($t:tt)*) => {
        create_node!($patch $($var_name)* | $node_type | $({$($node_args)*})? $([$($inlet_data)+])? $(| $($inputs_n)*)? $(|# $($inputs_f)+)?);
        patch_instruction!($patch $($t)*);
    };
    ($patch:ident $($outlets:ident $([$outs:expr])?),* -> $inlet:ident $([$in:expr])?; $($t:tt)*) => {
        connect_1!($patch $($outlets $([$outs])?),* -> $inlet $([$in])?);
        patch_instruction!($patch $($t)*);
    };
    ($patch:ident $outlet:ident $([$out:expr])? -> $($inlets:ident $([$ins:expr])?),*; $($t:tt)*) => {
        connect_2!($patch $outlet $([$out])? -> $($inlets $([$ins])?),*);
        patch_instruction!($patch $($t)*);
    };
    ($patch:ident $inlet:ident [$index:expr] <- $data:tt ; $($t:tt)*) => {
        bind_inlets!($patch $inlet [$index] <- $data);
        patch_instruction!($patch $($t)*);
    };
    ($patch:ident $inlet:ident <- $($data:tt),* ; $($t:tt)*) => {
        bind_inlets!($patch $inlet <- $($data)*);
        patch_instruction!($patch $($t)*);
    }
}

#[macro_export]
macro_rules! patch {
    ($patch_name:ident; $($tail:tt)*) => {
        paste! {
            fn $patch_name() -> Patch {
                let mut [<var_ $patch_name>] = Patch::default();
                patch_instruction!([<var_ $patch_name>] $($tail)*);
                [<var_ $patch_name>]
            }
        }
    };
}

pub use patch;
pub use patch_instruction;
pub use {connection::*, nodes::*};
