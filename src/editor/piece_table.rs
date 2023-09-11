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
        entry.buffer == Buffer::Add && 
        // minus 2 because the add buffer has been appended
        entry.start_index + entry.length == self.add_buffer.len() as u16 - 2
    }

    fn shrink_or_delete_entry(&mut self, entry: PieceTableEntry, index: u16) {
        if entry.length == 1 {
            self.rows.remove((index as i32 - 1) as usize);
        } else {
            self.rows[(index as i32 - 1) as usize] = PieceTableEntry {
                length: entry.length - 1,
                ..entry
            };
        }
    }

    fn index(&self, i: u16) -> char {
        let mut counter: u16 = 0;

        for entry in &self.rows {
            // found the correct entry
            if i <= counter + entry.length {
                let relative_index = i - counter;
                let correct_index = relative_index + entry.start_index;
                match entry.buffer {
                    Buffer::Original => return self.original_buffer.chars().nth(correct_index.into()).unwrap(),
                    Buffer::Add => return self.add_buffer.chars().nth(correct_index.into()).unwrap(),
                }
            } else {
                counter += entry.length;
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
        // the correct index of a char table 
        let mut correct_index: u16 = 0;
        let mut is_found: bool = false;

        for entry in &self.rows {
            if index <= length_counter + entry.length {
                is_found = true;
                // calculate the relative 'correct' index in the entry
                correct_index = index - length_counter + entry.start_index;
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
        } else {
            searched_entry = &self.rows[0];
        }

        let new_entry = PieceTableEntry {
            buffer: Buffer::Add,
            length: 1,
            start_index: self.add_buffer.len() as u16 - 1,
        };

        // start
        if correct_index == 0 {
            if previous_entry.is_none() {
                self.rows.insert(entry_index as usize, new_entry);
                return;
            }

            let previous_entry = previous_entry.unwrap();

            if self.is_appendable(previous_entry) {
                self.rows[(entry_index as i32 - 1) as usize] = PieceTableEntry {
                    length: previous_entry.length + 1,
                    ..*previous_entry
                };
                return;
            }
        }
        // end
        else if correct_index == searched_entry.length {
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
            length: correct_index,
            ..*searched_entry
        };

        let end_entry = PieceTableEntry {
            length: searched_entry.length - correct_index,
            start_index: searched_entry.start_index + correct_index,
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
        let mut previous_entry: Option<&PieceTableEntry> = None;
        // the correct index of a char table 
        let mut correct_index: u16 = 0;
        let mut is_found: bool = false;

        for entry in &self.rows {
            if index <= length_counter + entry.length {
                is_found = true;
                // calculate the relative 'correct' index in the entry
                correct_index = index - length_counter + entry.start_index;
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
        } else {
            searched_entry = &self.rows[0];
        }

        dbg!(correct_index, searched_entry, previous_entry);

        // start and end
        if correct_index == 0 {
            if previous_entry.is_none() {
                return;
            }
            let previous_entry = previous_entry.unwrap();
            self.shrink_or_delete_entry(previous_entry.clone(), correct_index);

            return;
        }
        // end
        else if correct_index == searched_entry.length {
            self.shrink_or_delete_entry(searched_entry.clone(), correct_index);

            return;
        }

        // middle
        let first_part_entry = PieceTableEntry {
            length: correct_index - 1,
            ..*searched_entry
        };

        let second_part_entry = PieceTableEntry {
            length: searched_entry.length - correct_index,
            start_index: searched_entry.start_index + correct_index,
            ..*searched_entry
        };

        self.rows[entry_index as usize] = first_part_entry;
        self.rows.insert(entry_index as usize + 1, second_part_entry);
    }
}
