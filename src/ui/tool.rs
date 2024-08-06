use std::default;

use crossterm::style::Color;

#[derive(Clone)]
pub struct Tool {
    pub tool_type: ToolType,
    pub color: Color,
    pub size_index: usize,
    pub stroke: Option<Vec<(u16, u16, char, char)>>,
}

impl Tool {
    const fn new(tool_type: ToolType, color: Color, size_index: usize) -> Self {
        Self {
            tool_type,
            color,
            size_index,
            stroke: None,
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub enum ToolType {
    #[default]
    Pen,
    Eraser,
    Text {
        /// The starting point of the text input. (Col, Row)
        start_pos: (u16, u16),
        /// The current text
        text: String,
        /// The index of the string, where the text cursor currently is
        cursor_index: usize,
        /// Is in text input
        is_in_text_input: bool,
    },
}

impl ToolType {
    pub fn text_default() -> Self {
        Self::Text {
            start_pos: (0, 0),
            text: String::new(),
            cursor_index: 0,
            is_in_text_input: false,
        }
    }
}

static mut TOOL: Tool = Tool::new(ToolType::Pen, Color::White, 4);

pub fn get_tool() -> Tool {
    unsafe { TOOL.clone() }
}

pub fn get_tool_mut() -> &'static mut Tool {
    unsafe { &mut TOOL }
}

pub fn set_tool(tool: Tool) {
    unsafe { TOOL = tool }
}

pub fn set_tool_type(tool_type: ToolType) {
    unsafe { TOOL.tool_type = tool_type }
}

pub fn set_size_index(size_index: usize) {
    unsafe { TOOL.size_index = size_index }
}

pub fn add_to_stroke(col: u16, row: u16, from: char, to: char) {
    unsafe {
        TOOL.stroke = match &TOOL.stroke {
            Some(vec) => {
                let mut v = vec.clone();
                v.push((col, row, from, to));
                Some(vec.to_vec())
            }
            None => Some(vec![(col, row, from, to)]),
        }
    }
}

pub fn finish_stroke() -> Vec<(u16, u16, char, char)> {
    unsafe {
        let stroke = TOOL.clone().stroke.unwrap_or_default();
        TOOL.stroke = None;
        stroke
    }
}
