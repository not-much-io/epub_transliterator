use anyhow::{anyhow, Result};
use log::{debug, error};
use std::{
    io::BufRead,
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn epub_to_html(path_to_input_epub: &PathBuf) -> Result<Vec<u8>> {
    call_command(
        Command::new("pandoc")
            .arg("--self-contained")
            .args(["--from", "epub"])
            .args(["--to", "html"])
            .arg(path_to_input_epub),
    )
}

pub fn html_to_epub(path_to_input_html: &PathBuf) -> Result<Vec<u8>> {
    call_command(
        Command::new("pandoc")
            .args(["--from", "html"])
            .args(["--to", "epub"])
            .arg(path_to_input_html),
    )
}

fn call_command(cmd: &mut Command) -> Result<Vec<u8>> {
    let output = cmd
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()?;

    for line in output.stderr.lines() {
        error!("[pandoc] {:?}", line?);
    }

    let exit_status = output.status;
    if !exit_status.success() {
        return Err(anyhow!(
            "pandoc returned a non null exit status: {}",
            exit_status
                .code()
                .expect("impossible state: no exit status after command completed")
        ));
    }

    debug!(
        "[pandoc] completed, produced {} bytes of data",
        output.stdout.len()
    );
    Ok(output.stdout)
}
