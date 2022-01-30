use crate::compiler::compiler::CompileResult;
use crate::compiler::compiler::Fragment;

pub trait Renderer {
	fn render_fragments(&mut self, fragments: &Vec<Fragment>) -> CompileResult;
}