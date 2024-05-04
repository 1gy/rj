// The Java Virtual Machine Instruction Set
// https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-6.html

use super::error::InstructionParseError;
use crate::parser::{be_i16, be_i32, be_i8, be_u16, be_u8};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Aaload,
    Aastore,
    AconstNull,
    Aload(u8),
    Aload0,
    Aload1,
    Aload2,
    Aload3,
    Anewarray(u16),
    Areturn,
    Arraylength,
    Astore(u8),
    Astore0,
    Astore1,
    Astore2,
    Astore3,
    Athrow,
    Baload,
    Bastore,
    Bipush(i8),
    Caload,
    Castore,
    Checkcast(u16),
    D2f,
    D2i,
    D2l,
    Dadd,
    Daload,
    Dastore,
    Dcmpg,
    Dcmpl,
    Dconst0,
    Dconst1,
    Ddiv,
    Dload(u8),
    Dload0,
    Dload1,
    Dload2,
    Dload3,
    Dmul,
    Dneg,
    Drem,
    Dreturn,
    Dstore(u8),
    Dstore0,
    Dstore1,
    Dstore2,
    Dstore3,
    Dsub,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    F2d,
    F2i,
    F2l,
    Fadd,
    Faload,
    Fastore,
    Fcmpg,
    Fcmpl,
    Fconst0,
    Fconst1,
    Fconst2,
    Fdiv,
    Fload(u8),
    Fload0,
    Fload1,
    Fload2,
    Fload3,
    Fmul,
    Fneg,
    Frem,
    Freturn,
    Fstore(u8),
    Fstore0,
    Fstore1,
    Fstore2,
    Fstore3,
    Fsub,
    Getfield(u16),
    Getstatic(u16),
    Goto(i16),
    GotoW(i32),
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iaload,
    Iand,
    Iastore,
    IconstM1,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,
    Idiv,
    IfAcmpeq(i16),
    IfAcmpne(i16),
    IfIcmpeq(i16),
    IfIcmpne(i16),
    IfIcmplt(i16),
    IfIcmpge(i16),
    IfIcmpgt(i16),
    IfIcmple(i16),
    Ifeq(i16),
    Ifne(i16),
    Iflt(i16),
    Ifge(i16),
    Ifgt(i16),
    Ifle(i16),
    Ifnonnull(i16),
    Ifnull(i16),
    Iinc(u8, i8),
    Iload(u8),
    Iload0,
    Iload1,
    Iload2,
    Iload3,
    Imul,
    Ineg,
    Instanceof(u16),
    Invokedynamic(u16, u8, u8),
    Invokeinterface(u16, u8, u8),
    Invokespecial(u16),
    Invokestatic(u16),
    Invokevirtual(u16),
    Ior,
    Irem,
    Ireturn,
    Ishl,
    Ishr,
    Istore(u8),
    Istore0,
    Istore1,
    Istore2,
    Istore3,
    Isub,
    Iushr,
    Ixor,
    Jsr(i16),
    JsrW(i32),
    L2d,
    L2f,
    L2i,
    Ladd,
    Laload,
    Land,
    Lastore,
    Lcmp,
    Lconst0,
    Lconst1,
    Ldc(u8),
    LdcW(u16),
    Ldc2W(u16),
    Ldiv,
    Lload(u8),
    Lload0,
    Lload1,
    Lload2,
    Lload3,
    Lmul,
    Lneg,
    Lookupswitch(i32, Vec<(i32, i32)>),
    Lor,
    Lrem,
    Lreturn,
    Lshl,
    Lshr,
    Lstore(u8),
    Lstore0,
    Lstore1,
    Lstore2,
    Lstore3,
    Lsub,
    Lushr,
    Lxor,
    Monitorenter,
    Monitorexit,
    Multianewarray(u16, u8),
    New(u16),
    Newarray(u8),
    Nop,
    Pop,
    Pop2,
    Putfield(u16),
    Putstatic(u16),
    Ret(u8),
    Return,
    Saload,
    Sastore,
    Sipush(i16),
    Swap,
    Tableswitch(i32, i32, i32, Vec<i32>),
    WideIload(u16),
    WideFload(u16),
    WideAload(u16),
    WideLload(u16),
    WideDload(u16),
    WideIstore(u16),
    WideFstore(u16),
    WideAstore(u16),
    WideLstore(u16),
    WideDstore(u16),
    WideRet(u16),
    WideIinc(u16, i16),
}

pub fn parse_instruction(input: &[u8]) -> Result<(&[u8], Instruction), InstructionParseError> {
    let (input, opcode) = be_u8(input)?;
    match opcode {
        0x32 => Ok((input, Instruction::Aaload)),
        0x53 => Ok((input, Instruction::Aastore)),
        0x01 => Ok((input, Instruction::AconstNull)),
        0x19 => {
            let (input, index) = be_u8(input)?;
            Ok((input, Instruction::Aload(index)))
        }
        0x2a => Ok((input, Instruction::Aload0)),
        0x2b => Ok((input, Instruction::Aload1)),
        0x2c => Ok((input, Instruction::Aload2)),
        0x2d => Ok((input, Instruction::Aload3)),
        0xbd => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::Anewarray(index)))
        }
        0xb0 => Ok((input, Instruction::Areturn)),
        0xbe => Ok((input, Instruction::Arraylength)),
        0x3a => {
            let (input, index) = be_u8(input)?;
            Ok((input, Instruction::Astore(index)))
        }
        0x4b => Ok((input, Instruction::Astore0)),
        0x4c => Ok((input, Instruction::Astore1)),
        0x4d => Ok((input, Instruction::Astore2)),
        0x4e => Ok((input, Instruction::Astore3)),
        0xbf => Ok((input, Instruction::Athrow)),
        0x33 => Ok((input, Instruction::Baload)),
        0x54 => Ok((input, Instruction::Bastore)),
        0x10 => {
            let (input, byte) = be_i8(input)?;
            Ok((input, Instruction::Bipush(byte)))
        }
        0x34 => Ok((input, Instruction::Caload)),
        0x55 => Ok((input, Instruction::Castore)),
        0xc0 => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::Checkcast(index)))
        }
        0x90 => Ok((input, Instruction::D2f)),
        0x8e => Ok((input, Instruction::D2i)),
        0x8f => Ok((input, Instruction::D2l)),
        0x63 => Ok((input, Instruction::Dadd)),
        0x31 => Ok((input, Instruction::Daload)),
        0x52 => Ok((input, Instruction::Dastore)),
        0x98 => Ok((input, Instruction::Dcmpg)),
        0x97 => Ok((input, Instruction::Dcmpl)),
        0x0e => Ok((input, Instruction::Dconst0)),
        0x0f => Ok((input, Instruction::Dconst1)),
        0x6f => Ok((input, Instruction::Ddiv)),
        0x18 => {
            let (input, index) = be_u8(input)?;
            Ok((input, Instruction::Dload(index)))
        }
        0x26 => Ok((input, Instruction::Dload0)),
        0x27 => Ok((input, Instruction::Dload1)),
        0x28 => Ok((input, Instruction::Dload2)),
        0x29 => Ok((input, Instruction::Dload3)),
        0x6b => Ok((input, Instruction::Dmul)),
        0x77 => Ok((input, Instruction::Dneg)),
        0x73 => Ok((input, Instruction::Drem)),
        0xaf => Ok((input, Instruction::Dreturn)),
        0x39 => {
            let (input, index) = be_u8(input)?;
            Ok((input, Instruction::Dstore(index)))
        }
        0x47 => Ok((input, Instruction::Dstore0)),
        0x48 => Ok((input, Instruction::Dstore1)),
        0x49 => Ok((input, Instruction::Dstore2)),
        0x4a => Ok((input, Instruction::Dstore3)),
        0x67 => Ok((input, Instruction::Dsub)),
        0x59 => Ok((input, Instruction::Dup)),
        0x5a => Ok((input, Instruction::DupX1)),
        0x5b => Ok((input, Instruction::DupX2)),
        0x5c => Ok((input, Instruction::Dup2)),
        0x5d => Ok((input, Instruction::Dup2X1)),
        0x5e => Ok((input, Instruction::Dup2X2)),
        0x8d => Ok((input, Instruction::F2d)),
        0x8b => Ok((input, Instruction::F2i)),
        0x8c => Ok((input, Instruction::F2l)),
        0x62 => Ok((input, Instruction::Fadd)),
        0x30 => Ok((input, Instruction::Faload)),
        0x51 => Ok((input, Instruction::Fastore)),
        0x96 => Ok((input, Instruction::Fcmpg)),
        0x95 => Ok((input, Instruction::Fcmpl)),
        0x0b => Ok((input, Instruction::Fconst0)),
        0x0c => Ok((input, Instruction::Fconst1)),
        0x0d => Ok((input, Instruction::Fconst2)),
        0x6e => Ok((input, Instruction::Fdiv)),
        0x17 => {
            let (input, index) = be_u8(input)?;
            Ok((input, Instruction::Fload(index)))
        }
        0x22 => Ok((input, Instruction::Fload0)),
        0x23 => Ok((input, Instruction::Fload1)),
        0x24 => Ok((input, Instruction::Fload2)),
        0x25 => Ok((input, Instruction::Fload3)),
        0x6a => Ok((input, Instruction::Fmul)),
        0x76 => Ok((input, Instruction::Fneg)),
        0x72 => Ok((input, Instruction::Frem)),
        0xae => Ok((input, Instruction::Freturn)),
        0x38 => {
            let (input, index) = be_u8(input)?;
            Ok((input, Instruction::Fstore(index)))
        }
        0x43 => Ok((input, Instruction::Fstore0)),
        0x44 => Ok((input, Instruction::Fstore1)),
        0x45 => Ok((input, Instruction::Fstore2)),
        0x46 => Ok((input, Instruction::Fstore3)),
        0x66 => Ok((input, Instruction::Fsub)),
        0xb4 => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::Getfield(index)))
        }
        0xb2 => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::Getstatic(index)))
        }
        0xa7 => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::Goto(offset)))
        }
        0xc8 => {
            let (input, offset) = be_i32(input)?;
            Ok((input, Instruction::GotoW(offset)))
        }
        0x91 => Ok((input, Instruction::I2b)),
        0x92 => Ok((input, Instruction::I2c)),
        0x87 => Ok((input, Instruction::I2d)),
        0x86 => Ok((input, Instruction::I2f)),
        0x85 => Ok((input, Instruction::I2l)),
        0x93 => Ok((input, Instruction::I2s)),
        0x60 => Ok((input, Instruction::Iadd)),
        0x2e => Ok((input, Instruction::Iaload)),
        0x7e => Ok((input, Instruction::Iand)),
        0x4f => Ok((input, Instruction::Iastore)),
        0x02 => Ok((input, Instruction::IconstM1)),
        0x03 => Ok((input, Instruction::Iconst0)),
        0x04 => Ok((input, Instruction::Iconst1)),
        0x05 => Ok((input, Instruction::Iconst2)),
        0x06 => Ok((input, Instruction::Iconst3)),
        0x07 => Ok((input, Instruction::Iconst4)),
        0x08 => Ok((input, Instruction::Iconst5)),
        0x6c => Ok((input, Instruction::Idiv)),
        0xa5 => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::IfAcmpeq(offset)))
        }
        0xa6 => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::IfAcmpne(offset)))
        }
        0x9f => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::IfIcmpeq(offset)))
        }
        0xa0 => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::IfIcmpne(offset)))
        }
        0xa1 => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::IfIcmplt(offset)))
        }
        0xa2 => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::IfIcmpge(offset)))
        }
        0xa3 => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::IfIcmpgt(offset)))
        }
        0xa4 => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::IfIcmple(offset)))
        }
        0x99 => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::Ifeq(offset)))
        }
        0x9a => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::Ifne(offset)))
        }
        0x9b => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::Iflt(offset)))
        }
        0x9c => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::Ifge(offset)))
        }
        0x9d => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::Ifgt(offset)))
        }
        0x9e => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::Ifle(offset)))
        }
        0xc7 => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::Ifnonnull(offset)))
        }
        0xc6 => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::Ifnull(offset)))
        }
        0x84 => {
            let (input, index) = be_u8(input)?;
            let (input, byte) = be_i8(input)?;
            Ok((input, Instruction::Iinc(index, byte)))
        }
        0x15 => {
            let (input, index) = be_u8(input)?;
            Ok((input, Instruction::Iload(index)))
        }
        0x1a => Ok((input, Instruction::Iload0)),
        0x1b => Ok((input, Instruction::Iload1)),
        0x1c => Ok((input, Instruction::Iload2)),
        0x1d => Ok((input, Instruction::Iload3)),
        0x68 => Ok((input, Instruction::Imul)),
        0x74 => Ok((input, Instruction::Ineg)),
        0xc1 => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::Instanceof(index)))
        }
        0xba => {
            let (input, index) = be_u16(input)?;
            let (input, _zero1) = be_u8(input)?;
            let (input, _zero2) = be_u8(input)?;
            // TODO: Check that zero1 and zero2 are zero
            Ok((input, Instruction::Invokedynamic(index, 0, 0)))
        }
        0xb9 => {
            let (input, index) = be_u16(input)?;
            let (input, count) = be_u8(input)?;
            let (input, _zero) = be_u8(input)?;
            // TODO: Check that zero is zero
            Ok((input, Instruction::Invokeinterface(index, count, 0)))
        }
        0xb7 => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::Invokespecial(index)))
        }
        0xb8 => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::Invokestatic(index)))
        }
        0xb6 => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::Invokevirtual(index)))
        }
        0x80 => Ok((input, Instruction::Ior)),
        0x70 => Ok((input, Instruction::Irem)),
        0xac => Ok((input, Instruction::Ireturn)),
        0x78 => Ok((input, Instruction::Ishl)),
        0x7a => Ok((input, Instruction::Ishr)),
        0x36 => {
            let (input, index) = be_u8(input)?;
            Ok((input, Instruction::Istore(index)))
        }
        0x3b => Ok((input, Instruction::Istore0)),
        0x3c => Ok((input, Instruction::Istore1)),
        0x3d => Ok((input, Instruction::Istore2)),
        0x3e => Ok((input, Instruction::Istore3)),
        0x64 => Ok((input, Instruction::Isub)),
        0x7c => Ok((input, Instruction::Iushr)),
        0x82 => Ok((input, Instruction::Ixor)),
        0xa8 => {
            let (input, offset) = be_i16(input)?;
            Ok((input, Instruction::Jsr(offset)))
        }
        0xc9 => {
            let (input, offset) = be_i32(input)?;
            Ok((input, Instruction::JsrW(offset)))
        }
        0x8a => Ok((input, Instruction::L2d)),
        0x89 => Ok((input, Instruction::L2f)),
        0x88 => Ok((input, Instruction::L2i)),
        0x61 => Ok((input, Instruction::Ladd)),
        0x2f => Ok((input, Instruction::Laload)),
        0x7f => Ok((input, Instruction::Land)),
        0x50 => Ok((input, Instruction::Lastore)),
        0x94 => Ok((input, Instruction::Lcmp)),
        0x09 => Ok((input, Instruction::Lconst0)),
        0x0a => Ok((input, Instruction::Lconst1)),
        0x12 => {
            let (input, index) = be_u8(input)?;
            Ok((input, Instruction::Ldc(index)))
        }
        0x13 => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::LdcW(index)))
        }
        0x14 => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::Ldc2W(index)))
        }
        0x6d => Ok((input, Instruction::Ldiv)),
        0x16 => {
            let (input, index) = be_u8(input)?;
            Ok((input, Instruction::Lload(index)))
        }
        0x1e => Ok((input, Instruction::Lload0)),
        0x1f => Ok((input, Instruction::Lload1)),
        0x20 => Ok((input, Instruction::Lload2)),
        0x21 => Ok((input, Instruction::Lload3)),
        0x69 => Ok((input, Instruction::Lmul)),
        0x75 => Ok((input, Instruction::Lneg)),
        0xab => {
            // 正しく実装するためにはpaddingのためにコードの先頭からのオフセットが必要
            unimplemented!("lookupswitch")
        }
        0x81 => Ok((input, Instruction::Lor)),
        0x71 => Ok((input, Instruction::Lrem)),
        0xad => Ok((input, Instruction::Lreturn)),
        0x79 => Ok((input, Instruction::Lshl)),
        0x7b => Ok((input, Instruction::Lshr)),
        0x37 => {
            let (input, index) = be_u8(input)?;
            Ok((input, Instruction::Lstore(index)))
        }
        0x3f => Ok((input, Instruction::Lstore0)),
        0x40 => Ok((input, Instruction::Lstore1)),
        0x41 => Ok((input, Instruction::Lstore2)),
        0x42 => Ok((input, Instruction::Lstore3)),
        0x65 => Ok((input, Instruction::Lsub)),
        0x7d => Ok((input, Instruction::Lushr)),
        0x83 => Ok((input, Instruction::Lxor)),
        0xc2 => Ok((input, Instruction::Monitorenter)),
        0xc3 => Ok((input, Instruction::Monitorexit)),
        0xc5 => {
            let (input, index) = be_u16(input)?;
            let (input, dimensions) = be_u8(input)?;
            Ok((input, Instruction::Multianewarray(index, dimensions)))
        }
        0xbb => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::New(index)))
        }
        0xbc => {
            let (input, atype) = be_u8(input)?;
            Ok((input, Instruction::Newarray(atype)))
        }
        0x00 => Ok((input, Instruction::Nop)),
        0x57 => Ok((input, Instruction::Pop)),
        0x58 => Ok((input, Instruction::Pop2)),
        0xb5 => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::Putfield(index)))
        }
        0xb3 => {
            let (input, index) = be_u16(input)?;
            Ok((input, Instruction::Putstatic(index)))
        }
        0xa9 => {
            let (input, index) = be_u8(input)?;
            Ok((input, Instruction::Ret(index)))
        }
        0xb1 => Ok((input, Instruction::Return)),
        0x35 => Ok((input, Instruction::Saload)),
        0x56 => Ok((input, Instruction::Sastore)),
        0x11 => {
            let (input, byte) = be_i16(input)?;
            Ok((input, Instruction::Sipush(byte)))
        }
        0x5f => Ok((input, Instruction::Swap)),
        0xaa => {
            // 正しく実装するためにはpaddingのためにコードの先頭からのオフセットが必要
            unimplemented!("tableswitch")
        }
        0xc4 => match parse_instruction(input) {
            Ok((_, Instruction::Iload(_))) => {
                let (input, _) = be_u8(input)?;
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideIload(index)))
            }
            Ok((_, Instruction::Fload(_))) => {
                let (input, _) = be_u8(input)?;
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideFload(index)))
            }
            Ok((_, Instruction::Aload(_))) => {
                let (input, _) = be_u8(input)?;
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideAload(index)))
            }
            Ok((_, Instruction::Lload(_))) => {
                let (input, _) = be_u8(input)?;
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideLload(index)))
            }
            Ok((_, Instruction::Dload(_))) => {
                let (input, _) = be_u8(input)?;
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideDload(index)))
            }
            Ok((_, Instruction::Istore(_))) => {
                let (input, _) = be_u8(input)?;
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideIstore(index)))
            }
            Ok((_, Instruction::Fstore(_))) => {
                let (input, _) = be_u8(input)?;
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideFstore(index)))
            }
            Ok((_, Instruction::Astore(_))) => {
                let (input, _) = be_u8(input)?;
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideAstore(index)))
            }
            Ok((_, Instruction::Lstore(_))) => {
                let (input, _) = be_u8(input)?;
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideLstore(index)))
            }
            Ok((_, Instruction::Dstore(_))) => {
                let (input, _) = be_u8(input)?;
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideDstore(index)))
            }
            Ok((_, Instruction::Ret(_))) => {
                let (input, _) = be_u8(input)?;
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideRet(index)))
            }
            Ok((_, Instruction::Iinc(_, _))) => {
                let (input, _) = be_u8(input)?;
                let (input, index) = be_u16(input)?;
                let (input, byte) = be_i16(input)?;
                Ok((input, Instruction::WideIinc(index, byte)))
            }
            _ => {
                let (_, opcode) = be_u8(input)?;
                Err(InstructionParseError::UnknownInstruction(opcode))
            }
        },
        _ => Err(InstructionParseError::UnknownInstruction(opcode)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let input = &[
            0x32, // aaload
            0x53, // aastore
            0x01, // aconst_null
            0x19, 0x01, // aload 1
            0x2a, // Aload0
            0x2b, // Aload1
            0x2c, // Aload2
            0x2d, // Aload3
            0xbd, 0x01, 0x02, // anewarray 258
            0xb0, // areturn
            0xbe, // arraylength
            0x3a, 0x01, // astore 1
            0x4b, // astore_0
            0x4c, // astore_1
            0x4d, // astore_2
            0x4e, // astore_3
            0xbf, // athrow
            0x33, // baload
            0x54, // bastore
            0x10, 0x01, // bipush 1
            0x34, // caload
            0x55, // castore
            0xc0, 0x01, 0x02, // checkcast 258
            0x90, // d2f
            0x8e, // d2i
            0x8f, // d2l
            0x63, // dadd
            0x31, // daload
            0x52, // dastore
            0x98, // dcmpg
            0x97, // dcmpl
            0x0e, // dconst_0
            0x0f, // dconst_1
            0x6f, // ddiv
            0x18, 0x01, // dload 1
            0x26, // dload_0
            0x27, // dload_1
            0x28, // dload_2
            0x29, // dload_3
            0x6b, // dmul
            0x77, // dneg
            0x73, // drem
            0xaf, // dreturn
            0x39, 0x01, // dstore 1
            0x47, // dstore_0
            0x48, // dstore_1
            0x49, // dstore_2
            0x4a, // dstore_3
            0x67, // dsub
            0x59, // dup
            0x5a, // dup_x1
            0x5b, // dup_x2
            0x5c, // dup2
            0x5d, // dup2_x1
            0x5e, // dup2_x2
            0x8d, // f2d
            0x8b, // f2i
            0x8c, // f2l
            0x62, // fadd
            0x30, // faload
            0x51, // fastore
            0x96, // fcmpg
            0x95, // fcmpl
            0x0b, // fconst_0
            0x0c, // fconst_1
            0x0d, // fconst_2
            0x6e, // fdiv
            0x17, 0x01, // fload 1
            0x22, // fload_0
            0x23, // fload_1
            0x24, // fload_2
            0x25, // fload_3
            0x6a, // fmul
            0x76, // fneg
            0x72, // frem
            0xae, // freturn
            0x38, 0x01, // fstore 1
            0x43, // fstore_0
            0x44, // fstore_1
            0x45, // fstore_2
            0x46, // fstore_3
            0x66, // fsub
            0xb4, 0x01, 0x02, // getfield 258
            0xb2, 0x01, 0x02, // getstatic 258
            0xa7, 0x01, 0x02, // goto 258
            0xc8, 0x01, 0x02, 0x03, 0x04, // goto_w 16909060
            0x91, // i2b
            0x92, // i2c
            0x87, // i2d
            0x86, // i2f
            0x85, // i2l
            0x93, // i2s
            0x60, // iadd
            0x2e, // iaload
            0x7e, // iand
            0x4f, // iastore
            0x02, // iconst_m1
            0x03, // iconst_0
            0x04, // iconst_1
            0x05, // iconst_2
            0x06, // iconst_3
            0x07, // iconst_4
            0x08, // iconst_5
            0x6c, // idiv
            0xa5, 0x01, 0x02, // if_acmpeq 258
            0xa6, 0x01, 0x02, // if_acmpne 258
            0x9f, 0x01, 0x02, // if_icmpeq 258
            0xa0, 0x01, 0x02, // if_icmpne 258
            0xa1, 0x01, 0x02, // if_icmplt 258
            0xa2, 0x01, 0x02, // if_icmpge 258
            0xa3, 0x01, 0x02, // if_icmpgt 258
            0xa4, 0x01, 0x02, // if_icmple 258
            0x99, 0x01, 0x02, // ifeq 258
            0x9a, 0x01, 0x02, // ifne 258
            0x9b, 0x01, 0x02, // iflt 258
            0x9c, 0x01, 0x02, // ifge 258
            0x9d, 0x01, 0x02, // ifgt 258
            0x9e, 0x01, 0x02, // ifle 258
            0xc7, 0x01, 0x02, // ifnonnull 258
            0xc6, 0x01, 0x02, // ifnull 258
            0x84, 0x01, 0x02, // iinc 1 2
            0x15, 0x01, // iload 1
            0x1a, // iload_0
            0x1b, // iload_1
            0x1c, // iload_2
            0x1d, // iload_3
            0x68, // imul
            0x74, // ineg
            0xc1, 0x01, 0x02, // instanceof 258
            0xba, 0x01, 0x02, 0x00, 0x00, // invokedynamic 258 0 0
            0xb9, 0x01, 0x02, 0x03, 0x00, // invokeinterface 258 3 0
            0xb7, 0x01, 0x02, // invokespecial 258
            0xb8, 0x01, 0x02, // invokestatic 258
            0xb6, 0x01, 0x02, // invokevirtual 258
            0x80, // ior
            0x70, // irem
            0xac, // ireturn
            0x78, // ishl
            0x7a, // ishr
            0x36, 0x01, // istore 1
            0x3b, // istore_0
            0x3c, // istore_1
            0x3d, // istore_2
            0x3e, // istore_3
            0x64, // isub
            0x7c, // iushr
            0x82, // ixor
            0xa8, 0x01, 0x02, // jsr 258
            0xc9, 0x01, 0x02, 0x03, 0x04, // jsr_w 16909060
            0x8a, // l2d
            0x89, // l2f
            0x88, // l2i
            0x61, // ladd
            0x2f, // laload
            0x7f, // land
            0x50, // lastore
            0x94, // lcmp
            0x09, // lconst_0
            0x0a, // lconst_1
            0x12, 0x01, // ldc 1
            0x13, 0x01, 0x02, // ldc_w 258
            0x14, 0x01, 0x02, // ldc2_w 258
            0x6d, // ldiv
            0x16, 0x01, // lload 1
            0x1e, // lload_0
            0x1f, // lload_1
            0x20, // lload_2
            0x21, // lload_3
            0x69, // lmul
            0x75, // lneg
            // lookupswitch
            0x81, // lor
            0x71, // lrem
            0xad, // lreturn
            0x79, // lshl
            0x7b, // lshr
            0x37, 0x01, // lstore 1
            0x3f, // lstore_0
            0x40, // lstore_1
            0x41, // lstore_2
            0x42, // lstore_3
            0x65, // lsub
            0x7d, // lushr
            0x83, // lxor
            0xc2, // monitorenter
            0xc3, // monitorexit
            0xc5, 0x01, 0x02, 0x03, // multianewarray 258 3
            0xbb, 0x01, 0x02, // new 258
            0xbc, 0x01, // newarray 1
            0x00, // nop
            0x57, // pop
            0x58, // pop2
            0xb5, 0x01, 0x02, // putfield 258
            0xb3, 0x01, 0x02, // putstatic 258
            0xa9, 0x01, // ret 1
            0xb1, // return
            0x35, // saload
            0x56, // sastore
            0x11, 0x01, 0x02, // sipush 258
            0x5f, // swap
            // tableswitch
            0xc4, 0x15, 0x01, 0x02, // wide iload 258
            0xc4, 0x17, 0x01, 0x02, // wide fload 258
            0xc4, 0x19, 0x01, 0x02, // wide aload 258
            0xc4, 0x16, 0x01, 0x02, // wide lload 258
            0xc4, 0x18, 0x01, 0x02, // wide dload 258
            0xc4, 0x36, 0x01, 0x02, // wide istore 258
            0xc4, 0x38, 0x01, 0x02, // wide fstore 258
            0xc4, 0x3a, 0x01, 0x02, // wide astore 258
            0xc4, 0x37, 0x01, 0x02, // wide lstore 258
            0xc4, 0x39, 0x01, 0x02, // wide dstore 258
            0xc4, 0xa9, 0x01, 0x02, // wide ret 258
            0xc4, 0x84, 0x01, 0x02, 0x03, 0x04, // wide iinc 258 772
        ];

        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Aaload);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Aastore);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::AconstNull);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Aload(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Aload0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Aload1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Aload2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Aload3);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Anewarray(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Areturn);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Arraylength);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Astore(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Astore0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Astore1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Astore2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Astore3);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Athrow);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Baload);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Bastore);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Bipush(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Caload);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Castore);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Checkcast(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::D2f);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::D2i);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::D2l);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dadd);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Daload);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dastore);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dcmpg);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dcmpl);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dconst0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dconst1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ddiv);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dload(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dload0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dload1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dload2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dload3);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dmul);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dneg);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Drem);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dreturn);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dstore(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dstore0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dstore1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dstore2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dstore3);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dsub);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dup);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::DupX1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::DupX2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dup2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dup2X1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Dup2X2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::F2d);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::F2i);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::F2l);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fadd);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Faload);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fastore);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fcmpg);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fcmpl);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fconst0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fconst1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fconst2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fdiv);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fload(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fload0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fload1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fload2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fload3);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fmul);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fneg);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Frem);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Freturn);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fstore(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fstore0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fstore1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fstore2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fstore3);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Fsub);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Getfield(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Getstatic(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Goto(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::GotoW(16909060));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::I2b);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::I2c);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::I2d);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::I2f);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::I2l);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::I2s);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iadd);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iaload);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iand);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iastore);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::IconstM1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iconst0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iconst1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iconst2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iconst3);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iconst4);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iconst5);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Idiv);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::IfAcmpeq(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::IfAcmpne(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::IfIcmpeq(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::IfIcmpne(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::IfIcmplt(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::IfIcmpge(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::IfIcmpgt(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::IfIcmple(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ifeq(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ifne(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iflt(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ifge(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ifgt(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ifle(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ifnonnull(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ifnull(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iinc(1, 2));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iload(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iload0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iload1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iload2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iload3);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Imul);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ineg);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Instanceof(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Invokedynamic(258, 0, 0));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Invokeinterface(258, 3, 0));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Invokespecial(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Invokestatic(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Invokevirtual(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ior);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Irem);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ireturn);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ishl);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ishr);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Istore(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Istore0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Istore1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Istore2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Istore3);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Isub);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Iushr);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ixor);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Jsr(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::JsrW(16909060));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::L2d);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::L2f);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::L2i);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ladd);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Laload);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Land);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lastore);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lcmp);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lconst0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lconst1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ldc(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::LdcW(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ldc2W(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ldiv);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lload(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lload0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lload1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lload2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lload3);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lmul);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lneg);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lor);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lrem);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lreturn);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lshl);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lshr);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lstore(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lstore0);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lstore1);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lstore2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lstore3);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lsub);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lushr);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Lxor);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Monitorenter);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Monitorexit);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Multianewarray(258, 3));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::New(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Newarray(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Nop);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Pop);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Pop2);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Putfield(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Putstatic(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Ret(1));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Return);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Saload);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Sastore);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Sipush(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::Swap);
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::WideIload(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::WideFload(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::WideAload(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::WideLload(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::WideDload(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::WideIstore(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::WideFstore(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::WideAstore(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::WideLstore(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::WideDstore(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::WideRet(258));
        let (input, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, Instruction::WideIinc(258, 772));

        assert_eq!(input.len(), 0);
    }
}
