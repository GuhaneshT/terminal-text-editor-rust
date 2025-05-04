use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    cursor,
};
use std::io::{self, stdout, Write};
use std::rc::Rc;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

// Rope data structure
#[derive(Clone)]
enum RopeNode {
    Leaf(String),
    Internal {
        left: Rc<RopeNode>,
        right: Rc<RopeNode>,
        weight: usize, // Length of left subtree
    },
}

#[derive(Clone)]
struct Rope {
    root: Rc<RopeNode>,
}

impl Rope {
    fn new() -> Self {
        Rope {
            root: Rc::new(RopeNode::Leaf(String::new())),
        }
    }

    fn from_string(s: &str) -> Self {
        Rope {
            root: Rc::new(RopeNode::Leaf(s.to_string())),
        }
    }

    fn len(&self) -> usize {
        self.total_len(&self.root)
    }
    
    fn total_len(&self, node: &Rc<RopeNode>) -> usize {
        match node.as_ref() {
            RopeNode::Leaf(s) => s.len(),
            RopeNode::Internal { left, right, .. } => {
                self.total_len(left) + self.total_len(right)
            }
        }
    }
    

    fn weight(&self, node: &Rc<RopeNode>) -> usize {
        match node.as_ref() {
            RopeNode::Leaf(s) => s.len(),
            RopeNode::Internal { weight, .. } => *weight,
        }
    }

    fn concat(left: Rope, right: Rope) -> Rope {
        let weight = left.len();
        Rope {
            root: Rc::new(RopeNode::Internal {
                left: left.root,
                right: right.root,
                weight,
            }),
        }
    }

    fn split(&self, index: usize) -> (Rope, Rope) {
        let index = index.min(self.len());
        let (left, right) = self.split_node(&self.root, index);
        (Rope { root: left }, Rope { root: right })
    }

    fn split_node(&self, node: &Rc<RopeNode>, index: usize) -> (Rc<RopeNode>, Rc<RopeNode>) {
        match node.as_ref() {
            RopeNode::Leaf(s) => {
                let index = index.min(s.len());
                let (left, right) = s.split_at(index);
                (
                    Rc::new(RopeNode::Leaf(left.to_string())),
                    Rc::new(RopeNode::Leaf(right.to_string())),
                )
            }
            RopeNode::Internal { left, right, weight } => {
                if index <= *weight {
                    let (ll, lr) = self.split_node(left, index);
                    (
                        ll,
                        Rc::new(RopeNode::Internal {
                            left: lr.clone(),
                            right: right.clone(),
                            weight: self.total_len(&lr),
                        }),
                    )
                } else {
                    let (rl, rr) = self.split_node(right, index - weight);
                    (
                        Rc::new(RopeNode::Internal {
                            left: left.clone(),
                            right: rl.clone(),
                            weight: self.total_len(&left),
                        }),
                        rr,
                    )
                }
            }
        }
    }
    

    fn insert(&self, index: usize, text: &str) -> Rope {
        let (left, right) = self.split(index);
        let middle = Rope::from_string(text);
        Rope::concat(Rope::concat(left, middle), right)
    }

    fn delete(&self, start: usize, len: usize) -> Rope {
        let (left, rest) = self.split(start);
        let rest_len = rest.len();
        let len = len.min(rest_len);
        let (_, right) = rest.split(len);
        Rope::concat(left, right)
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        self.collect(&self.root, &mut result);
        result
    }

    fn collect(&self, node: &Rc<RopeNode>, result: &mut String) {
        match node.as_ref() {
            RopeNode::Leaf(s) => result.push_str(s),
            RopeNode::Internal { left, right, .. } => {
                self.collect(left, result);
                self.collect(right, result);
            }
        }
    }

    fn char_at(&self, index: usize) -> Option<char> {
        self.get_char(&self.root, index)
    }

    fn get_char(&self, node: &Rc<RopeNode>, index: usize) -> Option<char> {
        match node.as_ref() {
            RopeNode::Leaf(s) => s.chars().nth(index),
            RopeNode::Internal { left, right, weight } => {
                if index < *weight {
                    self.get_char(left, index)
                } else {
                    self.get_char(right, index - weight)
                }
            }
        }
    }
}

// Undo/Redo action
#[derive(Clone)]
enum Action {
    Insert { index: usize, text: String },
    Delete { index: usize, text: String },
}

// Text editor state
struct Editor {
    rope: Rope,
    cursor: usize,
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>,
    filename: Option<String>,
    dirty: bool,
    last_key_time: Instant,
    status_message: Option<String>,
}

impl Editor {
    fn new() -> Self {
        Editor {
            rope: Rope::new(),
            cursor: 0,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            filename: None,
            dirty: false,
            last_key_time: Instant::now(),
            status_message: None,
        }
    }

    fn load_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let content = fs::read_to_string(&path)?;
        self.rope = Rope::from_string(&content);
        self.filename = Some(path.as_ref().to_string_lossy().into_owned());
        self.dirty = false;
        self.status_message = Some("File loaded successfully!".to_string());
        Ok(())
    }

    fn save_file(&mut self) -> io::Result<()> {
        if let Some(filename) = &self.filename {
            fs::write(filename, self.rope.to_string())?;
            self.dirty = false;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No filename specified"))
        }
    }

    fn insert(&mut self, text: &str) {
        if text.chars().all(|c| c.is_ascii_graphic() || c.is_whitespace() || c == '\n') {
            self.rope = self.rope.insert(self.cursor, text);
            self.undo_stack.push(Action::Insert {
                index: self.cursor,
                text: text.to_string(),
            });
            self.redo_stack.clear();
            self.cursor += text.len();
            self.dirty = true;
            self.status_message = None;
        }
    }

    fn delete(&mut self) {
        if self.cursor > 0 {
            let deleted_char = self.rope.char_at(self.cursor - 1).unwrap_or_default().to_string();
            self.rope = self.rope.delete(self.cursor - 1, 1);
            self.cursor -= 1;
            self.undo_stack.push(Action::Delete {
                index: self.cursor,
                text: deleted_char,
            });
            self.redo_stack.clear();
            self.dirty = true;
            self.status_message = None;
        }
    }

    fn undo(&mut self) {
        if let Some(action) = self.undo_stack.pop() {
            match action {
                Action::Insert { index, text } => {
                    self.rope = self.rope.delete(index, text.len());
                    self.cursor = index;
                    self.redo_stack.push(Action::Insert { index, text });
                }
                Action::Delete { index, text } => {
                    self.rope = self.rope.insert(index, &text);
                    self.cursor = index + text.len();
                    self.redo_stack.push(Action::Delete { index, text });
                }
            }
            self.dirty = true;
            self.status_message = Some("Undo performed".to_string());
        } else {
            self.status_message = Some("Nothing to undo".to_string());
        }
    }

    fn redo(&mut self) {
        if let Some(action) = self.redo_stack.pop() {
            match action {
                Action::Insert { index, text } => {
                    self.rope = self.rope.insert(index, &text);
                    self.cursor = index + text.len();
                    self.undo_stack.push(Action::Insert { index, text });
                }
                Action::Delete { index, text } => {
                    self.rope = self.rope.delete(index, text.len());
                    self.cursor = index;
                    self.undo_stack.push(Action::Delete { index, text });
                }
            }
            self.dirty = true;
            self.status_message = Some("Redo performed".to_string());
        } else {
            self.status_message = Some("Nothing to redo".to_string());
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.status_message = None;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor < self.rope.len() {
            self.cursor += 1;
            self.status_message = None;
        }
    }

    fn render(&self) -> io::Result<()> {
        let content = self.rope.to_string();
        let (_term_width, term_height) = terminal::size()?;
        let mut stdout = stdout();

        queue!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        let lines: Vec<&str> = content.split('\n').collect();
        // for (i, line) in lines.iter().take(term_height as usize - 1).enumerate() {
        //     queue!(stdout, cursor::MoveTo(0, i as u16), Print(line))?;
        // }

        let cursor_line = content[..self.cursor].chars().filter(|&c| c == '\n').count();
        let cursor_col = content[..self.cursor]
            .lines()
            .last()
            .map(|l| l.chars().count())
            .unwrap_or(0);

        
        use crossterm::style::{Attribute, SetAttribute, Print, Stylize};

        for (i, line) in lines.iter().enumerate().take(term_height as usize - 1) {
            queue!(stdout, cursor::MoveTo(0, i as u16))?;
        
            if i == cursor_line {
                let mut chars = line.chars().collect::<Vec<_>>();
                let col = cursor_col.min(chars.len());
        
                for (j, ch) in chars.iter().enumerate() {
                    if j == col {
                        queue!(
                            stdout,
                            SetAttribute(Attribute::Underlined),
                            Print(ch),
                            SetAttribute(Attribute::NoUnderline)
                        )?;
                    } else {
                        queue!(stdout, Print(ch))?;
                    }
                }
        
                // Underline a space if cursor is at end of line
                if col == chars.len() {
                    queue!(
                        stdout,
                        SetAttribute(Attribute::Underlined),
                        SetForegroundColor(Color::Cyan),
                        Print(" "),
                        SetAttribute(Attribute::NoUnderline)
                    )?;
                }
        
            } else {
                queue!(stdout, Print(line))?;
            }
        }
        


       

        queue!(stdout, cursor::MoveTo(cursor_col as u16, cursor_line as u16))?;

        let status = self.status_message.as_deref().unwrap_or("");
        queue!(
            stdout,
            cursor::MoveTo(0, term_height - 1),
            SetForegroundColor(Color::Cyan),
            Print(format!(
                "File: {} | Cursor: {} | {} | {}",
                self.filename.as_deref().unwrap_or("Untitled"),
                self.cursor,
                if self.dirty { "[Modified]" } else { "" },
                status
            )),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }
}


fn main() -> io::Result<()> {
    let mut editor = Editor::new();
    if let Some(filename) = std::env::args().nth(1) {
        editor.load_file(filename)?;
    }

    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen)?;

    const DEBOUNCE_DURATION: Duration = Duration::from_millis(10);

    loop {
        editor.render()?;
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
            let now = Instant::now();
            if now.duration_since(editor.last_key_time) < DEBOUNCE_DURATION {
                continue;
            }
            editor.last_key_time = now;

            match (code, modifiers) {
                (KeyCode::Char('a'), KeyModifiers::CONTROL) => break,
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    match editor.save_file() {
                        Ok(()) => editor.status_message = Some("File saved successfully!".to_string()),
                        Err(e) => editor.status_message = Some(format!("Save failed: {}", e)),
                    }
                }
                (KeyCode::Char('m'), KeyModifiers::CONTROL) => {
                    editor.status_message = Some("Menu opened".to_string());
                    // show_popup()?;
                }
                (KeyCode::Char('z'), KeyModifiers::CONTROL) => editor.undo(),
                (KeyCode::Char('y'), KeyModifiers::CONTROL) => editor.redo(),
                (KeyCode::Backspace, _) => editor.delete(),
                (KeyCode::Left, _) => editor.move_cursor_left(),
                (KeyCode::Right, _) => editor.move_cursor_right(),
                (KeyCode::Enter, _) => editor.insert("\n"),
                (KeyCode::Char('x'), KeyModifiers::CONTROL) => {
                    editor.filename = Some("newname".to_string());
                    
                }
                (KeyCode::Char(c), KeyModifiers::SHIFT) => editor.insert(&c.to_string().to_uppercase()),
                (KeyCode::Char(c), KeyModifiers::NONE) => editor.insert(&c.to_string()),
                _ => {}
            }
        }
    }

    execute!(stdout(), terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
