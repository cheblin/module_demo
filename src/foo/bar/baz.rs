#[path = "../../some_dir/some_src.rs"]
mod some_mod;

fn main_fn() {}

pub fn pub_fn() {
	crate::use_main_fn::pub_fn();
	some_mod::pub_fn();
	Baz{};
}

pub(in super) struct Baz;
