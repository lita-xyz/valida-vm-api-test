#[cfg(target_arch = "x86_64")]
use valida_vm_api_linux_x86::*;
#[cfg(target_arch = "arm")]
use valida_vm_api_linux_arm::*;
use tempfile::NamedTempFile;
use tmpfile_helper::*;
use std::fs;
use std::path::Path;
use std::default;

fn main() {
  let program = Path::new("test_data").join("program1");

  let valida = create_valida().unwrap();

  // stdin is an ASCII representation of character 'a'
  let stdin = bytes_to_temp_file("a".as_bytes()).unwrap();
  let stdout = NamedTempFile::new().unwrap();

  let run_status = valida.run(
      &program,
      stdout.as_ref(),
      stdin.as_ref(),
      Default::default(),
      Default::default());

  // Check that program terminated with success, i.e. STOP opcode
  assert_eq!(run_status, RunStatus::TerminatedWithStop);

  let stdout_content = fs::read_to_string(stdout.as_ref()).unwrap();
  // Check that stdout contains ('a' + 1)
  assert_eq!(stdout_content, "b");

  let proof = NamedTempFile::new().unwrap();
  let prove_status = valida.prove(
      &program, proof.as_ref(),
      stdin.as_ref(),
      Default::default(),
      Default::default(),
      Default::default());

  // Proving of a program that terminates with STOP opcode must succeed
  assert_eq!(prove_status, ProveStatus::Success);

  let verify_status_correct_statement = valida.verify(
      &program,
      proof.as_ref(),
      stdout.as_ref(),
      Default::default(),
      Default::default(),
      Default::default());

  // Verification of a program that terminates with STOP opcode and outputs 'b' must succeed
  assert_eq!(verify_status_correct_statement, VerifyStatus::Success);

  let incorrect_stdout = bytes_to_temp_file("c".as_bytes()).unwrap();
  let verify_status_incorrect_statement = valida.verify(
      &program,
      proof.as_ref(),
      incorrect_stdout.as_ref(),
      Default::default(),
      Default::default(),
      Default::default());

  // Verification of a program that terminates with STOP opcode
  // and outputs something else than `b` must fail
  assert_eq!(verify_status_incorrect_statement, VerifyStatus::Failure);
}
