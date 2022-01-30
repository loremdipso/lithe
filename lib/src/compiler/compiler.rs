#![allow(unused_variables, dead_code, unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::compiler::preprocessor;
use std::thread::sleep;
use crate::compiler::utils::{strings, strings::HandlebarType};
use log::info;
use crate::compiler::constants;
use crate::compiler::constants::imports::Import;
use std::collections::HashSet;
use crate::compiler::renderer::dom_renderer::DomRenderer;
use crate::compiler::renderer::renderer::Renderer;
use html_parser::Element;
use html_parser::{Dom, Node};
use std::collections::HashMap;

pub fn compile(r_str: &str) -> String {
	let mut compiler = Compiler::default();
	return serde_json::to_string_pretty(&compiler.compile(r_str)).unwrap();
}

#[derive(Default, Debug)]
pub struct Compiler {
	pub has_instance: bool,
	pub has_svelte_fragments: bool,
	pub has_style_fragments: bool,
	pub has_script_fragments: bool,
	pub hacky_style_index: usize,
	pub imports: HashMap<String, HashSet<String>>,
	pub short_name_counts: HashMap<String, isize>,
}

#[derive(Debug)]
pub struct Fragment {
	pub short_name: String,
	pub name: String,
	pub data: Option<String>,
	pub ttype: FragmentType,
	pub renderable: bool,
	pub attributes: Vec<Attribute>,
	pub children: Vec<Fragment>,
	pub has_parent: bool,
	pub contains_code: bool, // NOTE: only makes sense for Text nodes
}

impl Fragment {
    pub fn into_iter<'a>(&'a self) -> FragmentIterator<'a> {
        FragmentIterator { stack: vec![&self] }
    }
}

pub struct FragmentIterator<'a> {
	stack: Vec<&'a Fragment>
}

impl<'a> Iterator for FragmentIterator<'a> {
    type Item = &'a Fragment;

    fn next(&mut self) -> Option<Self::Item> {
		if self.stack.len() == 0 {
			return None;
		}

		let top = self.stack.pop().unwrap();
		for child in &top.children {
			self.stack.push(child);
		}
		return Some(top);
    }
}

#[derive(Debug)]
pub struct Attribute {
	pub key: String,
	pub value: Option<String>
}

#[derive(Debug, PartialEq)]
pub enum FragmentType {
	Text, // just text
	Basic, // h1, h2, div, etc.
	Svelte, // other svelte components
	Script, // script sections
	Style, // style sections
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompileResult {
	pub js: JSResult,
	pub css: JSResult, // TODO: why is this re-used here?
	pub ast: TODO,
	pub warnings: Vec<TODO>,
	pub vars: Vec<TODO>,
	pub stats: TODO,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JSResult {
	pub code: Option<String>,
	pub map: JSMap
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JSMap {
	version: usize,
	names: Vec<String>,
	sources: Vec<TODO>,
	sources_content: Vec<TODO>,
	mappings: String
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TODO {
}


impl Compiler {
	pub fn compile(&mut self, source: &str) -> CompileResult {
		let source = crate::time_function(||
			preprocessor::preprocess(source),
			"preprocess"
		);

		// NOTE: this only does basic HTML parsing via external library.
		// Complex info (JavaScript, for ex.) are only stored as unparsed strings
		let dom = crate::time_function(|| 
			Dom::parse(&source),
			"dom parsing"
		).unwrap();

		// convert HTML to Fragments and store any state we need to
		let fragments = crate::time_function(||
			self.convert_dom_nodes_to_fragments(&dom.children, false),
			"fragment generation"
		);

		// finally render those fragments
		let result = crate::time_function(|| {
			let mut renderer = DomRenderer::new(self);
			renderer.render_fragments(&fragments)
		}, "final render");
		return result;
	}

	fn convert_dom_nodes_to_fragments(&mut self, children: &Vec<Node>, has_parent: bool) -> Vec<Fragment> {
		let mut fragments = Vec::new();
		for child in children {
			fragments.extend(self.convert_dom_node_to_fragments(child, has_parent));
		}
		return fragments;
	}

	pub fn convert_dom_node_to_fragments(&mut self, child: &Node, has_parent: bool) -> Vec<Fragment> {
		match child {
			Node::Element(element) => {
				return vec![self.convert_dom_element_to_fragment(&element, has_parent)];
			},

			Node::Text(text) => {
				if has_parent {
					return vec![Fragment {
						name: "SHOULDN'T BE RENDERED".into(),
						short_name: "SHOULDN'T BE RENDERED".into(),
						attributes: vec![],
						data: Some(text.to_string()),
						ttype: FragmentType::Text,
						renderable: true,
						children: vec![],
						contains_code: false,
						has_parent
					}];
				} else {
					// special case: we might want to split up text elements
					let mut fragments = vec![];
					let pieces = strings::split_handlebars_into_pieces(&text);
					for (i, piece) in pieces.iter().enumerate() {
						match piece.ttype {
							HandlebarType::Text => {
								fragments.push(Fragment {
									name: "SHOULDN'T BE RENDERED".into(),
									short_name: self.generate_short_name("t"),
									attributes: vec![],
									data: Some(piece.value.clone()),
									ttype: FragmentType::Text,
									contains_code: false,
									renderable: true,
									children: vec![],
									has_parent
								});
							}
							HandlebarType::Code => {
								fragments.push(Fragment {
									name: "SHOULDN'T BE RENDERED".into(),
									short_name: self.generate_short_name("t"),
									attributes: vec![],
									data: Some(piece.value.trim().to_string()),
									ttype: FragmentType::Text,
									contains_code: true,
									renderable: true,
									children: vec![],
									has_parent
								});
							}
						}
					}
					return fragments;
				}
			},
			_ => todo!()
		}
	}

	pub fn convert_dom_element_to_fragment(&mut self, element: &Element, has_parent: bool) -> Fragment {
		// NOTE: looks like we assume a component is a svelte component if its first letter is capitalized
		if element.name.chars().nth(0).unwrap().is_uppercase() {
			return self.create_svelte_fragment(element, has_parent);
		}

		match &*element.name.to_lowercase() {
			"script" => self.create_script_fragment(element),
			"style" => self.create_style_fragment(element),
			name => self.create_basic_fragment(element, name, has_parent)
		}
	}

	pub fn create_basic_fragment(&mut self, element: &Element, name: &str, has_parent: bool) -> Fragment {
		Fragment {
			renderable: true,
			name: name.to_string(),
			short_name: self.generate_short_name(name),
			ttype: FragmentType::Basic,
			children: self.convert_dom_nodes_to_fragments(&element.children, true),
			attributes: self.extract_attributes(&element),
			data: None,
			contains_code: false,
			has_parent,
		}
	}

	pub fn create_script_fragment(&mut self, element: &Element) -> Fragment {
		if element.children.len() == 0 {
			// Should I handle this somehow?
			todo!();
		}

		if element.children.len() != 1 {
			panic!("this shouldn't happen");
		}

		let styles = match &element.children[0] {
			Node::Text(text) => text,
			_ => panic!()
		};

		self.has_style_fragments = true;
		Fragment {
			renderable: false,
			name: element.name.clone(),
			short_name: self.generate_style_name(&styles),
			ttype: FragmentType::Script,
			children: vec![],
			attributes: self.extract_attributes(&element),
			data: Some(styles.clone()),
			has_parent: false,
			contains_code: false,
		}
	}

	pub fn create_style_fragment(&mut self, element: &Element) -> Fragment {
		if element.children.len() == 0 {
			// Should I handle this somehow?
			todo!();
		}

		if element.children.len() != 1 {
			panic!("this shouldn't happen");
		}

		let styles = match &element.children[0] {
			Node::Text(text) => text,
			_ => panic!()
		};

		self.has_style_fragments = true;
		Fragment {
			renderable: false,
			name: element.name.clone(),
			short_name: self.generate_style_name(&styles),
			ttype: FragmentType::Style,
			children: vec![],
			attributes: self.extract_attributes(&element),
			data: Some(styles.clone()),
			has_parent: false,
			contains_code: false,
		}
	}

	pub fn create_svelte_fragment(&mut self, element: &Element, has_parent: bool) -> Fragment {
		if element.children.len() > 0 {
			todo!();
		}

		self.has_svelte_fragments = true;
		Fragment {
			renderable: true,
			name: element.name.clone(),
			short_name: self.generate_short_name(&element.name),
			ttype: FragmentType::Svelte,
			children: vec![],
			attributes: self.extract_attributes(&element),
			data: None,
			has_parent,
			contains_code: false,
		}
	}

	pub fn extract_attributes(&mut self, element: &Element) -> Vec<Attribute> {
		// converting hashmap to vector for (hopefully) ease of use
		let mut attributes = vec![];
		for (key, value) in &element.attributes {
			// NOTE: this is a hack, since the current parsing doesn't handle attributes with no values well
			let key_pieces: Vec<&str> = key.split(" ").collect();
			for (i, key) in key_pieces.iter().enumerate() {
				if i == key_pieces.len()-1 { // final piece, use real value
					attributes.push(Attribute {
						key: key.to_string(),
						value: value.clone()
					});
				} else {
					attributes.push(Attribute { // pre-final piece, it has no value
						key: key.to_string(),
						value: None
					});
				}
			}
		}

		if element.classes.len() > 0 {
			let mut classes = String::new();
			for (i, class) in element.classes.iter().enumerate() {
				classes.push_str(class);
				if i < element.classes.len()-1 {
					classes.push(' ');
				}
			}
			attributes.push(Attribute { // pre-final piece, it has no value
				key: constants::CLASS.to_string(),
				value: Some(classes)
			});
		}

		return attributes;
	}

	pub fn generate_style_name(&mut self, styles: &str) -> String {
		// TODO: how to do this for real?
		return format!("");
	}

	pub fn generate_short_name(&mut self, name: &str) -> String {
		let name = &name.to_lowercase();
		let count = self.short_name_counts.entry(name.to_string()).or_insert(-1);
		*count += 1;
		if *count == 0 {
			return format!("{}", name);
		} else {
			return format!("{}{}", name, count);
		}
	}

	pub fn insert_import(&mut self, import: &Import) {
		let entry = self.imports.entry(import.location.clone()).or_default();
		entry.insert(import.function.to_string());
	}

	pub fn render_imports(&self) -> String {
		let mut keys = self.imports.keys().into_iter().collect::<Vec<&String>>();
		keys.sort(); // sort the import files to get a consistent output
		let mut result = String::new();
		for (i, key) in keys.iter().enumerate() {
			let imports = &self.imports.get(*key).unwrap(); // guaranteed to exist
			let mut imports = imports.iter().collect::<Vec<&String>>();
			imports.sort(); // sort the imports to get a consistent output
			result.push_str("import {\n");
			for (i, import) in imports.iter().enumerate() {
				result.push_str("\t");
				result.push_str(import);
				if i < imports.len()-1 {
					result.push_str(",\n");
				} else {
					result.push_str("\n");
				}
			}
			result.push_str(&format!("}} from \"{}\";\n", key));

			if i < keys.len()-1 {
				result.push_str("\n");
			}
		}
		return result;
	}
}


#[cfg(test)]
mod tests {
	use super::compile;
	// use std::{thread, time};

	#[test]
	fn basic() {
		// let program = "<SCRIPT>let x = 5;</SCRIPT><h1>Hi</h1>";
		// let program = "<h1>Hi</h1>";
		// let program = "yo";

		// NOTE: newlines are stripped out :\
		// let program = "<h1>{{ x }}</h1>";
		// let program = "<h1 width=50>{{ x }}</h1>";

		// NOTE: duplicate attributes are ignored

		// NOTE: <div a b=1></div> yields only one attributes, "a b". We split it later on,
		// but if we re-write the parser we should fix that
		let program = "<canvas super width=\"hello {x}\" name=\"hi\"></canvas>";

		// dbg!(&program);
		let result = compile(program);
		println!("{}", result);
	}
}