use core::panic;
use std::cmp::max;
use std::fmt::format;
use std::ops::Add;
use std::thread::panicking;

use crate::cpu::instructions::{Instruction, ArithmeticTarget, JumpTest, LoadByteSource, LoadByteTarget, LoadType};
use crate::cpu::registers::{Registers, Flags};
use crate::cpu::memory::{MemoryBus};


struct CPU {
    registers: Registers,
    pc: u16, //program counter
    sp: u16, //stack pointer
    bus: MemoryBus,
}



impl CPU {
    // Advance in the program reading.
    fn step(&mut self){
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefix_instr = instruction_byte == 0xCB;
        if prefix_instr {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefix_instr){
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefix_instr {"cb"} else {""}, instruction_byte);
            panic!("Unknown instruction found for: 0x{}", description);
        };

        //TODO: Get the next pc from executions
        self.pc = next_pc;
    }

    // Read the next byte.
    fn read_next_byte(&self) -> u8 { 0 }

    // Read the next compound pair of bytes.
    fn read_next_word(&self) -> u16 { 0 }

    // Parse the expected execution found and perform what is expected of it.
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let val = self.registers.c;
                        let new_val = self.add(val);
                        self.registers.a = new_val;

                        self.pc.wrapping_add(1)
                    },
                    _ => {/* TODO: Targets */ self.pc}
                }
            },
            Instruction::CALL(test) => {
                let condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    _ => { panic!("TODO: Implement all JT") }
                };
                self.call(condition)
            },
            Instruction::RET(test) => {
                let condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    _ => { panic!("TODO: Implement all JT") }
                };
                self.ret(condition)
            },
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let src_val = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                            _ => {panic!("TODO: Implement all LBS")}
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = src_val,
                            LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), src_val),
                            _ => {panic!("TODO: Implement all LBT")}
                        }
                        match source {
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            _ => self.pc.wrapping_add(1),
                        }
                    }
                }
            },
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
            }
            _ => {/* TODO: instructions*/ self.pc}
        }
    }

    fn call(&mut self, is_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if is_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn ret(&mut self, is_jump: bool) -> u16 {
        if is_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    // Push into the stack.
    // Memory in GB grows downward, this means we advance from the end 
    // towards the beggining of the memory, thus we use the sub method.
    // Decrease sp by 1, write the most significant part into memory at new sp, 
    // then repeat with the least significant part.
    fn push(&mut self, val: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((val & 0xFF00) >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((val & 0x00FF) >> 8) as u8);
    }

    // Pop out of the stack
    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
    
        //join most and least significant bytes into an u16
        (msb << 8) | lsb 
    }

    // Jump to a memory addr.
    // The address we jump to is located in the two bytes following the instruction identifier as:
    // [instr. id.] [least sig. byte] [most sig. byte]
    fn jump(&self, is_jump: bool) -> u16 {
        // Gameboy is little endian so pc +2 is the most significant bit and +1 the least
        if is_jump {
            let lsb = self.bus.read_byte(self.pc + 1) as u16;
            let msb = self.bus.read_byte(self.pc + 2) as u16;
            (msb << 8) | lsb
        } else {
            self.pc.wrapping_add(3)
        }
    }

    // Perform addition to a register.
    fn add(&mut self, val: u8) -> u8 {
        //Is this A register alright?
        let (new_val, was_overflow) = self.registers.a.overflowing_add(val);

        // Set flags. Why am i using a Flags struct within a Registers one?
        self.registers.f.zero = new_val == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = was_overflow;
        // The result has overflowed into another register. Is this alright?
        self.registers.f.half_carry = (self.registers.a & 0xF) + (val & 0xF) > 0xF;

        new_val
    }
}