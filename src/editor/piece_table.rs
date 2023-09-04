#[derive(Debug)]
pub struct PieceTable {
    rows: Vec<PieceTableEntry>,
    original_buffer: String,
    add_buffer: String,
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

    fn index(&self, i: u16) -> char {
        let mut counter: u16 = 0;

        for entry in &self.rows {
            // found the correct entry
            if counter + entry.length <= i {
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
        return ' ';
    }

    pub fn insert(&mut self, c: char, index: u16) {
        // add char to the add_buffer
        self.add_buffer.push(c);
    }

    pub fn delete(&mut self, c: char) {

    }
}

#[derive(Clone, Debug)]
pub struct PieceTableEntry {
    buffer: Buffer,
    start_index: u16,
    length: u16,
}

#[derive(Clone, Debug)]
enum Buffer {
    Original,
    Add
}