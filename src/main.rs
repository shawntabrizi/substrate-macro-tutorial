#[macro_use]
mod decl_module;
#[macro_use]
mod decl_module2;
mod traits;

fn main() {
	println!("Hello, world!");
}

#[test]
fn decl_module_works() {
	decl_module!(
		pub struct Module1 {}
	);

	decl_module!(
		pub struct Module2 {
			fn on_initialize() {
				println!("Hello!");
			}
		}
	);

	decl_module!(
		pub struct Module3 {
			fn on_initialize() {
				println!("Hello!");
			}

			fn other_function() {
				println!("This is a different function");
			}
		}
	);

	use crate::traits::OnInitialize;

	Module1::on_initialize();
	Module2::on_initialize();
	Module3::other_function();
}

#[test]
fn decl_module2_works() {
	decl_module2!(
		pub struct Module1 {}
	);

	decl_module2!(
		pub struct Module2 {
			fn on_finalize() {
				println!("Final Hello!");
			}
		}
	);

	decl_module2!(
		pub struct Module3 {
			fn on_initialize() {
				println!("Initial Hello!");
			}
		}
	);

	decl_module2!(
		pub struct Module4 {
			fn on_initialize() {
				println!("Initial Hello!");
			}
			fn on_finalize() {
				println!("Final Hello!");
			}
		}
	);

	use crate::traits::OnInitialize;
	use crate::traits::OnFinalize;

	Module1::on_initialize();
	Module1::on_finalize();

	Module2::on_initialize();
	Module2::on_finalize();

	Module3::on_initialize();
	Module3::on_finalize();

	Module4::on_initialize();
	Module4::on_finalize();
}
