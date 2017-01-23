#[macro_export]
macro_rules! delegate {
    { to $base:ident; $($tt:tt)* } => { delegate! { @expand $base $($tt)* } };

    { @expand $base:ident fn $func:ident(self $(, $name:ident : $ty:ty)*) -> $ret:ty; $($rest:tt)* } => {
        fn $func(self $(, $name:$ty)*) -> $ret { self.$base.$func($($name),*) }
        delegate! { @expand $base $($rest)* }
    };

    { @expand $base:ident fn $func:ident(self $(, $name:ident : $ty:ty)*); $($rest:tt)* } => {
        fn $func(self $(, $name:$ty)*) { self.$base.$func($($name),*); }
        delegate! { @expand $base $($rest)* }
    };

    { @expand $base:ident fn $func:ident(&self $(, $name:ident : $ty:ty)*) -> $ret:ty; $($rest:tt)* } => {
        fn $func(&self $(, $name:$ty)*) -> $ret { self.$base.$func($($name),*) }
        delegate! { @expand $base $($rest)* }
    };

    { @expand $base:ident fn $func:ident(&self $(, $name:ident : $ty:ty)*); $($rest:tt)* } => {
        fn $func(&self $(, $name:$ty)*) { self.$base.$func($($name),*); }
        delegate! { @expand $base $($rest)* }
    };

    { @expand $base:ident fn $func:ident(&mut self $(, $name:ident : $ty:ty)*) -> $ret:ty; $($rest:tt)* } => {
        fn $func(&mut self $(, $name:$ty)*) -> $ret { self.$base.$func($($name),*) }
        delegate! { @expand $base $($rest)* }
    };

    { @expand $base:ident fn $func:ident(&mut self $(, $name:ident : $ty:ty)*); $($rest:tt)* } => {
        fn $func(&mut self $(, $name:$ty)*) { self.$base.$func($($name),*); }
        delegate! { @expand $base $($rest)* }
    };

    { @expand $base:ident } => {};
}
