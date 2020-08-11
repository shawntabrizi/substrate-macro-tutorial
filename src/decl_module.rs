pub use crate::traits::OnInitialize;

#[macro_export]
macro_rules! decl_module {
    // on_initialize is defined
    (
        pub struct $module:ident
        {} // on_initialize
        fn on_initialize( $( $param_name:ident : $param:ty ),* $(,)? ) { $( $impl:tt )* }
        $($rest:tt)*
    ) => {
        $crate::decl_module!(
            pub struct $module:ident
            {
                fn on_initialize( $( $param_name : $param ),* ) { $( $impl )* }
            }
            $($rest)*
        );
    };
    // on_initialize is not defined
    (
        pub struct $module:ident
		{} //on_initialize
		$($rest:tt)*
    ) => {
        $crate::decl_module!(
            pub struct $module
            {
                fn on_initialize() { println!("Not Implemented!"); }
			}
			$($rest)*
        );
    };
    // on_initialize is 100% captured, so we can implement the trait
    (
        pub struct $module:ident
        { $( $on_initialize:tt )* }
        $($rest:tt)*

    ) => {
        pub struct $module {}
        $($rest)*

        impl $crate::traits::OnInitialize for $module {
            $( $on_initialize )*
        }
    }

}
