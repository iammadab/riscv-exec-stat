#[derive(Default)]
pub struct HostIO {
    pub input_stream: Vec<u8>,
    pub input_cursor: usize,
}

impl HostIO {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_input_stream(&mut self, input: Vec<u8>) {
        self.input_stream = input;
        self.input_cursor = 0;
    }
}
