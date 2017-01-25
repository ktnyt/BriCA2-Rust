#[macro_export]
macro_rules! delegate {
    { for $base:ident; $($tt:tt)* } => { delegate! { @expand $base; $($tt)* } };

    { @expand $base:ident; fn $func:ident($($args:tt)*); $($rest:tt)* } =>
    { delegate! { @expand $base; fn $func($($args)*) -> (); $($rest)* } };

    { @expand $base:ident; fn $func:ident(self $(, $name:ident : $ty:ty)*) -> $ret:ty; $($rest:tt)* } => {
        fn $func(self $(, $name:$ty)*) -> $ret { self.$base.$func($($name),*) }
        delegate! { @expand $base; $($rest)* }
    };

    { @expand $base:ident; fn $func:ident(&self $(, $name:ident : $ty:ty)*) -> $ret:ty; $($rest:tt)* } => {
        fn $func(&self $(, $name:$ty)*) -> $ret { self.$base.$func($($name),*) }
        delegate! { @expand $base; $($rest)* }
    };

    { @expand $base:ident; fn $func:ident(&mut self $(, $name:ident : $ty:ty)*) -> $ret:ty; $($rest:tt)* } => {
        fn $func(&mut self $(, $name:$ty)*) -> $ret { self.$base.$func($($name),*) }
        delegate! { @expand $base; $($rest)* }
    };

    { @expand $base:ident; pub fn $func:ident($($args:tt)*); $($rest:tt)* } =>
    { delegate! { @expand $base; pub fn $func($($args)*) -> (); $($rest)* } };

    { @expand $base:ident; pub fn $func:ident(self $(, $name:ident : $ty:ty)*) -> $ret:ty; $($rest:tt)* } => {
        pub fn $func(self $(, $name:$ty)*) -> $ret { self.$base.$func($($name),*) }
        delegate! { @expand $base; $($rest)* }
    };

    { @expand $base:ident; pub fn $func:ident(&self $(, $name:ident : $ty:ty)*) -> $ret:ty; $($rest:tt)* } => {
        pub fn $func(&self $(, $name:$ty)*) -> $ret { self.$base.$func($($name),*) }
        delegate! { @expand $base; $($rest)* }
    };

    { @expand $base:ident; pub fn $func:ident(&mut self $(, $name:ident : $ty:ty)*) -> $ret:ty; $($rest:tt)* } => {
        pub fn $func(&mut self $(, $name:$ty)*) -> $ret { self.$base.$func($($name),*) }
        delegate! { @expand $base; $($rest)* }
    };

    { @expand $base:ident; } => {};
}
