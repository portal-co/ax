// THIS FILE IS AUTOGENERATED, DO NOT EDIT
// You can regenerate it using `make generate` after creating a new instruction file with `python3 generate.py <mneumonic>`

use super::{axecutor::Axecutor, errors::AxError};
use iced_x86::{
    Instruction,
    Mnemonic::{self, *},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

impl Axecutor {
    pub fn switch_instruction_mnemonic(&mut self, i: Instruction) -> Result<(), AxError> {
        match i.mnemonic() {
            Add => self.mnemonic_add(i),
            And => self.mnemonic_and(i),
            Call => self.mnemonic_call(i),
            Cdq => self.mnemonic_cdq(i),
            Cmp => self.mnemonic_cmp(i),
            Cqo => self.mnemonic_cqo(i),
            Cwd => self.mnemonic_cwd(i),
            Dec => self.mnemonic_dec(i),
            Div => self.mnemonic_div(i),
            Endbr64 => self.mnemonic_endbr64(i),
            Idiv => self.mnemonic_idiv(i),
            Imul => self.mnemonic_imul(i),
            Inc => self.mnemonic_inc(i),
            Int => self.mnemonic_int(i),
            Int1 => self.mnemonic_int1(i),
            Ja => self.mnemonic_ja(i),
            Jae => self.mnemonic_jae(i),
            Jb => self.mnemonic_jb(i),
            Jbe => self.mnemonic_jbe(i),
            Je => self.mnemonic_je(i),
            Jecxz => self.mnemonic_jecxz(i),
            Jg => self.mnemonic_jg(i),
            Jge => self.mnemonic_jge(i),
            Jl => self.mnemonic_jl(i),
            Jle => self.mnemonic_jle(i),
            Jmp => self.mnemonic_jmp(i),
            Jne => self.mnemonic_jne(i),
            Jno => self.mnemonic_jno(i),
            Jnp => self.mnemonic_jnp(i),
            Jns => self.mnemonic_jns(i),
            Jo => self.mnemonic_jo(i),
            Jp => self.mnemonic_jp(i),
            Jrcxz => self.mnemonic_jrcxz(i),
            Js => self.mnemonic_js(i),
            Lea => self.mnemonic_lea(i),
            Mov => self.mnemonic_mov(i),
            Movsxd => self.mnemonic_movsxd(i),
            Mul => self.mnemonic_mul(i),
            Nop => self.mnemonic_nop(i),
            Not => self.mnemonic_not(i),
            Pop => self.mnemonic_pop(i),
            Push => self.mnemonic_push(i),
            Ret => self.mnemonic_ret(i),
            Shl => self.mnemonic_shl(i),
            Sub => self.mnemonic_sub(i),
            Syscall => self.mnemonic_syscall(i),
            Test => self.mnemonic_test(i),
            Xor => self.mnemonic_xor(i),
            Int3 => self.mnemonic_int3(i),
            _ => Err(AxError::from(format!(
                "unimplemented mnemonic {:?}",
                i.mnemonic()
            ))),
        }
    }
}

#[wasm_bindgen(js_name = Mnemonic)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupportedMnemonic {
    Add = 7,
    And = 21,
    Call = 59,
    Cdq = 61,
    Cmp = 93,
    Cqo = 107,
    Cwd = 131,
    Dec = 137,
    Div = 138,
    Endbr64 = 152,
    Idiv = 276,
    Imul = 277,
    Inc = 279,
    Int = 287,
    Int1 = 288,
    Ja = 297,
    Jae = 298,
    Jb = 299,
    Jbe = 300,
    Je = 302,
    Jecxz = 303,
    Jg = 304,
    Jge = 305,
    Jl = 306,
    Jle = 307,
    Jmp = 308,
    Jne = 310,
    Jno = 311,
    Jnp = 312,
    Jns = 313,
    Jo = 314,
    Jp = 315,
    Jrcxz = 316,
    Js = 317,
    Lea = 374,
    Mov = 414,
    Movsxd = 451,
    Mul = 456,
    Nop = 465,
    Not = 466,
    Pop = 590,
    Push = 640,
    Ret = 662,
    Shl = 712,
    Sub = 740,
    Syscall = 746,
    Test = 751,
    Xor = 1518,
    Int3 = 1620,
}

impl SupportedMnemonic {
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}

impl From<Mnemonic> for SupportedMnemonic {
    fn from(mnemonic: Mnemonic) -> Self {
        match mnemonic {
            Add => SupportedMnemonic::Add,
            And => SupportedMnemonic::And,
            Call => SupportedMnemonic::Call,
            Cdq => SupportedMnemonic::Cdq,
            Cmp => SupportedMnemonic::Cmp,
            Cqo => SupportedMnemonic::Cqo,
            Cwd => SupportedMnemonic::Cwd,
            Dec => SupportedMnemonic::Dec,
            Div => SupportedMnemonic::Div,
            Endbr64 => SupportedMnemonic::Endbr64,
            Idiv => SupportedMnemonic::Idiv,
            Imul => SupportedMnemonic::Imul,
            Inc => SupportedMnemonic::Inc,
            Int => SupportedMnemonic::Int,
            Int1 => SupportedMnemonic::Int1,
            Ja => SupportedMnemonic::Ja,
            Jae => SupportedMnemonic::Jae,
            Jb => SupportedMnemonic::Jb,
            Jbe => SupportedMnemonic::Jbe,
            Je => SupportedMnemonic::Je,
            Jecxz => SupportedMnemonic::Jecxz,
            Jg => SupportedMnemonic::Jg,
            Jge => SupportedMnemonic::Jge,
            Jl => SupportedMnemonic::Jl,
            Jle => SupportedMnemonic::Jle,
            Jmp => SupportedMnemonic::Jmp,
            Jne => SupportedMnemonic::Jne,
            Jno => SupportedMnemonic::Jno,
            Jnp => SupportedMnemonic::Jnp,
            Jns => SupportedMnemonic::Jns,
            Jo => SupportedMnemonic::Jo,
            Jp => SupportedMnemonic::Jp,
            Jrcxz => SupportedMnemonic::Jrcxz,
            Js => SupportedMnemonic::Js,
            Lea => SupportedMnemonic::Lea,
            Mov => SupportedMnemonic::Mov,
            Movsxd => SupportedMnemonic::Movsxd,
            Mul => SupportedMnemonic::Mul,
            Nop => SupportedMnemonic::Nop,
            Not => SupportedMnemonic::Not,
            Pop => SupportedMnemonic::Pop,
            Push => SupportedMnemonic::Push,
            Ret => SupportedMnemonic::Ret,
            Shl => SupportedMnemonic::Shl,
            Sub => SupportedMnemonic::Sub,
            Syscall => SupportedMnemonic::Syscall,
            Test => SupportedMnemonic::Test,
            Xor => SupportedMnemonic::Xor,
            Int3 => SupportedMnemonic::Int3,
            _ => panic!("unimplemented mnemonic {:?}", mnemonic),
        }
    }
}
