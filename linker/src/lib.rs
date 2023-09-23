use std::path::Path;
pub fn link(obj_file: &Path, output: &Path) -> Result<(), std::io::Error> {
    use std::io::{Error, ErrorKind};
    use std::process::Command;
    let mut cmd = "cc";

    // link the .o file using host linker
    if cfg!(windows) {
        cmd = "cl.exe";
    } 
    let status = Command::new(cmd)
        .args(&[&obj_file, Path::new("-o"), output])
        .status()
        .map_err(|err| {
            if err.kind() == ErrorKind::NotFound {
                Error::new(
                    ErrorKind::NotFound,
                    "could not find host cc (for linking). Is it on your PATH?",
                )
            } else {
                err
            }
        })?;
    if !status.success() {
        Err(Error::new(ErrorKind::Other, "linking program failed"))
    } else {
        Ok(())
    }
}
