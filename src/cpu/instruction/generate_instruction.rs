macro_rules! generate_instruction {
    (
    $(#[$shared_docs:meta])*
    ($(#[$register_instruction_docs:meta])+
    $register_instruction_name:ident, $(#[$hl_instruction_docs:meta])+
    $hl_instruction_name:ident $(, $(#[$immediate_instruction_docs:meta])+
    $immediate_instruction_name:ident )? ),
    $( $cb_prefix:ident, )?
    $opcode:literal,
    $( $register_part_offset:literal, )?
    $cpu:ident,
    $memory:ident,
    $operand:ident,
    $($accumulator:ident ,)?
    $( (  $bit_ident:ident ) ,)?
    $( [  $dont_write:ident ] ,)?
    $($target_operand:literal ,)?
    $content:block,
    $(fn $test_name:ident() $test_content:block),*) => {
        #[allow(unused)]
        use crate::{
            cpu::{Cpu, Flag, Register, DoubleRegister, Bit},
            memory::MemoryDevice,
        };
        $(consume_first!{$accumulator use super::phases::TwoPhases;})?
        use super::phases::ThreePhases;

        use super::Instruction;

        $(#[$register_instruction_docs])*
        $(#[$shared_docs])*
        #[derive(Debug)]
pub struct $register_instruction_name {
            /// The operand register
            pub operand: Register,
            $(
                /// The selected bit.
                pub bit: consume_first!($bit_ident Bit)
            )?
        }

        impl Instruction for $register_instruction_name {
            fn execute<T: MemoryDevice>(
                &self,
                $cpu: &mut crate::cpu::CpuState,
                $memory: &mut T,
            ) -> super::InstructionEnum {
                let $operand = $cpu.read_register(self.operand);
                $(
                let $accumulator = $cpu.read_register(Register::A);
                )?
                $(
                    let $bit_ident = self.bit;
                )?

                #[deny(unused)]
                let result: u8 = $content;

                $(consume_first!($accumulator
                    $cpu.write_register(Register::A, result);
                );)?
                $(consume_first!($target_operand
                    $cpu.write_register(self.operand, result);
                );)?

                return $cpu.load_instruction($memory);
            }
            fn encode(&self) -> Vec<u8> {
                if(matches!(self.operand, Register::F)){
                    panic!(stringify!(Arithmetic instructions do not have an opcode for operating on Register::F. That opcode is used for $hl_instruction_name))
                }
                let base_code = $opcode & !(0b00000111u8 << [$($register_part_offset ,)? 0][0]);
                let operand_code = (self.operand.id() & 0b00000111u8) << [$($register_part_offset ,)? 0][0];
                let opcode = base_code | operand_code;

                $(
                    let selected_bit_code = consume_first!($bit_ident Into::<u8>::into(self.bit) << 3 );
                    let opcode = opcode | selected_bit_code;
                )?

                Vec::from([$(consume_first!($cb_prefix 0xcb) , )? opcode])
            }
        }

        $(#[$hl_instruction_docs])*
        $(#[$shared_docs])*
        #[derive(Debug)]
pub struct $hl_instruction_name {
            /// The current phase of the instruction.
            pub phase: ThreePhases,
            $(
                /// The selected bit.
                pub bit: consume_first!($bit_ident Bit),
            )?
            /// The operand will be loaded from HL. Stored here for correct memory timing.
            pub operand: u8
        }


        impl Instruction for $hl_instruction_name {
            fn execute<T: MemoryDevice>(
                &self,
                $cpu: &mut crate::cpu::CpuState,
                $memory: &mut T,
            ) -> super::InstructionEnum {
                match self.phase {
                    ThreePhases::Second => {

                        let $operand = self.operand;
                        $(
                            let $accumulator = $cpu.read_register(Register::A);
                        )?
                        $(
                            let $bit_ident = self.bit;
                        )?

                        #[deny(unused)]
                        let result: u8 = $content;

                        #[allow(unused)]
                        let next_phase_instruction = Self {
                            phase: ThreePhases::Third,
                            $(
                                bit: consume_first!($bit_ident self.bit),
                            )?
                            operand: self.operand
                        };

                        $(consume_first!($accumulator
                            $cpu.write_register(Register::A, result);
                            return $cpu.load_instruction($memory);
                        );)?
                        $(consume_first!($dont_write
                            if true {
                                let _unused = result;
                                return $cpu.load_instruction($memory);
                            }
                        );)?


                        $(consume_first!($target_operand
                            let address = $cpu.read_double_register(DoubleRegister::HL);
                            $memory.write(address, result);
                            return next_phase_instruction.into()
                        );)?



                    },
                    ThreePhases::Third  => {

                        return $cpu.load_instruction($memory);
                    },
                    #[allow(unreachable_patterns)]
                    ThreePhases::First  => {
                        let address = $cpu.read_double_register(DoubleRegister::HL);
                        let operand = $memory.read(address);
                        Self {
                            phase: ThreePhases::Second,
                            $(
                                bit: consume_first!($bit_ident self.bit),
                            )?
                            operand: operand,
                        }.into()
                    }
                }


            }
            fn encode(&self) -> Vec<u8> {

                let base_code = $opcode & !(0b00000111u8 << [$($register_part_offset ,)? 0][0]);
                let operand_code = 0b00000110 << [$($register_part_offset ,)? 0][0];
                let opcode = base_code | operand_code;

                $(
                    let selected_bit_code = consume_first!($bit_ident Into::<u8>::into(self.bit) << 3 );
                    let opcode = opcode | selected_bit_code;
                )?

                Vec::from([$(consume_first!($cb_prefix 0xcb) , )? opcode])
            }
        }


        $(
            $(#[$immediate_instruction_docs])*
        )?
            $(#[$shared_docs])*
        $(
            #[derive(Debug)]
pub struct $immediate_instruction_name {
                /// The immediate value. Will only valid in the second phase.
                pub value: u8,
                /// The current phase of the instruction.
                pub phase: TwoPhases,
            }

            impl Instruction for $immediate_instruction_name {
                fn execute<T: MemoryDevice>(
                    &self,
                    $cpu: &mut crate::cpu::CpuState,
                    $memory: &mut T,
                ) -> super::InstructionEnum {
                    match self.phase {
                    TwoPhases::First => {
                            let address = $cpu.advance_program_counter();
                            let $operand = $memory.read(address);
                            let $accumulator = $cpu.read_register(Register::A);

                            let result: u8 = $content;

                            $cpu.write_register(Register::A, result);

                            Self {
                                value: $operand,
                                phase: TwoPhases::Second
                            }.into()
                        },
                        TwoPhases::Second => {
                            return $cpu.load_instruction($memory);
                        }
                    }


                }
                fn encode(&self) -> Vec<u8> {
                    let opcode_immediate = $opcode + 0b01000110;
                    match self.phase {
                        TwoPhases::First => Vec::from([opcode_immediate]),
                        TwoPhases::Second => Vec::from([opcode_immediate, self.value]),
                    }
                }
            }
        )?
        struct __DocCommentBlackHole {}

        #[cfg(test)]
        mod tests {

            use super::$register_instruction_name;
            use crate::cpu::instruction::Instruction;
            use crate::cpu::{Cpu, CpuState, Flag, Register};
            use crate::memory::MemoryController;
            $(consume_first!{$bit_ident use crate::cpu::Bit;})?

            $(
                #[test]
                fn $test_name() $test_content
            )*

            // #[test]
            // fn generate_instruction_macro_is_called_correctly() {}
        }
    };
}

macro_rules! prepare_generate_instruction {
    (
    $dollar:tt,
    $register_instruction_name:ident
    $( , $bit:ident )? ) => {
        /// Hacky macro that can be used to test the instruction
        ///
        /// The macro takes two arguments in ().
        /// The first is the initial state and the second is the expected resulting state.
        /// You can specify the accumulator as `A` and the operand as `B` in each argument.
        /// You can initialize the flags to true by setting one or more `FLAG:` values in the initial state.
        /// If you dont specify `A:` or `B:` in the initial state it is initialized to `0`.
        ///
        /// If you dont specify `A:` or `B:` in the expected output state nothing happens. If you specify one of them, it will be asserted that they have that value.
        /// You can specify one or more `FLAG:` or `FLAG_UNSET:` values in the expected result state. The flags will be asserted.
        ///
        /// The order of the values is important, it always need to be (A: xxx, B: xxx, FLAG: xxx,) for the initial state and (A: xxx, B: xxx, FLAG: xxx, FLAG_UNSET: xxx,) for the expected result state.Every value is optional. `FLAG:` and `FLAG_UNSET` can be specified multiple times.
        ///
        /// Example:
        ///
        /// ```
        /// // assert_result!((A: 5, B: 3,), (A: 8, B: 3, FLAG_UNSET: Flag::Carry,));
        /// ```
        #[cfg(test)]
        macro_rules! assert_result {
            (($dollar(A: $accumulator:expr,)? $dollar(B: $operandd:expr,)? $dollar( FLAG: $initial_flags:expr ,)* $dollar( BIT: $select_bit:expr ,)? ), ($dollar(A: $accumulator_result:expr,)? $dollar(B: $operand_result:expr,)? $dollar( FLAG: $flag_result:expr ,)* $dollar( FLAG_UNSET: $flag_unset_result:expr ,)* )) => {
                {
                let mut cpu = CpuState::new();
                let mut memory = MemoryController::new();

                let accumulator_value = [$dollar($accumulator ,)? 0][0];
                let operand_value = [$dollar($operandd ,)? 0][0];
                #[allow(unused)]
                let flags_string = stringify!($dollar( $initial_flags )&*);

                cpu.write_register(Register::A, accumulator_value);
                cpu.write_register(Register::B, operand_value);
                cpu.write_flag(Flag::Zero, false);
                cpu.write_flag(Flag::Subtract, false);
                cpu.write_flag(Flag::HalfCarry, false);
                cpu.write_flag(Flag::Carry, false);
                $dollar( cpu.write_flag($initial_flags, true); )*


                let instruction = $register_instruction_name {
                    operand: Register::B,
                    $(
                        /// The affected bit.
                        bit: consume_first!($bit [$dollar($select_bit ,)? crate::cpu::Bit::Zero][0])
                    )?
                };
                instruction.execute(&mut cpu, &mut memory);

                #[allow(unused)]
                let result_value = cpu.read_register(Register::A);
                #[allow(unused)]
                let operand_result_value = cpu.read_register(Register::B);
                #[allow(unused)]
                let module_name = module_path!().rsplit("::").skip(1).next().expect("name");

                $dollar( assert_eq!(cpu.read_register(Register::A), $accumulator_result, "\n\n\n#####>    Expected accumulator to be {} but got {} instead!    <#####\n\nWhere      : {}:{}\nFlags      : {}\nAccumulator: {:#010b} {:#004x} {}\nOperand    : {:#010b} {:#004x} {}\nResult     : {:#010b} {:#004x} {}\n\n", $accumulator_result, result_value, file!(), line!(), flags_string, accumulator_value, accumulator_value, accumulator_value, operand_value, operand_value, operand_value, result_value, result_value, result_value); )*
                $dollar( assert_eq!(cpu.read_register(Register::B), $operand_result, "\n\n\n#####>    Expected operand to be {} but got {} instead!    <#####\n\nWhere      : {}:{}\nFlags      : {}\nAccumulator: {:#010b} {:#004x} {}\nOperand    : {:#010b} {:#004x} {}\nOperand    : {:#010b} {:#004x} {}\n\n", $operand_result, operand_result_value, file!(), line!(), flags_string, accumulator_value, accumulator_value, accumulator_value, operand_value, operand_value, operand_value, operand_result_value, operand_result_value, operand_result_value); )*
                $dollar( assert_eq!(cpu.read_flag($flag_result), true, "\n\n\n#####>    Expected {} to be set!    <#####\n\nWhere      : {}:{}\nFlags      : {}\nAccumulator: {:#010b} {:#004x} {}\nOperand    : {:#010b} {:#004x} {}\nResult     : {:#010b} {:#004x} {}\n\n", stringify!($flag_result), file!(), line!(), flags_string, accumulator_value, accumulator_value, accumulator_value, operand_value, operand_value, operand_value, result_value, result_value, result_value); )*
                $dollar( assert_eq!(cpu.read_flag($flag_unset_result), false, "\n\n\n#####>    Expected {} to be unset!    <#####\n\nWhere      : {}:{}\nFlags      : {}\nAccumulator: {:#010b} {:#004x} {}\nOperand    : {:#010b} {:#004x} {}\nResult     : {:#010b} {:#004x} {}\n\n", stringify!($flag_unset_result), file!(), line!(), flags_string, accumulator_value, accumulator_value, accumulator_value, operand_value, operand_value, operand_value, result_value, result_value, result_value); )*
}
            };

        }

    macro_rules! consume_first {
        ($first:tt $dollar($token:tt)*) => {
            $dollar($token)*
        };
    }

    };
}

pub(crate) use generate_instruction;
// pub(crate) use generate_instruction_tests;
pub(crate) use prepare_generate_instruction;
