use std::path::PathBuf;
use std::fs;
use std::io;

#[derive(Default)]
pub struct Buffer {
    pub content: String,
    lines: Vec<String>,
    cursor: Cursor,
    filename: Option<PathBuf>,
    modified: bool,
    syntax_name: Option<String>,
    is_new: bool,
    path: Option<PathBuf>,
}

#[derive(Default)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            is_new: true,
            lines: vec![String::new()],
            path: None,
            ..Default::default()
        }
    }

    pub fn set_filename(&mut self, filename: PathBuf) {
        self.filename = Some(filename);
    }

    pub fn get_filename(&self) -> Option<&PathBuf> {
        self.filename.as_ref()
    }

    pub fn should_quit(&self) -> bool {
        !self.modified
    }

    pub fn insert_char(&mut self, c: char) {
        let Cursor { line, column } = self.cursor;
        let current_line = &mut self.lines[line];
        current_line.insert(column, c);
        self.cursor.column += 1;
        self.modified = true;
    }

    pub fn delete_char(&mut self) {
        // ...existing code...
    }
    
    pub fn syntax_name(&self) -> Option<&str> {
        self.path
            .as_ref()
            .and_then(|p| p.extension())
            .and_then(|ext| ext.to_str())
    }

    pub fn set_syntax_name(&mut self, name: Option<String>) {
        self.syntax_name = name;
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn is_new(&self) -> bool {
        self.is_new
    }

    pub fn mark_saved(&mut self) {
        self.modified = false;
        self.is_new = false;
    }

    pub fn mark_modified(&mut self) {
        self.modified = true;
    }

    pub fn set_cursor(&mut self, line: usize, column: usize) {
        self.cursor.line = line;
        self.cursor.column = column;
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        (self.cursor.line, self.cursor.column)
    }

    pub fn insert_line(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn insert_at_cursor(&mut self, line: usize, col: usize, ch: char) {
        if let Some(line_str) = self.lines.get_mut(line) {
            if col <= line_str.len() {
                line_str.insert(col, ch);
                self.modified = true;
            }
        }
    }

    pub fn delete_at_cursor(&mut self, line: usize, col: usize) -> Option<char> {
        if let Some(line_str) = self.lines.get_mut(line) {
            if col < line_str.len() {
                self.modified = true;
                return Some(line_str.remove(col));
            }
        }
        None
    }

    pub fn line_length(&self, line: usize) -> usize {
        self.lines.get(line).map(|l| l.len()).unwrap_or(0)
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    // Add other buffer manipulation methods...

    pub fn write_temp_file(&self) -> io::Result<()> {
        if let Some(temp_path) = self.get_temp_path() {
            let content = self.lines.join("\n");
            fs::write(&temp_path, content)?;
            Ok(())
        } else {
            Ok(()) // No temp file for new buffers
        }
    }

    pub fn cleanup_temp_file(&self) -> io::Result<()> {
        if let Some(temp_path) = self.get_temp_path() {
            if temp_path.exists() {
                fs::remove_file(temp_path)?;
            }
        }
        Ok(())
    }

    pub fn check_for_temp_file(&self) -> Option<PathBuf> {
        self.get_temp_path().filter(|path| path.exists())
    }

    pub fn recover_from_temp(&mut self, temp_path: &PathBuf) -> io::Result<()> {
        let content = fs::read_to_string(temp_path)?;
        self.lines.clear();
        for line in content.lines() {
            self.insert_line(line.to_string());
        }
        self.mark_modified();
        Ok(())
    }

    pub fn get_line(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }

    pub fn delete_line(&mut self, index: usize) -> Option<String> {
        if index < self.lines.len() {
            Some(self.lines.remove(index))
        } else {
            None
        }
    }

    pub fn insert_line_at(&mut self, index: usize, line: String) {
        if index <= self.lines.len() {
            self.lines.insert(index, line);
        } else {
            self.lines.push(line);
        }
    }

    pub fn get_wrapped_lines(&self, width: usize) -> Vec<String> {
        let mut wrapped = Vec::new();
        for line in &self.lines {
            if line.len() <= width {
                wrapped.push(line.clone());
            } else {
                let mut current = line.as_str();
                while !current.is_empty() {
                    let mut split = width;
                    if split < current.len() {
                        // Try to split at word boundary
                        while split > 0 && !current.is_char_boundary(split) {
                            split -= 1;
                        }
                        // If no word boundary found, force split at width
                        if split == 0 {
                            split = width;
                            while split < current.len() && !current.is_char_boundary(split) {
                                split += 1;
                            }
                        }
                    } else {
                        split = current.len();
                    }
                    wrapped.push(current[..split].to_string());
                    current = &current[split..];
                }
            }
        }
        wrapped
    }

    pub fn get_real_position(&self, width: usize, visual_line: usize, visual_col: usize) -> (usize, usize) {
        let mut remaining_visual = visual_line;
        
        for (i, line) in self.lines.iter().enumerate() {
            let wrapped_count = (line.len() + width - 1) / width;
            if remaining_visual < wrapped_count {
                let real_col = remaining_visual * width + visual_col;
                return (i, real_col.min(line.len()));
            }
            remaining_visual -= wrapped_count;
        }
        
        (self.lines.len().saturating_sub(1), 0)
    }

    pub fn get_visual_position(&self, width: usize, real_line: usize, real_col: usize) -> (usize, usize) {
        let mut visual_line = 0;
        
        // Count wrapped lines before our target line
        for line in &self.lines[..real_line] {
            visual_line += (line.len() + width - 1) / width;
        }
        
        // Add the wrapped lines in our target line
        let _target_line = self.lines.get(real_line).unwrap_or(&String::new());
        visual_line += real_col / width;
        let visual_col = real_col % width;
        
        (visual_line, visual_col)
    }

    pub fn move_to_line(&mut self, line: usize) {
        if line < self.lines.len() {
            self.cursor.line = line;
            self.adjust_cursor_x();
        }
    }

    pub fn join_lines(&mut self) {
        let real_line = self.cursor.line;
        if real_line < self.lines.len() - 1 {
            let current = self.lines[real_line].clone();
            let next = self.lines.remove(real_line + 1);
            self.lines[real_line] = current + &next;
        }
    }

    fn adjust_cursor_x(&mut self) {
        let line_len = self.line_length(self.cursor.line);
        if self.cursor.column > line_len {
            self.cursor.column = line_len;
        }
    }

    fn get_temp_path(&self) -> Option<PathBuf> {
        self.filename.as_ref().map(|path| {
            let mut temp = path.clone();
            temp.set_extension("tmp");
            temp
        })
    }
}

#[derive(Default)]
pub struct BufferCollection {
    buffers: Vec<Buffer>,
    active: usize,
}

impl BufferCollection {
    pub fn new() -> Self {
        let mut collection = Self {
            buffers: Vec::new(),
            active: 0,
        };
        collection.buffers.push(Buffer::new());
        collection
    }

    pub fn add_buffer(&mut self) -> usize {
        self.buffers.push(Buffer::new());
        let idx = self.buffers.len() - 1;
        self.active = idx;
        idx
    }

    pub fn remove_buffer(&mut self, index: usize) {
        if index < self.buffers.len() && self.buffers.len() > 1 {
            self.buffers.remove(index);
            if self.active >= self.buffers.len() {
                self.active = self.buffers.len() - 1;
            }
        }
    }

    pub fn get_active(&self) -> &Buffer {
        &self.buffers[self.active]
    }

    pub fn get_active_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.active]
    }

    pub fn set_active(&mut self, index: usize) {
        if index < self.buffers.len() {
            self.active = index;
        }
    }

    pub fn get_buffer(&self, index: usize) -> Option<&Buffer> {
        self.buffers.get(index)
    }

    pub fn get_buffer_mut(&mut self, index: usize) -> Option<&mut Buffer> {
        self.buffers.get_mut(index)
    }

    pub fn open_file(&mut self, path: &str) -> std::io::Result<usize> {
        let content = std::fs::read_to_string(path)?;
        let mut buffer = Buffer::new();
        
        // Set filename and load content
        buffer.set_filename(PathBuf::from(path));
        for line in content.lines() {
            buffer.insert_line(line.to_string());
        }
        
        // Add to collection
        self.buffers.push(buffer);
        let idx = self.buffers.len() - 1;
        self.active = idx;
        Ok(idx)
    }

    pub fn save_file(&mut self, index: usize, path: Option<&str>) -> std::io::Result<()> {
        if let Some(buffer) = self.buffers.get_mut(index) {
            // Update filename if provided
            if let Some(new_path) = path {
                buffer.set_filename(PathBuf::from(new_path));
            }

            // Get current filename
            if let Some(filename) = buffer.get_filename() {
                let content: String = buffer.lines().join("\n");
                std::fs::write(filename, content)?;
                buffer.mark_saved();
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No filename specified"
                ))
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Buffer not found"
            ))
        }
    }

    pub fn get_filename(&self, index: usize) -> Option<String> {
        self.buffers.get(index)
            .and_then(|buf| buf.get_filename())
            .map(|path| path.to_string_lossy().into_owned())
    }

    pub fn is_modified(&self, index: usize) -> bool {
        self.buffers.get(index)
            .map(|buf| buf.is_modified())
            .unwrap_or(false)
    }

    pub fn get_active_index(&self) -> usize {
        self.active
    }

    pub fn add_buffer_with(&mut self, buffer: Buffer) -> usize {
        self.buffers.push(buffer);
        let idx = self.buffers.len() - 1;
        self.active = idx;
        idx
    }
}
