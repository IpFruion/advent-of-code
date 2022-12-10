pub trait TrimInPlace {
    fn trim_in_place(&mut self);
}

impl TrimInPlace for String {
    fn trim_in_place(&mut self) {
        let trimmed = self.trim();
        let start = trimmed.as_ptr() as usize - self.as_ptr() as usize;
        let end = trimmed.len();
        if start > 0 {
            self.drain(..start);
        }
        self.truncate(end);
    }
}
