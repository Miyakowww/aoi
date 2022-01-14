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

    fn get(&self, index: u8) -> AoType {
        if index as usize >= self.data.len() {
            AoType::default()
        } else {
            self.data[index as usize].clone()
        }
    }

    fn set(&mut self, index: u8, value: AoType) {
        while index as usize >= self.data.len() {
            self.data.push(AoType::default());
        }
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

    fn get(&self, index: u16) -> AoType {
        let chip_index = (index >> 8) as usize;
        if chip_index >= self.chips.len() {
            AoType::default()
        } else {
            let chip = self.chips[chip_index].as_ref().unwrap();
            chip.get((index & 0xff) as u8)
        }
    }

    fn set(&mut self, index: u16, value: AoType) {
        let chip_index = (index >> 8) as usize;
        while chip_index >= self.chips.len() {
            self.chips.push(None);
        }

        if self.chips[chip_index].is_none() {
            self.chips[chip_index] = Some(Box::new(Chip::new()));
        }
        self.chips[chip_index]
            .as_mut()
            .unwrap()
            .set((index & 0xFF) as u8, value);
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

    fn get(&self, index: u32) -> AoType {
        let page_index = ((index >> 16) & 0xFF) as usize;
        if page_index >= self.pages.len() {
            AoType::default()
        } else {
            let page = self.pages[page_index].as_ref().unwrap();
            page.get((index & 0xFFFF) as u16)
        }
    }

    fn set(&mut self, index: u32, value: AoType) {
        let page_index = ((index >> 16) & 0xFF) as usize;
        while page_index >= self.pages.len() {
            self.pages.push(None);
        }

        if self.pages[page_index].is_none() {
            self.pages[page_index] = Some(Box::new(Page::new()));
        }
        self.pages[page_index]
            .as_mut()
            .unwrap()
            .set((index & 0xFFFF) as u16, value);
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

    pub fn get(&self, index: u32) -> AoType {
        let section_index = ((index >> 24) & 0xFF) as usize;
        if section_index >= self.sections.len() {
            AoType::default()
        } else {
            let section = self.sections[section_index].as_ref().unwrap();
            section.get(index)
        }
    }

    pub fn set(&mut self, index: u32, value: AoType) {
        let section_index = ((index >> 24) & 0xFF) as usize;
        while section_index >= self.sections.len() {
            self.sections.push(None);
        }

        if self.sections[section_index].is_none() {
            self.sections[section_index] = Some(Box::new(Section::new()));
        }
        self.sections[section_index]
            .as_mut()
            .unwrap()
            .set(index & 0xFFFFFF, value);
    }
}
