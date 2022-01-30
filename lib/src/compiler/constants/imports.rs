use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Import {
	pub function: String,
	pub location: String,
}

impl Import {
	pub fn new(location: &'static str, function: &'static str) -> Import {
		Import {
			location: location.to_string(),
			function: function.to_string(),
		}
	}
}

lazy_static! {
	pub static ref SPACE: Import = Import::new("svelte/internal", "space");
	pub static ref TEXT: Import = Import::new("svelte/internal", "text");
	pub static ref ELEMENT: Import = Import::new("svelte/internal", "element");
	pub static ref NOOP: Import = Import::new("svelte/internal", "noop");
	pub static ref COMPONENT: Import = Import::new("svelte/internal", "SvelteComponent");
	pub static ref SAFE_NOT_EQUAL: Import = Import::new("svelte/internal", "safe_not_equal");
	pub static ref INSERT: Import = Import::new("svelte/internal", "insert");
	pub static ref DETACH: Import = Import::new("svelte/internal", "detach");
	pub static ref INIT: Import = Import::new("svelte/internal", "init");
	pub static ref ATTR: Import = Import::new("svelte/internal", "attr");
	pub static ref DESTROY_COMPONENT: Import = Import::new("svelte/internal", "destroy_component");
	pub static ref TRANSITION_IN: Import = Import::new("svelte/internal", "transition_in");
	pub static ref TRANSITION_OUT: Import = Import::new("svelte/internal", "transition_out");
	pub static ref MOUNT_COMPONENT: Import = Import::new("svelte/internal", "mount_component");
	pub static ref CREATE_COMPONENT: Import = Import::new("svelte/internal", "create_component");
}