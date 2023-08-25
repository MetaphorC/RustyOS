use x86_64::structures::idt::InterruptsDescriptionTable;

pub fn init_idt() {
    let mut idt = InterruptsDescriptionTable::new();
}


