use std::iter::Iterator;

#[derive(Default)]
pub struct RingBuffer {
    data: Box<[u64]>,

    write_pos: usize,
    read_pos: usize,
}

impl RingBuffer {
    pub fn new(buffer_size: usize) -> Self {
        RingBuffer {
            data: vec![0; buffer_size].into_boxed_slice(),
            write_pos: 0,
            read_pos: 0,
        }
    }

    pub fn from_vec(input: Vec<u64>) -> Self {
        RingBuffer {
            data: input.into_boxed_slice(),
            write_pos: 0,
            read_pos: 0,
        }
    }

    pub fn push(&mut self, value: u64) {
        self.data[self.write_pos] = value;

        self.write_pos += 1;
        if self.write_pos >= self.data.len() {
            self.write_pos = 0
        }
    }

    pub fn get_data(&self) -> Vec<u64> {
        self.data.to_vec()
    }
}

impl Iterator for RingBuffer {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let ret = self.data[self.read_pos];

        self.read_pos += 1;

        if self.read_pos >= self.data.len() {
            self.read_pos = 0;
        }

        Some(ret)
    }
}
