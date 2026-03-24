use elf::{
    ElfBytes,
    abi::{EM_RISCV, ET_EXEC, PT_LOAD},
    endian::LittleEndian,
    file::Class,
};

use crate::memory::MemoryDefault;

/// Decodes the elf bytes,
/// loads segments into memory and return the pc.
pub(crate) fn decode_elf(bytes: &[u8]) -> (MemoryDefault, u64) {
    let file =
        ElfBytes::<LittleEndian>::minimal_parse(bytes).expect("failed to parse the elf file");
    let ehdr = file.ehdr;

    assert_eq!(ehdr.class, Class::ELF64);
    assert_eq!(ehdr.e_type, ET_EXEC);
    assert_eq!(ehdr.e_machine, EM_RISCV);

    let entry = ehdr.e_entry;

    // load the program headers into memory
    let mut memory = MemoryDefault::default();

    // iterate over the program headers
    // load header of type `PT_LOAD` to memory
    let segments = file.segments().expect("has no program headers");
    for ph in segments.iter() {
        if ph.p_type != PT_LOAD {
            continue;
        }

        let offset = ph.p_offset as usize;
        let filesz = ph.p_filesz as usize;
        let vaddr = ph.p_vaddr;
        let memsz = ph.p_memsz as usize;

        if memsz < filesz {
            panic!("malformed elf file");
        }

        if filesz > 0 {
            let data = &bytes[offset..offset + filesz];
            memory.write_n_bytes(vaddr, data);
        }

        if memsz > filesz {
            memory.zero_fill(vaddr + filesz as u64, memsz - filesz);
        }
    }

    (memory, entry)
}
