#[macro_export]
macro_rules! delegate {
    { to $base:ident; $($tt:tt)* } =>
    { delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident fn $func:ident(self); $($tt:tt)* } =>
    { fn $func(self) { self.$base.$func(); } delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident fn $func:ident(&self); $($tt:tt)* } =>
    { fn $func(&self) { self.$base.$func(); } delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident fn $func:ident(&mut self); $($tt:tt)* } =>
    { fn $func(&mut self) { self.$base.$func(); } delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident fn $func:ident(self) -> $ret:ty; $($tt:tt)* } =>
    { fn $func(self) -> $ret { self.$base.$func() } delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident fn $func:ident(&self) -> $ret:ty; $($tt:tt)* } =>
    { fn $func(&self) -> $ret { self.$base.$func() } delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident fn $func:ident(&mut self) -> $ret:ty; $($tt:tt)* } =>
    { fn $func(&mut self) -> $ret { self.$base.$func() } delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident fn $func:ident(self, $($name:ident : $ty:ty),*); $($tt:tt)* } =>
    { fn $func(self, $($name:$ty),*) { self.$base.$func($($name),*); } delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident fn $func:ident(&self, $($name:ident : $ty:ty),*); $($tt:tt)* } =>
    { fn $func(&self, $($name:$ty),*) { self.$base.$func($($name),*); } delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident fn $func:ident(&mut self, $($name:ident : $ty:ty),*); $($tt:tt)* } =>
    { fn $func(&mut self, $($name:$ty),*) { self.$base.$func($($name),*); } delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident fn $func:ident(self, $($name:ident : $ty:ty),*) -> $ret:ty; $($tt:tt)* } =>
    { fn $func(self, $($name:$ty),*) -> $ret { self.$base.$func($($name),*) } delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident fn $func:ident(&self, $($name:ident : $ty:ty),*) -> $ret:ty; $($tt:tt)* } =>
    { fn $func(&self, $($name:$ty),*) -> $ret { self.$base.$func($($name),*) } delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident fn $func:ident(&mut self, $($name:ident : $ty:ty),*) -> $ret:ty; $($tt:tt)* } =>
    { fn $func(&mut self, $($name:$ty),*) -> $ret { self.$base.$func($($name),*) } delegate! { @expand_fn $base $($tt)* } };

    { @expand_fn $base:ident } => {};

    { @expand_fn $base:ident fn $func:ident(self); } =>
    { fn $func(self) { self.$base.$func(); } };

    { @expand_fn $base:ident fn $func:ident(&self); } =>
    { fn $func(&self) { self.$base.$func(); } };

    { @expand_fn $base:ident fn $func:ident(&mut self); } =>
    { fn $func(&mut self) { self.$base.$func(); } };

    { @expand_fn $base:ident fn $func:ident(self) -> $ret:ty; } =>
    { fn $func(self) -> $ret { self.$base.$func() } };

    { @expand_fn $base:ident fn $func:ident(&self) -> $ret:ty; } =>
    { fn $func(&self) -> $ret { self.$base.$func() } };

    { @expand_fn $base:ident fn $func:ident(&mut self) -> $ret:ty; } =>
    { fn $func(&mut self) -> $ret { self.$base.$func() } };

    { @expand_fn $base:ident fn $func:ident(self, $($name:ident : $ty:ty),*); } =>
    { fn $func(self, $($name:$ty),*) { self.$base.$func($($name),*); } };

    { @expand_fn $base:ident fn $func:ident(&self, $($name:ident : $ty:ty),*); } =>
    { fn $func(&self, $($name:$ty),*) { self.$base.$func($($name),*); } };

    { @expand_fn $base:ident fn $func:ident(&mut self, $($name:ident : $ty:ty),*); } =>
    { fn $func(&mut self, $($name:$ty),*) { self.$base.$func($($name),*); } };

    { @expand_fn $base:ident fn $func:ident(self, $($name:ident : $ty:ty),*) -> $ret:ty; } =>
    { fn $func(self, $($name:$ty),*) -> $ret { self.$base.$func($($name),*) } };

    { @expand_fn $base:ident fn $func:ident(&self, $($name:ident : $ty:ty),*) -> $ret:ty; } =>
    { fn $func(&self, $($name:$ty),*) -> $ret { self.$base.$func($($name),*) } };

    { @expand_fn $base:ident fn $func:ident(&mut self, $($name:ident : $ty:ty),*) -> $ret:ty; } =>
    { fn $func(&mut self, $($name:$ty),*) -> $ret { self.$base.$func($($name),*) } };
}
