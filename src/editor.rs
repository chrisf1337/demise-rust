use buffer::Buffer;

pub struct Editor {
    pub buffers: Vec<Buffer>,
    pub current_buffer_index: usize
}

impl<'a> Editor {
    pub fn new() -> Editor {
        Editor {
            buffers: vec![Buffer::new()],
            current_buffer_index: 0
        }
    }

    pub fn current_buffer(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer_index]
    }
}
