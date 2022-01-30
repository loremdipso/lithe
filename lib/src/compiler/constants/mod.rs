use lazy_static::lazy_static;

pub mod imports;

pub const LITHE_NAME: &'static str = "Lithe";
pub const LITHE_VERSION: &'static str = "v0.2";

lazy_static! {
	pub static ref LITHE_COMMENT: String = format!(
		"/* generated by {} {} */",
		LITHE_NAME,
		LITHE_VERSION
	);
}

pub const CREATE_FRAGMENT: &'static str = "create_fragment";
pub const TARGET: &'static str = "target";
pub const ANCHOR: &'static str = "anchor";
pub const DETACHING: &'static str = "detaching";
pub const NULL: &'static str = "null";
pub const INSTANCE: &'static str = "instance";
pub const CURRENT: &'static str = "current";
pub const LOCAL: &'static str = "local";
pub const GLOBAL_SELF: &'static str = "$$self";
pub const GLOBAL_FRAGMENT: &'static str = "$$.fragment";
pub const GLOBAL_PROPS: &'static str = "$$props";
pub const GLOBAL_INVALIDATE: &'static str = "$$invalidate";
pub const PROPS: &'static str = "props";
pub const CLASS: &'static str = "class";
pub const TEXT_CONTENT: &'static str = "textContent";