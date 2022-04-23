use std::fmt::Arguments;
use std::process::exit;

use crate::machine::Machine;
use crate::machine::register::RegisterId;
use crate::machine::thread::ThreadId;

impl Machine<'_> {
    pub fn error_pause(&self) -> ! {
        self.error(format_args!("No thread can continue."));
    }

    pub fn error_data_race(&self, register_id: RegisterId) -> ! {
        self.error(format_args!("Data race on register `{}`.", register_id));
    }

    pub fn error_cursor_address(&self, thread_id: ThreadId, address: u64) -> ! {
        self.error_thread(thread_id, format_args!("Cursor address {:#X} is out of the program bounds.", address));
    }

    pub fn error_invalid_opcode(&self, thread_id: ThreadId, opcode: u8) -> ! {
        self.error_thread(thread_id, format_args!("Invalid opcode `{:#X}`.", opcode));
    }

    pub fn error_invalid_register(&self, thread_id: ThreadId, register: u8) -> ! {
        self.error_thread(thread_id, format_args!("Invalid register {}.", register));
    }

    pub fn error_invalid_lock(&self, thread_id: ThreadId, lock: u8) -> ! {
        self.error_thread(thread_id, format_args!("Invalid lock {}.", lock));
    }

    pub fn error_invalid_thread(&self, thread_id: ThreadId, thread: u8) -> ! {
        self.error_thread(thread_id, format_args!("Invalid thread {}.", thread));
    }

    pub fn error_division_by_zero(&self, thread_id: ThreadId) -> ! {
        self.error_thread(thread_id, format_args!("Division by zero."));
    }

    pub fn error_input_read(&self, thread_id: ThreadId) -> ! {
        self.error_thread(thread_id, format_args!("Cannot read input."));
    }

    pub fn error_input_parse(&self, thread_id: ThreadId) -> ! {
        self.error_thread(thread_id, format_args!("Cannot parse input."));
    }
}

impl Machine<'_> {
    fn error(&self, message: Arguments) -> ! {
        eprintln!("ERROR (cycle {}): {}", self.counter, message);
        exit(0);
    }

    fn error_thread(&self, thread_id: ThreadId, message: Arguments) -> !{
        let thread = self.threads.get(thread_id);
        self.error(format_args!("In thread `{}`, address {:#X}. {}", thread.id(), thread.cursor(), message));
    }
}
