#[macro_export]
macro_rules! decl_module2 {
	// entry point
	(
		pub struct $module:ident {
			$($rest:tt)*
		}
	) => {
		$crate::decl_module2!(@parse
			pub struct $module
			{}
			{}
			$($rest)*
		);
	};
	// on_initialize is defined
	(@parse
		pub struct $module:ident
		{} // on_initialize
		{ $( $on_finalize:tt )* }
		fn on_initialize( $( $param_name:ident : $param:ty ),* $(,)? ) { $( $impl:tt )* }
		$($rest:tt)*
	) => {
		$crate::decl_module2!(@parse
			pub struct $module
			{
				fn on_initialize( $( $param_name : $param ),* ) { $( $impl )* }
			}
			{ $( $on_finalize )* }
			$($rest)*
		);
	};
	// on_finalize is defined
	(@parse
		pub struct $module:ident
		{ $( $on_initialize:tt )* }
		{} // on_finalize
		fn on_finalize( $( $param_name:ident : $param:ty ),* $(,)? ) { $( $impl:tt )* }
		$($rest:tt)*
	) => {
		$crate::decl_module2!(@parse
			pub struct $module
			{ $( $on_initialize )* }
			{
				fn on_finalize( $( $param_name : $param ),* ) { $( $impl )* }
			}
			$($rest)*
		);
	};
	// on_finalize is not defined
	(@parse
		pub struct $module:ident
		{ $( $on_initialize:tt )* }
		{} //on_finalize
		$($rest:tt)*
	) => {
		$crate::decl_module2!(@parse
			pub struct $module
			{ $( $on_initialize )* }
			{
				fn on_finalize() { println!("On Finalize Not Implemented!"); }
			}
			$($rest)*
		);
	};
	// on_initialize is not defined
	(@parse
		pub struct $module:ident
		{} //on_initialize
		{ $( $on_finalize:tt )* }
		$($rest:tt)*
	) => {
		$crate::decl_module2!(@parse
			pub struct $module
			{
				fn on_initialize() { println!("On Initialize Not Implemented!"); }
			}
			{ $( $on_finalize )* }
			$($rest)*
		);
	};
	// on_initialize and on_finalize is 100% captured, so we can implement the traits
	(@parse
		pub struct $module:ident
		{ $( $on_initialize:tt )* }
		{ $( $on_finalize:tt )* }
		$($rest:tt)*
	) => {
		pub struct $module {}

		impl $module {
			$($rest)*
		}

		impl $crate::traits::OnInitialize for $module {
			$( $on_initialize )*
		}

		impl $crate::traits::OnFinalize for $module {
			$( $on_finalize )*
		}
	}

}
