extern crate nix;

use nix::unistd::{fork, ForkResult, setsid, execvp, getpid, getppid};
use nix::sys::signal::{Signal, kill};
use std::ffi::CString;

pub fn gdb_probe(){
    match fork() {
       Ok(ForkResult::Parent { child, .. }) => {
           println!("GDB PROBE HIT, SLEEPING");
           kill(getpid(), Signal::SIGSTOP );
           println!("GDB ATTACHED, CONTINUE");
       }
       Ok(ForkResult::Child) => {
           let gdb =  format!("sudo ugdb --gdb=rust-gdb --pid {}", getppid());
           println!("gdb: {}", gdb);
           let argv = vec!["tmux", "neww", &gdb];
           let argv_c = argv.iter().map(|s| CString::new(*s).unwrap()).collect::<Vec<_>>();
           setsid().expect("could not setsid()");
           execvp(&CString::new("tmux").unwrap(), &argv_c[..]).expect("could not execv terminal, maybe urxvt is missing?");
       }

       Err(_) => println!("Fork failed"),
    }
}

#[cfg(test)]
mod tests {
    use crate::gdb_probe;
    #[test]
    fn it_works() {
        let x = 3+4;
        gdb_probe();
        assert!(false);
    }
}
