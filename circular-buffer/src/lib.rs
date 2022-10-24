pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    read_head: usize,
    write_head: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let data = vec![None; capacity];
        Self {
            data,
            read_head: 0,
            write_head: capacity - 1,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        let next_head = increment_head(self.write_head, self.data.len());
        if self.data[next_head].is_some() {
            return Err(Error::FullBuffer);
        }
        self.data[next_head] = Some(element);
        self.write_head = next_head;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.data[self.read_head].is_none() {
            return Err(Error::EmptyBuffer);
        }
        let e = self.data[self.read_head].take().unwrap();
        self.read_head = increment_head(self.read_head, self.data.len());
        Ok(e)
    }

    pub fn clear(&mut self) {
        (0..self.data.len()).for_each(|i| self.data[i] = None);
    }

    pub fn overwrite(&mut self, element: T) {
        let next_head = increment_head(self.write_head, self.data.len());
        if self.data[next_head].is_some() {
            self.read_head = increment_head(self.read_head, self.data.len());
        }
        self.data[next_head] = Some(element);
        self.write_head = next_head;
    }
}

fn increment_head(head: usize, capacity: usize) -> usize {
    (head + 1) % capacity
}
