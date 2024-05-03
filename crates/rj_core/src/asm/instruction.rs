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
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideIload(index)))
            }
            Ok((_, Instruction::Fload(_))) => {
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideFload(index)))
            }
            Ok((_, Instruction::Aload(_))) => {
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideAload(index)))
            }
            Ok((_, Instruction::Lload(_))) => {
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideLload(index)))
            }
            Ok((_, Instruction::Dload(_))) => {
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideDload(index)))
            }
            Ok((_, Instruction::Istore(_))) => {
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideIstore(index)))
            }
            Ok((_, Instruction::Fstore(_))) => {
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideFstore(index)))
            }
            Ok((_, Instruction::Astore(_))) => {
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideAstore(index)))
            }
            Ok((_, Instruction::Lstore(_))) => {
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideLstore(index)))
            }
            Ok((_, Instruction::Dstore(_))) => {
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideDstore(index)))
            }
            Ok((_, Instruction::Ret(_))) => {
                let (input, index) = be_u16(input)?;
                Ok((input, Instruction::WideRet(index)))
            }
            Ok((_, Instruction::Iinc(_, _))) => {
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
