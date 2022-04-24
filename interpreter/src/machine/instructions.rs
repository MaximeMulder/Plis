use std::process::exit;
use term_table::Table;
use term_table::row::Row;
use term_table::table_cell::TableCell;

use crate::machine::Machine;
use crate::machine::thread::ThreadId;
use crate::time::{ TIME_LOAD, TIME_STORE };

impl Machine<'_> {
    pub fn instruction_const(&mut self, thread: ThreadId, closure: impl Fn(&mut Machine, ThreadId) -> u64) {
        let register = self.next_register(thread);

        let constant = closure(self, thread);

        self.register_write(register, constant);
    }

    pub fn instruction_load(&mut self, thread_id: ThreadId, closure: fn(&Machine, ThreadId, u64) -> u64) {
        let address     = self.next_register(thread_id);
        let destination = self.next_register(thread_id);
        let lock_id     = self.next_lock(thread_id);

        let address = self.register_read(address);
        self.lock(lock_id);

        self.callback_delay(TIME_LOAD, move |machine| {
            let value = closure(machine, thread_id, address);
            machine.register_write(destination, value);
            machine.unlock(lock_id);
        });
    }

    pub fn instruction_store(&mut self, thread_id: ThreadId, closure: fn(&mut Machine, ThreadId, u64, u64)) {
        let source      = self.next_register(thread_id);
        let destination = self.next_register(thread_id);
        let lock_id     = self.next_lock(thread_id);

        let address = self.register_read(destination);
        let value   = self.register_read(source);
        self.lock(lock_id);

        self.callback_delay(TIME_STORE, move |machine| {
            closure(machine, thread_id, address, value);
            machine.unlock(lock_id);
        });
    }

    pub fn instruction_calcul(&mut self, thread_id: ThreadId, delay: usize, closure: fn(&Machine, ThreadId, u64, u64) -> u64) {
        let a       = self.next_register(thread_id);
        let b       = self.next_register(thread_id);
        let result  = self.next_register(thread_id);
        let lock_id = self.next_lock(thread_id);

        let a = self.register_read(a);
        let b = self.register_read(b);
        self.lock(lock_id);

        self.callback_delay(delay, move |machine| {
            let value = closure(machine, thread_id, a, b);
            machine.register_write(result, value);
            machine.unlock(lock_id);
        });
    }

    pub fn instruction_profile_reset(&mut self) {
        for callback in self.callbacks.iter_mut() {
            callback.0 -= self.counter;
        }

        self.counter = 0;
        for thread in self.threads.iter_mut() {
            thread.profile_reset();
        }
    }

    pub fn instruction_profile_dump(&mut self) {
        let mut table = Table::new();
        table.add_row(Row::new(vec!["Thread", "Active time", "Inactive time", "Wait time"]));
        for (i, thread) in self.threads.iter().enumerate() {
            let profile = thread.profile();
            let mut row = Row::new(vec![thread.id().to_string(), profile.active().to_string(), profile.inactive().to_string(), profile.waiting().to_string()]);
            if i != 0 {
                row.has_separator = false;
            }

            table.add_row(row);
        }

        table.add_row(Row::new(vec![TableCell::new_with_col_span(format!("Cycles: {}", self.counter), 4)]));
        println!("{}", table.render());
    }

    pub fn instruction_end(&self) -> ! {
        exit(0);
    }
}
