use std::collections::VecDeque;

const UNDO_LIST_MAX_SIZE: usize = 1024;

static mut UNDO_LIST: VecDeque<Vec<(u16, u16, char, char)>> = VecDeque::new();

static mut UNDO_PTR: usize = 0;

/// Pushes new element to the undo list
pub fn push_to_undolist(stroke: Vec<(u16, u16, char, char)>) {
    unsafe {
        if UNDO_PTR != 0 {
            UNDO_LIST.make_contiguous();
            UNDO_LIST.drain(..UNDO_PTR);
            UNDO_PTR = 0;
        }
        UNDO_LIST.push_front(stroke);

        if UNDO_LIST.len() > UNDO_LIST_MAX_SIZE {
            UNDO_LIST.pop_back();
        }
    }
}

/// Does an Undo.
pub fn get_prev_undo() -> Option<Vec<(u16, u16, char, char)>> {
    unsafe {
        if UNDO_PTR >= UNDO_LIST.len() {
            return None;
        }

        UNDO_PTR += 1;

        UNDO_LIST.get(UNDO_PTR).cloned()
    }
}

/// Does a Redo
pub fn get_next_undo() -> Option<Vec<(u16, u16, char, char)>> {
    unsafe {
        if UNDO_PTR == 0 {
            return None;
        }

        UNDO_PTR -= 1;

        UNDO_LIST.get(UNDO_PTR).cloned()
    }
}
