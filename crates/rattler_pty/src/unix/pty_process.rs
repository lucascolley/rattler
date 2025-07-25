pub use nix::sys::{signal, wait};
use nix::{
    self,
    fcntl::{open, OFlag},
    libc::STDERR_FILENO,
    pty::{grantpt, posix_openpt, unlockpt, PtyMaster, Winsize},
    sys::{
        stat,
        termios::{self, InputFlags, Termios},
    },
    unistd::{close, dup, dup2_stderr, dup2_stdin, dup2_stdout, fork, setsid, ForkResult, Pid},
};
use std::os::fd::AsFd;
use std::{
    self,
    fs::File,
    io,
    os::unix::{io::AsRawFd, process::CommandExt},
    process::Command,
    thread, time,
};

#[cfg(target_os = "linux")]
use nix::pty::ptsname_r;

/// Start a process in a forked tty so you can interact with it the same as you would
/// within a terminal
///
/// The process and pty session are killed upon dropping `PtyProcess`
pub struct PtyProcess {
    pub pty: PtyMaster,
    pub child_pid: Pid,
    kill_timeout: Option<time::Duration>,
}

#[cfg(target_os = "macos")]
/// `ptsname_r` is a linux extension but ptsname isn't thread-safe
/// instead of using a static mutex this calls `ioctl` with `TIOCPTYGNAME` directly
/// <https://blog.tarq.io/ptsname-on-osx-with-rust>/
fn ptsname_r(fd: &PtyMaster) -> nix::Result<String> {
    use nix::libc::{ioctl, TIOCPTYGNAME};
    use std::ffi::CStr;

    // the buffer size on OSX is 128, defined by sys/ttycom.h
    let mut buf: [i8; 128] = [0; 128];

    unsafe {
        match ioctl(fd.as_raw_fd(), u64::from(TIOCPTYGNAME), &mut buf) {
            0 => {
                let res = CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned();
                Ok(res)
            }
            _ => Err(nix::Error::last()),
        }
    }
}

#[derive(Default)]
pub struct PtyProcessOptions {
    pub echo: bool,
    pub window_size: Option<Winsize>,
}

impl PtyProcess {
    /// Start a process in a forked pty
    pub fn new(mut command: Command, opts: PtyProcessOptions) -> nix::Result<Self> {
        // Open a new PTY master
        let master_fd = posix_openpt(OFlag::O_RDWR)?;

        // Allow a slave to be generated for it
        grantpt(&master_fd)?;
        unlockpt(&master_fd)?;

        // on Linux this is the libc function, on OSX this is our implementation of ptsname_r
        let slave_name = ptsname_r(&master_fd)?;

        // Get the current window size if it was not specified
        let window_size = opts.window_size.unwrap_or_else(|| {
            // find current window size with ioctl
            let mut size: libc::winsize = unsafe { std::mem::zeroed() };
            // Query the terminal dimensions
            unsafe { libc::ioctl(io::stdout().as_raw_fd(), libc::TIOCGWINSZ, &mut size) };
            size
        });

        match unsafe { fork()? } {
            ForkResult::Child => {
                // Avoid leaking master fd
                close(master_fd.as_raw_fd())?;

                setsid()?; // create new session with child as session leader
                let slave_fd = open(
                    std::path::Path::new(&slave_name),
                    OFlag::O_RDWR,
                    stat::Mode::empty(),
                )?;

                // assign stdin, stdout, stderr to the tty, just like a terminal does
                dup2_stdin(&slave_fd)?;
                dup2_stdout(&slave_fd)?;
                dup2_stderr(&slave_fd)?;

                // Avoid leaking slave fd
                if slave_fd.as_raw_fd() > STDERR_FILENO {
                    close(slave_fd)?;
                }

                // Set `echo` and `window_size` for the pty
                set_echo(io::stdin(), opts.echo)?;
                set_window_size(io::stdout().as_raw_fd(), window_size)?;

                let _ = command.exec();
                Err(nix::Error::last())
            }
            ForkResult::Parent { child: child_pid } => Ok(PtyProcess {
                pty: master_fd,
                child_pid,
                kill_timeout: None,
            }),
        }
    }

    /// Get handle to pty fork for reading/writing
    pub fn get_file_handle(&self) -> nix::Result<File> {
        // needed because otherwise fd is closed both by dropping process and reader/writer
        let fd = dup(&self.pty)?;
        Ok(File::from(fd))
    }

    /// Get status of child process, non-blocking.
    ///
    /// This method runs waitpid on the process.
    /// This means: If you ran `exit()` before or `status()` this method will
    /// return `None`
    pub fn status(&self) -> Option<wait::WaitStatus> {
        wait::waitpid(self.child_pid, Some(wait::WaitPidFlag::WNOHANG)).ok()
    }

    /// Regularly exit the process, this method is blocking until the process is dead
    pub fn exit(&mut self) -> nix::Result<wait::WaitStatus> {
        self.kill(signal::SIGTERM)
    }

    /// Kill the process with a specific signal. This method blocks, until the process is dead
    ///
    /// repeatedly sends SIGTERM to the process until it died,
    /// the pty session is closed upon dropping `PtyMaster`,
    /// so we don't need to explicitly do that here.
    ///
    /// if `kill_timeout` is set and a repeated sending of signal does not result in the process
    /// being killed, then `kill -9` is sent after the `kill_timeout` duration has elapsed.
    pub fn kill(&mut self, sig: signal::Signal) -> nix::Result<wait::WaitStatus> {
        let start = time::Instant::now();
        loop {
            match signal::kill(self.child_pid, sig) {
                Ok(_) => {}
                // process was already killed before -> ignore
                Err(nix::errno::Errno::ESRCH) => {
                    return Ok(wait::WaitStatus::Exited(Pid::from_raw(0), 0));
                }
                Err(e) => return Err(e),
            }

            match self.status() {
                Some(status) if status != wait::WaitStatus::StillAlive => return Ok(status),
                Some(_) | None => thread::sleep(time::Duration::from_millis(100)),
            }
            // kill -9 if timeout is reached
            if let Some(timeout) = self.kill_timeout {
                if start.elapsed() > timeout {
                    signal::kill(self.child_pid, signal::Signal::SIGKILL)?;
                }
            }
        }
    }

    /// Set raw mode on stdin and return the original mode
    pub fn set_raw(&self) -> nix::Result<Termios> {
        let original_mode = termios::tcgetattr(io::stdin())?;
        let mut raw_mode = original_mode.clone();
        raw_mode.input_flags.remove(
            InputFlags::BRKINT
                | InputFlags::ICRNL
                | InputFlags::INPCK
                | InputFlags::ISTRIP
                | InputFlags::IXON,
        );
        raw_mode.output_flags.remove(termios::OutputFlags::OPOST);
        raw_mode
            .control_flags
            .remove(termios::ControlFlags::CSIZE | termios::ControlFlags::PARENB);
        raw_mode.control_flags.insert(termios::ControlFlags::CS8);
        raw_mode.local_flags.remove(
            termios::LocalFlags::ECHO
                | termios::LocalFlags::ICANON
                | termios::LocalFlags::IEXTEN
                | termios::LocalFlags::ISIG,
        );

        raw_mode.control_chars[termios::SpecialCharacterIndices::VMIN as usize] = 1;
        raw_mode.control_chars[termios::SpecialCharacterIndices::VTIME as usize] = 0;

        termios::tcsetattr(io::stdin(), termios::SetArg::TCSAFLUSH, &raw_mode)?;

        Ok(original_mode)
    }

    pub fn set_mode(&self, original_mode: Termios) -> nix::Result<()> {
        termios::tcsetattr(io::stdin(), termios::SetArg::TCSAFLUSH, &original_mode)?;
        Ok(())
    }

    pub fn set_window_size(&self, window_size: Winsize) -> nix::Result<()> {
        set_window_size(self.pty.as_raw_fd(), window_size)
    }
}

pub fn set_window_size(raw_fd: i32, window_size: Winsize) -> nix::Result<()> {
    unsafe { libc::ioctl(raw_fd, nix::libc::TIOCSWINSZ, &window_size) };
    Ok(())
}

pub fn set_echo<Fd: AsFd>(fd: Fd, echo: bool) -> nix::Result<()> {
    let mut flags = termios::tcgetattr(&fd)?;
    if echo {
        flags.local_flags.insert(termios::LocalFlags::ECHO);
    } else {
        flags.local_flags.remove(termios::LocalFlags::ECHO);
    }
    termios::tcsetattr(&fd, termios::SetArg::TCSANOW, &flags)?;
    Ok(())
}

impl Drop for PtyProcess {
    fn drop(&mut self) {
        if let Some(wait::WaitStatus::StillAlive) = self.status() {
            self.exit().expect("cannot exit");
        }
    }
}
