use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let batch_content = "@echo off
echo hi, this is a test
pause
";
    let temp_dir = env::temp_dir();
    let batch_file_path = temp_dir.join("batch4rust.bat");
    if let Err(e) = fs::write(&batch_file_path, batch_content) {
        eprintln!("Failure writing the batch :(\nError happened: {}", e);
        return;
    }
    if let Err(e) = Command::new("cmd")
        .args(&["/C", batch_file_path.to_str().unwrap()])
        .status()
    {
        eprintln!("Failed to execute :(\nError happened: {}", e);
    }
}