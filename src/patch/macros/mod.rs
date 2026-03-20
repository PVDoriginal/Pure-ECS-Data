pub mod node_creation;

#[macro_export]
macro_rules! patch_instruction {
    ($patch:ident) => {};
    ($patch:ident $($var_name:ident),* = $node_type:ty $({$($node_args:tt)*})?; $($t:tt)*) => {
        create_node!($patch $($var_name)* | $node_type | $({$($node_args)*})?);
        patch_instruction!($patch $($t)*);
    };
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

pub use node_creation::*;
pub use patch;
pub use patch_instruction;
