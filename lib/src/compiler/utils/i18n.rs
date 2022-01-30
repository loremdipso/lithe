#![allow(unused_variables, dead_code, unused_imports, non_upper_case_globals)]
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashSet, HashMap};
use lazy_static::lazy_static;
use regex::Regex;

// use super::super::super::utils::names::is_void;
// use super::shared::Node::Node;
// use super::Attribute::Attribute;
// use super::Binding::Binding;
// use super::EventHandler::EventHandler;
// use super::Transition::Transition;
// use super::Animation::Animation;
// use super::Action::Action;
// use super::Class::Class;
// use super::Text::Text;
// use super::super::super::utils::namespaces::namespaces;
// use super::shared::map_children::map_children;
// use super::super::super::utils::patterns::dimensions;
// use super::super::super::utils::fuzzymatch::fuzzymatch;
// use super::super::super::utils::list::list;
// use super::Let::Let;
// use super::shared::TemplateScope::TemplateScope;
// use super::interfaces::INode;
// use super::super::Component::Component;
// use super::super::compiler_warnings::CompilerWarning;
// use super::super::compiler_errors::CompilerError;


lazy_static! {
    pub static ref svg: Regex = Regex::new(r"^(?:altGlyph|altGlyphDef|altGlyphItem|animate|animateColor|animateMotion|animateTransform|circle|clipPath|color-profile|cursor|defs|desc|discard|ellipse|feBlend|feColorMatrix|feComponentTransfer|feComposite|feConvolveMatrix|feDiffuseLighting|feDisplacementMap|feDistantLight|feDropShadow|feFlood|feFuncA|feFuncB|feFuncG|feFuncR|feGaussianBlur|feImage|feMerge|feMergeNode|feMorphology|feOffset|fePointLight|feSpecularLighting|feSpotLight|feTile|feTurbulence|filter|font|font-face|font-face-format|font-face-name|font-face-src|font-face-uri|foreignObject|g|glyph|glyphRef|hatch|hatchpath|hkern|image|line|linearGradient|marker|mask|mesh|meshgradient|meshpatch|meshrow|metadata|missing-glyph|mpath|path|pattern|polygon|polyline|radialGradient|rect|set|solidcolor|stop|svg|switch|symbol|text|textPath|tref|tspan|unknown|use|view|vkern)$").unwrap();

	pub static ref aria_attributes: Vec<&'static str> = vec!["activedescendant", "atomic", "autocomplete", "busy", "checked", "colcount", "colindex", "colspan", "controls", "current", "describedby", "details", "disabled", "dropeffect", "errormessage", "expanded", "flowto", "grabbed", "haspopup", "hidden", "invalid", "keyshortcuts", "label", "labelledby", "level", "live", "modal", "multiline", "multiselectable", "orientation", "owns", "placeholder", "posinset", "pressed", "readonly", "relevant", "required", "roledescription", "rowcount", "rowindex", "rowspan", "selected", "setsize", "sort", "valuemax", "valuemin", "valuenow", "valuetext"];
	pub static ref aria_attribute_set: HashSet<String> = {
		let mut a = HashSet::new();
		for attribute in aria_attributes.iter() {
			a.insert(attribute.to_string());
		}
		a
	};

	pub static ref aria_roles: Vec<&'static str> = vec!["alert", "alertdialog", "application", "article", "banner", "blockquote", "button", "caption", "cell", "checkbox", "code", "columnheader", "combobox", "complementary", "contentinfo", "definition", "deletion", "dialog", "directory", "document", "emphasis", "feed", "figure", "form", "generic", "graphics-document", "graphics-object", "graphics-symbol", "grid gridcell", "group", "heading", "img", "link", "list", "listbox", "listitem", "log", "main", "marquee", "math", "meter", "menu", "menubar", "menuitem", "menuitemcheckbox", "menuitemradio", "navigation", "none", "note", "option", "paragraph", "presentation", "progressbar", "radio", "radiogroup", "region", "row", "rowgroup", "rowheader", "scrollbar", "search", "searchbox", "separator", "slider", "spinbutton", "status", "strong", "subscript", "superscript", "switch", "tab", "table", "tablist", "tabpanel", "term", "textbox", "time", "timer", "toolbar", "tooltip", "tree", "treegrid", "treeitem"];
	pub static ref aria_role_set: HashSet<String> = {
		let mut a = HashSet::new();
		for role in aria_roles.iter() {
			a.insert(role.to_string());
		}
		a
	};

	pub static ref a11y_required_attributes: HashMap<&'static str, Vec<&'static str>> = {
		let mut a = HashMap::new();
		a.insert("a", vec!["href"]);
		a.insert("area", vec!["alt", "aria-label", "aria-labelledby"]);

		// html-has-lang
		a.insert("html", vec!["lang"]);

		// iframe-has-title
		a.insert("iframe", vec!["title"]);
		a.insert("img", vec!["alt"]);
		a.insert("object", vec!["title", "aria-label", "aria-labelledby"]);
		a
	};

	pub static ref a11y_distracting_elements: HashSet<&'static str> = {
		let mut a = HashSet::new();
		a.insert("blink");
		a.insert("marquee");
		a
	};

	pub static ref a11y_required_content: HashSet<&'static str> = {
		let mut a = HashSet::new();

		// anchor-has-content
		a.insert("a");

		// heading-has-content
		a.insert("h1");
		a.insert("h2");
		a.insert("h3");
		a.insert("h4");
		a.insert("h5");
		a.insert("h6");
		a
	};

	pub static ref a11y_labelable: HashSet<&'static str> = {
		let mut a = HashSet::new();
		a.insert("button");
		a.insert("input");
		a.insert("keygen");
		a.insert("meter");
		a.insert("output");
		a.insert("progress");
		a.insert("select");
		a.insert("textarea");
		a
	};

	pub static ref invisible_elements: HashSet<&'static str> = {
		let mut i = HashSet::new();
		i.insert("meta");
		i.insert("html");
		i.insert("script");
		i.insert("style");
		i
	};

	pub static ref valid_modifiers: HashSet<&'static str> = {
		let mut v = HashSet::new();
		v.insert("preventDefault");
		v.insert("stopPropagation");
		v.insert("capture");
		v.insert("once");
		v.insert("passive");
		v.insert("nonpassive");
		v.insert("self");
		v.insert("trusted");
		v
	};

	pub static ref passive_events: HashSet<&'static str> = {
		let mut p = HashSet::new();
		p.insert("wheel");
		p.insert("touchstart");
		p.insert("touchmove");
		p.insert("touchend");
		p.insert("touchcancel");
		p
	};

	pub static ref react_attributes: HashMap<&'static str, &'static str> = {
		let mut r = HashMap::new();
		r.insert("className", "class");
		r.insert("htmlFor", "for");
		r
	};

	pub static ref attributes_to_compact_whitespace: Vec<&'static str> = vec!["class", "style"];
}

// pub fn get_namespace(parent: Element, element: Element, explicit_namespace: String) {
// 	// let parent_element = parent.find_nearest(/^Element/);

// 	// if (!parent_element) {
// 	// 	return explicit_namespace || (svg.test(element.name)
// 	// 		? namespaces.svg
// 	// 		: null);
// 	// }

// 	// if (parent_element.namespace != namespaces.foreign) {
// 	// 	if (svg.test(element.name.toLowerCase())) return namespaces.svg;
// 	// 	if (parent_element.name.toLowerCase() == "foreignobject") return null;
// 	// }

// 	// return parent_element.namespace;
// 	todo!();
// }

// #[derive(Debug, Default)]
// pub struct Element {
// 	base: Node,
// 	name: String,
// 	scope: TemplateScope,
// 	attributes: Vec<Attribute>,
// 	actions: Vec<Action>,
// 	bindings: Vec<Binding>,
// 	classes: Vec<Class>,
// 	handlers: Vec<EventHandler>,
// 	lets: Vec<Let>,
// 	intro: Option<Transition>,
// 	outro: Option<Transition>,
// 	animation: Option<Animation>,
// 	children: Vec<INode>,
// 	namespace: String,
// 	needs_manual_style_scoping: bool
// }

// pub struct TODO {
// }

// impl Element {
// 	pub fn new(component: Component, parent: Node, scope: TemplateScope, info: TODO) -> Element {
// 		// let this = Element::default();
// 		// this.base = Node::new(component, parent, scope, info);
// 		// this.name = info.name;

// 		// this.namespace = get_namespace(parent as Element, this, component.namespace);

// 		// if (this.namespace != namespaces.foreign) {
// 		// 	if (this.name == "textarea") {
// 		// 		if (info.children.len() > 0) {
// 		// 			let value_attribute = info.attributes.find(node => node.name == "value");
// 		// 			if (value_attribute) {
// 		// 				component.error(value_attribute, compiler_errors.textarea_duplicate_value);
// 		// 				return;
// 		// 			}

// 		// 			// this is an egregious hack, but it"s the easiest way to get <textarea>
// 		// 			// children treated the same way as a value attribute
// 		// 			info.attributes.push({
// 		// 				type: "Attribute",
// 		// 				name: "value",
// 		// 				value: info.children
// 		// 			});

// 		// 			info.children = [];
// 		// 		}
// 		// 	}

// 		// 	if (this.name == "option") {
// 		// 		// Special case — treat these the same way:
// 		// 		//   <option>{foo}</option>
// 		// 		//   <option value={foo}>{foo}</option>
// 		// 		let value_attribute = info.attributes.find(attribute => attribute.name == "value");

// 		// 		if (!value_attribute) {
// 		// 			info.attributes.push({
// 		// 				type: "Attribute",
// 		// 				name: "value",
// 		// 				value: info.children,
// 		// 				synthetic: true
// 		// 			});
// 		// 		}
// 		// 	}
// 		// }
// 		// let has_let = info.attributes.some(node => node.type == "Let");
// 		// if (has_let) {
// 		// 	scope = scope.child();
// 		// }

// 		// // Binding relies on Attribute, defer its evaluation
// 		// let order = ["Binding"]; // everything else is -1
// 		// info.attributes.sort((a, b) => order.indexOf(a.type) - order.indexOf(b.type));

// 		// for let node in info.attributes {
// 		// 	match node {
// 		// 		Action(node):
// 		// 			this.actions.push(new Action(component, this, scope, node));
// 		// 			break;

// 		// 		Attribute(node) | Spread(node): {
// 		// 			// special case
// 		// 			if (node.name == "xmlns") this.namespace = node.value[0].data;

// 		// 			this.attributes.push(new Attribute(component, this, scope, node));
// 		// 			break;

// 		// 		Binding(node):
// 		// 			this.bindings.push(new Binding(component, this, scope, node));
// 		// 			break;

// 		// 		Class(node):
// 		// 			this.classes.push(new Class(component, this, scope, node));
// 		// 			break;

// 		// 		EventHandler(node):
// 		// 			this.handlers.push(new EventHandler(component, this, scope, node));
// 		// 			break;

// 		// 		Let(node);
// 		// 			let l = new Let(component, this, scope, node);
// 		// 			this.lets.push(l);
// 		// 			let dependencies = HashSet::new([l.name.name]);

// 		// 			l.names.forEach(name => {
// 		// 				scope.add(name, dependencies, this);
// 		// 			});
// 		// 			break;
// 		// 		}

// 		// 		Transition(node):
// 		// 			{
// 		// 				let transition = new Transition(component, this, scope, node);
// 		// 				if (node.intro) this.intro = transition;
// 		// 				if (node.outro) this.outro = transition;
// 		// 				break;
// 		// 			}

// 		// 		Animation(node):
// 		// 			this.animation = new Animation(component, this, scope, node);
// 		// 			break;

// 		// 		_ -> {
// 		// 			panic!("Not implemented: {:?}", node);
// 		// 		}
// 		// 	}
// 		// }

// 		// this.scope = scope;
// 		// this.children = map_children(component, this, this.scope, info.children);
// 		// this.validate();
// 		// this.optimise();
// 		// component.apply_stylesheet(this);

// 		// return this;
// 		todo!();
// 	}

// 	pub fn validate() {
// 		// if (this.component.var_lookup.has(this.name) && this.component.var_lookup.get(this.name).imported) {
// 		// 	this.component.warn(this, compiler_warnings.component_name_lowercase(this.name));
// 		// }

// 		// this.validate_attributes();
// 		// this.validate_event_handlers();
// 		// if (this.namespace == namespaces.foreign) {
// 		// 	this.validate_bindings_foreign();
// 		// } else {
// 		// 	this.validate_attributes_a11y();
// 		// 	this.validate_special_cases();
// 		// 	this.validate_bindings();
// 		// 	this.validate_content();
// 		// }
// 		todo!();
// 	}

// 	pub fn validate_attributes() {
// 		todo!();
// 		// let { component, parent } = this;

// 		// this.attributes.forEach(attribute => {
// 		// 	if (attribute.is_spread) return;

// 		// 	let name = attribute.name.toLowerCase();

// 		// 	// Errors

// 		// 	if (/(^[0-9-.])|[\^$@%&#?!|()[\]{}^*+~;]/.test(name)) {
// 		// 		return component.error(attribute, compiler_errors.illegal_attribute(name));
// 		// 	}

// 		// 	if (name == "slot") {
// 		// 		if (!attribute.is_static) {
// 		// 			return component.error(attribute, compiler_errors.invalid_slot_attribute);
// 		// 		}

// 		// 		if (component.slot_outlets.has(name)) {
// 		// 			return component.error(attribute, compiler_errors.duplicate_slot_attribute(name));

// 		// 			// this code was unreachable. Still needed?
// 		// 			// component.slot_outlets.add(name);
// 		// 		}

// 		// 		// TODO: should this also deal with DefaultSlotTemplate?
// 		// 		if (!(parent.type == "SlotTemplate" || within_custom_element(parent))) {
// 		// 			return component.error(attribute, compiler_errors.invalid_slotted_content);
// 		// 		}
// 		// 	}

// 		// 	// Warnings

// 		// 	if (this.namespace != namespaces.foreign) {
// 		// 		if (name == "is") {
// 		// 			component.warn(attribute, compiler_warnings.avoid_is);
// 		// 		}

// 		// 		if (react_attributes.has(attribute.name)) {
// 		// 			component.warn(attribute, compiler_warnings.invalid_html_attribute(attribute.name, react_attributes.get(attribute.name)));
// 		// 		}
// 		// 	}
// 		// });
// 	}

// 	pub fn validate_attributes_a11y() {
// 		// let { component } = this;

// 		// this.attributes.forEach(attribute => {
// 		// 	if (attribute.is_spread) return;

// 		// 	let name = attribute.name.toLowerCase();

// 		// 	// aria-props
// 		// 	if (name.startsWith("aria-")) {
// 		// 		if (invisible_elements.has(this.name)) {
// 		// 			// aria-unsupported-elements
// 		// 			component.warn(attribute, compiler_warnings.a11y_aria_attributes(this.name));
// 		// 		}

// 		// 		let type = name.slice(5);
// 		// 		if (!aria_attribute_set.has(type)) {
// 		// 			let match = fuzzymatch(type, aria_attributes);
// 		// 			component.warn(attribute, compiler_warnings.a11y_unknown_aria_attribute(type, match));
// 		// 		}

// 		// 		if (name == "aria-hidden" && /^h[1-6]$/.test(this.name)) {
// 		// 			component.warn(attribute, compiler_warnings.a11y_hidden(this.name));
// 		// 		}
// 		// 	}

// 		// 	// aria-role
// 		// 	if (name == "role") {
// 		// 		if (invisible_elements.has(this.name)) {
// 		// 			// aria-unsupported-elements
// 		// 			component.warn(attribute, compiler_warnings.a11y_misplaced_role(this.name));
// 		// 		}

// 		// 		let value = attribute.get_static_value();
// 		// 		// @ts-ignore
// 		// 		if (value && !aria_role_set.has(value)) {
// 		// 			// @ts-ignore
// 		// 			let match = fuzzymatch(value, aria_roles);
// 		// 			component.warn(attribute, compiler_warnings.a11y_unknown_role(value, match));
// 		// 		}
// 		// 	}

// 		// 	// no-access-key
// 		// 	if (name == "accesskey") {
// 		// 		component.warn(attribute, compiler_warnings.a11y_accesskey);
// 		// 	}

// 		// 	// no-autofocus
// 		// 	if (name == "autofocus") {
// 		// 		component.warn(attribute, compiler_warnings.a11y_autofocus);
// 		// 	}

// 		// 	// scope
// 		// 	if (name == "scope" && this.name != "th") {
// 		// 		component.warn(attribute, compiler_warnings.a11y_misplaced_scope);
// 		// 	}

// 		// 	// tabindex-no-positive
// 		// 	if (name == "tabindex") {
// 		// 		let value = attribute.get_static_value();
// 		// 		// @ts-ignore todo is tabindex=true correct case?
// 		// 		if (!isNaN(value) && +value > 0) {
// 		// 			component.warn(attribute, compiler_warnings.a11y_positive_tabindex);
// 		// 		}
// 		// 	}
// 		// });
// 		todo!();
// 	}


// 	pub fn validate_special_cases(&self) {
// 		// let component = self.base.component;
// 		// let attributes = self.attributes;
// 		// let handlers = self.handlers;

// 		// let attribute_map = HashMap::new();
// 		// let handlers_map = HashMap::new();

// 		// for attribute in attributes {
// 		// 	attribute_map.insert(attribute.name, attribute);
// 		// }

// 		// for handler in handlers {
// 		// 	handlers_map.insert(handler.name, handler);
// 		// }

// 		// if (this.name == "a") {
// 		// 	let href_attribute = attribute_map.get("href") || attribute_map.get("xlink:href");
// 		// 	let id_attribute = attribute_map.get("id");
// 		// 	let name_attribute = attribute_map.get("name");

// 		// 	if (href_attribute) {
// 		// 		let href_value = href_attribute.get_static_value();

// 		// 		if (href_value == "" || href_value == "#" || /^\W*javascript:/i.test(href_value)) {
// 		// 			component.warn(href_attribute, compiler_warnings.a11y_invalid_attribute(href_attribute.name, href_value));
// 		// 		}
// 		// 	} else {
// 		// 		let id_attribute_valid = id_attribute && id_attribute.get_static_value() != "";
// 		// 		let name_attribute_valid = name_attribute && name_attribute.get_static_value() != "";

// 		// 		if (!id_attribute_valid && !name_attribute_valid) {
// 		// 			component.warn(this, compiler_warnings.a11y_missing_attribute("a", "an", "href"));
// 		// 		}
// 		// 	}
// 		// } else {
// 		// 	let required_attributes = a11y_required_attributes[this.name];
// 		// 	if (required_attributes) {
// 		// 		let has_attribute = required_attributes.some(name => attribute_map.has(name));

// 		// 		if (!has_attribute) {
// 		// 			should_have_attribute(this, required_attributes);
// 		// 		}
// 		// 	}
// 		// }

// 		// if (this.name == "input") {
// 		// 	let type = attribute_map.get("type");
// 		// 	if (type && type.get_static_value() == "image") {
// 		// 		let required_attributes = ["alt", "aria-label", "aria-labelledby"];
// 		// 		let has_attribute = required_attributes.some(name => attribute_map.has(name));

// 		// 		if (!has_attribute) {
// 		// 			should_have_attribute(this, required_attributes, "input type="image"");
// 		// 		}
// 		// 	}
// 		// }

// 		// if self.name == "img" {
// 		// 	let alt_attribute = attribute_map.get("alt");
// 		// 	let aria_hidden_attribute = attribute_map.get("aria-hidden");

// 		// 	let aria_hidden_exist = aria_hidden_attribute && aria_hidden_attribute.get_static_value();

// 		// 	if (alt_attribute && !aria_hidden_exist) {
// 		// 		let alt_value = alt_attribute.get_static_value();

// 		// 		let re = Regex::new(r"\b(image|picture|photo)\b/i");
// 		// 		if re.test(alt_value) {
// 		// 			component.warn(this, compiler_warnings.a11y_img_redundant_alt);
// 		// 		}
// 		// 	}
// 		// }

// 		// if self.name == "label" {
// 		// 	let has_input_child = this.children.some(i i instanceof Element && a11y_labelable.has(i.name)));
// 		// 	if (!attribute_map.has("for") && !has_input_child) {
// 		// 		component.warn(this, compiler_warnings.a11y_label_has_associated_control);
// 		// 	}
// 		// }

// 		// if (this.name == "video") {
// 		// 	if (attribute_map.has("muted")) {
// 		// 		return;
// 		// 	}

// 		// 	let has_caption;
// 		// 	let track = this.children.find((i: Element) => i.name == "track");
// 		// 	if (track) {
// 		// 		has_caption = track.attributes.find(a => a.name == "kind" && a.get_static_value() == "captions");
// 		// 	}

// 		// 	if (!has_caption) {
// 		// 		component.warn(this, compiler_warnings.a11y_media_has_caption);
// 		// 	}
// 		// }

// 		// if (a11y_distracting_elements.has(this.name)) {
// 		// 	// no-distracting-elements
// 		// 	component.warn(this, compiler_warnings.a11y_distracting_elements(this.name));
// 		// }

// 		// if (this.name == "figcaption") {
// 		// 	let { parent } = this;
// 		// 	let is_figure_parent = false;

// 		// 	while (parent) {
// 		// 		if ((parent as Element).name == "figure") {
// 		// 			is_figure_parent = true;
// 		// 			break;
// 		// 		}
// 		// 		if (parent.type == "Element") {
// 		// 			break;
// 		// 		}
// 		// 		parent = parent.parent;
// 		// 	}

// 		// 	if (!is_figure_parent) {
// 		// 		component.warn(this, compiler_warnings.a11y_structure_immediate);
// 		// 	}
// 		// }

// 		// if (self.name == "figure") {
// 		// 	let children = this.children.filter(node => {
// 		// 		if (node.type == "Comment") return false;
// 		// 		if (node.type == "Text") return /\S/.test(node.data);
// 		// 		return true;
// 		// 	});

// 		// 	let index = children.findIndex(child child as Element).name == "figcaption");

// 		// 	if (index != -1 && (index != 0 && index != children.len() - 1)) {
// 		// 		component.warn(children[index], compiler_warnings.a11y_structure_first_or_last);
// 		// 	}
// 		// }

// 		// if (handlers_map.has("mouseover") && !handlers_map.has("focus")) {
// 		// 	component.warn(this, compiler_warnings.a11y_mouse_events_have_key_events("mouseover", "focus"));
// 		// }

// 		// if (handlers_map.has("mouseout") && !handlers_map.has("blur")) {
// 		// 	component.warn(this, compiler_warnings.a11y_mouse_events_have_key_events("mouseout", "blur"));
// 		// }
// 		todo!();
// 	}

// 	pub fn validate_bindings_foreign() {
// 		todo!();
// 		// this.bindings.forEach(binding => {
// 		// 	if (binding.name != "this") {
// 		// 		return this.component.error(binding, compiler_errors.invalid_binding_foreign(binding.name));
// 		// 	}
// 		// });
// 	}

// 	pub fn validate_bindings() {
// 		todo!();
// 		// let { component } = this;

// 		// let check_type_attribute = () => {
// 		// 	let attribute = this.attributes.find(
// 		// 		(attribute: Attribute) => attribute.name == "type"
// 		// 	);

// 		// 	if (!attribute) return null;

// 		// 	if (!attribute.is_static) {
// 		// 		return component.error(attribute, compiler_errors.invalid_type);
// 		// 	}

// 		// 	let value = attribute.get_static_value();

// 		// 	if (value == true) {
// 		// 		return component.error(attribute, compiler_errors.missing_type);
// 		// 	}

// 		// 	return value;
// 		// };

// 		// this.bindings.forEach(binding => {
// 		// 	let { name } = binding;

// 		// 	if (name == "value") {
// 		// 		if (
// 		// 			this.name != "input" &&
// 		// 			this.name != "textarea" &&
// 		// 			this.name != "select"
// 		// 		) {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_elements(this.name, "value"));
// 		// 		}

// 		// 		if (this.name == "select") {
// 		// 			let attribute = this.attributes.find(
// 		// 				(attribute: Attribute) => attribute.name == "multiple"
// 		// 			);

// 		// 			if (attribute && !attribute.is_static) {
// 		// 				return component.error(attribute, compiler_errors.dynamic_multiple_attribute);
// 		// 			}
// 		// 		} else {
// 		// 			check_type_attribute();
// 		// 		}
// 		// 	} else if (name == "checked" || name == "indeterminate") {
// 		// 		if (this.name != "input") {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_elements(this.name, name));
// 		// 		}

// 		// 		let type = check_type_attribute();

// 		// 		if (type != "checkbox") {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_no_checkbox(name, type == "radio"));
// 		// 		}
// 		// 	} else if (name == "group") {
// 		// 		if (this.name != "input") {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_elements(this.name, "group"));
// 		// 		}

// 		// 		let type = check_type_attribute();

// 		// 		if (type != "checkbox" && type != "radio") {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_element_with("<input type="checkbox"> or <input type="radio">", "group"));
// 		// 		}
// 		// 	} else if (name == "files") {
// 		// 		if (this.name != "input") {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_elements(this.name, "files"));
// 		// 		}

// 		// 		let type = check_type_attribute();

// 		// 		if (type != "file") {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_element_with("<input type="file">", "files"));
// 		// 		}

// 		// 	} else if (name == "open") {
// 		// 		if (this.name != "details") {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_element_with("<details>", name));
// 		// 		}
// 		// 	} else if (
// 		// 		name == "currentTime" ||
// 		// 		name == "duration" ||
// 		// 		name == "paused" ||
// 		// 		name == "buffered" ||
// 		// 		name == "seekable" ||
// 		// 		name == "played" ||
// 		// 		name == "volume" ||
// 		// 		name == "muted" ||
// 		// 		name == "playbackRate" ||
// 		// 		name == "seeking" ||
// 		// 		name == "ended"
// 		// 	) {
// 		// 		if (this.name != "audio" && this.name != "video") {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_element_with("audio> or <video>", name));
// 		// 		}
// 		// 	} else if (
// 		// 		name == "videoHeight" ||
// 		// 		name == "videoWidth"
// 		// 	) {
// 		// 		if (this.name != "video") {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_element_with("<video>", name));
// 		// 		}
// 		// 	} else if (dimensions.test(name)) {
// 		// 		if (this.name == "svg" && (name == "offsetWidth" || name == "offsetHeight")) {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_on(binding.name, `<svg>. Use "${name.replace("offset", "client")}" instead`));
// 		// 		} else if (svg.test(this.name)) {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_on(binding.name, "SVG elements"));
// 		// 		} else if (is_void(this.name)) {
// 		// 			return component.error(binding, compiler_errors.invalid_binding_on(binding.name, `void elements like <${this.name}>. Use a wrapper element instead`));
// 		// 		}
// 		// 	} else if (
// 		// 		name == "textContent" ||
// 		// 		name == "innerHTML"
// 		// 	) {
// 		// 		let contenteditable = this.attributes.find(
// 		// 			(attribute: Attribute) => attribute.name == "contenteditable"
// 		// 		);

// 		// 		if (!contenteditable) {
// 		// 			return component.error(binding, compiler_errors.missing_contenteditable_attribute);
// 		// 		} else if (contenteditable && !contenteditable.is_static) {
// 		// 			return component.error(contenteditable, compiler_errors.dynamic_contenteditable_attribute);
// 		// 		}
// 		// 	} else if (name != "this") {
// 		// 		return component.error(binding, compiler_errors.invalid_binding(binding.name));
// 		// 	}
// 		// });
// 	}

// 	pub fn validate_content() {
// 		// if (!a11y_required_content.has(this.name)) return;
// 		// if (
// 		// 	this.bindings
// 		// 		.some((binding) => ["textContent", "innerHTML"].includes(binding.name))
// 		// ) return;

// 		// if (this.children.len() == 0) {
// 		// 	this.component.warn(this, compiler_warnings.a11y_missing_content(this.name));
// 		// }
// 		todo!();
// 	}

// 	pub fn validate_event_handlers() {
// 		// let { component } = this;

// 		// this.handlers.forEach(handler => {
// 		// 	if (handler.modifiers.has("passive") && handler.modifiers.has("preventDefault")) {
// 		// 		return component.error(handler, compiler_errors.invalid_event_modifier_combination("passive", "preventDefault"));
// 		// 	}

// 		// 	if (handler.modifiers.has("passive") && handler.modifiers.has("nonpassive")) {
// 		// 		return component.error(handler, compiler_errors.invalid_event_modifier_combination("passive", "nonpassive"));
// 		// 	}

// 		// 	handler.modifiers.forEach(modifier => {
// 		// 		if (!valid_modifiers.has(modifier)) {
// 		// 			return component.error(handler, compiler_errors.invalid_event_modifier(list(Array.from(valid_modifiers))));
// 		// 		}

// 		// 		if (modifier == "passive") {
// 		// 			if (passive_events.has(handler.name)) {
// 		// 				if (handler.can_make_passive) {
// 		// 					component.warn(handler, compiler_warnings.redundant_event_modifier_for_touch);
// 		// 				}
// 		// 			} else {
// 		// 				component.warn(handler, compiler_warnings.redundant_event_modifier_passive);
// 		// 			}
// 		// 		}

// 		// 		if (component.compile_options.legacy && (modifier == "once" || modifier == "passive")) {
// 		// 			// TODO this could be supported, but it would need a few changes to
// 		// 			// how event listeners work
// 		// 			return component.error(handler, compiler_errors.invalid_event_modifier_legacy(modifier));
// 		// 		}
// 		// 	});

// 		// 	if (passive_events.has(handler.name) && handler.can_make_passive && !handler.modifiers.has("preventDefault") && !handler.modifiers.has("nonpassive")) {
// 		// 		// touch/wheel events should be passive by default
// 		// 		handler.modifiers.add("passive");
// 		// 	}
// 		// });
// 		todo!();
// 	}

// 	pub fn is_media_node(&self) -> bool {
// 		return self.name == "audio" || self.name == "video";
// 	}

// 	pub fn add_css_class() {
// 		// if (this.attributes.some(attr => attr.is_spread)) {
// 		// 	this.needs_manual_style_scoping = true;
// 		// 	return;
// 		// }

// 		// let { id } = this.component.stylesheet;

// 		// let class_attribute = this.attributes.find(a => a.name == "class");

// 		// if (class_attribute && !class_attribute.is_true) {
// 		// 	if (class_attribute.chunks.len() == 1 && class_attribute.chunks[0].type == "Text") {
// 		// 		(class_attribute.chunks[0] as Text).data += ` ${id}`;
// 		// 	} else {
// 		// 		(class_attribute.chunks as Node[]).push(
// 		// 			new Text(this.component, this, this.scope, {
// 		// 				type: "Text",
// 		// 				data: ` ${id}`,
// 		// 				synthetic: true
// 		// 			} as any)
// 		// 		);
// 		// 	}
// 		// } else {
// 		// 	this.attributes.push(
// 		// 		new Attribute(this.component, this, this.scope, {
// 		// 			type: "Attribute",
// 		// 			name: "class",
// 		// 			value: [{ type: "Text", data: id, synthetic: true }]
// 		// 		} as any)
// 		// 	);
// 		// }
// 		todo!();
// 	}

// 	pub fn slot_template_name() -> String {
// 		todo!();
// 		// return this.attributes.find(attribute => attribute.name == "slot").get_static_value() as String;
// 	}

// 	pub fn optimise() {
// 		// attributes_to_compact_whitespace.forEach(attribute_name => {
// 		// 	let attribute = this.attributes.find(a => a.name == attribute_name);
// 		// 	if (attribute && !attribute.is_true) {
// 		// 		attribute.chunks.forEach((chunk, index) => {
// 		// 			if (chunk.type == "Text") {
// 		// 				let data = chunk.data.replace(/[\s\n\t]+/g, " ");
// 		// 				if (index == 0) {
// 		// 					data = data.trimLeft();
// 		// 				} else if (index == attribute.chunks.len() - 1) {
// 		// 					data = data.trimRight();
// 		// 				}
// 		// 				chunk.data = data;
// 		// 			}
// 		// 		});
// 		// 	}
// 		// });
// 		todo!();
// 	}
// }

// pub fn should_have_attribute(
// 	node: Node,
// 	attributes: Vec<String>
// ) {
// 	// let name = node.name;
// 	// let article = /^[aeiou]/.test(attributes[0]) ? "an" : "a";
// 	// let sequence = attributes.len() > 1 ?
// 	// 	attributes.slice(0, -1).join(", ") + ` or ${attributes[attributes.len() - 1]}` :
// 	// 	attributes[0];

// 	// node.component.warn(node, compiler_warnings.a11y_missing_attribute(name, article, sequence));
// 	todo!();
// }

// pub fn within_custom_element(parent: INode) -> bool {
// 	// while (parent) {
// 	// 	if (parent.type == "InlineComponent") return false;
// 	// 	if (parent.type == "Element" && /-/.test(parent.name)) return true;
// 	// 	parent = parent.parent;
// 	// }
// 	// return false;
// 	todo!();
// }
