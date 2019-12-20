use super::Int;
use std::borrow::Cow;

pub struct Memory {
    pub cells: Vec<Int>,
    rel_base: Int,
}

impl Memory {
    pub fn load(program: Cow<'_, [Int]>) -> Self {
        Self {
            cells: program.into_owned(),
            rel_base: 0,
        }
    }

    pub fn read_4(&mut self, offset: usize) -> &[Int; 4] {
        if offset + 3 >= self.cells.len() {
            self.grow(offset + 4)
        }

        unsafe {
            let slice = self.cells.get_unchecked(offset..offset + 4);
            &*(slice.as_ptr() as *const _)
        }
    }

    pub fn get(&mut self, offset: Int) -> &mut Int {
        let offset = offset as usize;

        if offset >= self.cells.len() {
            self.grow(offset);
        }

        unsafe { self.cells.get_unchecked_mut(offset) }
    }

    pub fn get_relative(&mut self, base_offset: Int) -> &mut Int {
        self.get(base_offset + self.rel_base)
    }

    pub fn move_relative_base(&mut self, delta: Int) {
        self.rel_base += delta;
    }

    fn grow(&mut self, min_size: usize) {
        self.cells.resize(min_size + 1024, 0)
    }
}
