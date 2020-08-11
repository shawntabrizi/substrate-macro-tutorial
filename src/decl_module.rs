pub use crate::traits::OnInitialize;

#[macro_export]
macro_rules! decl_module {
    // entry point
    (
        pub struct $module:ident {
            $($rest:tt)*
        }
    ) => {
        $crate::decl_module!(@parse
            pub struct $module
            {}
            $($rest)*
        );
    };
    // on_initialize is defined
    (@parse
        pub struct $module:ident
        {} // on_initialize
        fn on_initialize( $( $param_name:ident : $param:ty ),* $(,)? ) { $( $impl:tt )* }
        $($rest:tt)*
    ) => {
        $crate::decl_module!(@parse
            pub struct $module
            {
                fn on_initialize( $( $param_name : $param ),* ) { $( $impl )* }
            }
            $($rest)*
        );
    };
    // on_initialize is not defined
    (@parse
        pub struct $module:ident
		{} //on_initialize
		$($rest:tt)*
    ) => {
        $crate::decl_module!(@parse
            pub struct $module
            {
                fn on_initialize() { println!("Not Implemented!"); }
			}
			$($rest)*
        );
    };
    // on_initialize is 100% captured, so we can implement the trait
    (@parse
        pub struct $module:ident
        { $( $on_initialize:tt )* }
        $($rest:tt)*

    ) => {
        pub struct $module {}

        impl $module {
            $(rest)*
        }

        impl $crate::traits::OnInitialize for $module {
            $( $on_initialize )*
        }
    }

}
