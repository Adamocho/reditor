#[derive(Debug)]
pub struct PieceTable {
    rows: Vec<PieceTableEntry>,
    original_buffer: String,
    add_buffer: String,
}

#[derive(Clone, Debug)]
pub struct PieceTableEntry {
    buffer: Buffer,
    start_index: u16,
    length: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Buffer {
    Original,
    Add
}

impl PieceTable {
    pub fn new(original_buffer: String) -> Self {
        Self {
            rows: vec![
                PieceTableEntry {
                    buffer: Buffer::Original,
                    start_index: 0,
                    length: original_buffer.len() as u16
                }
            ],
            original_buffer,
            add_buffer: String::new(),
        }
    }

    pub fn set_rows(&mut self, rows: &Vec<PieceTableEntry>) {
        self.rows = rows.to_vec();
    }

    fn is_appendable(&self, entry: &PieceTableEntry) -> bool {
        if self.add_buffer.len() < 2 {
            return false;
        }

        // guaranteed this will not panic due to the previous check
        let add_buffer_length_modified = self.add_buffer.len().checked_sub(1).unwrap();

        // minus 2 because the add buffer has been appended
        entry.buffer == Buffer::Add && 
        entry.start_index + entry.length == add_buffer_length_modified as u16
    }

    fn shrink_or_delete_entry(&mut self, index: u16) {
        let entry = self.rows.get(index as usize);

        if entry.is_none() {
            return;
        }

        // this cannot panic
        let entry = entry.unwrap();

        if entry.length < 2 {
            self.rows.remove(index as usize);
        } else {
            self.rows[index as usize] = PieceTableEntry {
                length: entry.length - 1,
                buffer: entry.buffer,
                start_index: entry.start_index,
            };
        }
    }

    pub fn index(&self, index: u16) -> char {
        let mut length_counter: u16 = 0;
        let absolute_index;

        for entry in &self.rows {
            if index < length_counter + entry.length {
                absolute_index = index - length_counter + entry.start_index;
                match entry.buffer {
                    Buffer::Original => return self.original_buffer.chars().nth(absolute_index.into()).unwrap_or(' '),
                    Buffer::Add => return self.add_buffer.chars().nth(absolute_index.into()).unwrap_or(' '),
                }
            } else {
                length_counter += entry.length;
            }
        };

        // if not found, return an empty space
        // not ideal, maybe return an Option<char>??
        return ' ';
    }

    pub fn insert(&mut self, c: char, index: u16) {
        // add char to the add_buffer
        self.add_buffer.push(c);

        let mut length_counter: u16 = 0;
        let mut entry_index: u16 = 0;
        let mut searched_entry: &PieceTableEntry;
        let mut previous_entry: Option<&PieceTableEntry> = None;
        let mut relative_index: u16 = 0;
        let mut is_found: bool = false;

        if self.rows.len() == 0 {
            return;
        }
        searched_entry = &self.rows[0];

        for entry in &self.rows {
            if index < length_counter + entry.length {
                is_found = true;
                relative_index = index - length_counter;
                searched_entry = entry;
                break
            } else {
                length_counter += entry.length;
            }
            entry_index += 1;
            previous_entry = Some(entry);
        };

        if !is_found {
            return ();
        }

        let new_entry = PieceTableEntry {
            buffer: Buffer::Add,
            length: 1,
            start_index: self.add_buffer.len().checked_sub(1).unwrap_or(0) as u16,
        };

        // start
        if relative_index == 0 {
            if let Some(previous_entry) = previous_entry {
                if self.is_appendable(previous_entry) {
                    self.rows[entry_index.checked_sub(1).unwrap() as usize] = PieceTableEntry {
                        length: previous_entry.length + 1,
                        ..*previous_entry
                    };
                }
            } else if self.is_appendable(searched_entry) {
                self.rows[entry_index as usize] = PieceTableEntry {
                    length: searched_entry.length + 1,
                    ..*searched_entry
                };
            } else {
                self.rows.insert(entry_index as usize, new_entry);
            }
            return;
        }
        // end
        else if relative_index == searched_entry.length - 1 {
            if self.is_appendable(searched_entry) {
                self.rows[entry_index as usize] = PieceTableEntry {
                    length: searched_entry.length + 1,
                    ..*searched_entry
                };
            } else {
                self.rows.insert(entry_index as usize + 1, new_entry);
            }
            return;
        }
        
        // middle
        let start_entry = PieceTableEntry {
            length: relative_index,
            ..*searched_entry
        };
        let end_entry = PieceTableEntry {
            length: searched_entry.length - relative_index,
            start_index: searched_entry.start_index + relative_index,
            ..*searched_entry
        };

        self.rows[entry_index as usize] = start_entry;
        self.rows.insert(entry_index as usize + 1, new_entry);
        self.rows.insert(entry_index as usize + 2, end_entry);
    }

    pub fn delete(&mut self, index: u16) {
        let mut length_counter: u16 = 0;
        let mut entry_index: u16 = 0;
        let mut searched_entry: &PieceTableEntry;
        let mut relative_index: u16 = 0;
        let mut is_found: bool = false;

        if self.rows.len() == 0 {
            return;
        }

        searched_entry = &self.rows[0];

        for entry in &self.rows {
            if index < length_counter + entry.length {
                is_found = true;
                relative_index = index - length_counter;
                searched_entry = entry;
                break
            } else {
                length_counter += entry.length;
            }
            entry_index += 1;
        };
        
        if !is_found {
            return ();
        }
        
        // start
        if relative_index == 0 {
            if entry_index > 0 {
                self.shrink_or_delete_entry(entry_index - 1);
            }
            return;
        }

        // end
        if relative_index == searched_entry.length - 1 {
            self.shrink_or_delete_entry(entry_index);
            return;
        }
        
        // middle
        let first_part_entry = PieceTableEntry {
            length: relative_index - 1,
            ..*searched_entry
        };
        let second_part_entry = PieceTableEntry {
            length: searched_entry.length - relative_index,
            start_index: searched_entry.start_index + relative_index,
            ..*searched_entry
        };

        if second_part_entry.length > 0 {
            self.rows.insert(entry_index as usize + 1, second_part_entry);
        }

        if first_part_entry.length > 0 {
            self.rows[entry_index as usize] = first_part_entry;
        } else {
            self.rows.remove(entry_index as usize);
        }
    }
}
