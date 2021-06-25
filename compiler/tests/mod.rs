// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

// allow the use of TestCompiler::parse_program_from_string for tests

#![allow(deprecated)]

pub mod canonicalization;
pub mod type_inference;

use leo_asg::{new_alloc_context, new_context, AsgContext};
use leo_compiler::{compiler::Compiler, errors::CompilerError, OutputBytes};
use snarkvm_eval::edwards_bls12::Fq;
use snarkvm_eval::edwards_bls12::EdwardsGroupType;
use snarkvm_r1cs::TestConstraintSystem;

use std::path::PathBuf;

pub const TEST_OUTPUT_DIRECTORY: &str = "/output/";

pub type TestCompiler = Compiler<'static>;

//convenience function for tests, leaks memory
pub(crate) fn make_test_context() -> AsgContext<'static> {
    let allocator = Box::leak(Box::new(new_alloc_context()));
    new_context(allocator)
}

fn new_compiler() -> TestCompiler {
    let program_name = "test".to_string();
    let path = PathBuf::from("/test/src/main.leo");
    let output_dir = PathBuf::from(TEST_OUTPUT_DIRECTORY);

    TestCompiler::new(program_name, path, output_dir, make_test_context(), None)
}

pub(crate) fn parse_program(program_string: &str) -> Result<TestCompiler, CompilerError> {
    let mut compiler = new_compiler();

    compiler.parse_program_from_string(program_string)?;

    Ok(compiler)
}

pub(crate) fn get_output(program: TestCompiler) -> OutputBytes {
    // synthesize the circuit on the test constraint system
    let mut cs = TestConstraintSystem::<Fq>::new();

    let output = program.compile::<Fq, EdwardsGroupType, _>(&mut cs, &Default::default()).unwrap();

    // assert the constraint system is satisfied
    assert!(cs.is_satisfied());

    output.output.into()
}

pub(crate) fn assert_satisfied(program: TestCompiler) {
    let empty_output_bytes = include_bytes!("compiler_output/empty.out");
    let res = get_output(program);

    // assert that the output is empty
    assert_eq!(empty_output_bytes, res.bytes().as_slice());
}
