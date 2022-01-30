
pub fn indent_block(block: &str, indent_level: usize) -> String {
	let mut result = String::new();
	let lines: Vec<&str> = block.trim().split("\n").collect();
	for (i, line) in lines.iter().enumerate() {
		for _ in 0..indent_level {
			result.push_str("\t");
		}
		result.push_str(line);

		if i < lines.len() - 1 {
			result.push_str("\n");
		}
	}
	return result;
}

pub fn is_bind(statement: &str) -> bool {
	return statement.starts_with("bind:");
}

pub fn contains_handlebars(block: &str) -> bool {
	// TODO: how should we do this, for real?
	// TODO: handle things like quoted strings and template literals, etc.
	let mut faux_stack: isize = 0; // counts the opening/closing braces
	let mut has_braces = false;
	for c in block.chars() {
		if c == '{' {
			has_braces = true;
			faux_stack += 1;
		} else if c == '}' {
			faux_stack -= 1;
			if faux_stack < 0 {
				panic!("Improperly formed template: {}", block);
			}
		}
	}
	return has_braces;
}

#[derive(Clone, Debug)]
pub struct HandlebarPiece {
	pub value: String,
	pub ttype: HandlebarType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HandlebarType {
	Code,
	Text,
}

impl HandlebarPiece {

	pub fn new(ttype: HandlebarType) -> Self {
		Self {
			ttype,
			value: String::new()
		}
	}

	pub fn add_to_vec(&self, pieces: &mut Vec<HandlebarPiece>) {
		if self.value.len() > 0 {
			pieces.push(self.clone());
		}
	}
}

pub fn split_handlebars_into_pieces(block: &str) -> Vec<HandlebarPiece> {
	let mut faux_stack: isize = 0; // counts the opening/closing braces
	// TODO: how should we do this, for real?
	let mut pieces: Vec<HandlebarPiece> = vec![];
	let mut piece = HandlebarPiece::new(HandlebarType::Text);

	for c in block.chars() {
		if c == '{' {
			faux_stack += 1;
			if faux_stack == 1 {
				piece.add_to_vec(&mut pieces);
				piece = HandlebarPiece::new(HandlebarType::Code);
			} else {
				// nested code block, probably
				piece.value.push(c);
			}
		} else if c == '}' {
			faux_stack -= 1;
			match faux_stack.partial_cmp(&0).expect("NaN") {
				std::cmp::Ordering::Greater => {
					piece.value.push(c);
				},
				std::cmp::Ordering::Equal => {
					piece.add_to_vec(&mut pieces);
					piece = HandlebarPiece::new(HandlebarType::Text);
				},
				std::cmp::Ordering::Less => panic!("this shouldn't happen"),
			}
		} else {
			piece.value.push(c);
		}
	}

	match piece.ttype {
		HandlebarType::Code => {
			panic!("Handlebar parsing error");
		}
		HandlebarType::Text => {
			piece.add_to_vec(&mut pieces);
		}
	}
	return pieces;
}