use super::Int;
use std::borrow::Cow;

pub struct Memory {
    pub cells: Vec<Int>,
    rel_base: Int,
}

#[derive(Debug, Clone, Copy)]
pub struct AddressRelative(pub Int);
#[derive(Debug, Clone, Copy)]
pub struct AddressAbsolute(pub Int);

pub trait Address<Idx> {
    fn get(&mut self, idx: Idx) -> &mut Int;
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

    pub fn move_relative_base(&mut self, delta: Int) {
        self.rel_base += delta;
    }

    fn grow(&mut self, min_size: usize) {
        self.cells.resize(min_size + 1024, 0)
    }
}

impl Address<AddressAbsolute> for Memory {
    fn get(&mut self, offset: AddressAbsolute) -> &mut Int {
        let offset = offset.0 as usize;

        if offset >= self.cells.len() {
            self.grow(offset);
        }

        unsafe { self.cells.get_unchecked_mut(offset) }
    }
}

impl Address<AddressRelative> for Memory {
    fn get(&mut self, rel_offset: AddressRelative) -> &mut Int {
        let abs_offset = AddressAbsolute(rel_offset.0 + self.rel_base);

        self.get(abs_offset)
    }
}
