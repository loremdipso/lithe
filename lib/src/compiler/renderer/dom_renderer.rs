#![allow(unused_mut, unused_variables, dead_code, unused_imports)]
use crate::compiler::compiler::CompileResult;
use std::collections::HashMap;
use super::renderer::Renderer;
use crate::compiler::compiler::{Fragment, FragmentType, Compiler};
use crate::compiler::{constants, constants::imports};
use crate::compiler::utils::{script, strings, strings::HandlebarType};

#[derive(Debug)]
pub struct DomRenderer<'a> {
	pub compiler: &'a mut Compiler,
	extra_variables: HashMap<String, Vec<String>>,
}

impl Renderer for DomRenderer<'_> {
	fn render_fragments(&mut self, fragments: &Vec<Fragment>) -> CompileResult {
		// grab the pieces for create_fragment
		let variables = &self.render_variables(fragments);
		let creator = &self.render_fragments_creator(None, fragments);
		let mounter = &self.render_mounter(None, fragments);
		let p = &self.render_p(None, fragments);
		let transition_in = &self.render_transition_in(None, fragments);
		let transition_out = &self.render_transition_out(None, fragments);
		let detach = &self.render_detach(None, fragments);

		// create create_fragment
		let create_fragment = &self.render_create_fragment(
			fragments,
			variables,
			creator,
			mounter,
			p,
			transition_in,
			transition_out,
			detach
		);

		// define component
		let component = &self.render_component(fragments, create_fragment.len() > 0);

		// define instance
		let instance = &self.render_instance(fragments);

		// at this point all of our imports should be imported, so let's just put it all together
		let mut code: String = String::new();
		code.push_str(&constants::LITHE_COMMENT);
		code.push_str("\n");
		code.push_str(&self.compiler.render_imports());
		code.push_str("\n");
		code.push_str(&create_fragment);
		code.push_str("\n");
		if instance.len() > 0 {
			code.push_str(&instance);
			code.push_str("\n");
		}
		code.push_str(&component.trim());
		
		// TODO: fill out the rest of this return value
		let mut result = CompileResult::default();
		result.js.code = Some(code);
		return result;
	}
}

impl DomRenderer<'_> {
	pub fn new(compiler: &mut Compiler) -> DomRenderer {
		DomRenderer {
			compiler,
			extra_variables: HashMap::new()
		}
	}

	fn render_instance(&mut self, fragments: &Vec<Fragment>) -> String {
		let mut content = String::new();
		let mut variables: Vec<String> = vec![];

		for fragment in fragments {
			for fragment in fragment.into_iter() {
				if fragment.ttype == FragmentType::Script {
					// looks like we want to parse the script, extract the variables/exports, and move on
					todo!();
				}
			}
		}

		if !self.compiler.has_instance {
			return content
		} else {
			let mut result = String::new();
			result.push_str(&format!("function {}({}, {}, {}) {{\n",
				constants::INSTANCE,
				constants::GLOBAL_SELF,
				constants::GLOBAL_PROPS,
				constants::GLOBAL_INVALIDATE,
			));

			result.push_str(&strings::indent_block(&content, 1));

			// TODO: figure out the return value thing

			return result;
		}
	}

	fn get_exports_string(&mut self, fragments: &Vec<Fragment>) -> String {
		let exports = self.get_exports(fragments);
		if exports.len() == 0 {
			return "{}".to_string();
		}

		let mut result = String::new();
		result.push_str("{ ");
		for export in exports {
			result.push_str(&export);
		}
		result.push_str(" }");
		return result;
	}

	fn get_exports(&mut self, fragments: &Vec<Fragment>) -> Vec<String> {
		let mut exports = vec![];
		for fragment in fragments {
			for fragment in fragment.into_iter() {
				if fragment.ttype == FragmentType::Script {
					if let Some(data) = &fragment.data {
						let script_exports = script::extract_exports(data);
						exports.extend(script_exports);
					}
				}
			}
		}
		return exports;
	}

	fn render_component(&mut self, fragments: &Vec<Fragment>, did_render_fragment: bool) -> String {
		let mut result = String::new();
		let component_name = "Component"; // TODO: is this configurable?
		let instance = if self.compiler.has_instance {
			constants::INSTANCE
		} else {
			constants::NULL
		};

		self.compiler.insert_import(&imports::COMPONENT);
		result.push_str(&format!("class {} extends {} {{\n", component_name, imports::COMPONENT.function));
		result.push_str("\tconstructor(options) {\n");
		result.push_str("\t\tsuper();\n");

		let exports = self.get_exports_string(fragments);

		self.compiler.insert_import(&imports::INIT);
		self.compiler.insert_import(&imports::SAFE_NOT_EQUAL);
		result.push_str(&format!("\t\t{}(this, options, {}, {}, {}, {});\n",
			imports::INIT.function,
			instance,
			if did_render_fragment { constants::CREATE_FRAGMENT } else { constants::NULL },
			imports::SAFE_NOT_EQUAL.function,
			exports,
		));

		result.push_str("\t}\n");
		result.push_str("}\n\n");
		result.push_str(&format!("export default {};\n", component_name));

		return result;
	}

	fn render_create_fragment(
		&mut self,
		fragments: &Vec<Fragment>,
		variables: &str,
		creator: &str,
		mounter: &str,
		p: &str,
		transition_in: &str,
		transition_out: &str,
		detach: &str
	) -> String {
		let mut result = String::new();
		if fragments.len() == 0 || (
			variables.len() == 0 &&
			creator.len() == 0 &&
			mounter.len() == 0 &&
			p.len() == 0 &&
			transition_in.len() == 0 &&
			transition_out.len() == 0
		) {
			// special case: nothing here, we can quit early
			return result;
		}

		result.push_str(&format!("function {}(ctx) {{\n", constants::CREATE_FRAGMENT));
		result.push_str(variables);

		result.push_str("\n\treturn {\n");

		let indent_level = 3;

		// creator
		result.push_str("\t\tc() {\n");
		result.push_str(&strings::indent_block(creator, indent_level));
		result.push_str("\n\t\t},\n");

		// mounter
		result.push_str("\t\tm(target, anchor) {\n");
		result.push_str(&strings::indent_block(mounter, indent_level));
		result.push_str("\n\t\t},\n");

		// whatever p is
		if p.len() == 0 {
			self.compiler.insert_import(&imports::NOOP);
			result.push_str(&format!("\t\tp: {},\n", imports::NOOP.function));
		} else {
			todo!();
		}

		// transition in
		if transition_in.len() == 0 {
			self.compiler.insert_import(&imports::NOOP);
			result.push_str(&format!("\t\ti: {},\n", imports::NOOP.function));
		} else {
			result.push_str("\t\ti(local) {\n");
			result.push_str(&strings::indent_block(transition_in, indent_level));
			result.push_str("\n\t\t},\n");
		}

		// transition out
		if transition_out.len() == 0 {
			self.compiler.insert_import(&imports::NOOP);
			result.push_str(&format!("\t\to: {},\n", imports::NOOP.function));
		} else {
			result.push_str("\t\to(local) {\n");
			result.push_str(&strings::indent_block(transition_out, indent_level));
			result.push_str("\n\t\t},\n");
		}

		// detach
		if detach.len() == 0 {
			self.compiler.insert_import(&imports::NOOP);
			result.push_str(&format!("\t\td: {},\n", imports::NOOP.function));
		} else {
			result.push_str(&format!("\t\td({}) {{\n", constants::DETACHING));
			result.push_str(&strings::indent_block(detach, indent_level));
			result.push_str("\n\t\t}\n");
		}

		result.push_str("\t};\n");
		result.push_str("}\n");

		return result;
	}

	fn attributes_to_props(&self, fragment: &Fragment) -> String {
		let mut props = String::new();
		for attribute in &fragment.attributes {
			todo!();
		}
		return props;
	}

	fn render_mounter(&mut self, parent: Option<&Fragment>, fragments: &Vec<Fragment>) -> String {
		let mut result = String::new();
		for fragment in fragments {
			for fragment in fragment.into_iter() {
				match fragment.ttype {
					FragmentType::Basic | FragmentType::Text => {
						if fragment.has_parent && fragment.ttype == FragmentType::Text {
							continue;
						}
						self.compiler.insert_import(&imports::INSERT);
						result.push_str(&format!("{}({}, {}, {});\n",
							imports::INSERT.function,
							constants::TARGET,
							fragment.short_name,
							constants::ANCHOR,
						));
					}

					FragmentType::Svelte => {
						self.compiler.insert_import(&imports::MOUNT_COMPONENT);
						result.push_str(&format!("{}({}, {}, {});\n",
							imports::MOUNT_COMPONENT.function,
							fragment.short_name,
							constants::TARGET,
							constants::ANCHOR,
						));
					}

					_ => {} // no-op
				}
			}
		}

		if self.compiler.has_svelte_fragments {
			result.push_str(&format!("{} = true;\n", constants::CURRENT));
		}

		return result;
	}

	fn render_p(&mut self, parent: Option<&Fragment>, fragments: &Vec<Fragment>) -> String {
		// What even is p?
		// TODO
		return format!("");
	}

	fn render_transition_in(&mut self, parent: Option<&Fragment>, fragments: &Vec<Fragment>) -> String {
		let mut result = String::new();
		if !self.compiler.has_svelte_fragments {
			return result;
		}
		result.push_str(&format!("if ({}) return;\n", constants::CURRENT));

		for fragment in fragments {
			for fragment in fragment.into_iter() {
				if fragment.renderable {
					match fragment.ttype {
						FragmentType::Svelte => {
							self.compiler.insert_import(&imports::TRANSITION_IN);
							result.push_str(&format!("{}({}.{}, {});\n",
								imports::TRANSITION_IN.function,
								fragment.short_name,
								constants::GLOBAL_FRAGMENT,
								constants::LOCAL
							));
						}

						_ => {} // No-op
					}
				}
			}
		}

		result.push_str(&format!("{} = true;\n", constants::CURRENT));
		return result;
	}

	fn render_transition_out(&mut self, parent: Option<&Fragment>, fragments: &Vec<Fragment>) -> String {
		let mut result = String::new();
		if !self.compiler.has_svelte_fragments {
			return result;
		}

		for fragment in fragments {
			for fragment in fragment.into_iter() {
				if fragment.renderable {
					match fragment.ttype {
						FragmentType::Svelte => {
							self.compiler.insert_import(&imports::TRANSITION_OUT);
							result.push_str(&format!("{}({}.{}, {});\n",
								imports::TRANSITION_OUT.function,
								fragment.short_name,
								constants::GLOBAL_FRAGMENT,
								constants::LOCAL
							));
						}

						_ => {} // No-op
					}
				}
			}
		}

		result.push_str(&format!("{} = false;\n", constants::CURRENT));
		return result;
	}

	fn render_detach(&mut self, parent: Option<&Fragment>, fragments: &Vec<Fragment>) -> String {
		let mut complex_detach = String::new();
		let mut basic_detach = String::new();

		for fragment in fragments {
			for fragment in fragment.into_iter() {
				if fragment.renderable {
					match fragment.ttype {
						FragmentType::Basic | FragmentType::Text => {
							// special case: fragment is text and has no parent, so we're probably hosting it
							if fragment.has_parent && fragment.ttype == FragmentType::Text {
								continue;
							}

							self.compiler.insert_import(&imports::DETACH);
							basic_detach.push_str(&format!("{}({});\n",
								imports::DETACH.function,
								fragment.short_name,
							));
						}

						FragmentType::Svelte => {
							self.compiler.insert_import(&imports::DESTROY_COMPONENT);
							complex_detach.push_str(&format!("{}({}, {});\n",
								imports::DESTROY_COMPONENT.function,
								fragment.short_name,
								constants::DETACHING
							));
						}

						_ => {} // No-op
					}
				}
			}
		}

		let mut result = String::new();
		if basic_detach.len() > 0 {
			result.push_str(&format!("if ({}) {{\n", constants::DETACHING));
			result.push_str(&strings::indent_block(&basic_detach, 1));
			result.push_str("\n}\n");
		}
		if complex_detach.len() > 0 {
			result.push_str(&complex_detach);
			result.push_str("\n");
		}
		return result;
	}

	fn render_variables(&mut self, fragments: &Vec<Fragment>) -> String {
		let mut result = String::new();

		// basic variable definitions
		for fragment in fragments {
			for fragment in fragment.into_iter() {
				if fragment.renderable {
					// Special case: only parentless text-like elements get their own separate variable
					if fragment.ttype == FragmentType::Text && fragment.has_parent {
						continue;
					}

					match fragment.ttype {
						FragmentType::Basic |
						FragmentType::Svelte |
						FragmentType::Text
						=> {
							// TODO: which fragments should we exclude?
							result.push_str(&format!("\tlet {};\n", fragment.short_name));
							
							if let Some(extra_names) = self.extra_variables.get(&fragment.short_name) {
								for name in extra_names {
									result.push_str(&format!("\tlet {};\n", name));
								}
							}
						}
						_ => {}
					}
				}
			}
		}

		if self.compiler.has_svelte_fragments {
			// NOTE: let's assume we need current
			result.push_str(&format!("\tlet {};\n", constants::CURRENT));
		}

		// svelte variable definitions
		for fragment in fragments {
			for fragment in fragment.into_iter() {
				if fragment.ttype == FragmentType::Svelte {
					// TODO: what should this be?
					let mut args = String::new();
					let props = self.attributes_to_props(fragment);
					if props.len() > 0 {
						args.push_str(&format!(" {}: {} ", constants::PROPS, props));
					}

					// TODO: do we need extra names or anything here?
					result.push_str(&format!("\t{} = new {}({{{}}});\n",
						fragment.short_name,
						fragment.name,
						args
					));
				}
			}
		}

		return result;
	}

	fn render_fragments_creator(&mut self, parent: Option<&Fragment>, fragments: &Vec<Fragment>) -> String {
		let mut result: String = String::new();
		for fragment in fragments {
			if let Some(fragment) = self.render_fragment_creator(parent, fragment) {
				result.push_str(&fragment);
			}
		}
		return result;
	}

	fn render_fragment_creator(&mut self, parent: Option<&Fragment>, fragment: &Fragment) -> Option<String> {
		if !fragment.renderable {
			return None;
		}

		match fragment.ttype {
			FragmentType::Basic => Some(self.render_basic_fragment_creator(parent, fragment)),
			FragmentType::Svelte => Some(self.render_svelte_fragment_creator(parent, fragment)),
			FragmentType::Text => Some(self.render_text_fragment_creator(parent, fragment)),

			FragmentType::Style => todo!(),
			FragmentType::Script => todo!(),
		}
	}

	fn render_basic_fragment_creator(&mut self, parent: Option<&Fragment>, fragment: &Fragment) -> String {
		let mut result = String::new();
		self.compiler.insert_import(&imports::ELEMENT);
		result.push_str(&format!("{} = {}(\"{}\");\n",
			fragment.short_name,
			imports::ELEMENT.function,
			fragment.name
		));

		if fragment.attributes.len() > 0 {
			self.compiler.insert_import(&imports::ATTR);
			for attribute in &fragment.attributes {
				let mut setter = "\"\"".to_string();
				if let Some(value) = &attribute.value {
					if strings::is_bind(&attribute.key) {
						todo!();
					} else if strings::contains_handlebars(&value) {
						// add the extra property
						let name = format!("{}_{}_{}",
							fragment.short_name,
							&attribute.key,
							"value" // TODO: can this be anything else?
						);
						let extras = self.extra_variables.entry(fragment.short_name.clone()).or_default();
						extras.push(name.clone());

						let pieces = strings::split_handlebars_into_pieces(&value);
						let mut dynamic_setter = String::new();
						for (i, piece) in pieces.iter().enumerate() {
							match piece.ttype {
								HandlebarType::Text => {
									dynamic_setter.push_str(&format!("\"{}\"", piece.value));
								}
								HandlebarType::Code => {
									// TODO: validate that these code portions are correct
									dynamic_setter.push_str(&format!("{}", piece.value));
								}
							}

							if i < pieces.len()-1 {
								dynamic_setter.push_str(" + ");
							}
						}

						setter = format!("{} = {}", name, dynamic_setter);
					} else {
						// TODO: handle non-strings, bools and ints etc.
						setter = format!("\"{}\"", value);
					}
				}
				result.push_str(&format!("{}({}, \"{}\", {});\n",
					imports::ATTR.function,
					fragment.short_name,
					attribute.key,
					setter
				));
			}
		}

		result.push_str(&self.render_fragments_creator(Some(&fragment), &fragment.children));
		return result;
	}

	fn render_text_fragment_creator(&mut self, parent: Option<&Fragment>, fragment: &Fragment) -> String {
		let mut result = String::new();
		if let Some(text) = &fragment.data {
			if let Some(parent) = parent {
				let text = if strings::contains_handlebars(&text) {
					let mut new_text = String::new();
					new_text.push('`');
					let pieces = strings::split_handlebars_into_pieces(&text);
					for (i, piece) in pieces.iter().enumerate() {
						match piece.ttype {
							HandlebarType::Text => {
								new_text.push_str(&format!("{}", piece.value));
							}
							HandlebarType::Code => {
								// TODO: validate that these code portions are correct
								new_text.push_str(&format!("${{{}}}", piece.value));
							}
						}
					}
					new_text.push('`');
					new_text
				} else {
					// TODO: is this a necessary operation?
					format!("\"{}\"", text)
				};

				result.push_str(&format!("{}.{} = {};",
					parent.short_name,
					constants::TEXT_CONTENT,
					text
				));
			} else {
				self.compiler.insert_import(&imports::TEXT);
				let function = &imports::TEXT.function;

				if text.len() == 0 {
					result.push_str(&format!("{} = {}();\n",
						fragment.short_name,
						function,
					));
				} else {
					if fragment.contains_code {
						result.push_str(&format!("{} = {}({});\n",
							fragment.short_name,
							function,
							text
						));
					} else {
						result.push_str(&format!("{} = {}(\"{}\");\n",
							fragment.short_name,
							function,
							text
						));
					}
				}
			}
		}
		return result;
	}

	fn render_svelte_fragment_creator(&mut self, parent: Option<&Fragment>, fragment: &Fragment) -> String {
		self.compiler.insert_import(&imports::CREATE_COMPONENT);

		let mut result = String::new();
		result.push_str(&format!("{}({}.{});\n",
			imports::CREATE_COMPONENT.function,
			fragment.short_name,
			constants::GLOBAL_FRAGMENT,
		));
		return result;
	}
}