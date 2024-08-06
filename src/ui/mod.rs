mod data;
mod tool;

use crossterm::{
    cursor,
    event::{
        KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    },
    style::Stylize,
    terminal::{self, ClearType},
    ExecutableCommand,
};

use crate::StreamHandler;

use self::tool::ToolType;

const SIZES: [char; 8] = ['.', '-', '+', '*', 'x', 'X', '#', '█'];

const UI_LENGTH: u16 = 22;
const UI_HEIGHT: u16 = 3;

#[rustfmt::skip]
pub fn ui_string() -> String {
    let size = SIZES[tool::get_tool().size_index];
    let mut pen = format!("󰏪 ({})", size);
    let mut eraser = "󰇾".to_owned();
    let mut text = "T".to_owned();

    match tool::get_tool().tool_type {
        ToolType::Pen => {
            pen = pen.blue().to_string();
        } ToolType::Eraser => {
            eraser = eraser.blue().to_string();
        } ToolType::Text { .. } => {
            text = text.blue().to_string();
        }
    }

    [
                "╭───────┬───┬───┬───╮".to_owned(),
        format!("│ {} │ {} │ {} │  │", pen, eraser, text),
                "╰───────┴───┴───┴───╯".to_owned(),
    ]
    .join("\n")
}

pub fn handle_mouse_event(stream: &mut StreamHandler, event: MouseEvent) -> std::io::Result<()> {
    match event {
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column,
            row,
            ..
        } => {
            if column < UI_LENGTH && row < UI_HEIGHT {
                handle_ui_click(stream, column, row)?;
            }

            stream.execute(cursor::MoveTo(column, row))?;
            let tool = tool::get_tool_mut();
            match &mut tool.tool_type {
                ToolType::Pen => {
                    stream.print(SIZES[tool.size_index].to_string(), 0)?;
                }
                ToolType::Eraser => {
                    stream.print(" ".to_owned(), 0)?;
                }
                ToolType::Text {
                    start_pos,
                    is_in_text_input,
                    ..
                } => {
                    if !*is_in_text_input {
                        tool.stroke = Some(vec![]);

                        *is_in_text_input = true;
                        *start_pos = (column, row);

                        stream.print("T".to_owned(), 0)?;
                    }
                }
            }
        }
        MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column,
            row,
            ..
        } => {
            if column < UI_LENGTH && row < UI_HEIGHT {
                return Ok(());
            }

            stream.execute(cursor::MoveTo(column, row))?;
            let tool = tool::get_tool();
            match tool.tool_type {
                ToolType::Pen => {
                    stream.print(SIZES[tool.size_index].to_string(), 0)?;
                }
                ToolType::Eraser => {
                    stream.print(" ".to_owned(), 0)?;
                }
                _ => {}
            }
        }
        MouseEvent {
            kind: MouseEventKind::ScrollUp,
            ..
        } => {
            let mut idx = tool::get_tool().size_index;
            idx = (idx + 1).min(SIZES.len() - 1);
            tool::set_size_index(idx);
        }
        MouseEvent {
            kind: MouseEventKind::ScrollDown,
            ..
        } => {
            let mut idx = tool::get_tool().size_index;
            idx = (idx as i32 - 1).max(0) as usize;
            tool::set_size_index(idx);
        }
        _ => {}
    }
    Ok(())
}

fn handle_ui_click(stream: &mut StreamHandler, col: u16, row: u16) -> std::io::Result<()> {
    match col {
        1..=5 => {
            tool::set_tool_type(ToolType::Pen);
        }
        10..=12 => {
            tool::set_tool_type(ToolType::Eraser);
        }
        14..=16 => {
            tool::set_tool_type(ToolType::text_default());
        }
        18..=20 => {
            stream.execute(terminal::Clear(ClearType::All))?;
        }
        _ => {}
    }

    Ok(())
}

pub fn is_in_text_field() -> bool {
    matches!(
        tool::get_tool().tool_type,
        ToolType::Text {
            is_in_text_input: true,
            ..
        }
    )
}

pub fn handle_text_input(stream: &mut StreamHandler, event: KeyEvent) -> std::io::Result<()> {
    let ToolType::Text {
        start_pos: (col, row),
        mut text,
        mut cursor_index,
        is_in_text_input: true,
    } = tool::get_tool().tool_type
    else {
        // Just a random error (Hate not being able to do good error handling without crates)
        return Err(std::io::Error::last_os_error());
    };

    let mut is_in_text_input: bool = true;
    let mut just_deleted: bool = false;

    match event {
        KeyEvent {
            code: KeyCode::Esc, ..
        } => {
            let _stroke = tool::finish_stroke();

            is_in_text_input = false;
        }
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
        } => {
            let (lhs, rhs) = text.split_at(cursor_index);

            text = format!("{}{}{}", lhs, c, rhs);
            cursor_index += 1;
        }
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            ..
        } => {
            let (lhs, rhs) = text.split_at(cursor_index);

            text = format!("{}{}{}", lhs, c.to_uppercase(), rhs);
            cursor_index += 1;
        }
        KeyEvent {
            code: KeyCode::Backspace,
            kind: KeyEventKind::Press,
            ..
        } => {
            if cursor_index != 0 {
                let (lhs, rhs) = text.split_at(cursor_index);

                let mut lhs = lhs.to_owned();
                lhs.pop();

                text = lhs.to_owned() + rhs;
                cursor_index -= 1;

                if cursor_index == (text.len() as i32 - 1).min(0) as usize {
                    just_deleted = true;
                }
            }
        }
        KeyEvent {
            code: KeyCode::Delete,
            kind: KeyEventKind::Press,
            ..
        } => {
            let (lhs, rhs) = text.split_at(cursor_index);

            if !rhs.is_empty() {
                let rhs = &rhs[1..];

                text = lhs.to_owned() + rhs;
            }
        }
        KeyEvent {
            code: KeyCode::Left,
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            ..
        } => {
            cursor_index = cursor_index.saturating_sub(1);
        }
        KeyEvent {
            code: KeyCode::Right,
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            ..
        } => {
            if cursor_index < text.len() {
                cursor_index += 1;
            }
        }

        _ => {}
    }

    // This is so the text cursor isn't left behind
    if just_deleted {
        text.push(' ');
    }

    if is_in_text_input {
        text.push(' ');
        let mut out_text = String::new();
        for (i, c) in text.chars().enumerate() {
            if i == cursor_index {
                let string = c.underlined().blue().to_string();
                out_text.push_str(&string);
            } else {
                out_text.push(c);
            }
        }
        text.pop();

        stream.print_at(out_text.blue().to_string(), 0, col, row)?;
    } else {
        stream.print_at(text.clone(), 0, col, row)?;
    }

    if just_deleted {
        text.pop();
    }

    if is_in_text_input {
        tool::set_tool_type(ToolType::Text {
            start_pos: (col, row),
            text,
            cursor_index,
            is_in_text_input,
        });
    } else {
        tool::set_tool_type(ToolType::text_default());
    }

    stream.execute(cursor::Hide)?;

    Ok(())
}
