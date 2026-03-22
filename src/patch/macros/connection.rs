#[macro_export]
macro_rules! connect_1 {
    ($patch:ident $($outlets:ident $([$outs:expr])?),* -> $inlet:ident $([$in:expr])?) => {
        let inlet = get_inlet!($inlet $([$in])?);
        $($patch.connect(get_outlet!($outlets $([$outs])?), inlet.clone());)*
    };
    ($patch:ident $($outlets:ident $([$outs:expr])?),* => $inlet:ident $([$in:expr])?) => {
        let inlet = get_inlet_signal!($inlet $([$in])?);
        $($patch.connect_signal(get_outlet_signal!($outlets $([$outs])?), inlet.clone());)*
    };
}

#[macro_export]
macro_rules! connect_2 {
    ($patch:ident $outlet:ident $([$out:expr])? -> $($inlets:ident $([$ins:expr])?),*) => {
        let outlet = get_outlet!($outlet $([$out])?);
        $($patch.connect(outlet.clone(), get_inlet!($inlets $([$ins])?));)*
    };
    ($patch:ident $outlet:ident $([$out:expr])? => $($inlets:ident $([$ins:expr])?),*) => {
        let outlet = get_outlet_signal!($outlet $([$out])?);
        $($patch.connect_signal(outlet.clone(), get_inlet_signal!($inlets $([$ins])?));)*
    };
}

#[macro_export]
macro_rules! get_inlet {
    ($inlet:ident) => {
        $inlet.inlet::<0>();
    };
    ($inlet:ident[$inl:expr]) => {
        $inlet.inlet::<$inl>();
    };
}

#[macro_export]
macro_rules! get_inlet_signal {
    ($inlet:ident) => {
        $inlet.inlet_signal::<0>();
    };
    ($inlet:ident[$inl:expr]) => {
        $inlet.inlet_signal::<$inl>();
    };
}

#[macro_export]
macro_rules! get_outlet {
    ($outlet:ident) => {
        $outlet.outlet::<0>();
    };
    ($outlet:ident[$out:expr]) => {
        $outlet.outlet::<$out>();
    };
}

#[macro_export]
macro_rules! get_outlet_signal {
    ($outlet:ident) => {
        $outlet.outlet_signal::<0>();
    };
    ($outlet:ident[$out:expr]) => {
        $outlet.outlet_signal::<$out>();
    };
}

pub use connect_1;
pub use connect_2;
pub use get_inlet;
pub use get_inlet_signal;
pub use get_outlet;
pub use get_outlet_signal;
