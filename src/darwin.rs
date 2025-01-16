use std::{ffi::OsStr, path::Path};

use libc::{c_char, c_int, c_void};

/// A wrapper around a process ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Pid(pub u32);

/// Get the current process ID. This is equivalent to `std::process::id()`.
pub fn getpid() -> Pid {
    // SAFETY: We know this is safe to call. The function never fails.
    let pid = unsafe { libc::getpid() };
    Pid(pid as _)
}

/// A wrapper around a file descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Fd(pub c_int);

/// A wrapper around a Mach fileport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FilePort(pub u32);

/// An error that occurs when an unexpected value is encountered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueError {
    UnexpectedEnumValue,
    InvalidString,
}

/// Convert a C string to a Rust string.
fn libc_str_to_str(array: &[c_char]) -> Result<&str, ValueError> {
    // Find the first NUL, otherwise use the full array
    let nul_index = array.iter().position(|&c| c == 0).unwrap_or(array.len());
    // SAFETY: We know this is actually u8.
    std::str::from_utf8(unsafe { std::mem::transmute::<&[c_char], &[u8]>(&array[..nul_index]) })
        .map_err(|_| ValueError::InvalidString)
}

/// Convert a C string to a Rust path.
fn libc_str_to_path(array: &[c_char]) -> Result<&Path, ValueError> {
    // Find the first NUL, otherwise use the full array
    let nul_index = array.iter().position(|&c| c == 0).unwrap_or(array.len());
    // SAFETY: We know this is actually u8.
    Ok(Path::new(unsafe {
        OsStr::from_encoded_bytes_unchecked(std::mem::transmute::<&[c_char], &[u8]>(
            &array[..nul_index],
        ))
    }))
}

/// A trait for types that have a flavor.
trait HasFlavor {
    const FLAVOR: ProcPidInfoFlavor;
}

trait HasFlavorList {
    const FLAVOR: ProcPidInfoFlavor;
}

/// For `proc_pidinfo`.
#[allow(non_camel_case_types, unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcPidInfoFlavor {
    PROC_PIDLISTFDS = 1,
    PROC_PIDTASKALLINFO = 2,
    PROC_PIDTBSDINFO = 3,
    PROC_PIDTASKINFO = 4,
    PROC_PIDTHREADINFO = 5,
    PROC_PIDLISTTHREADS = 6,
    PROC_PIDREGIONINFO = 7,
    PROC_PIDREGIONPATHINFO = 8,
    PROC_PIDVNODEPATHINFO = 9,
    PROC_PIDTHREADPATHINFO = 10,
    PROC_PIDPATHINFO = 11,
    PROC_PIDWORKQUEUEINFO = 12,
    PROC_PIDT_SHORTBSDINFO = 13,
    PROC_PIDLISTFILEPORTS = 14,
    PROC_PIDTHREADID64INFO = 15,
    PROC_PID_RUSAGE = 16,
}

trait HasFdFlavor {
    const FLAVOR: ProcPidFdInfoFlavor;
}

/// For `proc_pidfdinfo`.
#[allow(non_camel_case_types, unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum ProcPidFdInfoFlavor {
    PROC_PIDFDVNODEINFO = 1,
    PROC_PIDFDVNODEPATHINFO = 2,
    PROC_PIDFDSOCKETINFO = 3,
    PROC_PIDFDPSEMINFO = 4,
    PROC_PIDFDPSHMINFO = 5,
    PROC_PIDFDPIPEINFO = 6,
    PROC_PIDFDKQUEUEINFO = 7,
    PROC_PIDFDATALKINFO = 8,
    PROC_PIDFDCHANNELINFO = 10,
}

/// A type for the file descriptor.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ProcFDType {
    ATALK = 0,
    VNODE = 1,
    SOCKET = 2,
    PSHM = 3,
    PSEM = 4,
    KQUEUE = 5,
    PIPE = 6,
    FSEVENTS = 7,
    NETPOLICY = 9,
    CHANNEL = 10,
    NEXUS = 11,
}

/// Information about file descriptors. Usable with [`proc_pidinfo_list`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProcFDInfo {
    pub proc_fd: Fd,
    pub proc_fdtype: u32,
}

impl ProcFDInfo {
    pub fn fd_type(&self) -> Result<ProcFDType, ValueError> {
        match self.proc_fdtype {
            0 => Ok(ProcFDType::ATALK),
            1 => Ok(ProcFDType::VNODE),
            2 => Ok(ProcFDType::SOCKET),
            3 => Ok(ProcFDType::PSHM),
            4 => Ok(ProcFDType::PSEM),
            5 => Ok(ProcFDType::KQUEUE),
            6 => Ok(ProcFDType::PIPE),
            7 => Ok(ProcFDType::FSEVENTS),
            9 => Ok(ProcFDType::NETPOLICY),
            10 => Ok(ProcFDType::CHANNEL),
            11 => Ok(ProcFDType::NEXUS),
            _ => Err(ValueError::UnexpectedEnumValue),
        }
    }
}

impl HasFlavorList for ProcFDInfo {
    const FLAVOR: ProcPidInfoFlavor = ProcPidInfoFlavor::PROC_PIDLISTFDS;
}

/// Information about file ports. Usable with [`proc_pidinfo_list`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProcFilePortInfo {
    pub proc_fileport: FilePort,
    pub proc_fdtype: u32,
}

impl HasFlavorList for ProcFilePortInfo {
    const FLAVOR: ProcPidInfoFlavor = ProcPidInfoFlavor::PROC_PIDLISTFILEPORTS;
}

impl ProcFilePortInfo {
    pub fn fd_type(&self) -> Result<ProcFDType, ValueError> {
        match self.proc_fdtype {
            0 => Ok(ProcFDType::ATALK),
            1 => Ok(ProcFDType::VNODE),
            2 => Ok(ProcFDType::SOCKET),
            3 => Ok(ProcFDType::PSHM),
            4 => Ok(ProcFDType::PSEM),
            5 => Ok(ProcFDType::KQUEUE),
            6 => Ok(ProcFDType::PIPE),
            7 => Ok(ProcFDType::FSEVENTS),
            9 => Ok(ProcFDType::NETPOLICY),
            10 => Ok(ProcFDType::CHANNEL),
            11 => Ok(ProcFDType::NEXUS),
            _ => Err(ValueError::UnexpectedEnumValue),
        }
    }
}

/// Task information about a process. Usable with [`proc_pidinfo`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProcTaskInfo {
    pub pti_virtual_size: u64,
    pub pti_resident_size: u64,
    pub pti_total_user: u64,
    pub pti_total_system: u64,
    pub pti_threads_user: u64,
    pub pti_threads_system: u64,
    pub pti_policy: i32,
    pub pti_faults: i32,
    pub pti_pageins: i32,
    pub pti_cow_faults: i32,
    pub pti_messages_sent: i32,
    pub pti_messages_received: i32,
    pub pti_syscalls_mach: i32,
    pub pti_syscalls_unix: i32,
    pub pti_csw: i32,
    pub pti_threadnum: i32,
    pub pti_numrunning: i32,
    pub pti_priority: i32,
}

/// BSD-style information about a process. Usable with [`proc_pidinfo`].
///
/// In some cases, [`ProcBSDInfo`] may not be available, while [`ProcBSDShortInfo`] is.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProcBSDInfo {
    pub pbi_flags: u32,
    pub pbi_status: u32,
    pub pbi_xstatus: u32,
    pub pbi_pid: Pid,
    pub pbi_ppid: Pid,
    pub pbi_uid: libc::uid_t,
    pub pbi_gid: libc::gid_t,
    pub pbi_ruid: libc::uid_t,
    pub pbi_rgid: libc::gid_t,
    pub pbi_svuid: libc::uid_t,
    pub pbi_svgid: libc::gid_t,
    pub rfu_1: u32,
    pub pbi_comm: [c_char; libc::MAXCOMLEN],
    pub pbi_name: [c_char; 2 * libc::MAXCOMLEN],
    pub pbi_nfiles: u32,
    pub pbi_pgid: u32,
    pub pbi_pjobc: u32,
    pub e_tdev: u32,
    pub e_tpgid: u32,
    pub pbi_nice: i32,
    pub pbi_start_tvsec: u64,
    pub pbi_start_tvusec: u64,
}

impl HasFlavor for ProcBSDInfo {
    const FLAVOR: ProcPidInfoFlavor = ProcPidInfoFlavor::PROC_PIDTBSDINFO;
}

/// A short version of [`ProcBSDInfo`]. Usable with [`proc_pidinfo`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProcBSDShortInfo {
    pub pbsi_pid: Pid,
    pub pbsi_ppid: Pid,
    pub pbsi_pgid: libc::gid_t,
    pub pbsi_status: u32,
    pub pbsi_comm: [c_char; libc::MAXCOMLEN],
    pub pbsi_flags: u32,
    pub pbsi_uid: libc::uid_t,
    pub pbsi_gid: libc::gid_t,
    pub pbsi_ruid: libc::uid_t,
    pub pbsi_rgid: libc::gid_t,
    pub pbsi_svuid: libc::uid_t,
    pub pbsi_svgid: libc::gid_t,
    pub pbsi_rfu: u32,
}

impl ProcBSDShortInfo {
    pub fn comm(&self) -> Result<&str, ValueError> {
        libc_str_to_str(&self.pbsi_comm)
    }
}

impl HasFlavor for ProcBSDShortInfo {
    const FLAVOR: ProcPidInfoFlavor = ProcPidInfoFlavor::PROC_PIDT_SHORTBSDINFO;
}

impl HasFlavor for ProcTaskInfo {
    const FLAVOR: ProcPidInfoFlavor = ProcPidInfoFlavor::PROC_PIDTASKINFO;
}

/// Task information about a process. Usable with [`proc_pidinfo`].
///
/// Returns both [`ProcBSDInfo`] and [`ProcTaskInfo`] in one struct.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProcTaskAllInfo {
    pub pbsd: ProcBSDInfo,
    pub ptinfo: ProcTaskInfo,
}

impl HasFlavor for ProcTaskAllInfo {
    const FLAVOR: ProcPidInfoFlavor = ProcPidInfoFlavor::PROC_PIDTASKALLINFO;
}

/// Get an info struct for a given process.
///
/// Supports:
///
/// - [`ProcTaskInfo`]
/// - [`ProcTaskAllInfo`]
/// - [`ProcBSDInfo`]
/// - [`ProcBSDShortInfo`]
///
/// ```
/// use proc_pidinfo::*;
///
/// # let pid = getpid();
/// let info = proc_pidinfo::<ProcBSDShortInfo>(pid).unwrap().unwrap();
/// println!("{:?}", info);
/// ```
#[allow(private_bounds)]
pub fn proc_pidinfo<T: HasFlavor>(pid: Pid) -> Result<Option<T>, std::io::Error> {
    // SAFETY: We check the size of the return value to ensure it's valid.
    unsafe {
        let mut value = std::mem::MaybeUninit::<T>::uninit();
        let buffersize = std::mem::size_of::<T>() as c_int;
        let res = libc::proc_pidinfo(
            pid.0 as _,
            T::FLAVOR as c_int,
            0,
            value.as_mut_ptr() as *mut c_void,
            buffersize,
        );
        if res < 0 {
            return Err(std::io::Error::from_raw_os_error(res));
        }
        if res == 0 {
            return Ok(None);
        }
        if res != buffersize {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unexpected buffer size {res} != {buffersize}"),
            ));
        }
        Ok(Some(value.assume_init()))
    }
}

/// Get an info struct for the current process. A convenience function that calls
/// [`proc_pidinfo`] with the current process ID.
#[allow(private_bounds)]
pub fn proc_pidinfo_self<T: HasFlavor>() -> Result<Option<T>, std::io::Error> {
    proc_pidinfo(getpid())
}

/// Get a list-type info struct for a given process.
///
/// Supports:
///
/// - [`ProcFDInfo`]
/// - [`ProcFilePortInfo`]
///
/// ```
/// use proc_pidinfo::*;
///
/// # let pid = getpid();
/// for fd in proc_pidinfo_list::<ProcFDInfo>(pid).unwrap() {
///     println!("{:?}", fd);
///     if fd.fd_type() == Ok(ProcFDType::VNODE) {
///         let vnode = proc_pidfdinfo::<VnodeFdInfo>(pid, fd.proc_fd).unwrap().unwrap();
///         println!("Vnode: {:?}", vnode);
///         if let Some(vnode) = proc_pidfdinfo::<VnodeFdInfoWithPath>(pid, fd.proc_fd).unwrap() {
///             println!("Path: {:?}", vnode.path().unwrap());
///         }
///     } else if fd.fd_type() == Ok(ProcFDType::PIPE) {
///         let pipe = proc_pidfdinfo::<PipeFdInfo>(pid, fd.proc_fd).unwrap().unwrap();
///         println!("Pipe: {:?}", pipe);
///     } else {
///         println!("Unknown fd type: {:?}", fd.fd_type().unwrap_err());
///     }
/// }
/// ```
#[allow(private_bounds)]
pub fn proc_pidinfo_list<T: HasFlavorList>(pid: Pid) -> Result<Vec<T>, std::io::Error> {
    // SAFETY: We check the size of the return value to ensure it's valid.
    unsafe {
        // First call with NULL to get a suggested buffer size
        let res = libc::proc_pidinfo(pid.0 as _, T::FLAVOR as c_int, 0, std::ptr::null_mut(), 0);
        if res < 0 {
            return Err(std::io::Error::from_raw_os_error(res));
        }
        let initial_buffer = if res == 0 {
            std::mem::size_of::<T>() * 16
        } else {
            res as usize
        };

        // Use the initial buffer size guess, then keep doubling until we get a result
        let mut buffer = Vec::with_capacity(initial_buffer);
        loop {
            let buffersize = buffer.capacity() as c_int;
            let res = libc::proc_pidinfo(
                pid.0 as _,
                T::FLAVOR as c_int,
                0,
                buffer.as_mut_ptr() as *mut c_void,
                buffersize,
            );
            // We don't know the expected count, so we keep trying until we get less bytes
            // than the buffer size.
            if res == buffersize {
                buffer.reserve(buffer.capacity() * 2);
                continue;
            }
            if res < 0 {
                return Err(std::io::Error::from_raw_os_error(res));
            }
            if res as usize % std::mem::size_of::<T>() != 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unexpected buffer size",
                ));
            }
            buffer.set_len(res as usize / std::mem::size_of::<T>());
            return Ok(buffer);
        }
    }
}

/// Get an info struct for the current process. A convenience function that calls
/// [`proc_pidinfo_list`] with the current process ID.
#[allow(private_bounds)]
pub fn proc_pidinfo_list_self<T: HasFlavorList>() -> Result<Vec<T>, std::io::Error> {
    proc_pidinfo_list(getpid())
}

/// General information about a file descriptor. See [`VnodeFdInfo`]
/// or [`VnodeFdInfoWithPath`] for more specific information.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProcFileInfo {
    pub fi_openflags: u32,
    pub fi_status: u32,
    pub fi_offset: libc::off_t,
    pub fi_type: i32,
    pub fi_guardflags: u32,
}

/// General information about a vnode. See [`VnodeFdInfo`],
/// [`VnodeFdInfoWithPath`], or [`VnodeInfoPath`] for more specific information.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VInfoStat {
    pub vst_dev: u32,
    pub vst_mode: u16,
    pub vst_nlink: u16,
    pub vst_ino: u64,
    pub vst_uid: libc::uid_t,
    pub vst_gid: libc::gid_t,
    pub vst_atime: i64,
    pub vst_atimensec: i64,
    pub vst_mtime: i64,
    pub vst_mtimensec: i64,
    pub vst_ctime: i64,
    pub vst_ctimensec: i64,
    pub vst_birthtime: i64,
    pub vst_birthtimensec: i64,
    pub vst_size: libc::off_t,
    pub vst_blocks: i64,
    pub vst_blksize: i32,
    pub vst_flags: u32,
    pub vst_gen: u32,
    pub vst_rdev: u32,
    pub vst_qspare: [i64; 2],
}

/// General information about a vnode. See [`VnodeFdInfo`] or [`VnodeFdInfoWithPath`]
/// for more specific information.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VnodeInfo {
    pub vi_stat: VInfoStat,
    pub vi_type: c_int,
    pub vi_pad: c_int,
    pub vi_fsid: [i32; 2],
}

/// Path information about a vnode. See [`VnodeFdInfoWithPath`] for more specific information.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VnodeInfoPath {
    pub vip_vi: VnodeInfo,
    pub vip_path: [c_char; libc::MAXPATHLEN as usize],
}

impl VnodeInfoPath {
    pub fn path(&self) -> Result<&Path, ValueError> {
        libc_str_to_path(&self.vip_path)
    }
}

/// Information about [`ProcFDType::VNODE`] file descriptors.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VnodeFdInfo {
    pub pfi: ProcFileInfo,
    pub pvi: VnodeInfo,
}

impl HasFdFlavor for VnodeFdInfo {
    const FLAVOR: ProcPidFdInfoFlavor = ProcPidFdInfoFlavor::PROC_PIDFDVNODEINFO;
}

/// Information about [`ProcFDType::VNODE`] file descriptors.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VnodeFdInfoWithPath {
    pub pfi: ProcFileInfo,
    pub pvip: VnodeInfoPath,
}

impl VnodeFdInfoWithPath {
    pub fn path(&self) -> Result<&Path, ValueError> {
        self.pvip.path()
    }
}

impl HasFdFlavor for VnodeFdInfoWithPath {
    const FLAVOR: ProcPidFdInfoFlavor = ProcPidFdInfoFlavor::PROC_PIDFDVNODEPATHINFO;
}

/// General information about a pipe. See [`PipeFdInfo`] for more specific information.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PipeInfo {
    pub pipe_stat: VInfoStat,
    pub pipe_handle: u64,
    pub pipe_peerhandle: u64,
    pub pipe_status: c_int,
    pub rfu_1: c_int,
}

/// Information about [`ProcFDType::PIPE`] file descriptors.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PipeFdInfo {
    pub pfi: ProcFileInfo,
    pub pipe_info: PipeInfo,
}

impl HasFdFlavor for PipeFdInfo {
    const FLAVOR: ProcPidFdInfoFlavor = ProcPidFdInfoFlavor::PROC_PIDFDPIPEINFO;
}

/// Get an info struct for a given process and file descriptor.
///
/// ```
/// use proc_pidinfo::*;
///
/// # let pid = getpid();
/// for fd in proc_pidinfo_list::<ProcFDInfo>(pid).unwrap() {
///     println!("{:?}", fd);
///     if fd.fd_type() == Ok(ProcFDType::VNODE) {
///         let vnode = proc_pidfdinfo::<VnodeFdInfo>(pid, fd.proc_fd).unwrap().unwrap();
///         println!("Vnode: {:?}", vnode);
///         if let Some(vnode) = proc_pidfdinfo::<VnodeFdInfoWithPath>(pid, fd.proc_fd).unwrap() {
///             println!("Path: {:?}", vnode.path().unwrap());
///         }
///     } else if fd.fd_type() == Ok(ProcFDType::PIPE) {
///         let pipe = proc_pidfdinfo::<PipeFdInfo>(pid, fd.proc_fd).unwrap().unwrap();
///         println!("Pipe: {:?}", pipe);
///     } else {
///         println!("Unknown fd type: {:?}", fd.fd_type().unwrap_err());
///     }
/// }
/// ```
#[allow(private_bounds)]
pub fn proc_pidfdinfo<T: HasFdFlavor>(pid: Pid, fd: Fd) -> Result<Option<T>, std::io::Error> {
    unsafe {
        let mut value = std::mem::MaybeUninit::<T>::uninit();
        let buffersize = std::mem::size_of::<T>() as c_int;
        let res = libc::proc_pidfdinfo(
            pid.0 as _,
            fd.0,
            T::FLAVOR as c_int,
            value.as_mut_ptr() as *mut c_void,
            buffersize,
        );
        if res < 0 {
            return Err(std::io::Error::from_raw_os_error(res));
        }
        if res == 0 {
            return Ok(None);
        }
        if res != buffersize {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unexpected buffer size {res} != {buffersize}"),
            ));
        }
        Ok(Some(value.assume_init()))
    }
}

/// Get an info struct for the current process.
///
/// This is a convenience function that calls [`proc_pidfdinfo`] with the current process ID.
#[allow(private_bounds)]
pub fn proc_pidfdinfo_self<T: HasFdFlavor>(fd: Fd) -> Result<Option<T>, std::io::Error> {
    proc_pidfdinfo(getpid(), fd)
}

/// Get an info struct for a given process and fileport.
///
/// ```
/// use proc_pidinfo::*;
///
/// # let pid = getpid();
/// for port in proc_pidinfo_list::<ProcFilePortInfo>(pid).unwrap() {
///     println!("{:?}", port);
///     if port.fd_type() == Ok(ProcFDType::VNODE) {
///         let vnode = proc_pidfileportinfo::<VnodeFdInfo>(pid, port.proc_fileport).unwrap().unwrap();
///         println!("Vnode: {:?}", vnode);
///         if let Some(vnode) = proc_pidfileportinfo::<VnodeFdInfoWithPath>(pid, port.proc_fileport).unwrap() {
///             println!("Path: {:?}", vnode.path().unwrap());
///         }
///     } else if port.fd_type() == Ok(ProcFDType::PIPE) {
///         let pipe = proc_pidfileportinfo::<PipeFdInfo>(pid, port.proc_fileport).unwrap().unwrap();
///         println!("Pipe: {:?}", pipe);
///     } else {
///         println!("Unknown fd type: {:?}", port.fd_type().unwrap_err());
///     }
/// }
/// ```
#[allow(private_bounds)]
pub fn proc_pidfileportinfo<T: HasFdFlavor>(
    pid: Pid,
    fileport: FilePort,
) -> Result<Option<T>, std::io::Error> {
    unsafe {
        let mut value = std::mem::MaybeUninit::<T>::uninit();
        let buffersize = std::mem::size_of::<T>() as c_int;
        let res = libc::proc_pidfileportinfo(
            pid.0 as _,
            fileport.0,
            T::FLAVOR as c_int,
            value.as_mut_ptr() as *mut c_void,
            buffersize,
        );
        if res < 0 {
            return Err(std::io::Error::from_raw_os_error(res));
        }
        if res == 0 {
            return Ok(None);
        }
        if res != buffersize {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unexpected buffer size {res} != {buffersize}"),
            ));
        }
        Ok(Some(value.assume_init()))
    }
}

/// Get an info struct for the current process.
///
/// This is a convenience function that calls [`proc_pidfdinfo`] with the current process ID.
#[allow(private_bounds)]
pub fn proc_pidfileportinfo_self<T: HasFdFlavor>(
    fileport: FilePort,
) -> Result<Option<T>, std::io::Error> {
    proc_pidfileportinfo(getpid(), fileport)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proc_pidinfo_pid_zero() {
        let result = proc_pidinfo_list::<ProcFDInfo>(Pid(0)).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_proc_pidinfo_fileport_zero() {
        let result = proc_pidinfo_list::<ProcFilePortInfo>(Pid(0)).unwrap();
        assert!(result.is_empty());
    }

    // This will work if launchd is process 1 and you run w/sudo
    #[test]
    fn test_proc_pidinfo_fileport_one() {
        let result = proc_pidinfo_list::<ProcFilePortInfo>(Pid(1)).unwrap();
        for port in result {
            println!("{:?}", port);
            if port.fd_type() == Ok(ProcFDType::VNODE) {
                let vnode = proc_pidfileportinfo::<VnodeFdInfo>(Pid(1), port.proc_fileport)
                    .unwrap()
                    .unwrap();
                println!("{:?}", vnode);
            }
        }
    }

    #[test]
    fn test_proc_pidinfo_self() {
        let result = proc_pidinfo_list_self::<ProcFDInfo>().unwrap();
        for fd in result {
            if fd.fd_type() == Ok(ProcFDType::VNODE) {
                let vnode = proc_pidfdinfo_self::<VnodeFdInfo>(fd.proc_fd)
                    .unwrap()
                    .unwrap();
                println!("{:?}", vnode);
                if let Some(vnode) = proc_pidfdinfo_self::<VnodeFdInfoWithPath>(fd.proc_fd).unwrap()
                {
                    println!("Path: {:?}", vnode.path().unwrap());
                }
            } else {
                let res = proc_pidfdinfo_self::<VnodeFdInfo>(fd.proc_fd).unwrap();
                assert!(res.is_none());
            }
        }
    }

    #[test]
    fn test_proc_pidinfo_fileport_self() {
        let result = proc_pidinfo_list_self::<ProcFilePortInfo>().unwrap();
        for port in result {
            if port.fd_type() == Ok(ProcFDType::VNODE) {
                let vnode = proc_pidfileportinfo_self::<VnodeFdInfo>(port.proc_fileport)
                    .unwrap()
                    .unwrap();
                println!("{:?}", vnode);
                if let Some(vnode) =
                    proc_pidfileportinfo_self::<VnodeFdInfoWithPath>(port.proc_fileport).unwrap()
                {
                    println!("Path: {:?}", vnode.path().unwrap());
                }
            } else {
                let res = proc_pidfileportinfo_self::<VnodeFdInfo>(port.proc_fileport).unwrap();
                assert!(res.is_none());
            }
        }
    }

    #[test]
    fn test_proc_task_info_self() {
        let result = proc_pidinfo_self::<ProcTaskAllInfo>().unwrap().unwrap();
        assert_eq!(result.pbsd.pbi_pid, getpid());
        println!("{:?}", result);
    }

    #[test]
    fn test_proc_task_info_short_self() {
        let result = proc_pidinfo_self::<ProcBSDShortInfo>().unwrap().unwrap();
        assert_eq!(result.pbsi_pid, getpid());
        println!("{:?}", result);
    }

    #[test]
    fn test_proc_task_info_short_zero() {
        let result = proc_pidinfo::<ProcBSDShortInfo>(Pid(0)).unwrap().unwrap();
        assert_eq!(result.pbsi_pid, Pid(0));
        println!("{:?}", result);
        println!("{}", result.comm().unwrap());
    }
}
