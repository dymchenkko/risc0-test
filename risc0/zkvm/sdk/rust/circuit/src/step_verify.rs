// This code is automatically generated

use anyhow::Result;
use risc0_zkp::{
    adapter::{CircuitStep, CircuitStepContext, CircuitStepDef, CircuitStepVerify, CustomStep},
    core::fp::Fp,
};

use super::CircuitImpl;

#[rustfmt::skip]
const DEF: CircuitStepDef = CircuitStepDef {
    block: &[CircuitStep::Const(2, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:28"),
CircuitStep::Const(3, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:28"),
CircuitStep::Const(4, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:28"),
CircuitStep::Const(1509949441, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:46"),
CircuitStep::Const(16, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:46"),
CircuitStep::Const(64, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:46"),
CircuitStep::Const(256, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:46"),
CircuitStep::Const(1024, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:46"),
CircuitStep::Const(4096, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:46"),
CircuitStep::Const(16384, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:46"),
CircuitStep::Const(65536, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:46"),
CircuitStep::Const(262144, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:46"),
CircuitStep::Const(1048576, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:46"),
CircuitStep::Const(4194304, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:46"),
CircuitStep::Const(2013265801, "external/risc0/risc0/zkvm/circuit/multiply_cycle.cpp:69"),
CircuitStep::Const(1, "circuits/rv32im-legacy/port.cpp:207"),
CircuitStep::Get(0, 0, 0, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:23"),
CircuitStep::Get(0, 3, 0, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:25"),
CircuitStep::Get(0, 4, 0, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:30"),
CircuitStep::Get(0, 5, 0, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:37"),
CircuitStep::Get(0, 6, 0, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:55"),
CircuitStep::Get(0, 1, 0, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:63"),
CircuitStep::Get(0, 2, 0, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:143"),
CircuitStep::Add(21, 22, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:192"),
CircuitStep::Add(23, 17, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:192"),
CircuitStep::Add(24, 18, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:192"),
CircuitStep::Add(25, 19, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:192"),
CircuitStep::Add(26, 20, "external/risc0/risc0/zkvm/circuit/data_regs.cpp:192"),
CircuitStep::If(27, &[CircuitStep::Nondet(&[CircuitStep::Extern("memCheck", "", &[], "external/risc0/risc0/zkvm/circuit/mem_check.cpp:24"),
CircuitStep::Set(2, 28, 148, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:25"),
CircuitStep::Set(2, 29, 144, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:26"),
CircuitStep::Set(2, 30, 147, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:27"),
CircuitStep::Set(2, 31, 145, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:28"),
CircuitStep::Set(2, 32, 146, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:29"),
], "external/risc0/risc0/zkvm/circuit/mem_check.cpp:23"),
CircuitStep::If(16, &[CircuitStep::Get(2, 144, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:37"),
CircuitStep::Get(2, 144, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:38"),
CircuitStep::Nondet(&[CircuitStep::Sub(28, 29, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:39"),
CircuitStep::IsZero(30, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:39"),
CircuitStep::Sub(15, 31, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:39"),
CircuitStep::Sub(15, 32, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:39"),
CircuitStep::Set(2, 33, 149, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:39"),
], "external/risc0/risc0/zkvm/circuit/mem_check.cpp:39"),
CircuitStep::Get(2, 149, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:40"),
CircuitStep::If(30, &[CircuitStep::Sub(28, 29, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:41"),
CircuitStep::EqZero(31, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:41"),
CircuitStep::Get(2, 147, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:42"),
CircuitStep::Sub(15, 32, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:42"),
CircuitStep::If(33, &[CircuitStep::Get(2, 145, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:43"),
CircuitStep::Get(2, 145, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:43"),
CircuitStep::Sub(35, 34, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:43"),
CircuitStep::EqZero(36, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:43"),
CircuitStep::Get(2, 146, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:44"),
CircuitStep::Get(2, 146, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:44"),
CircuitStep::Sub(38, 37, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:44"),
CircuitStep::EqZero(39, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:44"),
], "external/risc0/risc0/zkvm/circuit/mem_check.cpp:42"),
CircuitStep::Get(2, 148, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 148, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(35, 34, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(36, 15, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Nondet(&[CircuitStep::BitAnd(37, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Set(2, 38, 150, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(37, 38, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(39, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::BitAnd(40, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Set(2, 41, 151, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(40, 41, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(42, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::BitAnd(43, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Set(2, 44, 152, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(43, 44, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(45, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::BitAnd(46, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Set(2, 47, 153, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(46, 47, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(48, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::BitAnd(49, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Set(2, 50, 154, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(49, 50, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(51, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::BitAnd(52, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Set(2, 53, 155, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(52, 53, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(54, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::BitAnd(55, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Set(2, 56, 156, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(55, 56, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(57, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::BitAnd(58, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Set(2, 59, 157, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(58, 59, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(60, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::BitAnd(61, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Set(2, 62, 158, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(61, 62, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(63, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::BitAnd(64, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Set(2, 65, 159, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(64, 65, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(66, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::BitAnd(67, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Set(2, 68, 160, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(67, 68, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(69, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::BitAnd(70, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Set(2, 71, 161, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
], "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 150, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 151, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(39, 2, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Add(38, 40, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 152, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(42, 4, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Add(41, 43, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 153, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(45, 5, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Add(44, 46, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 154, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(48, 6, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Add(47, 49, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 155, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(51, 7, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Add(50, 52, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 156, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(54, 8, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Add(53, 55, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 157, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(57, 9, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Add(56, 58, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 158, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(60, 10, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Add(59, 61, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 159, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(63, 11, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Add(62, 64, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 160, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(66, 12, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Add(65, 67, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Get(2, 161, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(69, 13, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Add(68, 70, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Sub(37, 71, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::Mul(72, 14, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
CircuitStep::EqZero(73, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:46"),
], "external/risc0/risc0/zkvm/circuit/mem_check.cpp:40"),
CircuitStep::Sub(15, 30, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::If(31, &[CircuitStep::Sub(28, 29, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(32, 15, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Nondet(&[CircuitStep::BitAnd(33, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Set(2, 34, 150, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(33, 34, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(35, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::BitAnd(36, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Set(2, 37, 151, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(36, 37, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(38, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::BitAnd(39, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Set(2, 40, 152, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(39, 40, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(41, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::BitAnd(42, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Set(2, 43, 153, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(42, 43, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(44, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::BitAnd(45, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Set(2, 46, 154, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(45, 46, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(47, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::BitAnd(48, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Set(2, 49, 155, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(48, 49, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(50, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::BitAnd(51, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Set(2, 52, 156, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(51, 52, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(53, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::BitAnd(54, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Set(2, 55, 157, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(54, 55, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(56, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::BitAnd(57, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Set(2, 58, 158, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(57, 58, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(59, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::BitAnd(60, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Set(2, 61, 159, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(60, 61, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(62, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::BitAnd(63, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Set(2, 64, 160, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(63, 64, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(65, 3, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::BitAnd(66, 1, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Set(2, 67, 161, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
], "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Get(2, 150, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Get(2, 151, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(35, 2, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Add(34, 36, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Get(2, 152, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(38, 4, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Add(37, 39, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Get(2, 153, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(41, 5, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Add(40, 42, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Get(2, 154, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(44, 6, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Add(43, 45, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Get(2, 155, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(47, 7, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Add(46, 48, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Get(2, 156, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(50, 8, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Add(49, 51, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Get(2, 157, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(53, 9, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Add(52, 54, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Get(2, 158, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(56, 10, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Add(55, 57, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Get(2, 159, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(59, 11, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Add(58, 60, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Get(2, 160, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(62, 12, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Add(61, 63, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Get(2, 161, 0, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(65, 13, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Add(64, 66, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Sub(33, 67, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::Mul(68, 14, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
CircuitStep::EqZero(69, "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
], "external/risc0/risc0/zkvm/circuit/mem_check.cpp:48"),
], "external/risc0/risc0/zkvm/circuit/mem_check.cpp:33"),
CircuitStep::Get(2, 149, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(28, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(28, 29, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(30, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Get(2, 150, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(31, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(31, 32, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(31, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(33, 34, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(31, 1, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(35, 36, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(37, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Get(2, 151, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(38, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(38, 39, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(38, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(40, 41, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(38, 1, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(42, 43, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(44, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Get(2, 152, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(45, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(45, 46, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(45, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(47, 48, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(45, 1, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(49, 50, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(51, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Get(2, 153, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(52, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(52, 53, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(52, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(54, 55, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(52, 1, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(56, 57, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(58, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Get(2, 154, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(59, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(59, 60, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(59, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(61, 62, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(59, 1, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(63, 64, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(65, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Get(2, 155, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(66, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(66, 67, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(66, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(68, 69, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(66, 1, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(70, 71, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(72, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Get(2, 156, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(73, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(73, 74, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(73, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(75, 76, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(73, 1, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(77, 78, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(79, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Get(2, 157, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(80, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(80, 81, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(80, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(82, 83, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(80, 1, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(84, 85, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(86, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Get(2, 158, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(87, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(87, 88, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(87, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(89, 90, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(87, 1, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(91, 92, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(93, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Get(2, 159, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(94, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(94, 95, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(94, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(96, 97, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(94, 1, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(98, 99, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(100, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Get(2, 160, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(101, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(101, 102, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(101, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(103, 104, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(101, 1, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(105, 106, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(107, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Get(2, 161, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(108, 15, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(108, 109, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(108, 0, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(110, 111, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Sub(108, 1, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::Mul(112, 113, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
CircuitStep::EqZero(114, "external/risc0/risc0/zkvm/circuit/mem_check.h:28"),
], "external/risc0/risc0/zkvm/circuit/data_regs.cpp:194"),
],
    ret: 15,
};

impl<S: CustomStep> CircuitStepVerify<S> for CircuitImpl {
    #[allow(unused)]
    fn step_verify(
        &self,
        ctx: &CircuitStepContext,
        custom: &mut S,
        args: &mut [&mut [Fp]],
    ) -> Result<Fp> {
        DEF.step(ctx, custom, args)
    }
}
