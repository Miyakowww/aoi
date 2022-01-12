use crate::AoType;

struct Chip {
    data: Vec<AoType>,
}

impl Chip {
    fn new() -> Chip {
        Chip {
            data: Vec::with_capacity(256),
        }
    }

    fn check(&mut self, index: u8) {
        while index as usize >= self.data.len() {
            self.data.push(AoType::default());
        }
    }

    fn get(&mut self, index: u8) -> &AoType {
        self.check(index);
        &self.data[index as usize]
    }

    fn set(&mut self, index: u8, value: AoType) {
        self.check(index);
        self.data[index as usize] = value;
    }
}

struct Page {
    chips: Vec<Option<Box<Chip>>>,
}

impl Page {
    fn new() -> Page {
        Page {
            chips: Vec::with_capacity(256),
        }
    }

    fn get_chip(&mut self, index: u16) -> &mut Chip {
        let chip_index = (index >> 8) as usize;
        while chip_index >= self.chips.len() {
            self.chips.push(None);
        }

        if self.chips[chip_index].is_none() {
            self.chips[chip_index] = Some(Box::new(Chip::new()));
        }
        self.chips[chip_index].as_mut().unwrap()
    }

    fn get(&mut self, index: u16) -> &AoType {
        self.get_chip(index).get((index & 0xFF) as u8)
    }

    fn set(&mut self, index: u16, value: AoType) {
        self.get_chip(index).set((index & 0xFF) as u8, value);
    }
}

struct Section {
    pages: Vec<Option<Box<Page>>>,
}

impl Section {
    fn new() -> Section {
        Section {
            pages: Vec::with_capacity(256),
        }
    }

    fn get_page(&mut self, index: u32) -> &mut Page {
        let page_index = ((index >> 16) & 0xFF) as usize;
        while page_index >= self.pages.len() {
            self.pages.push(None);
        }

        if self.pages[page_index].is_none() {
            self.pages[page_index] = Some(Box::new(Page::new()));
        }
        self.pages[page_index].as_mut().unwrap()
    }

    fn get(&mut self, index: u32) -> &AoType {
        self.get_page(index).get((index & 0xFFFF) as u16)
    }

    fn set(&mut self, index: u32, value: AoType) {
        self.get_page(index).set((index & 0xFFFF) as u16, value);
    }
}

#[derive(Default)]
pub struct Memory {
    sections: Vec<Option<Box<Section>>>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            sections: Vec::with_capacity(256),
        }
    }

    fn get_section(&mut self, index: u32) -> &mut Section {
        let section_index = ((index >> 24) & 0xFF) as usize;
        while section_index >= self.sections.len() {
            self.sections.push(None);
        }

        if self.sections[section_index].is_none() {
            self.sections[section_index] = Some(Box::new(Section::new()));
        }
        self.sections[section_index].as_mut().unwrap()
    }

    pub fn get(&mut self, index: u32) -> &AoType {
        self.get_section(index).get(index & 0xFFFFFF)
    }

    pub fn set(&mut self, index: u32, value: AoType) {
        self.get_section(index).set(index & 0xFFFFFF, value);
    }
}
