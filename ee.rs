#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::ee_lib::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _win_st;
    fn clearok(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn delwin(_: *mut WINDOW) -> libc::c_int;
    fn echo() -> libc::c_int;
    fn endwin() -> libc::c_int;
    fn idlok(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn initscr() -> *mut WINDOW;
    fn keypad(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn newwin(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut WINDOW;
    fn nl() -> libc::c_int;
    fn noecho() -> libc::c_int;
    fn nonl() -> libc::c_int;
    fn noraw() -> libc::c_int;
    fn raw() -> libc::c_int;
    fn resetty() -> libc::c_int;
    fn reset_prog_mode() -> libc::c_int;
    fn savetty() -> libc::c_int;
    fn waddch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    fn waddnstr(_: *mut WINDOW, _: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn wattrset(_: *mut WINDOW, _: libc::c_int) -> libc::c_int;
    fn wclrtobot(_: *mut WINDOW) -> libc::c_int;
    fn wclrtoeol(_: *mut WINDOW) -> libc::c_int;
    fn werase(_: *mut WINDOW) -> libc::c_int;
    fn wgetch(_: *mut WINDOW) -> libc::c_int;
    fn winsdelln(_: *mut WINDOW, _: libc::c_int) -> libc::c_int;
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wnoutrefresh(_: *mut WINDOW) -> libc::c_int;
    fn wprintw(_: *mut WINDOW, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn wrefresh(_: *mut WINDOW) -> libc::c_int;
    fn wtouchln(
        _: *mut WINDOW,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn getmaxy(_: *const WINDOW) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn getchar() -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stdscr: *mut WINDOW;
    static mut COLS: libc::c_int;
    static mut LINES: libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn toupper(_: libc::c_int) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getuid() -> __uid_t;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn catopen(__cat_name: *const libc::c_char, __flag: libc::c_int) -> nl_catd;
    fn catgets(
        __catalog: nl_catd,
        __set: libc::c_int,
        __number: libc::c_int,
        __string: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn catclose(__catalog: nl_catd) -> libc::c_int;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type chtype = libc::c_uint;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type WINDOW = _win_st;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub type nl_catd = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text {
    pub line: *mut libc::c_uchar,
    pub line_number: libc::c_int,
    pub line_length: libc::c_int,
    pub max_length: libc::c_int,
    pub next_line: *mut text,
    pub prev_line: *mut text,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct files {
    pub name: *mut libc::c_uchar,
    pub next_name: *mut files,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu_entries {
    pub item_string: *mut libc::c_char,
    pub procedure: Option::<unsafe extern "C" fn(*mut menu_entries) -> libc::c_int>,
    pub ptr_argument: *mut menu_entries,
    pub iprocedure: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub nprocedure: Option::<unsafe extern "C" fn() -> ()>,
    pub argument: libc::c_int,
}
#[no_mangle]
pub static mut ee_copyright_message: *mut libc::c_char = b"Copyright (c) 1986, 1990, 1991, 1992, 1993, 1994, 1995, 1996, 2009 Hugh Mahon \0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut version: *mut libc::c_char = b"@(#) ee, version 1.5.2 $Revision: 1.104 $\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut catalog: nl_catd = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut first_line: *mut text = 0 as *const text as *mut text;
#[no_mangle]
pub static mut dlt_line: *mut text = 0 as *const text as *mut text;
#[no_mangle]
pub static mut curr_line: *mut text = 0 as *const text as *mut text;
#[no_mangle]
pub static mut tmp_line: *mut text = 0 as *const text as *mut text;
#[no_mangle]
pub static mut srch_line: *mut text = 0 as *const text as *mut text;
#[no_mangle]
pub static mut top_of_stack: *mut files = 0 as *const files as *mut files;
#[no_mangle]
pub static mut d_wrd_len: libc::c_int = 0;
#[no_mangle]
pub static mut position: libc::c_int = 0;
#[no_mangle]
pub static mut scr_pos: libc::c_int = 0;
#[no_mangle]
pub static mut scr_vert: libc::c_int = 0;
#[no_mangle]
pub static mut scr_horz: libc::c_int = 0;
#[no_mangle]
pub static mut absolute_lin: libc::c_int = 0;
#[no_mangle]
pub static mut tmp_vert: libc::c_int = 0;
#[no_mangle]
pub static mut tmp_horz: libc::c_int = 0;
#[no_mangle]
pub static mut input_file: libc::c_int = 0;
#[no_mangle]
pub static mut recv_file: libc::c_int = 0;
#[no_mangle]
pub static mut edit: libc::c_int = 0;
#[no_mangle]
pub static mut gold: libc::c_int = 0;
#[no_mangle]
pub static mut fildes: libc::c_int = 0;
#[no_mangle]
pub static mut case_sen: libc::c_int = 0;
#[no_mangle]
pub static mut last_line: libc::c_int = 0;
#[no_mangle]
pub static mut last_col: libc::c_int = 0;
#[no_mangle]
pub static mut horiz_offset: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut clear_com_win: libc::c_int = 0;
#[no_mangle]
pub static mut text_changes: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut get_fd: libc::c_int = 0;
#[no_mangle]
pub static mut info_window: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut info_type: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut expand_tabs: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut right_margin: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut observ_margins: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut shell_fork: libc::c_int = 0;
#[no_mangle]
pub static mut temp_stdin: libc::c_int = 0;
#[no_mangle]
pub static mut temp_stdout: libc::c_int = 0;
#[no_mangle]
pub static mut temp_stderr: libc::c_int = 0;
#[no_mangle]
pub static mut pipe_out: [libc::c_int; 2] = [0; 2];
#[no_mangle]
pub static mut pipe_in: [libc::c_int; 2] = [0; 2];
#[no_mangle]
pub static mut out_pipe: libc::c_int = 0;
#[no_mangle]
pub static mut in_pipe: libc::c_int = 0;
#[no_mangle]
pub static mut formatted: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut auto_format: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut restricted: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut nohighlight: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut eightbit: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut local_LINES: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut local_COLS: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut curses_initialized: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut emacs_keys_mode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut ee_chinese: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut point: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut srch_str: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut u_srch_str: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut srch_1: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut srch_2: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut srch_3: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut in_file_name: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut tmp_file: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut d_char: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut d_word: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut d_line: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut in_string: [libc::c_char; 513] = [0; 513];
#[no_mangle]
pub static mut print_command: *mut libc::c_uchar = b"lpr\0" as *const u8
    as *const libc::c_char as *mut libc::c_uchar;
#[no_mangle]
pub static mut start_at_line: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[export_name = "in"]
pub static mut in_0: libc::c_int = 0;
#[no_mangle]
pub static mut temp_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut bit_bucket: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut table: [*mut libc::c_char; 32] = [
    b"^@\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^A\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^B\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^C\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^D\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^E\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^F\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^H\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^J\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^K\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^M\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^N\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^O\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^R\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^S\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^T\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^U\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^V\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^W\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^X\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^Y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^Z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^[\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^\\\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^^\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"^_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut com_win: *mut WINDOW = 0 as *const WINDOW as *mut WINDOW;
#[no_mangle]
pub static mut text_win: *mut WINDOW = 0 as *const WINDOW as *mut WINDOW;
#[no_mangle]
pub static mut help_win: *mut WINDOW = 0 as *const WINDOW as *mut WINDOW;
#[no_mangle]
pub static mut info_win: *mut WINDOW = 0 as *const WINDOW as *mut WINDOW;
#[no_mangle]
pub static mut modes_menu: [menu_entries; 12] = unsafe {
    [
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: Some(dump_ee_conf as unsafe extern "C" fn() -> ()),
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: 0 as *const libc::c_char as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut mode_strings: [*mut libc::c_char; 11] = [0 as *const libc::c_char
    as *mut libc::c_char; 11];
#[no_mangle]
pub static mut config_dump_menu: [menu_entries; 4] = [
    {
        let mut init = menu_entries {
            item_string: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            procedure: None,
            ptr_argument: 0 as *const menu_entries as *mut menu_entries,
            iprocedure: None,
            nprocedure: None,
            argument: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = menu_entries {
            item_string: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            procedure: None,
            ptr_argument: 0 as *const menu_entries as *mut menu_entries,
            iprocedure: None,
            nprocedure: None,
            argument: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = menu_entries {
            item_string: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            procedure: None,
            ptr_argument: 0 as *const menu_entries as *mut menu_entries,
            iprocedure: None,
            nprocedure: None,
            argument: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = menu_entries {
            item_string: 0 as *const libc::c_char as *mut libc::c_char,
            procedure: None,
            ptr_argument: 0 as *const menu_entries as *mut menu_entries,
            iprocedure: None,
            nprocedure: None,
            argument: -(1 as libc::c_int),
        };
        init
    },
];
#[no_mangle]
pub static mut leave_menu: [menu_entries; 4] = unsafe {
    [
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: Some(finish as unsafe extern "C" fn() -> ()),
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: Some(
                    quit as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
                ),
                nprocedure: None,
                argument: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: 0 as *const libc::c_char as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut file_menu: [menu_entries; 6] = unsafe {
    [
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: Some(
                    file_op as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
                ),
                nprocedure: None,
                argument: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: Some(
                    file_op as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
                ),
                nprocedure: None,
                argument: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: Some(
                    file_op as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
                ),
                nprocedure: None,
                argument: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: Some(print_buffer as unsafe extern "C" fn() -> ()),
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: 0 as *const libc::c_char as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut search_menu: [menu_entries; 4] = unsafe {
    [
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: Some(search_prompt as unsafe extern "C" fn() -> ()),
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: Some(
                    search as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
                ),
                nprocedure: None,
                argument: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: 0 as *const libc::c_char as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut spell_menu: [menu_entries; 4] = unsafe {
    [
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: Some(spell_op as unsafe extern "C" fn() -> ()),
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: Some(ispell_op as unsafe extern "C" fn() -> ()),
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: 0 as *const libc::c_char as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut misc_menu: [menu_entries; 5] = unsafe {
    [
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: Some(Format as unsafe extern "C" fn() -> ()),
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: Some(shell_op as unsafe extern "C" fn() -> ()),
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: Some(
                    menu_op as unsafe extern "C" fn(*mut menu_entries) -> libc::c_int,
                ),
                ptr_argument: spell_menu.as_ptr() as *mut _,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: 0 as *const libc::c_char as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut main_menu: [menu_entries; 9] = unsafe {
    [
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: Some(leave_op as unsafe extern "C" fn() -> ()),
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: Some(help as unsafe extern "C" fn() -> ()),
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: Some(
                    menu_op as unsafe extern "C" fn(*mut menu_entries) -> libc::c_int,
                ),
                ptr_argument: file_menu.as_ptr() as *mut _,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: Some(redraw as unsafe extern "C" fn() -> ()),
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: Some(modes_op as unsafe extern "C" fn() -> ()),
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: Some(
                    menu_op as unsafe extern "C" fn(*mut menu_entries) -> libc::c_int,
                ),
                ptr_argument: search_menu.as_ptr() as *mut _,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                procedure: Some(
                    menu_op as unsafe extern "C" fn(*mut menu_entries) -> libc::c_int,
                ),
                ptr_argument: misc_menu.as_ptr() as *mut _,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = menu_entries {
                item_string: 0 as *const libc::c_char as *mut libc::c_char,
                procedure: None,
                ptr_argument: 0 as *const menu_entries as *mut menu_entries,
                iprocedure: None,
                nprocedure: None,
                argument: -(1 as libc::c_int),
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut help_text: [*mut libc::c_char; 23] = [0 as *const libc::c_char
    as *mut libc::c_char; 23];
#[no_mangle]
pub static mut control_keys: [*mut libc::c_char; 5] = [0 as *const libc::c_char
    as *mut libc::c_char; 5];
#[no_mangle]
pub static mut emacs_help_text: [*mut libc::c_char; 22] = [0 as *const libc::c_char
    as *mut libc::c_char; 22];
#[no_mangle]
pub static mut emacs_control_keys: [*mut libc::c_char; 5] = [0 as *const libc::c_char
    as *mut libc::c_char; 5];
#[no_mangle]
pub static mut command_strings: [*mut libc::c_char; 5] = [0 as *const libc::c_char
    as *mut libc::c_char; 5];
#[no_mangle]
pub static mut commands: [*mut libc::c_char; 32] = [0 as *const libc::c_char
    as *mut libc::c_char; 32];
#[no_mangle]
pub static mut init_strings: [*mut libc::c_char; 22] = [0 as *const libc::c_char
    as *mut libc::c_char; 22];
#[no_mangle]
pub static mut com_win_message: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut no_file_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut ascii_code_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut printer_msg_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut command_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut file_write_prompt_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut file_read_prompt_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut char_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut unkn_cmd_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut non_unique_cmd_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut line_num_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut line_len_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut current_file_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut usage0: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut usage1: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut usage2: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut usage3: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut usage4: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut file_is_dir_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut new_file_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut cant_open_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut open_file_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut file_read_fin_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut reading_file_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut read_only_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut file_read_lines_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut save_file_name_prompt: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut file_not_saved_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut changes_made_prompt: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut yes_char: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut file_exists_prompt: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut create_file_fail_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut writing_file_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut file_written_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut searching_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut str_not_found_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut search_prompt_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut exec_err_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut continue_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut menu_cancel_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut menu_size_err_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut press_any_key_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut shell_prompt: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut formatting_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut shell_echo_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut spell_in_prog_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut margin_prompt: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut restricted_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut ON: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut OFF: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut HELP: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut WRITE: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut READ: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut LINE: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut FILE_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut CHARACTER: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut REDRAW: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut RESEQUENCE: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut AUTHOR: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut VERSION: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut CASE: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut NOCASE: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut EXPAND: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut NOEXPAND: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut Exit_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut QUIT_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut INFO: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut NOINFO: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut MARGINS: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut NOMARGINS: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut AUTOFORMAT: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut NOAUTOFORMAT: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut Echo: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut PRINTCOMMAND: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut RIGHTMARGIN: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut HIGHLIGHT: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut NOHIGHLIGHT: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut EIGHTBIT: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut NOEIGHTBIT: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut EMACS_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut NOEMACS_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut conf_dump_err_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut conf_dump_success_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut conf_not_saved_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut ree_no_file_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut cancel_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut menu_too_lrg_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut more_above_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut more_below_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut separator: *mut libc::c_char = b"===============================================================================\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut chinese_cmd: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut nochinese_cmd: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut counter: libc::c_int = 0;
    counter = 1 as libc::c_int;
    while counter < 24 as libc::c_int {
        signal(
            counter,
            ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        counter += 1;
        counter;
    }
    if isatty(0 as libc::c_int) == 0 || isatty(1 as libc::c_int) == 0 {
        fprintf(
            stderr,
            b"ee's standard input and output must be a terminal\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    signal(17 as libc::c_int, None);
    signal(11 as libc::c_int, None);
    signal(
        2 as libc::c_int,
        Some(edit_abort as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    d_char = malloc(3 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    d_word = malloc(150 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    *d_word = '\0' as i32 as libc::c_uchar;
    d_line = 0 as *mut libc::c_uchar;
    dlt_line = txtalloc();
    (*dlt_line).line = d_line;
    (*dlt_line).line_length = 0 as libc::c_int;
    first_line = txtalloc();
    curr_line = first_line;
    point = malloc(10 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    (*curr_line).line = point;
    (*curr_line).line_length = 1 as libc::c_int;
    (*curr_line).max_length = 10 as libc::c_int;
    (*curr_line).prev_line = 0 as *mut text;
    (*curr_line).next_line = 0 as *mut text;
    (*curr_line).line_number = 1 as libc::c_int;
    srch_str = 0 as *mut libc::c_uchar;
    u_srch_str = 0 as *mut libc::c_uchar;
    position = 1 as libc::c_int;
    scr_pos = 0 as libc::c_int;
    scr_vert = 0 as libc::c_int;
    scr_horz = 0 as libc::c_int;
    absolute_lin = 1 as libc::c_int;
    bit_bucket = fopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    edit = 1 as libc::c_int;
    case_sen = 0 as libc::c_int;
    gold = case_sen;
    shell_fork = 1 as libc::c_int;
    strings_init();
    ee_init();
    if argc > 0 as libc::c_int {
        get_options(argc, argv);
    }
    set_up_term();
    if right_margin == 0 as libc::c_int {
        right_margin = COLS - 1 as libc::c_int;
    }
    if top_of_stack.is_null() {
        if restrict_mode() != 0 {
            wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
            werase(com_win);
            wprintw(
                com_win,
                b"%s\0" as *const u8 as *const libc::c_char,
                ree_no_file_msg,
            );
            wrefresh(com_win);
            edit_abort(0 as libc::c_int);
        }
        wprintw(com_win, b"%s\0" as *const u8 as *const libc::c_char, no_file_string);
        wrefresh(com_win);
    } else {
        check_fp();
    }
    clear_com_win = 1 as libc::c_int;
    counter = 0 as libc::c_int;
    while edit != 0 {
        if info_window != 0 {
            if nohighlight == 0 {
                wattrset(
                    info_win,
                    ((1 as libc::c_uint) << 8 as libc::c_int + 8 as libc::c_int)
                        as libc::c_int,
                );
            }
            wmove(info_win, 5 as libc::c_int, 0 as libc::c_int);
            wprintw(info_win, b"%s\0" as *const u8 as *const libc::c_char, separator);
            wmove(info_win, 5 as libc::c_int, 5 as libc::c_int);
            wprintw(
                info_win,
                b"line %d col %d lines from top %d \0" as *const u8
                    as *const libc::c_char,
                (*curr_line).line_number,
                scr_horz,
                absolute_lin,
            );
            wattrset(
                info_win,
                (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint) as libc::c_int,
            );
            wrefresh(info_win);
        }
        wrefresh(text_win);
        in_0 = wgetch(text_win);
        if in_0 == -(1 as libc::c_int) {
            exit(0 as libc::c_int);
        }
        resize_check();
        if clear_com_win != 0 {
            clear_com_win = 0 as libc::c_int;
            wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
            werase(com_win);
            if info_window == 0 {
                wprintw(
                    com_win,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    com_win_message,
                );
            }
            wrefresh(com_win);
        }
        if in_0 > 255 as libc::c_int {
            function_key();
        } else if in_0 == '\u{8}' as i32 || in_0 == 127 as libc::c_int {
            in_0 = 8 as libc::c_int;
            delete(1 as libc::c_int);
        } else if in_0 > 31 as libc::c_int || in_0 == 9 as libc::c_int {
            insert(in_0);
        } else if in_0 >= 0 as libc::c_int && in_0 <= 31 as libc::c_int {
            if emacs_keys_mode != 0 {
                emacs_control();
            } else {
                control();
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn resiz_line(
    mut factor: libc::c_int,
    mut rline: *mut text,
    mut rpos: libc::c_int,
) -> *mut libc::c_uchar {
    let mut rpoint: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut resiz_var: libc::c_int = 0;
    (*rline).max_length += factor;
    (*rline)
        .line = realloc(
        (*rline).line as *mut libc::c_void,
        (*rline).max_length as libc::c_ulong,
    ) as *mut libc::c_uchar;
    rpoint = (*rline).line;
    resiz_var = 1 as libc::c_int;
    while resiz_var < rpos {
        rpoint = rpoint.offset(1);
        rpoint;
        resiz_var += 1;
        resiz_var;
    }
    return rpoint;
}
#[no_mangle]
pub unsafe extern "C" fn insert(mut character: libc::c_int) {
    let mut counter: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut temp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if character == '\t' as i32 && expand_tabs != 0 {
        counter = len_char('\t' as i32, scr_horz);
        while counter > 0 as libc::c_int {
            insert(' ' as i32);
            counter -= 1;
            counter;
        }
        if auto_format != 0 {
            Auto_Format();
        }
        return;
    }
    text_changes = 1 as libc::c_int;
    if (*curr_line).max_length - (*curr_line).line_length < 5 as libc::c_int {
        point = resiz_line(10 as libc::c_int, curr_line, position);
    }
    (*curr_line).line_length += 1;
    (*curr_line).line_length;
    temp = point;
    counter = position;
    while counter < (*curr_line).line_length {
        counter += 1;
        counter;
        temp = temp.offset(1);
        temp;
    }
    temp = temp.offset(1);
    temp;
    while point < temp {
        temp2 = temp.offset(-(1 as libc::c_int as isize));
        *temp = *temp2;
        temp = temp.offset(-1);
        temp;
    }
    *point = character as libc::c_uchar;
    wclrtoeol(text_win);
    if *(*__ctype_b_loc()).offset(character as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        scr_horz += out_char(text_win, character, scr_horz);
        scr_pos = scr_horz;
        point = point.offset(1);
        point;
        position += 1;
        position;
    } else {
        waddch(text_win, character as libc::c_uchar as chtype);
        scr_horz += 1;
        scr_pos = scr_horz;
        point = point.offset(1);
        point;
        position += 1;
        position;
    }
    if observ_margins != 0 && right_margin < scr_pos {
        counter = position;
        while scr_pos > right_margin {
            prev_word();
        }
        if scr_pos == 0 as libc::c_int {
            while position < counter {
                right(1 as libc::c_int);
            }
        } else {
            counter -= position;
            insert_line(1 as libc::c_int);
            value = 0 as libc::c_int;
            while value < counter {
                right(1 as libc::c_int);
                value += 1;
                value;
            }
        }
    }
    if scr_horz - horiz_offset > last_col {
        horiz_offset += 8 as libc::c_int;
        midscreen(scr_vert, point);
    }
    if auto_format != 0 && character == ' ' as i32 && formatted == 0 {
        Auto_Format();
    } else if character != ' ' as i32 && character != '\t' as i32 {
        formatted = 0 as libc::c_int;
    }
    draw_line(scr_vert, scr_horz, point, position, (*curr_line).line_length);
}
#[no_mangle]
pub unsafe extern "C" fn delete(mut disp: libc::c_int) {
    let mut tp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp_buff: *mut text = 0 as *mut text;
    let mut temp_vert: libc::c_int = 0;
    let mut temp_pos: libc::c_int = 0;
    let mut del_width: libc::c_int = 1 as libc::c_int;
    if point != (*curr_line).line {
        text_changes = 1 as libc::c_int;
        tp = point;
        temp2 = tp;
        if ee_chinese != 0 && position >= 2 as libc::c_int
            && *point.offset(-(2 as libc::c_int as isize)) as libc::c_int
                > 127 as libc::c_int
        {
            del_width = 2 as libc::c_int;
        }
        tp = tp.offset(-(del_width as isize));
        point = point.offset(-(del_width as isize));
        position -= del_width;
        temp_pos = position;
        (*curr_line).line_length -= del_width;
        if (*tp as libc::c_int) < ' ' as i32 || *tp as libc::c_int >= 127 as libc::c_int
        {
            scanline(tp);
        } else {
            scr_horz -= del_width;
        }
        scr_pos = scr_horz;
        if in_0 == 8 as libc::c_int {
            if del_width == 1 as libc::c_int {
                *d_char = *point;
            } else {
                *d_char.offset(0 as libc::c_int as isize) = *point;
                *d_char
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *point.offset(1 as libc::c_int as isize);
            }
            *d_char.offset(del_width as isize) = '\0' as i32 as libc::c_uchar;
        }
        while temp_pos <= (*curr_line).line_length {
            temp_pos += 1;
            temp_pos;
            *tp = *temp2;
            tp = tp.offset(1);
            tp;
            temp2 = temp2.offset(1);
            temp2;
        }
        if scr_horz < horiz_offset && horiz_offset > 0 as libc::c_int {
            horiz_offset -= 8 as libc::c_int;
            midscreen(scr_vert, point);
        }
    } else if !((*curr_line).prev_line).is_null() {
        text_changes = 1 as libc::c_int;
        left(disp);
        temp_buff = (*curr_line).next_line;
        point = resiz_line((*temp_buff).line_length, curr_line, position);
        if !((*temp_buff).next_line).is_null() {
            (*(*temp_buff).next_line).prev_line = curr_line;
        }
        (*curr_line).next_line = (*temp_buff).next_line;
        temp2 = (*temp_buff).line;
        if in_0 == 8 as libc::c_int {
            *d_char.offset(0 as libc::c_int as isize) = '\n' as i32 as libc::c_uchar;
            *d_char.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_uchar;
        }
        tp = point;
        temp_pos = 1 as libc::c_int;
        while temp_pos < (*temp_buff).line_length {
            (*curr_line).line_length += 1;
            (*curr_line).line_length;
            temp_pos += 1;
            temp_pos;
            *tp = *temp2;
            tp = tp.offset(1);
            tp;
            temp2 = temp2.offset(1);
            temp2;
        }
        *tp = '\0' as i32 as libc::c_uchar;
        free((*temp_buff).line as *mut libc::c_void);
        free(temp_buff as *mut libc::c_void);
        temp_buff = curr_line;
        temp_vert = scr_vert;
        scr_pos = scr_horz;
        if scr_vert < last_line {
            wmove(text_win, scr_vert + 1 as libc::c_int, 0 as libc::c_int);
            winsdelln(text_win, -(1 as libc::c_int));
        }
        while !temp_buff.is_null() && temp_vert < last_line {
            temp_buff = (*temp_buff).next_line;
            temp_vert += 1;
            temp_vert;
        }
        if temp_vert == last_line && !temp_buff.is_null() {
            tp = (*temp_buff).line;
            wmove(text_win, last_line, 0 as libc::c_int);
            wclrtobot(text_win);
            draw_line(
                last_line,
                0 as libc::c_int,
                tp,
                1 as libc::c_int,
                (*temp_buff).line_length,
            );
            wmove(text_win, scr_vert, scr_horz - horiz_offset);
        }
    }
    draw_line(scr_vert, scr_horz, point, position, (*curr_line).line_length);
    formatted = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scanline(mut pos: *mut libc::c_uchar) {
    let mut temp: libc::c_int = 0;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    ptr = (*curr_line).line;
    temp = 0 as libc::c_int;
    while ptr < pos {
        if *ptr as libc::c_int <= 8 as libc::c_int {
            temp += 2 as libc::c_int;
        } else if *ptr as libc::c_int == 9 as libc::c_int {
            temp += tabshift(temp);
        } else if *ptr as libc::c_int >= 10 as libc::c_int
            && *ptr as libc::c_int <= 31 as libc::c_int
        {
            temp += 2 as libc::c_int;
        } else if *ptr as libc::c_int >= 32 as libc::c_int
            && (*ptr as libc::c_int) < 127 as libc::c_int
        {
            temp += 1;
            temp;
        } else if *ptr as libc::c_int == 127 as libc::c_int {
            temp += 2 as libc::c_int;
        } else if eightbit == 0 {
            temp += 5 as libc::c_int;
        } else {
            temp += 1;
            temp;
        }
        ptr = ptr.offset(1);
        ptr;
    }
    scr_horz = temp;
    if scr_horz - horiz_offset > last_col {
        horiz_offset = scr_horz - scr_horz % 8 as libc::c_int
            - (COLS - 8 as libc::c_int);
        midscreen(scr_vert, point);
    } else if scr_horz < horiz_offset {
        horiz_offset = if 0 as libc::c_int > scr_horz - scr_horz % 8 as libc::c_int {
            0 as libc::c_int
        } else {
            scr_horz - scr_horz % 8 as libc::c_int
        };
        midscreen(scr_vert, point);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tabshift(mut temp_int: libc::c_int) -> libc::c_int {
    let mut leftover: libc::c_int = 0;
    leftover = (temp_int + 1 as libc::c_int) % 8 as libc::c_int;
    if leftover == 0 as libc::c_int {
        return 1 as libc::c_int
    } else {
        return 9 as libc::c_int - leftover
    };
}
#[no_mangle]
pub unsafe extern "C" fn out_char(
    mut window: *mut WINDOW,
    mut character: libc::c_int,
    mut column: libc::c_int,
) -> libc::c_int {
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut string2: [libc::c_char; 8] = [0; 8];
    if character == 9 as libc::c_int {
        i1 = tabshift(column);
        i2 = 0 as libc::c_int;
        while i2 < i1 && column + i2 + 1 as libc::c_int - horiz_offset < last_col {
            waddch(window, ' ' as i32 as chtype);
            i2 += 1;
            i2;
        }
        return i1;
    } else if character >= '\0' as i32 && character < ' ' as i32 {
        string = table[character as usize];
    } else if character < 0 as libc::c_int || character >= 127 as libc::c_int {
        if character == 127 as libc::c_int {
            string = b"^?\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if eightbit == 0 {
            sprintf(
                string2.as_mut_ptr(),
                b"<%d>\0" as *const u8 as *const libc::c_char,
                if character < 0 as libc::c_int {
                    character + 256 as libc::c_int
                } else {
                    character
                },
            );
            string = string2.as_mut_ptr();
        } else {
            waddch(window, character as libc::c_uchar as chtype);
            return 1 as libc::c_int;
        }
    } else {
        waddch(window, character as libc::c_uchar as chtype);
        return 1 as libc::c_int;
    }
    i2 = 0 as libc::c_int;
    while *string.offset(i2 as isize) as libc::c_int != '\0' as i32
        && column + i2 + 1 as libc::c_int - horiz_offset < last_col
    {
        waddch(window, *string.offset(i2 as isize) as libc::c_uchar as chtype);
        i2 += 1;
        i2;
    }
    return strlen(string) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn len_char(
    mut character: libc::c_int,
    mut column: libc::c_int,
) -> libc::c_int {
    let mut length: libc::c_int = 0;
    if character == '\t' as i32 {
        length = tabshift(column);
    } else if character >= 0 as libc::c_int && character < 32 as libc::c_int {
        length = 2 as libc::c_int;
    } else if character >= 32 as libc::c_int && character <= 126 as libc::c_int {
        length = 1 as libc::c_int;
    } else if character == 127 as libc::c_int {
        length = 2 as libc::c_int;
    } else if (character > 126 as libc::c_int || character < 0 as libc::c_int)
        && eightbit == 0
    {
        length = 5 as libc::c_int;
    } else {
        length = 1 as libc::c_int;
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn draw_line(
    mut vertical: libc::c_int,
    mut horiz: libc::c_int,
    mut ptr: *mut libc::c_uchar,
    mut t_pos: libc::c_int,
    mut length: libc::c_int,
) {
    let mut d: libc::c_int = 0;
    let mut temp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut abs_column: libc::c_int = 0;
    let mut column: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut posit: libc::c_int = 0;
    abs_column = horiz;
    column = horiz - horiz_offset;
    row = vertical;
    temp = ptr;
    d = 0 as libc::c_int;
    posit = t_pos;
    if column < 0 as libc::c_int {
        wmove(text_win, row, 0 as libc::c_int);
        wclrtoeol(text_win);
    }
    while column < 0 as libc::c_int {
        d = len_char(*temp as libc::c_int, abs_column);
        abs_column += d;
        column += d;
        posit += 1;
        posit;
        temp = temp.offset(1);
        temp;
    }
    wmove(text_win, row, column);
    wclrtoeol(text_win);
    while posit < length && column <= last_col {
        if *(*__ctype_b_loc()).offset(*temp as libc::c_int as isize) as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            column += len_char(*temp as libc::c_int, abs_column);
            abs_column += out_char(text_win, *temp as libc::c_int, abs_column);
        } else {
            abs_column += 1;
            abs_column;
            column += 1;
            column;
            waddch(text_win, *temp as chtype);
        }
        posit += 1;
        posit;
        temp = temp.offset(1);
        temp;
    }
    if column < last_col {
        wclrtoeol(text_win);
    }
    wmove(text_win, vertical, horiz - horiz_offset);
}
#[no_mangle]
pub unsafe extern "C" fn insert_line(mut disp: libc::c_int) {
    let mut temp_pos: libc::c_int = 0;
    let mut temp_pos2: libc::c_int = 0;
    let mut temp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut extra: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp_nod: *mut text = 0 as *mut text;
    text_changes = 1 as libc::c_int;
    wmove(text_win, scr_vert, scr_horz - horiz_offset);
    wclrtoeol(text_win);
    temp_nod = txtalloc();
    extra = malloc(10 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    (*temp_nod).line = extra;
    (*temp_nod).line_length = 1 as libc::c_int;
    (*temp_nod).max_length = 10 as libc::c_int;
    (*temp_nod).line_number = (*curr_line).line_number + 1 as libc::c_int;
    (*temp_nod).next_line = (*curr_line).next_line;
    if !((*temp_nod).next_line).is_null() {
        (*(*temp_nod).next_line).prev_line = temp_nod;
    }
    (*temp_nod).prev_line = curr_line;
    (*curr_line).next_line = temp_nod;
    temp_pos2 = position;
    temp = point;
    if temp_pos2 < (*curr_line).line_length {
        temp_pos = 1 as libc::c_int;
        while temp_pos2 < (*curr_line).line_length {
            if (*temp_nod).max_length - (*temp_nod).line_length < 5 as libc::c_int {
                extra = resiz_line(10 as libc::c_int, temp_nod, temp_pos);
            }
            (*temp_nod).line_length += 1;
            (*temp_nod).line_length;
            temp_pos += 1;
            temp_pos;
            temp_pos2 += 1;
            temp_pos2;
            *extra = *temp;
            extra = extra.offset(1);
            extra;
            temp = temp.offset(1);
            temp;
        }
        temp = point;
        *temp = '\0' as i32 as libc::c_uchar;
        temp = resiz_line(
            1 as libc::c_int - (*temp_nod).line_length,
            curr_line,
            position,
        );
        (*curr_line)
            .line_length = temp
            .offset(1 as libc::c_int as isize)
            .offset_from((*curr_line).line) as libc::c_long as libc::c_int;
    }
    (*curr_line).line_length = position;
    absolute_lin += 1;
    absolute_lin;
    curr_line = temp_nod;
    *extra = '\0' as i32 as libc::c_uchar;
    position = 1 as libc::c_int;
    point = (*curr_line).line;
    if disp != 0 {
        if scr_vert < last_line {
            scr_vert += 1;
            scr_vert;
            wclrtoeol(text_win);
            wmove(text_win, scr_vert, 0 as libc::c_int);
            winsdelln(text_win, 1 as libc::c_int);
        } else {
            wmove(text_win, 0 as libc::c_int, 0 as libc::c_int);
            winsdelln(text_win, -(1 as libc::c_int));
            wmove(text_win, last_line, 0 as libc::c_int);
            wclrtobot(text_win);
        }
        scr_horz = 0 as libc::c_int;
        scr_pos = scr_horz;
        if horiz_offset != 0 {
            horiz_offset = 0 as libc::c_int;
            midscreen(scr_vert, point);
        }
        draw_line(scr_vert, scr_horz, point, position, (*curr_line).line_length);
    }
}
#[no_mangle]
pub unsafe extern "C" fn txtalloc() -> *mut text {
    return malloc(::core::mem::size_of::<text>() as libc::c_ulong) as *mut text;
}
#[no_mangle]
pub unsafe extern "C" fn name_alloc() -> *mut files {
    return malloc(::core::mem::size_of::<files>() as libc::c_ulong) as *mut files;
}
#[no_mangle]
pub unsafe extern "C" fn next_word(
    mut string: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    while *string as libc::c_int != '\0' as i32
        && (*string as libc::c_int != 32 as libc::c_int
            && *string as libc::c_int != 9 as libc::c_int)
    {
        string = string.offset(1);
        string;
    }
    while *string as libc::c_int != '\0' as i32
        && (*string as libc::c_int == 32 as libc::c_int
            || *string as libc::c_int == 9 as libc::c_int)
    {
        string = string.offset(1);
        string;
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn prev_word() {
    if position != 1 as libc::c_int {
        if position != 1 as libc::c_int
            && (*point.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
                || *point.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '\t' as i32)
        {
            while position != 1 as libc::c_int
                && (*point as libc::c_int != ' ' as i32
                    && *point as libc::c_int != '\t' as i32)
            {
                left(1 as libc::c_int);
            }
        }
        while position != 1 as libc::c_int
            && (*point as libc::c_int == ' ' as i32
                || *point as libc::c_int == '\t' as i32)
        {
            left(1 as libc::c_int);
        }
        while position != 1 as libc::c_int
            && (*point as libc::c_int != ' ' as i32
                && *point as libc::c_int != '\t' as i32)
        {
            left(1 as libc::c_int);
        }
        if position != 1 as libc::c_int
            && (*point as libc::c_int == ' ' as i32
                || *point as libc::c_int == '\t' as i32)
        {
            right(1 as libc::c_int);
        }
    } else {
        left(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn control() {
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    if in_0 == 1 as libc::c_int {
        string = get_string(ascii_code_str, 1 as libc::c_int);
        if *string as libc::c_int != '\0' as i32 {
            in_0 = atoi(string);
            wmove(text_win, scr_vert, scr_horz - horiz_offset);
            insert(in_0);
        }
        free(string as *mut libc::c_void);
    } else if in_0 == 2 as libc::c_int {
        bottom();
    } else if in_0 == 3 as libc::c_int {
        command_prompt();
    } else if in_0 == 4 as libc::c_int {
        down();
    } else if in_0 == 5 as libc::c_int {
        search_prompt();
    } else if in_0 == 6 as libc::c_int {
        undel_char();
    } else if in_0 == 7 as libc::c_int {
        bol();
    } else if in_0 == 8 as libc::c_int {
        delete(1 as libc::c_int);
    } else if !(in_0 == 9 as libc::c_int) {
        if in_0 == 10 as libc::c_int {
            insert_line(1 as libc::c_int);
        } else if in_0 == 11 as libc::c_int {
            del_char();
        } else if in_0 == 12 as libc::c_int {
            left(1 as libc::c_int);
        } else if in_0 == 13 as libc::c_int {
            insert_line(1 as libc::c_int);
        } else if in_0 == 14 as libc::c_int {
            move_rel(
                'd' as i32,
                if 5 as libc::c_int > last_line - 5 as libc::c_int {
                    5 as libc::c_int
                } else {
                    last_line - 5 as libc::c_int
                },
            );
        } else if in_0 == 15 as libc::c_int {
            eol();
        } else if in_0 == 16 as libc::c_int {
            move_rel(
                'u' as i32,
                if 5 as libc::c_int > last_line - 5 as libc::c_int {
                    5 as libc::c_int
                } else {
                    last_line - 5 as libc::c_int
                },
            );
        } else if !(in_0 == 17 as libc::c_int) {
            if in_0 == 18 as libc::c_int {
                right(1 as libc::c_int);
            } else if !(in_0 == 19 as libc::c_int) {
                if in_0 == 20 as libc::c_int {
                    top();
                } else if in_0 == 21 as libc::c_int {
                    up();
                } else if in_0 == 22 as libc::c_int {
                    undel_word();
                } else if in_0 == 23 as libc::c_int {
                    del_word();
                } else if in_0 == 24 as libc::c_int {
                    search(1 as libc::c_int);
                } else if in_0 == 25 as libc::c_int {
                    del_line();
                } else if in_0 == 26 as libc::c_int {
                    undel_line();
                } else if in_0 == 27 as libc::c_int {
                    menu_op(main_menu.as_mut_ptr());
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn emacs_control() {
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    if in_0 == 1 as libc::c_int {
        bol();
    } else if in_0 == 2 as libc::c_int {
        left(1 as libc::c_int);
    } else if in_0 == 3 as libc::c_int {
        command_prompt();
    } else if in_0 == 4 as libc::c_int {
        del_char();
    } else if in_0 == 5 as libc::c_int {
        eol();
    } else if in_0 == 6 as libc::c_int {
        right(1 as libc::c_int);
    } else if in_0 == 7 as libc::c_int {
        move_rel(
            'u' as i32,
            if 5 as libc::c_int > last_line - 5 as libc::c_int {
                5 as libc::c_int
            } else {
                last_line - 5 as libc::c_int
            },
        );
    } else if in_0 == 8 as libc::c_int {
        delete(1 as libc::c_int);
    } else if !(in_0 == 9 as libc::c_int) {
        if in_0 == 10 as libc::c_int {
            undel_char();
        } else if in_0 == 11 as libc::c_int {
            del_line();
        } else if in_0 == 12 as libc::c_int {
            undel_line();
        } else if in_0 == 13 as libc::c_int {
            insert_line(1 as libc::c_int);
        } else if in_0 == 14 as libc::c_int {
            down();
        } else if in_0 == 15 as libc::c_int {
            string = get_string(ascii_code_str, 1 as libc::c_int);
            if *string as libc::c_int != '\0' as i32 {
                in_0 = atoi(string);
                wmove(text_win, scr_vert, scr_horz - horiz_offset);
                insert(in_0);
            }
            free(string as *mut libc::c_void);
        } else if in_0 == 16 as libc::c_int {
            up();
        } else if !(in_0 == 17 as libc::c_int) {
            if in_0 == 18 as libc::c_int {
                undel_word();
            } else if !(in_0 == 19 as libc::c_int) {
                if in_0 == 20 as libc::c_int {
                    top();
                } else if in_0 == 21 as libc::c_int {
                    bottom();
                } else if in_0 == 22 as libc::c_int {
                    move_rel(
                        'd' as i32,
                        if 5 as libc::c_int > last_line - 5 as libc::c_int {
                            5 as libc::c_int
                        } else {
                            last_line - 5 as libc::c_int
                        },
                    );
                } else if in_0 == 23 as libc::c_int {
                    del_word();
                } else if in_0 == 24 as libc::c_int {
                    search(1 as libc::c_int);
                } else if in_0 == 25 as libc::c_int {
                    search_prompt();
                } else if in_0 == 26 as libc::c_int {
                    adv_word();
                } else if in_0 == 27 as libc::c_int {
                    menu_op(main_menu.as_mut_ptr());
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn bottom() {
    while !((*curr_line).next_line).is_null() {
        curr_line = (*curr_line).next_line;
        absolute_lin += 1;
        absolute_lin;
    }
    point = (*curr_line).line;
    if horiz_offset != 0 {
        horiz_offset = 0 as libc::c_int;
    }
    position = 1 as libc::c_int;
    midscreen(last_line, point);
    scr_pos = scr_horz;
}
#[no_mangle]
pub unsafe extern "C" fn top() {
    while !((*curr_line).prev_line).is_null() {
        curr_line = (*curr_line).prev_line;
        absolute_lin -= 1;
        absolute_lin;
    }
    point = (*curr_line).line;
    if horiz_offset != 0 {
        horiz_offset = 0 as libc::c_int;
    }
    position = 1 as libc::c_int;
    midscreen(0 as libc::c_int, point);
    scr_pos = scr_horz;
}
#[no_mangle]
pub unsafe extern "C" fn nextline() {
    curr_line = (*curr_line).next_line;
    absolute_lin += 1;
    absolute_lin;
    point = (*curr_line).line;
    position = 1 as libc::c_int;
    if scr_vert == last_line {
        wmove(text_win, 0 as libc::c_int, 0 as libc::c_int);
        winsdelln(text_win, -(1 as libc::c_int));
        wmove(text_win, last_line, 0 as libc::c_int);
        wclrtobot(text_win);
        draw_line(
            last_line,
            0 as libc::c_int,
            point,
            1 as libc::c_int,
            (*curr_line).line_length,
        );
    } else {
        scr_vert += 1;
        scr_vert;
    };
}
#[no_mangle]
pub unsafe extern "C" fn prevline() {
    curr_line = (*curr_line).prev_line;
    absolute_lin -= 1;
    absolute_lin;
    point = (*curr_line).line;
    position = 1 as libc::c_int;
    if scr_vert == 0 as libc::c_int {
        winsdelln(text_win, 1 as libc::c_int);
        draw_line(
            0 as libc::c_int,
            0 as libc::c_int,
            point,
            1 as libc::c_int,
            (*curr_line).line_length,
        );
    } else {
        scr_vert -= 1;
        scr_vert;
    }
    while position < (*curr_line).line_length {
        position += 1;
        position;
        point = point.offset(1);
        point;
    }
}
#[no_mangle]
pub unsafe extern "C" fn left(mut disp: libc::c_int) {
    if point != (*curr_line).line {
        if ee_chinese != 0 && position >= 2 as libc::c_int
            && *point.offset(-(2 as libc::c_int as isize)) as libc::c_int
                > 127 as libc::c_int
        {
            point = point.offset(-1);
            point;
            position -= 1;
            position;
        }
        point = point.offset(-1);
        point;
        position -= 1;
        position;
        scanline(point);
        wmove(text_win, scr_vert, scr_horz - horiz_offset);
        scr_pos = scr_horz;
    } else if !((*curr_line).prev_line).is_null() {
        if disp == 0 {
            absolute_lin -= 1;
            absolute_lin;
            curr_line = (*curr_line).prev_line;
            point = ((*curr_line).line).offset((*curr_line).line_length as isize);
            position = (*curr_line).line_length;
            return;
        }
        position = 1 as libc::c_int;
        prevline();
        scanline(point);
        scr_pos = scr_horz;
        wmove(text_win, scr_vert, scr_horz - horiz_offset);
    }
}
#[no_mangle]
pub unsafe extern "C" fn right(mut disp: libc::c_int) {
    if position < (*curr_line).line_length {
        if ee_chinese != 0 && *point as libc::c_int > 127 as libc::c_int
            && (*curr_line).line_length - position >= 2 as libc::c_int
        {
            point = point.offset(1);
            point;
            position += 1;
            position;
        }
        point = point.offset(1);
        point;
        position += 1;
        position;
        scanline(point);
        wmove(text_win, scr_vert, scr_horz - horiz_offset);
        scr_pos = scr_horz;
    } else if !((*curr_line).next_line).is_null() {
        if disp == 0 {
            absolute_lin += 1;
            absolute_lin;
            curr_line = (*curr_line).next_line;
            point = (*curr_line).line;
            position = 1 as libc::c_int;
            return;
        }
        nextline();
        scr_horz = 0 as libc::c_int;
        scr_pos = scr_horz;
        if horiz_offset != 0 {
            horiz_offset = 0 as libc::c_int;
            midscreen(scr_vert, point);
        }
        wmove(text_win, scr_vert, scr_horz - horiz_offset);
        position = 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_pos() {
    scr_horz = 0 as libc::c_int;
    position = 1 as libc::c_int;
    while scr_horz < scr_pos && position < (*curr_line).line_length {
        if *point as libc::c_int == 9 as libc::c_int {
            scr_horz += tabshift(scr_horz);
        } else if (*point as libc::c_int) < ' ' as i32 {
            scr_horz += 2 as libc::c_int;
        } else if ee_chinese != 0 && *point as libc::c_int > 127 as libc::c_int
            && (*curr_line).line_length - position >= 2 as libc::c_int
        {
            scr_horz += 2 as libc::c_int;
            point = point.offset(1);
            point;
            position += 1;
            position;
        } else {
            scr_horz += 1;
            scr_horz;
        }
        position += 1;
        position;
        point = point.offset(1);
        point;
    }
    if scr_horz - horiz_offset > last_col {
        horiz_offset = scr_horz - scr_horz % 8 as libc::c_int
            - (COLS - 8 as libc::c_int);
        midscreen(scr_vert, point);
    } else if scr_horz < horiz_offset {
        horiz_offset = if 0 as libc::c_int > scr_horz - scr_horz % 8 as libc::c_int {
            0 as libc::c_int
        } else {
            scr_horz - scr_horz % 8 as libc::c_int
        };
        midscreen(scr_vert, point);
    }
    wmove(text_win, scr_vert, scr_horz - horiz_offset);
}
#[no_mangle]
pub unsafe extern "C" fn up() {
    if !((*curr_line).prev_line).is_null() {
        prevline();
        point = (*curr_line).line;
        find_pos();
    }
}
#[no_mangle]
pub unsafe extern "C" fn down() {
    if !((*curr_line).next_line).is_null() {
        nextline();
        find_pos();
    }
}
#[no_mangle]
pub unsafe extern "C" fn function_key() {
    if in_0 == 0o404 as libc::c_int {
        left(1 as libc::c_int);
    } else if in_0 == 0o405 as libc::c_int {
        right(1 as libc::c_int);
    } else if in_0 == 0o406 as libc::c_int {
        bol();
    } else if in_0 == 0o550 as libc::c_int {
        eol();
    } else if in_0 == 0o403 as libc::c_int {
        up();
    } else if in_0 == 0o402 as libc::c_int {
        down();
    } else if in_0 == 0o522 as libc::c_int {
        move_rel(
            'd' as i32,
            if 5 as libc::c_int > last_line - 5 as libc::c_int {
                5 as libc::c_int
            } else {
                last_line - 5 as libc::c_int
            },
        );
    } else if in_0 == 0o523 as libc::c_int {
        move_rel(
            'u' as i32,
            if 5 as libc::c_int > last_line - 5 as libc::c_int {
                5 as libc::c_int
            } else {
                last_line - 5 as libc::c_int
            },
        );
    } else if in_0 == 0o510 as libc::c_int {
        del_line();
    } else if in_0 == 0o512 as libc::c_int {
        del_char();
    } else if in_0 == 0o407 as libc::c_int {
        delete(1 as libc::c_int);
    } else if in_0 == 0o511 as libc::c_int {
        insert_line(1 as libc::c_int);
        left(1 as libc::c_int);
    } else if in_0 == 0o410 as libc::c_int + 1 as libc::c_int {
        gold = (gold == 0) as libc::c_int;
    } else if in_0 == 0o410 as libc::c_int + 2 as libc::c_int {
        if gold != 0 {
            gold = 0 as libc::c_int;
            undel_line();
        } else {
            undel_char();
        }
    } else if in_0 == 0o410 as libc::c_int + 3 as libc::c_int {
        if gold != 0 {
            gold = 0 as libc::c_int;
            undel_word();
        } else {
            del_word();
        }
    } else if in_0 == 0o410 as libc::c_int + 4 as libc::c_int {
        if gold != 0 {
            gold = 0 as libc::c_int;
            paint_info_win();
            midscreen(scr_vert, point);
        } else {
            adv_word();
        }
    } else if in_0 == 0o410 as libc::c_int + 5 as libc::c_int {
        if gold != 0 {
            gold = 0 as libc::c_int;
            search_prompt();
        } else {
            search(1 as libc::c_int);
        }
    } else if in_0 == 0o410 as libc::c_int + 6 as libc::c_int {
        if gold != 0 {
            gold = 0 as libc::c_int;
            bottom();
        } else {
            top();
        }
    } else if in_0 == 0o410 as libc::c_int + 7 as libc::c_int {
        if gold != 0 {
            gold = 0 as libc::c_int;
            eol();
        } else {
            bol();
        }
    } else if in_0 == 0o410 as libc::c_int + 8 as libc::c_int {
        if gold != 0 {
            gold = 0 as libc::c_int;
            command_prompt();
        } else {
            adv_line();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_buffer() {
    let mut buffer: [libc::c_char; 256] = [0; 256];
    sprintf(
        buffer.as_mut_ptr(),
        b">!%s\0" as *const u8 as *const libc::c_char,
        print_command,
    );
    wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
    wclrtoeol(com_win);
    wprintw(com_win, printer_msg_str, print_command);
    wrefresh(com_win);
    command(buffer.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn command_prompt() {
    let mut cmd_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = 0;
    info_type = 2 as libc::c_int;
    paint_info_win();
    cmd_str = get_string(command_str, 1 as libc::c_int);
    result = unique_test(cmd_str, commands.as_mut_ptr());
    if result != 1 as libc::c_int {
        werase(com_win);
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        if result == 0 as libc::c_int {
            wprintw(com_win, unkn_cmd_str, cmd_str);
        } else {
            wprintw(
                com_win,
                b"%s\0" as *const u8 as *const libc::c_char,
                non_unique_cmd_msg,
            );
        }
        wrefresh(com_win);
        info_type = 1 as libc::c_int;
        paint_info_win();
        if !cmd_str.is_null() {
            free(cmd_str as *mut libc::c_void);
        }
        return;
    }
    command(cmd_str);
    wrefresh(com_win);
    wmove(text_win, scr_vert, scr_horz - horiz_offset);
    info_type = 1 as libc::c_int;
    paint_info_win();
    if !cmd_str.is_null() {
        free(cmd_str as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn command(mut cmd_str1: *mut libc::c_char) {
    let mut cmd_str2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd_str: *mut libc::c_char = cmd_str1;
    clear_com_win = 1 as libc::c_int;
    if compare(cmd_str, HELP, 0 as libc::c_int) != 0 {
        help();
    } else if compare(cmd_str, WRITE, 0 as libc::c_int) != 0 {
        if restrict_mode() != 0 {
            return;
        }
        cmd_str = next_word(cmd_str as *mut libc::c_uchar) as *mut libc::c_char;
        if *cmd_str as libc::c_int == '\0' as i32 {
            cmd_str2 = get_string(file_write_prompt_str, 1 as libc::c_int);
            cmd_str = cmd_str2;
        }
        tmp_file = resolve_name(cmd_str);
        write_file(tmp_file, 1 as libc::c_int);
        if tmp_file != cmd_str {
            free(tmp_file as *mut libc::c_void);
        }
    } else if compare(cmd_str, READ, 0 as libc::c_int) != 0 {
        if restrict_mode() != 0 {
            return;
        }
        cmd_str = next_word(cmd_str as *mut libc::c_uchar) as *mut libc::c_char;
        if *cmd_str as libc::c_int == '\0' as i32 {
            cmd_str2 = get_string(file_read_prompt_str, 1 as libc::c_int);
            cmd_str = cmd_str2;
        }
        tmp_file = cmd_str;
        recv_file = 1 as libc::c_int;
        tmp_file = resolve_name(cmd_str);
        check_fp();
        if tmp_file != cmd_str {
            free(tmp_file as *mut libc::c_void);
        }
    } else if compare(cmd_str, LINE, 0 as libc::c_int) != 0 {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wclrtoeol(com_win);
        wprintw(com_win, line_num_str, (*curr_line).line_number);
        wprintw(com_win, line_len_str, (*curr_line).line_length);
    } else if compare(cmd_str, FILE_str, 0 as libc::c_int) != 0 {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wclrtoeol(com_win);
        if in_file_name.is_null() {
            wprintw(
                com_win,
                b"%s\0" as *const u8 as *const libc::c_char,
                no_file_string,
            );
        } else {
            wprintw(com_win, current_file_str, in_file_name);
        }
    } else if *cmd_str as libc::c_int >= '0' as i32
        && *cmd_str as libc::c_int <= '9' as i32
    {
        goto_line(cmd_str);
    } else if compare(cmd_str, CHARACTER, 0 as libc::c_int) != 0 {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wclrtoeol(com_win);
        wprintw(com_win, char_str, *point as libc::c_int);
    } else if compare(cmd_str, REDRAW, 0 as libc::c_int) != 0 {
        redraw();
    } else if compare(cmd_str, RESEQUENCE, 0 as libc::c_int) != 0 {
        tmp_line = (*first_line).next_line;
        while !tmp_line.is_null() {
            (*tmp_line)
                .line_number = (*(*tmp_line).prev_line).line_number + 1 as libc::c_int;
            tmp_line = (*tmp_line).next_line;
        }
    } else if compare(cmd_str, AUTHOR, 0 as libc::c_int) != 0 {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wclrtoeol(com_win);
        wprintw(com_win, b"written by Hugh Mahon\0" as *const u8 as *const libc::c_char);
    } else if compare(cmd_str, VERSION, 0 as libc::c_int) != 0 {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wclrtoeol(com_win);
        wprintw(com_win, b"%s\0" as *const u8 as *const libc::c_char, version);
    } else if compare(cmd_str, CASE, 0 as libc::c_int) != 0 {
        case_sen = 1 as libc::c_int;
    } else if compare(cmd_str, NOCASE, 0 as libc::c_int) != 0 {
        case_sen = 0 as libc::c_int;
    } else if compare(cmd_str, EXPAND, 0 as libc::c_int) != 0 {
        expand_tabs = 1 as libc::c_int;
    } else if compare(cmd_str, NOEXPAND, 0 as libc::c_int) != 0 {
        expand_tabs = 0 as libc::c_int;
    } else if compare(cmd_str, Exit_string, 0 as libc::c_int) != 0 {
        finish();
    } else if compare(cmd_str, chinese_cmd, 0 as libc::c_int) != 0 {
        ee_chinese = 1 as libc::c_int;
    } else if compare(cmd_str, nochinese_cmd, 0 as libc::c_int) != 0 {
        ee_chinese = 0 as libc::c_int;
    } else if compare(cmd_str, QUIT_string, 0 as libc::c_int) != 0 {
        quit(0 as libc::c_int);
    } else if *cmd_str as libc::c_int == '!' as i32 {
        cmd_str = cmd_str.offset(1);
        cmd_str;
        if *cmd_str as libc::c_int == ' ' as i32
            || *cmd_str as libc::c_int == 9 as libc::c_int
        {
            cmd_str = next_word(cmd_str as *mut libc::c_uchar) as *mut libc::c_char;
        }
        sh_command(cmd_str);
    } else if *cmd_str as libc::c_int == '<' as i32 && in_pipe == 0 {
        in_pipe = 1 as libc::c_int;
        shell_fork = 0 as libc::c_int;
        cmd_str = cmd_str.offset(1);
        cmd_str;
        if *cmd_str as libc::c_int == ' ' as i32
            || *cmd_str as libc::c_int == '\t' as i32
        {
            cmd_str = next_word(cmd_str as *mut libc::c_uchar) as *mut libc::c_char;
        }
        command(cmd_str);
        in_pipe = 0 as libc::c_int;
        shell_fork = 1 as libc::c_int;
    } else if *cmd_str as libc::c_int == '>' as i32 && out_pipe == 0 {
        out_pipe = 1 as libc::c_int;
        cmd_str = cmd_str.offset(1);
        cmd_str;
        if *cmd_str as libc::c_int == ' ' as i32
            || *cmd_str as libc::c_int == '\t' as i32
        {
            cmd_str = next_word(cmd_str as *mut libc::c_uchar) as *mut libc::c_char;
        }
        command(cmd_str);
        out_pipe = 0 as libc::c_int;
    } else {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wclrtoeol(com_win);
        wprintw(com_win, unkn_cmd_str, cmd_str);
    }
    if !cmd_str2.is_null() {
        free(cmd_str2 as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn scan(
    mut line: *mut libc::c_char,
    mut offset: libc::c_int,
    mut column: libc::c_int,
) -> libc::c_int {
    let mut stemp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    stemp = line;
    i = 0 as libc::c_int;
    j = column;
    while i < offset {
        i += 1;
        i;
        j += len_char(*stemp as libc::c_int, j);
        stemp = stemp.offset(1);
        stemp;
    }
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn get_string(
    mut prompt: *mut libc::c_char,
    mut advance: libc::c_int,
) -> *mut libc::c_char {
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nam_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g_point: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_int: libc::c_int = 0;
    let mut g_horz: libc::c_int = 0;
    let mut g_position: libc::c_int = 0;
    let mut g_pos: libc::c_int = 0;
    let mut esc_flag: libc::c_int = 0;
    tmp_string = malloc(512 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    g_point = tmp_string;
    wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
    wclrtoeol(com_win);
    waddnstr(com_win, prompt, -(1 as libc::c_int));
    wrefresh(com_win);
    nam_str = tmp_string;
    clear_com_win = 1 as libc::c_int;
    g_position = scan(prompt, strlen(prompt) as libc::c_int, 0 as libc::c_int);
    g_horz = g_position;
    g_pos = 0 as libc::c_int;
    loop {
        esc_flag = 0 as libc::c_int;
        in_0 = wgetch(com_win);
        if in_0 == -(1 as libc::c_int) {
            exit(0 as libc::c_int);
        }
        if (in_0 == 8 as libc::c_int || in_0 == 127 as libc::c_int
            || in_0 == 0o407 as libc::c_int) && g_pos > 0 as libc::c_int
        {
            tmp_int = g_horz;
            g_pos -= 1;
            g_pos;
            g_horz = scan(g_point, g_pos, g_position);
            tmp_int = tmp_int - g_horz;
            while (0 as libc::c_int) < tmp_int {
                if g_horz + tmp_int < last_col - 1 as libc::c_int {
                    waddch(com_win, '\u{8}' as i32 as chtype);
                    waddch(com_win, ' ' as i32 as chtype);
                    waddch(com_win, '\u{8}' as i32 as chtype);
                }
                tmp_int -= 1;
                tmp_int;
            }
            nam_str = nam_str.offset(-1);
            nam_str;
        } else if in_0 != 8 as libc::c_int && in_0 != 127 as libc::c_int
            && in_0 != '\n' as i32 && in_0 != '\r' as i32 && in_0 < 256 as libc::c_int
        {
            if in_0 == '\u{16}' as i32 {
                esc_flag = 1 as libc::c_int;
                in_0 = wgetch(com_win);
                if in_0 == -(1 as libc::c_int) {
                    exit(0 as libc::c_int);
                }
            }
            *nam_str = in_0 as libc::c_char;
            g_pos += 1;
            g_pos;
            if *(*__ctype_b_loc()).offset(in_0 as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                == 0 && g_horz < last_col - 1 as libc::c_int
            {
                g_horz += out_char(com_win, in_0, g_horz);
            } else {
                g_horz += 1;
                g_horz;
                if g_horz < last_col - 1 as libc::c_int {
                    waddch(com_win, in_0 as libc::c_uchar as chtype);
                }
            }
            nam_str = nam_str.offset(1);
            nam_str;
        }
        wrefresh(com_win);
        if esc_flag != 0 {
            in_0 = '\0' as i32;
        }
        if !(in_0 != '\n' as i32 && in_0 != '\r' as i32) {
            break;
        }
    }
    *nam_str = '\0' as i32 as libc::c_char;
    nam_str = tmp_string;
    if (*nam_str as libc::c_int == ' ' as i32
        || *nam_str as libc::c_int == 9 as libc::c_int) && advance != 0
    {
        nam_str = next_word(nam_str as *mut libc::c_uchar) as *mut libc::c_char;
    }
    string = malloc((strlen(nam_str)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    strcpy(string, nam_str);
    free(tmp_string as *mut libc::c_void);
    wrefresh(com_win);
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn compare(
    mut string1: *mut libc::c_char,
    mut string2: *mut libc::c_char,
    mut sensitive: libc::c_int,
) -> libc::c_int {
    let mut strng1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strng2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut equal: libc::c_int = 0;
    strng1 = string1;
    strng2 = string2;
    if strng1.is_null() || strng2.is_null() || *strng1 as libc::c_int == '\0' as i32
        || *strng2 as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    equal = 1 as libc::c_int;
    while equal != 0 {
        if sensitive != 0 {
            if *strng1 as libc::c_int != *strng2 as libc::c_int {
                equal = 0 as libc::c_int;
            }
        } else if toupper(*strng1 as libc::c_uchar as libc::c_int)
            != toupper(*strng2 as libc::c_uchar as libc::c_int)
        {
            equal = 0 as libc::c_int;
        }
        strng1 = strng1.offset(1);
        strng1;
        strng2 = strng2.offset(1);
        strng2;
        if *strng1 as libc::c_int == '\0' as i32 || *strng2 as libc::c_int == '\0' as i32
            || *strng1 as libc::c_int == ' ' as i32
            || *strng2 as libc::c_int == ' ' as i32
        {
            break;
        }
    }
    return equal;
}
#[no_mangle]
pub unsafe extern "C" fn goto_line(mut cmd_str: *mut libc::c_char) {
    let mut number: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut direction: libc::c_char = '\0' as i32 as libc::c_char;
    let mut t_line: *mut text = 0 as *mut text;
    ptr = cmd_str;
    i = 0 as libc::c_int;
    while *ptr as libc::c_int >= '0' as i32 && *ptr as libc::c_int <= '9' as i32 {
        i = i * 10 as libc::c_int + (*ptr as libc::c_int - '0' as i32);
        ptr = ptr.offset(1);
        ptr;
    }
    number = i;
    i = 0 as libc::c_int;
    t_line = curr_line;
    while (*t_line).line_number > number && !((*t_line).prev_line).is_null() {
        i += 1;
        i;
        t_line = (*t_line).prev_line;
        direction = 'u' as i32 as libc::c_char;
    }
    while (*t_line).line_number < number && !((*t_line).next_line).is_null() {
        i += 1;
        i;
        direction = 'd' as i32 as libc::c_char;
        t_line = (*t_line).next_line;
    }
    if i < 30 as libc::c_int && i > 0 as libc::c_int {
        move_rel(direction as libc::c_int, i);
    } else {
        if direction as libc::c_int != 'd' as i32 {
            absolute_lin += i;
        } else {
            absolute_lin -= i;
        }
        curr_line = t_line;
        point = (*curr_line).line;
        position = 1 as libc::c_int;
        midscreen(last_line / 2 as libc::c_int, point);
        scr_pos = scr_horz;
    }
    wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
    wclrtoeol(com_win);
    wprintw(com_win, line_num_str, (*curr_line).line_number);
    wmove(text_win, scr_vert, scr_horz - horiz_offset);
}
#[no_mangle]
pub unsafe extern "C" fn midscreen(mut line: libc::c_int, mut pnt: *mut libc::c_uchar) {
    let mut mid_line: *mut text = 0 as *mut text;
    let mut i: libc::c_int = 0;
    line = if line < last_line { line } else { last_line };
    mid_line = curr_line;
    i = 0 as libc::c_int;
    while i < line && !((*curr_line).prev_line).is_null() {
        curr_line = (*curr_line).prev_line;
        i += 1;
        i;
    }
    scr_horz = 0 as libc::c_int;
    scr_vert = scr_horz;
    wmove(text_win, 0 as libc::c_int, 0 as libc::c_int);
    draw_screen();
    scr_vert = i;
    curr_line = mid_line;
    scanline(pnt);
    wmove(text_win, scr_vert, scr_horz - horiz_offset);
}
#[no_mangle]
pub unsafe extern "C" fn get_options(
    mut numargs: libc::c_int,
    mut arguments: *mut *mut libc::c_char,
) {
    let mut buff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0;
    let mut temp_names: *mut files = 0 as *mut files;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut no_more_opts: libc::c_int = 0 as libc::c_int;
    name = strrchr(*arguments.offset(0 as libc::c_int as isize), '/' as i32);
    if name.is_null() {
        name = *arguments.offset(0 as libc::c_int as isize);
    } else {
        name = name.offset(1);
        name;
    }
    if strcmp(name, b"ree\0" as *const u8 as *const libc::c_char) == 0 {
        restricted = 1 as libc::c_int;
    }
    top_of_stack = 0 as *mut files;
    input_file = 0 as libc::c_int;
    recv_file = 0 as libc::c_int;
    count = 1 as libc::c_int;
    while count < numargs && no_more_opts == 0 {
        buff = *arguments.offset(count as isize);
        if strcmp(b"-i\0" as *const u8 as *const libc::c_char, buff) == 0 {
            info_window = 0 as libc::c_int;
        } else if strcmp(b"-e\0" as *const u8 as *const libc::c_char, buff) == 0 {
            expand_tabs = 0 as libc::c_int;
        } else if strcmp(b"-h\0" as *const u8 as *const libc::c_char, buff) == 0 {
            nohighlight = 1 as libc::c_int;
        } else if strcmp(b"-?\0" as *const u8 as *const libc::c_char, buff) == 0 {
            fprintf(stderr, usage0, *arguments.offset(0 as libc::c_int as isize));
            fputs(usage1, stderr);
            fputs(usage2, stderr);
            fputs(usage3, stderr);
            fputs(usage4, stderr);
            exit(1 as libc::c_int);
        } else if *buff as libc::c_int == '+' as i32 && start_at_line.is_null() {
            buff = buff.offset(1);
            buff;
            start_at_line = buff as *mut libc::c_uchar;
        } else if strcmp(b"--\0" as *const u8 as *const libc::c_char, buff) == 0 {
            no_more_opts = 1 as libc::c_int;
        } else {
            count -= 1;
            count;
            no_more_opts = 1 as libc::c_int;
        }
        count += 1;
        count;
    }
    while count < numargs {
        buff = *arguments.offset(count as isize);
        if top_of_stack.is_null() {
            top_of_stack = name_alloc();
            temp_names = top_of_stack;
        } else {
            (*temp_names).next_name = name_alloc();
            temp_names = (*temp_names).next_name;
        }
        (*temp_names)
            .name = malloc(
            (strlen(buff)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_uchar;
        ptr = (*temp_names).name as *mut libc::c_char;
        while *buff as libc::c_int != '\0' as i32 {
            *ptr = *buff;
            buff = buff.offset(1);
            buff;
            ptr = ptr.offset(1);
            ptr;
        }
        *ptr = '\0' as i32 as libc::c_char;
        (*temp_names).next_name = 0 as *mut files;
        input_file = 1 as libc::c_int;
        recv_file = 1 as libc::c_int;
        count += 1;
        count;
    }
}
#[no_mangle]
pub unsafe extern "C" fn check_fp() {
    let mut line_num: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    clear_com_win = 1 as libc::c_int;
    tmp_vert = scr_vert;
    tmp_horz = scr_horz;
    tmp_line = curr_line;
    if input_file != 0 {
        tmp_file = (*top_of_stack).name as *mut libc::c_char;
        in_file_name = tmp_file as *mut libc::c_uchar;
        top_of_stack = (*top_of_stack).next_name;
    }
    temp = stat(tmp_file, &mut buf);
    buf.st_mode &= !(0o7777 as libc::c_int) as __mode_t;
    if temp != -(1 as libc::c_int) && buf.st_mode != 0o100000 as libc::c_int as __mode_t
        && buf.st_mode != 0 as libc::c_int as __mode_t
    {
        wprintw(com_win, file_is_dir_msg, tmp_file);
        wrefresh(com_win);
        if input_file != 0 {
            quit(0 as libc::c_int);
            return;
        } else {
            return
        }
    }
    get_fd = open(tmp_file, 0 as libc::c_int);
    if get_fd == -(1 as libc::c_int) {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wclrtoeol(com_win);
        if input_file != 0 {
            wprintw(com_win, new_file_msg, tmp_file);
        } else {
            wprintw(com_win, cant_open_msg, tmp_file);
        }
        wrefresh(com_win);
        wmove(text_win, scr_vert, scr_horz - horiz_offset);
        wrefresh(text_win);
        recv_file = 0 as libc::c_int;
        input_file = 0 as libc::c_int;
        return;
    } else {
        get_file(tmp_file);
    }
    recv_file = 0 as libc::c_int;
    line_num = (*curr_line).line_number;
    scr_vert = tmp_vert;
    scr_horz = tmp_horz;
    if input_file != 0 {
        curr_line = first_line;
    } else {
        curr_line = tmp_line;
    }
    point = (*curr_line).line;
    draw_screen();
    if input_file != 0 {
        input_file = 0 as libc::c_int;
        if !start_at_line.is_null() {
            line_num = atoi(start_at_line as *const libc::c_char) - 1 as libc::c_int;
            move_rel('d' as i32, line_num);
            line_num = 0 as libc::c_int;
            start_at_line = 0 as *mut libc::c_uchar;
        }
    } else {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wclrtoeol(com_win);
        text_changes = 1 as libc::c_int;
        if !tmp_file.is_null() && *tmp_file as libc::c_int != '\0' as i32 {
            wprintw(com_win, file_read_fin_msg, tmp_file);
        }
    }
    wrefresh(com_win);
    wmove(text_win, scr_vert, scr_horz - horiz_offset);
    wrefresh(text_win);
}
#[no_mangle]
pub unsafe extern "C" fn get_file(mut file_name: *mut libc::c_char) {
    let mut can_read: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut append: libc::c_int = 0;
    let mut temp_line: *mut text = 0 as *mut text;
    let mut ro_flag: libc::c_char = 0 as libc::c_int as libc::c_char;
    if recv_file != 0 {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wclrtoeol(com_win);
        wprintw(com_win, reading_file_msg, file_name);
        if access(file_name, 2 as libc::c_int) != 0 {
            if *__errno_location() == 20 as libc::c_int
                || *__errno_location() == 13 as libc::c_int
                || *__errno_location() == 30 as libc::c_int
                || *__errno_location() == 26 as libc::c_int
                || *__errno_location() == 14 as libc::c_int
            {
                wprintw(
                    com_win,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    read_only_msg,
                );
                ro_flag = 1 as libc::c_int as libc::c_char;
            }
        }
        wrefresh(com_win);
    }
    if (*curr_line).line_length > 1 as libc::c_int {
        insert_line(0 as libc::c_int);
        left(0 as libc::c_int);
        append = 0 as libc::c_int;
    } else {
        append = 1 as libc::c_int;
    }
    can_read = 0 as libc::c_int;
    loop {
        length = read(
            get_fd,
            in_string.as_mut_ptr() as *mut libc::c_void,
            512 as libc::c_int as size_t,
        ) as libc::c_int;
        if !(length != 0 as libc::c_int && length != -(1 as libc::c_int)) {
            break;
        }
        can_read = 1 as libc::c_int;
        get_line(length, in_string.as_mut_ptr() as *mut libc::c_uchar, &mut append);
    }
    if can_read != 0 && (*curr_line).line_length == 1 as libc::c_int {
        temp_line = (*curr_line).prev_line;
        (*temp_line).next_line = (*curr_line).next_line;
        if !((*temp_line).next_line).is_null() {
            (*(*temp_line).next_line).prev_line = temp_line;
        }
        if !((*curr_line).line).is_null() {
            free((*curr_line).line as *mut libc::c_void);
        }
        free(curr_line as *mut libc::c_void);
        curr_line = temp_line;
    }
    if input_file != 0 {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wclrtoeol(com_win);
        wprintw(com_win, file_read_lines_msg, in_file_name, (*curr_line).line_number);
        if ro_flag != 0 {
            wprintw(com_win, b"%s\0" as *const u8 as *const libc::c_char, read_only_msg);
        }
        wrefresh(com_win);
    } else if can_read != 0 {
        text_changes = 1 as libc::c_int;
    }
    if recv_file != 0 {
        in_0 = -(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_line(
    mut length: libc::c_int,
    mut in_string_0: *mut libc::c_uchar,
    mut append: *mut libc::c_int,
) {
    let mut str1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut str2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut num: libc::c_int = 0;
    let mut char_count: libc::c_int = 0;
    let mut temp_counter: libc::c_int = 0;
    let mut tline: *mut text = 0 as *mut text;
    let mut first_time: libc::c_int = 0;
    str2 = in_string_0;
    num = 0 as libc::c_int;
    first_time = 1 as libc::c_int;
    while num < length {
        if first_time == 0 {
            if num < length {
                str2 = str2.offset(1);
                str2;
                num += 1;
                num;
            }
        } else {
            first_time = 0 as libc::c_int;
        }
        str1 = str2;
        char_count = 1 as libc::c_int;
        while *str2 as libc::c_int != '\n' as i32 && num < length {
            str2 = str2.offset(1);
            str2;
            num += 1;
            num;
            char_count += 1;
            char_count;
        }
        if *append == 0 {
            tline = txtalloc();
            (*tline).line_number = (*curr_line).line_number + 1 as libc::c_int;
            (*tline).next_line = (*curr_line).next_line;
            (*tline).prev_line = curr_line;
            (*curr_line).next_line = tline;
            if !((*tline).next_line).is_null() {
                (*(*tline).next_line).prev_line = tline;
            }
            curr_line = tline;
            point = malloc(char_count as libc::c_ulong) as *mut libc::c_uchar;
            (*curr_line).line = point;
            (*curr_line).line_length = char_count;
            (*curr_line).max_length = char_count;
        } else {
            point = resiz_line(char_count, curr_line, (*curr_line).line_length);
            (*curr_line).line_length += char_count - 1 as libc::c_int;
        }
        temp_counter = 1 as libc::c_int;
        while temp_counter < char_count {
            *point = *str1;
            point = point.offset(1);
            point;
            str1 = str1.offset(1);
            str1;
            temp_counter += 1;
            temp_counter;
        }
        *point = '\0' as i32 as libc::c_uchar;
        *append = 0 as libc::c_int;
        if num == length && *str2 as libc::c_int != '\n' as i32 {
            *append = 1 as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn draw_screen() {
    let mut temp_line: *mut text = 0 as *mut text;
    let mut line_out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp_vert: libc::c_int = 0;
    temp_line = curr_line;
    temp_vert = scr_vert;
    wclrtobot(text_win);
    while !temp_line.is_null() && temp_vert <= last_line {
        line_out = (*temp_line).line;
        draw_line(
            temp_vert,
            0 as libc::c_int,
            line_out,
            1 as libc::c_int,
            (*temp_line).line_length,
        );
        temp_vert += 1;
        temp_vert;
        temp_line = (*temp_line).next_line;
    }
    wmove(text_win, temp_vert, 0 as libc::c_int);
    wmove(text_win, scr_vert, scr_horz - horiz_offset);
}
#[no_mangle]
pub unsafe extern "C" fn finish() {
    let mut file_name: *mut libc::c_char = in_file_name as *mut libc::c_char;
    if file_name.is_null() || *file_name as libc::c_int == '\0' as i32 {
        file_name = get_string(save_file_name_prompt, 1 as libc::c_int);
    }
    if file_name.is_null() || *file_name as libc::c_int == '\0' as i32 {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wprintw(
            com_win,
            b"%s\0" as *const u8 as *const libc::c_char,
            file_not_saved_msg,
        );
        wclrtoeol(com_win);
        wrefresh(com_win);
        clear_com_win = 1 as libc::c_int;
        return;
    }
    tmp_file = resolve_name(file_name);
    if tmp_file != file_name {
        free(file_name as *mut libc::c_void);
        file_name = tmp_file;
    }
    if write_file(file_name, 1 as libc::c_int) != 0 {
        text_changes = 0 as libc::c_int;
        quit(0 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn quit(mut noverify: libc::c_int) -> libc::c_int {
    let mut ans: *mut libc::c_char = 0 as *mut libc::c_char;
    wtouchln(text_win, 0 as libc::c_int, getmaxy(text_win), 1 as libc::c_int);
    wrefresh(text_win);
    if text_changes != 0 && noverify == 0 {
        ans = get_string(changes_made_prompt, 1 as libc::c_int);
        if toupper(*ans as libc::c_uchar as libc::c_int)
            == toupper(*yes_char as libc::c_uchar as libc::c_int)
        {
            text_changes = 0 as libc::c_int;
        } else {
            return 0 as libc::c_int
        }
        free(ans as *mut libc::c_void);
    }
    if top_of_stack.is_null() {
        if info_window != 0 {
            wrefresh(info_win);
        }
        wrefresh(com_win);
        resetty();
        endwin();
        putchar('\n' as i32);
        exit(0 as libc::c_int);
    } else {
        delete_text();
        recv_file = 1 as libc::c_int;
        input_file = 1 as libc::c_int;
        check_fp();
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn edit_abort(mut arg: libc::c_int) {
    wrefresh(com_win);
    resetty();
    endwin();
    putchar('\n' as i32);
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn delete_text() {
    while !((*curr_line).next_line).is_null() {
        curr_line = (*curr_line).next_line;
    }
    while curr_line != first_line {
        free((*curr_line).line as *mut libc::c_void);
        curr_line = (*curr_line).prev_line;
        absolute_lin -= 1;
        absolute_lin;
        free((*curr_line).next_line as *mut libc::c_void);
    }
    (*curr_line).next_line = 0 as *mut text;
    *(*curr_line).line = '\0' as i32 as libc::c_uchar;
    (*curr_line).line_length = 1 as libc::c_int;
    (*curr_line).line_number = 1 as libc::c_int;
    point = (*curr_line).line;
    scr_horz = 0 as libc::c_int;
    scr_vert = scr_horz;
    scr_pos = scr_vert;
    position = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn write_file(
    mut file_name: *mut libc::c_char,
    mut warn_if_exists: libc::c_int,
) -> libc::c_int {
    let mut cr: libc::c_char = 0;
    let mut tmp_point: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out_line: *mut text = 0 as *mut text;
    let mut lines: libc::c_int = 0;
    let mut charac: libc::c_int = 0;
    let mut temp_pos: libc::c_int = 0;
    let mut write_flag: libc::c_int = 1 as libc::c_int;
    lines = 0 as libc::c_int;
    charac = lines;
    if warn_if_exists != 0
        && (in_file_name.is_null()
            || strcmp(in_file_name as *const libc::c_char, file_name) != 0)
    {
        temp_fp = fopen(file_name, b"r\0" as *const u8 as *const libc::c_char);
        if !temp_fp.is_null() {
            tmp_point = get_string(file_exists_prompt, 1 as libc::c_int);
            if toupper(*tmp_point as libc::c_uchar as libc::c_int)
                == toupper(*yes_char as libc::c_uchar as libc::c_int)
            {
                write_flag = 1 as libc::c_int;
            } else {
                write_flag = 0 as libc::c_int;
            }
            fclose(temp_fp);
            free(tmp_point as *mut libc::c_void);
        }
    }
    clear_com_win = 1 as libc::c_int;
    if write_flag != 0 {
        temp_fp = fopen(file_name, b"w\0" as *const u8 as *const libc::c_char);
        if temp_fp.is_null() {
            clear_com_win = 1 as libc::c_int;
            wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
            wclrtoeol(com_win);
            wprintw(com_win, create_file_fail_msg, file_name);
            wrefresh(com_win);
            return 0 as libc::c_int;
        } else {
            wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
            wclrtoeol(com_win);
            wprintw(com_win, writing_file_msg, file_name);
            wrefresh(com_win);
            cr = '\n' as i32 as libc::c_char;
            out_line = first_line;
            while !out_line.is_null() {
                temp_pos = 1 as libc::c_int;
                tmp_point = (*out_line).line as *mut libc::c_char;
                while temp_pos < (*out_line).line_length {
                    putc(*tmp_point as libc::c_int, temp_fp);
                    tmp_point = tmp_point.offset(1);
                    tmp_point;
                    temp_pos += 1;
                    temp_pos;
                }
                charac += (*out_line).line_length;
                out_line = (*out_line).next_line;
                putc(cr as libc::c_int, temp_fp);
                lines += 1;
                lines;
            }
            fclose(temp_fp);
            wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
            wclrtoeol(com_win);
            wprintw(com_win, file_written_msg, file_name, lines, charac);
            wrefresh(com_win);
            return 1 as libc::c_int;
        }
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn search(mut display_message: libc::c_int) -> libc::c_int {
    let mut lines_moved: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    if srch_str.is_null() || *srch_str as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    if display_message != 0 {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wclrtoeol(com_win);
        wprintw(com_win, b"%s\0" as *const u8 as *const libc::c_char, searching_msg);
        wrefresh(com_win);
        clear_com_win = 1 as libc::c_int;
    }
    lines_moved = 0 as libc::c_int;
    found = 0 as libc::c_int;
    srch_line = curr_line;
    srch_1 = point;
    if position < (*curr_line).line_length {
        srch_1 = srch_1.offset(1);
        srch_1;
    }
    iter = position + 1 as libc::c_int;
    while found == 0 && !srch_line.is_null() {
        while iter < (*srch_line).line_length && found == 0 {
            srch_2 = srch_1;
            if case_sen != 0 {
                srch_3 = srch_str;
                while *srch_2 as libc::c_int == *srch_3 as libc::c_int
                    && *srch_3 as libc::c_int != '\0' as i32
                {
                    found = 1 as libc::c_int;
                    srch_2 = srch_2.offset(1);
                    srch_2;
                    srch_3 = srch_3.offset(1);
                    srch_3;
                }
            } else {
                srch_3 = u_srch_str;
                while toupper(*srch_2 as libc::c_int) == *srch_3 as libc::c_int
                    && *srch_3 as libc::c_int != '\0' as i32
                {
                    found = 1 as libc::c_int;
                    srch_2 = srch_2.offset(1);
                    srch_2;
                    srch_3 = srch_3.offset(1);
                    srch_3;
                }
            }
            if !(*srch_3 as libc::c_int == '\0' as i32 && found != 0) {
                found = 0 as libc::c_int;
                if iter < (*srch_line).line_length {
                    srch_1 = srch_1.offset(1);
                    srch_1;
                }
                iter += 1;
                iter;
            }
        }
        if found == 0 {
            srch_line = (*srch_line).next_line;
            if !srch_line.is_null() {
                srch_1 = (*srch_line).line;
            }
            iter = 1 as libc::c_int;
            lines_moved += 1;
            lines_moved;
        }
    }
    if found != 0 {
        if display_message != 0 {
            wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
            wclrtoeol(com_win);
            wrefresh(com_win);
        }
        if lines_moved == 0 as libc::c_int {
            while position < iter {
                right(1 as libc::c_int);
            }
        } else if lines_moved < 30 as libc::c_int {
            move_rel('d' as i32, lines_moved);
            while position < iter {
                right(1 as libc::c_int);
            }
        } else {
            absolute_lin += lines_moved;
            curr_line = srch_line;
            point = srch_1;
            position = iter;
            scanline(point);
            scr_pos = scr_horz;
            midscreen(last_line / 2 as libc::c_int, point);
        }
    } else {
        if display_message != 0 {
            wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
            wclrtoeol(com_win);
            wprintw(com_win, str_not_found_msg, srch_str);
            wrefresh(com_win);
        }
        wmove(text_win, scr_vert, scr_horz - horiz_offset);
    }
    return found;
}
#[no_mangle]
pub unsafe extern "C" fn search_prompt() {
    if !srch_str.is_null() {
        free(srch_str as *mut libc::c_void);
    }
    if !u_srch_str.is_null() && *u_srch_str as libc::c_int != '\0' as i32 {
        free(u_srch_str as *mut libc::c_void);
    }
    srch_str = get_string(search_prompt_str, 0 as libc::c_int) as *mut libc::c_uchar;
    gold = 0 as libc::c_int;
    srch_3 = srch_str;
    u_srch_str = malloc(
        (strlen(srch_str as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_uchar;
    srch_1 = u_srch_str;
    while *srch_3 as libc::c_int != '\0' as i32 {
        *srch_1 = toupper(*srch_3 as libc::c_int) as libc::c_uchar;
        srch_1 = srch_1.offset(1);
        srch_1;
        srch_3 = srch_3.offset(1);
        srch_3;
    }
    *srch_1 = '\0' as i32 as libc::c_uchar;
    search(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn del_char() {
    in_0 = 8 as libc::c_int;
    if position < (*curr_line).line_length {
        if ee_chinese != 0 && *point as libc::c_int > 127 as libc::c_int
            && (*curr_line).line_length - position >= 2 as libc::c_int
        {
            point = point.offset(1);
            point;
            position += 1;
            position;
        }
        position += 1;
        position;
        point = point.offset(1);
        point;
        scanline(point);
        delete(1 as libc::c_int);
    } else {
        right(1 as libc::c_int);
        delete(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn undel_char() {
    if *d_char.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        insert_line(1 as libc::c_int);
    } else {
        in_0 = *d_char.offset(0 as libc::c_int as isize) as libc::c_int;
        insert(in_0);
        if *d_char.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
            in_0 = *d_char.offset(1 as libc::c_int as isize) as libc::c_int;
            insert(in_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn del_word() {
    let mut tposit: libc::c_int = 0;
    let mut difference: libc::c_int = 0;
    let mut d_word2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut d_word3: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp_char: [libc::c_uchar; 3] = [0; 3];
    if !d_word.is_null() {
        free(d_word as *mut libc::c_void);
    }
    d_word = malloc((*curr_line).line_length as libc::c_ulong) as *mut libc::c_uchar;
    tmp_char[0 as libc::c_int as usize] = *d_char.offset(0 as libc::c_int as isize);
    tmp_char[1 as libc::c_int as usize] = *d_char.offset(1 as libc::c_int as isize);
    tmp_char[2 as libc::c_int as usize] = *d_char.offset(2 as libc::c_int as isize);
    d_word3 = point;
    d_word2 = d_word;
    tposit = position;
    while tposit < (*curr_line).line_length
        && (*d_word3 as libc::c_int != ' ' as i32
            && *d_word3 as libc::c_int != '\t' as i32)
    {
        tposit += 1;
        tposit;
        *d_word2 = *d_word3;
        d_word2 = d_word2.offset(1);
        d_word2;
        d_word3 = d_word3.offset(1);
        d_word3;
    }
    while tposit < (*curr_line).line_length
        && (*d_word3 as libc::c_int == ' ' as i32
            || *d_word3 as libc::c_int == '\t' as i32)
    {
        tposit += 1;
        tposit;
        *d_word2 = *d_word3;
        d_word2 = d_word2.offset(1);
        d_word2;
        d_word3 = d_word3.offset(1);
        d_word3;
    }
    *d_word2 = '\0' as i32 as libc::c_uchar;
    difference = d_word2.offset_from(d_word) as libc::c_long as libc::c_int;
    d_wrd_len = difference;
    d_word2 = point;
    while tposit < (*curr_line).line_length {
        tposit += 1;
        tposit;
        *d_word2 = *d_word3;
        d_word2 = d_word2.offset(1);
        d_word2;
        d_word3 = d_word3.offset(1);
        d_word3;
    }
    (*curr_line).line_length -= difference;
    *d_word2 = '\0' as i32 as libc::c_uchar;
    draw_line(scr_vert, scr_horz, point, position, (*curr_line).line_length);
    *d_char.offset(0 as libc::c_int as isize) = tmp_char[0 as libc::c_int as usize];
    *d_char.offset(1 as libc::c_int as isize) = tmp_char[1 as libc::c_int as usize];
    *d_char.offset(2 as libc::c_int as isize) = tmp_char[2 as libc::c_int as usize];
    text_changes = 1 as libc::c_int;
    formatted = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn undel_word() {
    let mut temp: libc::c_int = 0;
    let mut tposit: libc::c_int = 0;
    let mut tmp_old_ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp_space: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp_ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut d_word_ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (*curr_line).max_length - ((*curr_line).line_length + d_wrd_len)
        < 5 as libc::c_int
    {
        point = resiz_line(d_wrd_len, curr_line, position);
    }
    tmp_space = malloc(((*curr_line).line_length + d_wrd_len) as libc::c_ulong)
        as *mut libc::c_uchar;
    tmp_ptr = tmp_space;
    d_word_ptr = d_word;
    temp = 1 as libc::c_int;
    while temp <= d_wrd_len {
        temp += 1;
        temp;
        *tmp_ptr = *d_word_ptr;
        tmp_ptr = tmp_ptr.offset(1);
        tmp_ptr;
        d_word_ptr = d_word_ptr.offset(1);
        d_word_ptr;
    }
    tmp_old_ptr = point;
    tposit = position;
    while tposit < (*curr_line).line_length {
        temp += 1;
        temp;
        tposit += 1;
        tposit;
        *tmp_ptr = *tmp_old_ptr;
        tmp_ptr = tmp_ptr.offset(1);
        tmp_ptr;
        tmp_old_ptr = tmp_old_ptr.offset(1);
        tmp_old_ptr;
    }
    (*curr_line).line_length += d_wrd_len;
    tmp_old_ptr = point;
    *tmp_ptr = '\0' as i32 as libc::c_uchar;
    tmp_ptr = tmp_space;
    tposit = 1 as libc::c_int;
    while tposit < temp {
        tposit += 1;
        tposit;
        *tmp_old_ptr = *tmp_ptr;
        tmp_ptr = tmp_ptr.offset(1);
        tmp_ptr;
        tmp_old_ptr = tmp_old_ptr.offset(1);
        tmp_old_ptr;
    }
    *tmp_old_ptr = '\0' as i32 as libc::c_uchar;
    free(tmp_space as *mut libc::c_void);
    draw_line(scr_vert, scr_horz, point, position, (*curr_line).line_length);
}
#[no_mangle]
pub unsafe extern "C" fn del_line() {
    let mut dl1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dl2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tposit: libc::c_int = 0;
    if !d_line.is_null() {
        free(d_line as *mut libc::c_void);
    }
    d_line = malloc((*curr_line).line_length as libc::c_ulong) as *mut libc::c_uchar;
    dl1 = d_line;
    dl2 = point;
    tposit = position;
    while tposit < (*curr_line).line_length {
        *dl1 = *dl2;
        dl1 = dl1.offset(1);
        dl1;
        dl2 = dl2.offset(1);
        dl2;
        tposit += 1;
        tposit;
    }
    (*dlt_line).line_length = 1 as libc::c_int + tposit - position;
    *dl1 = '\0' as i32 as libc::c_uchar;
    *point = '\0' as i32 as libc::c_uchar;
    (*curr_line).line_length = position;
    wclrtoeol(text_win);
    if !((*curr_line).next_line).is_null() {
        right(0 as libc::c_int);
        delete(0 as libc::c_int);
    }
    text_changes = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn undel_line() {
    let mut ud1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ud2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tposit: libc::c_int = 0;
    if (*dlt_line).line_length == 0 as libc::c_int {
        return;
    }
    insert_line(1 as libc::c_int);
    left(1 as libc::c_int);
    point = resiz_line((*dlt_line).line_length, curr_line, position);
    (*curr_line).line_length += (*dlt_line).line_length - 1 as libc::c_int;
    ud1 = point;
    ud2 = d_line;
    tposit = 1 as libc::c_int;
    while tposit < (*dlt_line).line_length {
        tposit += 1;
        tposit;
        *ud1 = *ud2;
        ud1 = ud1.offset(1);
        ud1;
        ud2 = ud2.offset(1);
        ud2;
    }
    *ud1 = '\0' as i32 as libc::c_uchar;
    draw_line(scr_vert, scr_horz, point, position, (*curr_line).line_length);
}
#[no_mangle]
pub unsafe extern "C" fn adv_word() {
    while position < (*curr_line).line_length
        && (*point as libc::c_int != 32 as libc::c_int
            && *point as libc::c_int != 9 as libc::c_int)
    {
        right(1 as libc::c_int);
    }
    while position < (*curr_line).line_length
        && (*point as libc::c_int == 32 as libc::c_int
            || *point as libc::c_int == 9 as libc::c_int)
    {
        right(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn move_rel(mut direction: libc::c_int, mut lines: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if direction == 'u' as i32 {
        scr_pos = 0 as libc::c_int;
        while position > 1 as libc::c_int {
            left(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < lines {
            up();
            i += 1;
            i;
        }
        if last_line > 5 as libc::c_int && scr_vert < 4 as libc::c_int {
            tmp = point as *mut libc::c_char;
            tmp_line = curr_line;
            i = 0 as libc::c_int;
            while i < 5 as libc::c_int && !((*curr_line).prev_line).is_null() {
                up();
                i += 1;
                i;
            }
            scr_vert = scr_vert + i;
            curr_line = tmp_line;
            absolute_lin += i;
            point = tmp as *mut libc::c_uchar;
            scanline(point);
        }
    } else {
        if position != 1 as libc::c_int && !((*curr_line).next_line).is_null() {
            nextline();
            scr_horz = 0 as libc::c_int;
            scr_pos = scr_horz;
            if horiz_offset != 0 {
                horiz_offset = 0 as libc::c_int;
                midscreen(scr_vert, point);
            }
        } else {
            adv_line();
        }
        i = 1 as libc::c_int;
        while i < lines {
            down();
            i += 1;
            i;
        }
        if last_line > 10 as libc::c_int && scr_vert > last_line - 5 as libc::c_int {
            tmp = point as *mut libc::c_char;
            tmp_line = curr_line;
            i = 0 as libc::c_int;
            while i < 5 as libc::c_int && !((*curr_line).next_line).is_null() {
                down();
                i += 1;
                i;
            }
            absolute_lin -= i;
            scr_vert = scr_vert - i;
            curr_line = tmp_line;
            point = tmp as *mut libc::c_uchar;
            scanline(point);
        }
    }
    wmove(text_win, scr_vert, scr_horz - horiz_offset);
}
#[no_mangle]
pub unsafe extern "C" fn eol() {
    if position < (*curr_line).line_length {
        while position < (*curr_line).line_length {
            right(1 as libc::c_int);
        }
    } else if !((*curr_line).next_line).is_null() {
        right(1 as libc::c_int);
        while position < (*curr_line).line_length {
            right(1 as libc::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn bol() {
    if point != (*curr_line).line {
        while point != (*curr_line).line {
            left(1 as libc::c_int);
        }
    } else if !((*curr_line).prev_line).is_null() {
        scr_pos = 0 as libc::c_int;
        up();
    }
}
#[no_mangle]
pub unsafe extern "C" fn adv_line() {
    if point != (*curr_line).line || scr_pos > 0 as libc::c_int {
        while position < (*curr_line).line_length {
            right(1 as libc::c_int);
        }
        right(1 as libc::c_int);
    } else if !((*curr_line).next_line).is_null() {
        scr_pos = 0 as libc::c_int;
        down();
    }
}
#[no_mangle]
pub unsafe extern "C" fn from_top() {
    let mut tmpline: *mut text = first_line;
    let mut x: libc::c_int = 1 as libc::c_int;
    while !tmpline.is_null() && tmpline != curr_line {
        x += 1;
        x;
        tmpline = (*tmpline).next_line;
    }
    absolute_lin = x;
}
#[no_mangle]
pub unsafe extern "C" fn sh_command(mut string: *mut libc::c_char) {
    let mut temp_point: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last_slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut parent: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut return_val: libc::c_int = 0;
    let mut line_holder: *mut text = 0 as *mut text;
    if restrict_mode() != 0 {
        return;
    }
    path = getenv(b"SHELL\0" as *const u8 as *const libc::c_char);
    if path.is_null() {
        path = b"/bin/sh\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    temp_point = path;
    last_slash = temp_point;
    while *temp_point as libc::c_int != '\0' as i32 {
        if *temp_point as libc::c_int == '/' as i32 {
            temp_point = temp_point.offset(1);
            last_slash = temp_point;
        } else {
            temp_point = temp_point.offset(1);
            temp_point;
        }
    }
    if in_pipe == 0 {
        keypad(com_win, 0 as libc::c_int != 0);
        keypad(text_win, 0 as libc::c_int != 0);
        echo();
        nl();
        noraw();
        resetty();
        endwin();
    }
    if in_pipe != 0 {
        pipe(pipe_in.as_mut_ptr());
        parent = fork();
        if parent == 0 {
            in_pipe = 0 as libc::c_int;
            temp_stdout = dup(1 as libc::c_int);
            close(1 as libc::c_int);
            dup(pipe_in[1 as libc::c_int as usize]);
            temp_stderr = dup(2 as libc::c_int);
            close(2 as libc::c_int);
            dup(pipe_in[1 as libc::c_int as usize]);
            close(pipe_in[1 as libc::c_int as usize]);
        } else {
            signal(
                17 as libc::c_int,
                ::core::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t),
            );
            line_holder = curr_line;
            tmp_vert = scr_vert;
            close(pipe_in[1 as libc::c_int as usize]);
            get_fd = pipe_in[0 as libc::c_int as usize];
            get_file(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            close(pipe_in[0 as libc::c_int as usize]);
            scr_vert = tmp_vert;
            scr_pos = 0 as libc::c_int;
            scr_horz = scr_pos;
            position = 1 as libc::c_int;
            curr_line = line_holder;
            from_top();
            point = (*curr_line).line;
            out_pipe = 0 as libc::c_int;
            signal(17 as libc::c_int, None);
        }
    }
    if in_pipe == 0 {
        signal(
            2 as libc::c_int,
            ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        if out_pipe != 0 {
            pipe(pipe_out.as_mut_ptr());
        }
        parent = fork();
        if parent == 0 {
            if shell_fork != 0 {
                putchar('\n' as i32);
            }
            if out_pipe != 0 {
                close(0 as libc::c_int);
                dup(pipe_out[0 as libc::c_int as usize]);
                close(pipe_out[0 as libc::c_int as usize]);
                close(pipe_out[1 as libc::c_int as usize]);
            }
            value = 1 as libc::c_int;
            while value < 24 as libc::c_int {
                signal(value, None);
                value += 1;
                value;
            }
            execl(
                path,
                last_slash,
                b"-c\0" as *const u8 as *const libc::c_char,
                string,
                0 as *mut libc::c_void,
            );
            fprintf(stderr, exec_err_msg, path);
            exit(-(1 as libc::c_int));
        } else {
            if out_pipe != 0 {
                close(pipe_out[0 as libc::c_int as usize]);
                line_holder = first_line;
                while !line_holder.is_null() {
                    write(
                        pipe_out[1 as libc::c_int as usize],
                        (*line_holder).line as *const libc::c_void,
                        ((*line_holder).line_length - 1 as libc::c_int) as size_t,
                    );
                    write(
                        pipe_out[1 as libc::c_int as usize],
                        b"\n\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        1 as libc::c_int as size_t,
                    );
                    line_holder = (*line_holder).next_line;
                }
                close(pipe_out[1 as libc::c_int as usize]);
                out_pipe = 0 as libc::c_int;
            }
            loop {
                return_val = wait(0 as *mut libc::c_int);
                if !(return_val != parent && return_val != -(1 as libc::c_int)) {
                    break;
                }
            }
            if shell_fork == 0 {
                exit(0 as libc::c_int);
            }
        }
        signal(
            2 as libc::c_int,
            Some(edit_abort as unsafe extern "C" fn(libc::c_int) -> ()),
        );
    }
    if shell_fork != 0 {
        fputs(continue_msg, stdout);
        fflush(stdout);
        loop {
            in_0 = getchar();
            if !(in_0 != '\n' as i32) {
                break;
            }
        }
    }
    if in_pipe == 0 {
        reset_prog_mode();
        noecho();
        nonl();
        raw();
        keypad(text_win, 1 as libc::c_int != 0);
        keypad(com_win, 1 as libc::c_int != 0);
        if info_window != 0 {
            clearok(info_win, 1 as libc::c_int != 0);
        }
    }
    redraw();
}
#[no_mangle]
pub unsafe extern "C" fn set_up_term() {
    if curses_initialized == 0 {
        initscr();
        savetty();
        noecho();
        raw();
        nonl();
        curses_initialized = 1 as libc::c_int;
    }
    if LINES > 15 as libc::c_int && COLS >= 80 as libc::c_int && info_window != 0 {
        last_line = LINES - 8 as libc::c_int;
    } else {
        info_window = 0 as libc::c_int;
        last_line = LINES - 2 as libc::c_int;
    }
    idlok(stdscr, 1 as libc::c_int != 0);
    com_win = newwin(1 as libc::c_int, COLS, LINES - 1 as libc::c_int, 0 as libc::c_int);
    keypad(com_win, 1 as libc::c_int != 0);
    idlok(com_win, 1 as libc::c_int != 0);
    wrefresh(com_win);
    if info_window == 0 {
        text_win = newwin(
            LINES - 1 as libc::c_int,
            COLS,
            0 as libc::c_int,
            0 as libc::c_int,
        );
    } else {
        text_win = newwin(
            LINES - 7 as libc::c_int,
            COLS,
            6 as libc::c_int,
            0 as libc::c_int,
        );
    }
    keypad(text_win, 1 as libc::c_int != 0);
    idlok(text_win, 1 as libc::c_int != 0);
    wrefresh(text_win);
    help_win = newwin(
        LINES - 1 as libc::c_int,
        COLS,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    keypad(help_win, 1 as libc::c_int != 0);
    idlok(help_win, 1 as libc::c_int != 0);
    if info_window != 0 {
        info_type = 1 as libc::c_int;
        info_win = newwin(6 as libc::c_int, COLS, 0 as libc::c_int, 0 as libc::c_int);
        werase(info_win);
        paint_info_win();
    }
    last_col = COLS - 1 as libc::c_int;
    local_LINES = LINES;
    local_COLS = COLS;
}
#[no_mangle]
pub unsafe extern "C" fn resize_check() {
    if LINES == local_LINES && COLS == local_COLS {
        return;
    }
    if info_window != 0 {
        delwin(info_win);
    }
    delwin(text_win);
    delwin(com_win);
    delwin(help_win);
    set_up_term();
    redraw();
    wrefresh(text_win);
}
static mut item_alpha: [libc::c_char; 38] = unsafe {
    *::core::mem::transmute::<
        &[u8; 38],
        &mut [libc::c_char; 38],
    >(b"abcdefghijklmnopqrstuvwxyz0123456789 \0")
};
#[no_mangle]
pub unsafe extern "C" fn menu_op(mut menu_list: *mut menu_entries) -> libc::c_int {
    let mut temp_win: *mut WINDOW = 0 as *mut WINDOW;
    let mut max_width: libc::c_int = 0;
    let mut max_height: libc::c_int = 0;
    let mut x_off: libc::c_int = 0;
    let mut y_off: libc::c_int = 0;
    let mut counter: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut input: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut list_size: libc::c_int = 0;
    let mut top_offset: libc::c_int = 0;
    let mut vert_size: libc::c_int = 0;
    let mut off_start: libc::c_int = 1 as libc::c_int;
    list_size = 1 as libc::c_int;
    while !((*menu_list.offset((list_size + 1 as libc::c_int) as isize)).item_string)
        .is_null()
    {
        list_size += 1;
        list_size;
    }
    max_width = 0 as libc::c_int;
    counter = 0 as libc::c_int;
    while counter <= list_size {
        length = strlen((*menu_list.offset(counter as isize)).item_string)
            as libc::c_int;
        if length > max_width {
            max_width = length;
        }
        counter += 1;
        counter;
    }
    max_width += 3 as libc::c_int;
    max_width = (if max_width as libc::c_ulong > strlen(menu_cancel_msg) {
        max_width as libc::c_ulong
    } else {
        strlen(menu_cancel_msg)
    }) as libc::c_int;
    max_width = (if max_width as libc::c_ulong
        > (if strlen(more_above_str) > strlen(more_below_str) {
            strlen(more_above_str)
        } else {
            strlen(more_below_str)
        })
    {
        max_width as libc::c_ulong
    } else if strlen(more_above_str) > strlen(more_below_str) {
        strlen(more_above_str)
    } else {
        strlen(more_below_str)
    }) as libc::c_int;
    max_width += 6 as libc::c_int;
    if max_width > COLS {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        werase(com_win);
        wprintw(com_win, b"%s\0" as *const u8 as *const libc::c_char, menu_too_lrg_msg);
        wrefresh(com_win);
        clear_com_win = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    top_offset = 0 as libc::c_int;
    if list_size > LINES {
        max_height = LINES;
        if max_height > 11 as libc::c_int {
            vert_size = max_height - 8 as libc::c_int;
        } else {
            vert_size = max_height;
        }
    } else {
        vert_size = list_size;
        max_height = list_size;
    }
    if LINES >= vert_size + 8 as libc::c_int {
        if (*menu_list.offset(0 as libc::c_int as isize)).argument != 1 as libc::c_int {
            max_height = vert_size + 8 as libc::c_int;
        } else {
            max_height = vert_size + 7 as libc::c_int;
        }
        top_offset = 4 as libc::c_int;
    }
    x_off = (COLS - max_width) / 2 as libc::c_int;
    y_off = (LINES - max_height - 1 as libc::c_int) / 2 as libc::c_int;
    temp_win = newwin(max_height, max_width, y_off, x_off);
    keypad(temp_win, 1 as libc::c_int != 0);
    paint_menu(
        menu_list,
        max_width,
        max_height,
        list_size,
        top_offset,
        temp_win,
        off_start,
        vert_size,
    );
    counter = 1 as libc::c_int;
    loop {
        if off_start > 2 as libc::c_int {
            wmove(
                temp_win,
                1 as libc::c_int + counter + top_offset - off_start,
                3 as libc::c_int,
            );
        } else {
            wmove(temp_win, counter + top_offset - off_start, 3 as libc::c_int);
        }
        wrefresh(temp_win);
        in_0 = wgetch(temp_win);
        input = in_0;
        if input == -(1 as libc::c_int) {
            exit(0 as libc::c_int);
        }
        if input & !(0x7f as libc::c_int) == 0 as libc::c_int
            && *(*__ctype_b_loc()).offset(input as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            if *(*__ctype_b_loc()).offset(input as isize) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                temp = 1 as libc::c_int + tolower(input) - 'a' as i32;
            } else if *(*__ctype_b_loc()).offset(input as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                temp = 2 as libc::c_int + 'z' as i32 - 'a' as i32 + (input - '0' as i32);
            }
            if temp <= list_size {
                input = '\n' as i32;
                counter = temp;
            }
        } else {
            let mut current_block_70: u64;
            match input {
                4 => {
                    current_block_70 = 465217320847907322;
                }
                32 | 261 | 258 => {
                    current_block_70 = 465217320847907322;
                }
                8 => {
                    current_block_70 = 17577181410495481415;
                }
                21 => {
                    current_block_70 = 17577181410495481415;
                }
                127 | 263 | 260 | 259 => {
                    current_block_70 = 12747546757854770334;
                }
                27 => {
                    if (*menu_list.offset(0 as libc::c_int as isize)).argument
                        != 1 as libc::c_int
                    {
                        counter = 0 as libc::c_int;
                    }
                    current_block_70 = 7494008139977416618;
                }
                12 | 18 => {
                    paint_menu(
                        menu_list,
                        max_width,
                        max_height,
                        list_size,
                        top_offset,
                        temp_win,
                        off_start,
                        vert_size,
                    );
                    current_block_70 = 7494008139977416618;
                }
                _ => {
                    current_block_70 = 7494008139977416618;
                }
            }
            match current_block_70 {
                465217320847907322 => {
                    counter += 1;
                    counter;
                    if counter > list_size {
                        counter = 1 as libc::c_int;
                    }
                    current_block_70 = 7494008139977416618;
                }
                17577181410495481415 => {
                    current_block_70 = 12747546757854770334;
                }
                _ => {}
            }
            match current_block_70 {
                12747546757854770334 => {
                    counter -= 1;
                    counter;
                    if counter == 0 as libc::c_int {
                        counter = list_size;
                    }
                }
                _ => {}
            }
        }
        if list_size - off_start >= vert_size - 1 as libc::c_int
            && counter > off_start + vert_size - 3 as libc::c_int
            && off_start > 1 as libc::c_int
        {
            if counter == list_size {
                off_start = list_size - vert_size + 2 as libc::c_int;
            } else {
                off_start += 1;
                off_start;
            }
            paint_menu(
                menu_list,
                max_width,
                max_height,
                list_size,
                top_offset,
                temp_win,
                off_start,
                vert_size,
            );
        } else if list_size != vert_size
            && counter > off_start + vert_size - 2 as libc::c_int
        {
            if counter == list_size {
                off_start = 2 as libc::c_int + (list_size - vert_size);
            } else if off_start == 1 as libc::c_int {
                off_start = 3 as libc::c_int;
            } else {
                off_start += 1;
                off_start;
            }
            paint_menu(
                menu_list,
                max_width,
                max_height,
                list_size,
                top_offset,
                temp_win,
                off_start,
                vert_size,
            );
        } else if counter < off_start {
            if counter <= 2 as libc::c_int {
                off_start = 1 as libc::c_int;
            } else {
                off_start = counter;
            }
            paint_menu(
                menu_list,
                max_width,
                max_height,
                list_size,
                top_offset,
                temp_win,
                off_start,
                vert_size,
            );
        }
        if !(input != '\r' as i32 && input != '\n' as i32 && counter != 0 as libc::c_int)
        {
            break;
        }
    }
    werase(temp_win);
    wrefresh(temp_win);
    delwin(temp_win);
    if ((*menu_list.offset(counter as isize)).procedure).is_some()
        || ((*menu_list.offset(counter as isize)).iprocedure).is_some()
        || ((*menu_list.offset(counter as isize)).nprocedure).is_some()
    {
        if (*menu_list.offset(counter as isize)).argument != -(1 as libc::c_int) {
            (Some(
                ((*menu_list.offset(counter as isize)).iprocedure)
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )((*menu_list.offset(counter as isize)).argument);
        } else if !((*menu_list.offset(counter as isize)).ptr_argument).is_null() {
            (Some(
                ((*menu_list.offset(counter as isize)).procedure)
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )((*menu_list.offset(counter as isize)).ptr_argument);
        } else {
            (Some(
                ((*menu_list.offset(counter as isize)).nprocedure)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")();
        }
    }
    if info_window != 0 {
        paint_info_win();
    }
    redraw();
    return counter;
}
#[no_mangle]
pub unsafe extern "C" fn paint_menu(
    mut menu_list: *mut menu_entries,
    mut max_width: libc::c_int,
    mut max_height: libc::c_int,
    mut list_size: libc::c_int,
    mut top_offset: libc::c_int,
    mut menu_win: *mut WINDOW,
    mut off_start: libc::c_int,
    mut vert_size: libc::c_int,
) {
    let mut counter: libc::c_int = 0;
    let mut temp_int: libc::c_int = 0;
    werase(menu_win);
    if max_height > vert_size {
        wmove(menu_win, 1 as libc::c_int, 1 as libc::c_int);
        if nohighlight == 0 {
            wattrset(
                menu_win,
                ((1 as libc::c_uint) << 8 as libc::c_int + 8 as libc::c_int)
                    as libc::c_int,
            );
        }
        waddch(menu_win, '+' as i32 as chtype);
        counter = 0 as libc::c_int;
        while counter < max_width - 4 as libc::c_int {
            waddch(menu_win, '-' as i32 as chtype);
            counter += 1;
            counter;
        }
        waddch(menu_win, '+' as i32 as chtype);
        wmove(menu_win, max_height - 2 as libc::c_int, 1 as libc::c_int);
        waddch(menu_win, '+' as i32 as chtype);
        counter = 0 as libc::c_int;
        while counter < max_width - 4 as libc::c_int {
            waddch(menu_win, '-' as i32 as chtype);
            counter += 1;
            counter;
        }
        waddch(menu_win, '+' as i32 as chtype);
        wattrset(
            menu_win,
            (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint) as libc::c_int,
        );
        wmove(menu_win, 2 as libc::c_int, 3 as libc::c_int);
        waddnstr(
            menu_win,
            (*menu_list.offset(0 as libc::c_int as isize)).item_string,
            -(1 as libc::c_int),
        );
        wmove(menu_win, max_height - 3 as libc::c_int, 3 as libc::c_int);
        if (*menu_list.offset(0 as libc::c_int as isize)).argument != 1 as libc::c_int {
            waddnstr(menu_win, menu_cancel_msg, -(1 as libc::c_int));
        }
    }
    if nohighlight == 0 {
        wattrset(
            menu_win,
            ((1 as libc::c_uint) << 8 as libc::c_int + 8 as libc::c_int) as libc::c_int,
        );
    }
    counter = 0 as libc::c_int;
    while counter < vert_size + top_offset {
        if top_offset == 4 as libc::c_int {
            temp_int = counter + 2 as libc::c_int;
        } else {
            temp_int = counter;
        }
        wmove(menu_win, temp_int, 1 as libc::c_int);
        waddch(menu_win, '|' as i32 as chtype);
        wmove(menu_win, temp_int, max_width - 2 as libc::c_int);
        waddch(menu_win, '|' as i32 as chtype);
        counter += 1;
        counter;
    }
    wattrset(
        menu_win,
        (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint) as libc::c_int,
    );
    if list_size > vert_size {
        if off_start >= 3 as libc::c_int {
            temp_int = 1 as libc::c_int;
            wmove(menu_win, top_offset, 3 as libc::c_int);
            waddnstr(menu_win, more_above_str, -(1 as libc::c_int));
        } else {
            temp_int = 0 as libc::c_int;
        }
        counter = off_start;
        while temp_int + counter - off_start < vert_size - 1 as libc::c_int {
            wmove(
                menu_win,
                top_offset + temp_int + (counter - off_start),
                3 as libc::c_int,
            );
            if list_size > 1 as libc::c_int {
                wprintw(
                    menu_win,
                    b"%c) \0" as *const u8 as *const libc::c_char,
                    item_alpha[(if (counter - 1 as libc::c_int) < 36 as libc::c_int {
                        counter - 1 as libc::c_int
                    } else {
                        36 as libc::c_int
                    }) as usize] as libc::c_int,
                );
            }
            waddnstr(
                menu_win,
                (*menu_list.offset(counter as isize)).item_string,
                -(1 as libc::c_int),
            );
            counter += 1;
            counter;
        }
        wmove(menu_win, top_offset + (vert_size - 1 as libc::c_int), 3 as libc::c_int);
        if counter == list_size {
            if list_size > 1 as libc::c_int {
                wprintw(
                    menu_win,
                    b"%c) \0" as *const u8 as *const libc::c_char,
                    item_alpha[(if (counter - 1 as libc::c_int) < 36 as libc::c_int {
                        counter - 1 as libc::c_int
                    } else {
                        36 as libc::c_int
                    }) as usize] as libc::c_int,
                );
            }
            wprintw(
                menu_win,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*menu_list.offset(counter as isize)).item_string,
            );
        } else {
            wprintw(
                menu_win,
                b"%s\0" as *const u8 as *const libc::c_char,
                more_below_str,
            );
        }
    } else {
        counter = 1 as libc::c_int;
        while counter <= list_size {
            wmove(menu_win, top_offset + counter - 1 as libc::c_int, 3 as libc::c_int);
            if list_size > 1 as libc::c_int {
                wprintw(
                    menu_win,
                    b"%c) \0" as *const u8 as *const libc::c_char,
                    item_alpha[(if (counter - 1 as libc::c_int) < 36 as libc::c_int {
                        counter - 1 as libc::c_int
                    } else {
                        36 as libc::c_int
                    }) as usize] as libc::c_int,
                );
            }
            waddnstr(
                menu_win,
                (*menu_list.offset(counter as isize)).item_string,
                -(1 as libc::c_int),
            );
            counter += 1;
            counter;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn help() {
    let mut counter: libc::c_int = 0;
    werase(help_win);
    clearok(help_win, 1 as libc::c_int != 0);
    counter = 0 as libc::c_int;
    while counter < 22 as libc::c_int {
        wmove(help_win, counter, 0 as libc::c_int);
        waddnstr(
            help_win,
            if emacs_keys_mode != 0 {
                emacs_help_text[counter as usize]
            } else {
                help_text[counter as usize]
            },
            -(1 as libc::c_int),
        );
        counter += 1;
        counter;
    }
    wrefresh(help_win);
    werase(com_win);
    wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
    wprintw(com_win, b"%s\0" as *const u8 as *const libc::c_char, press_any_key_msg);
    wrefresh(com_win);
    counter = wgetch(com_win);
    if counter == -(1 as libc::c_int) {
        exit(0 as libc::c_int);
    }
    werase(com_win);
    wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
    werase(help_win);
    wrefresh(help_win);
    wrefresh(com_win);
    redraw();
}
#[no_mangle]
pub unsafe extern "C" fn paint_info_win() {
    let mut counter: libc::c_int = 0;
    if info_window == 0 {
        return;
    }
    werase(info_win);
    counter = 0 as libc::c_int;
    while counter < 5 as libc::c_int {
        wmove(info_win, counter, 0 as libc::c_int);
        wclrtoeol(info_win);
        if info_type == 1 as libc::c_int {
            waddnstr(
                info_win,
                if emacs_keys_mode != 0 {
                    emacs_control_keys[counter as usize]
                } else {
                    control_keys[counter as usize]
                },
                -(1 as libc::c_int),
            );
        } else if info_type == 2 as libc::c_int {
            waddnstr(info_win, command_strings[counter as usize], -(1 as libc::c_int));
        }
        counter += 1;
        counter;
    }
    wmove(info_win, 5 as libc::c_int, 0 as libc::c_int);
    if nohighlight == 0 {
        wattrset(
            info_win,
            ((1 as libc::c_uint) << 8 as libc::c_int + 8 as libc::c_int) as libc::c_int,
        );
    }
    waddnstr(info_win, separator, -(1 as libc::c_int));
    wattrset(
        info_win,
        (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint) as libc::c_int,
    );
    wrefresh(info_win);
}
#[no_mangle]
pub unsafe extern "C" fn no_info_window() {
    if info_window == 0 {
        return;
    }
    delwin(info_win);
    delwin(text_win);
    info_window = 0 as libc::c_int;
    last_line = LINES - 2 as libc::c_int;
    text_win = newwin(
        LINES - 1 as libc::c_int,
        COLS,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    keypad(text_win, 1 as libc::c_int != 0);
    idlok(text_win, 1 as libc::c_int != 0);
    clearok(text_win, 1 as libc::c_int != 0);
    midscreen(scr_vert, point);
    wrefresh(text_win);
    clear_com_win = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn create_info_window() {
    if info_window != 0 {
        return;
    }
    last_line = LINES - 8 as libc::c_int;
    delwin(text_win);
    text_win = newwin(
        LINES - 7 as libc::c_int,
        COLS,
        6 as libc::c_int,
        0 as libc::c_int,
    );
    keypad(text_win, 1 as libc::c_int != 0);
    idlok(text_win, 1 as libc::c_int != 0);
    werase(text_win);
    info_window = 1 as libc::c_int;
    info_win = newwin(6 as libc::c_int, COLS, 0 as libc::c_int, 0 as libc::c_int);
    werase(info_win);
    info_type = 1 as libc::c_int;
    midscreen(if scr_vert < last_line { scr_vert } else { last_line }, point);
    clearok(info_win, 1 as libc::c_int != 0);
    paint_info_win();
    wrefresh(text_win);
    clear_com_win = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn file_op(mut arg: libc::c_int) -> libc::c_int {
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flag: libc::c_int = 0;
    if restrict_mode() != 0 {
        return 0 as libc::c_int;
    }
    if arg == 1 as libc::c_int {
        string = get_string(file_read_prompt_str, 1 as libc::c_int);
        recv_file = 1 as libc::c_int;
        tmp_file = resolve_name(string);
        check_fp();
        if tmp_file != string {
            free(tmp_file as *mut libc::c_void);
        }
        free(string as *mut libc::c_void);
    } else if arg == 2 as libc::c_int {
        string = get_string(file_write_prompt_str, 1 as libc::c_int);
        tmp_file = resolve_name(string);
        write_file(tmp_file, 1 as libc::c_int);
        if tmp_file != string {
            free(tmp_file as *mut libc::c_void);
        }
        free(string as *mut libc::c_void);
    } else if arg == 3 as libc::c_int {
        if !in_file_name.is_null() {
            flag = 1 as libc::c_int;
        } else {
            flag = 0 as libc::c_int;
        }
        string = in_file_name as *mut libc::c_char;
        if string.is_null() || *string as libc::c_int == '\0' as i32 {
            string = get_string(save_file_name_prompt, 1 as libc::c_int);
        }
        if string.is_null() || *string as libc::c_int == '\0' as i32 {
            wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
            wprintw(
                com_win,
                b"%s\0" as *const u8 as *const libc::c_char,
                file_not_saved_msg,
            );
            wclrtoeol(com_win);
            wrefresh(com_win);
            clear_com_win = 1 as libc::c_int;
            return 0 as libc::c_int;
        }
        if flag == 0 {
            tmp_file = resolve_name(string);
            if tmp_file != string {
                free(string as *mut libc::c_void);
                string = tmp_file;
            }
        }
        if write_file(string, 1 as libc::c_int) != 0 {
            in_file_name = string as *mut libc::c_uchar;
            text_changes = 0 as libc::c_int;
        } else if flag == 0 {
            free(string as *mut libc::c_void);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn shell_op() {
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    string = get_string(shell_prompt, 1 as libc::c_int);
    if !string.is_null() && *string as libc::c_int != '\0' as i32 {
        sh_command(string);
        free(string as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn leave_op() {
    if text_changes != 0 {
        menu_op(leave_menu.as_mut_ptr());
    } else {
        quit(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn redraw() {
    if info_window != 0 {
        clearok(info_win, 1 as libc::c_int != 0);
        paint_info_win();
    } else {
        clearok(text_win, 1 as libc::c_int != 0);
    }
    midscreen(scr_vert, point);
}
#[no_mangle]
pub unsafe extern "C" fn Blank_Line(mut test_line: *mut text) -> libc::c_int {
    let mut line: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut length: libc::c_int = 0;
    if test_line.is_null() {
        return 1 as libc::c_int;
    }
    length = 1 as libc::c_int;
    line = (*test_line).line;
    if *line as libc::c_int == '.' as i32 || *line as libc::c_int == '>' as i32 {
        return 1 as libc::c_int;
    }
    while (*line as libc::c_int == ' ' as i32 || *line as libc::c_int == '\t' as i32)
        && length < (*test_line).line_length
    {
        length += 1;
        length;
        line = line.offset(1);
        line;
    }
    if length != (*test_line).line_length {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn Format() {
    let mut string_count: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut temp_case: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut tmp_af: libc::c_int = 0;
    let mut counter: libc::c_int = 0;
    let mut line: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp_srchstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp_dword: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp_d_char: [libc::c_uchar; 3] = [0; 3];
    temp_d_char[0 as libc::c_int as usize] = *d_char.offset(0 as libc::c_int as isize);
    temp_d_char[1 as libc::c_int as usize] = *d_char.offset(1 as libc::c_int as isize);
    temp_d_char[2 as libc::c_int as usize] = *d_char.offset(2 as libc::c_int as isize);
    if observ_margins == 0 || Blank_Line(curr_line) != 0 {
        return;
    }
    wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
    wclrtoeol(com_win);
    wprintw(com_win, b"%s\0" as *const u8 as *const libc::c_char, formatting_msg);
    wrefresh(com_win);
    tmp_af = auto_format;
    auto_format = 0 as libc::c_int;
    offset = position;
    if position != 1 as libc::c_int {
        prev_word();
    }
    temp_dword = d_word;
    d_word = 0 as *mut libc::c_uchar;
    temp_case = case_sen;
    case_sen = 1 as libc::c_int;
    tmp_srchstr = srch_str;
    srch_str = malloc(
        (1 as libc::c_int + (*curr_line).line_length - position) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    temp2 = srch_str;
    if *point as libc::c_int == ' ' as i32 || *point as libc::c_int == '\t' as i32 {
        adv_word();
    }
    offset -= position;
    counter = position;
    temp1 = point;
    line = temp1;
    while *temp1 as libc::c_int != '\0' as i32 && *temp1 as libc::c_int != ' ' as i32
        && *temp1 as libc::c_int != '\t' as i32 && counter < (*curr_line).line_length
    {
        *temp2 = *temp1;
        temp2 = temp2.offset(1);
        temp2;
        temp1 = temp1.offset(1);
        temp1;
        counter += 1;
        counter;
    }
    *temp2 = '\0' as i32 as libc::c_uchar;
    if position != 1 as libc::c_int {
        bol();
    }
    while Blank_Line((*curr_line).prev_line) == 0 {
        bol();
    }
    string_count = 0 as libc::c_int;
    status = 1 as libc::c_int;
    while line != point && status != 0 {
        status = search(0 as libc::c_int);
        string_count += 1;
        string_count;
    }
    wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
    wclrtoeol(com_win);
    wprintw(com_win, b"%s\0" as *const u8 as *const libc::c_char, formatting_msg);
    wrefresh(com_win);
    if position != 1 as libc::c_int {
        bol();
    }
    while Blank_Line((*curr_line).prev_line) == 0 {
        bol();
    }
    observ_margins = 0 as libc::c_int;
    while Blank_Line((*curr_line).next_line) == 0 {
        eol();
        left(1 as libc::c_int);
        if *point as libc::c_int != ' ' as i32 {
            right(1 as libc::c_int);
            insert(' ' as i32);
        } else {
            right(1 as libc::c_int);
        }
        del_char();
        if *point as libc::c_int == ' ' as i32 || *point as libc::c_int == '\t' as i32 {
            del_word();
        }
    }
    bol();
    adv_word();
    while position < (*curr_line).line_length {
        if *point as libc::c_int == ' ' as i32
            && *point.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32
        {
            del_char();
        } else {
            right(1 as libc::c_int);
        }
    }
    bol();
    while position < (*curr_line).line_length {
        if *point as libc::c_int == '.' as i32
            && *point.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32
        {
            right(1 as libc::c_int);
            insert(' ' as i32);
            insert(' ' as i32);
            while *point as libc::c_int == ' ' as i32 {
                del_char();
            }
        }
        right(1 as libc::c_int);
    }
    observ_margins = 1 as libc::c_int;
    bol();
    wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
    wclrtoeol(com_win);
    wprintw(com_win, b"%s\0" as *const u8 as *const libc::c_char, formatting_msg);
    wrefresh(com_win);
    while position < (*curr_line).line_length {
        while scr_pos < right_margin && position < (*curr_line).line_length {
            right(1 as libc::c_int);
        }
        if position < (*curr_line).line_length {
            prev_word();
            if position == 1 as libc::c_int {
                adv_word();
            }
            insert_line(1 as libc::c_int);
        }
    }
    bol();
    while Blank_Line((*curr_line).prev_line) == 0 {
        bol();
    }
    while status != 0 && string_count > 0 as libc::c_int {
        search(0 as libc::c_int);
        string_count -= 1;
        string_count;
    }
    while offset > 0 as libc::c_int {
        offset -= 1;
        offset;
        right(1 as libc::c_int);
    }
    if !d_word.is_null() {
        free(d_word as *mut libc::c_void);
    }
    d_word = temp_dword;
    case_sen = temp_case;
    free(srch_str as *mut libc::c_void);
    srch_str = tmp_srchstr;
    *d_char.offset(0 as libc::c_int as isize) = temp_d_char[0 as libc::c_int as usize];
    *d_char.offset(1 as libc::c_int as isize) = temp_d_char[1 as libc::c_int as usize];
    *d_char.offset(2 as libc::c_int as isize) = temp_d_char[2 as libc::c_int as usize];
    auto_format = tmp_af;
    midscreen(scr_vert, point);
    werase(com_win);
    wrefresh(com_win);
}
#[no_mangle]
pub static mut init_name: [*mut libc::c_uchar; 3] = [
    b"/usr/share/misc/init.ee\0" as *const u8 as *const libc::c_char
        as *mut libc::c_uchar,
    0 as *const libc::c_uchar as *mut libc::c_uchar,
    b".init.ee\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn ee_init() {
    let mut init_file: *mut FILE = 0 as *mut FILE;
    let mut string: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut str1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut str2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut home: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut counter: libc::c_int = 0;
    let mut temp_int: libc::c_int = 0;
    string = getenv(b"HOME\0" as *const u8 as *const libc::c_char) as *mut libc::c_uchar;
    if string.is_null() {
        string = b"/tmp\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar;
    }
    home = malloc(
        (strlen(string as *const libc::c_char))
            .wrapping_add(10 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    str1 = home as *mut libc::c_uchar;
    strcpy(home, string as *const libc::c_char);
    strcat(home, b"/.init.ee\0" as *const u8 as *const libc::c_char);
    init_name[1 as libc::c_int as usize] = home as *mut libc::c_uchar;
    string = malloc(512 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    counter = 0 as libc::c_int;
    while counter < 3 as libc::c_int {
        if access(init_name[counter as usize] as *const libc::c_char, 4 as libc::c_int)
            == 0
        {
            init_file = fopen(
                init_name[counter as usize] as *const libc::c_char,
                b"r\0" as *const u8 as *const libc::c_char,
            );
            loop {
                str2 = fgets(string as *mut libc::c_char, 512 as libc::c_int, init_file)
                    as *mut libc::c_uchar;
                if str2.is_null() {
                    break;
                }
                str2 = string;
                str1 = str2;
                while *str2 as libc::c_int != '\n' as i32 {
                    str2 = str2.offset(1);
                    str2;
                }
                *str2 = '\0' as i32 as libc::c_uchar;
                if unique_test(string as *mut libc::c_char, init_strings.as_mut_ptr())
                    != 1 as libc::c_int
                {
                    continue;
                }
                if compare(str1 as *mut libc::c_char, CASE, 0 as libc::c_int) != 0 {
                    case_sen = 1 as libc::c_int;
                } else if compare(str1 as *mut libc::c_char, NOCASE, 0 as libc::c_int)
                    != 0
                {
                    case_sen = 0 as libc::c_int;
                } else if compare(str1 as *mut libc::c_char, EXPAND, 0 as libc::c_int)
                    != 0
                {
                    expand_tabs = 1 as libc::c_int;
                } else if compare(str1 as *mut libc::c_char, NOEXPAND, 0 as libc::c_int)
                    != 0
                {
                    expand_tabs = 0 as libc::c_int;
                } else if compare(str1 as *mut libc::c_char, INFO, 0 as libc::c_int) != 0
                {
                    info_window = 1 as libc::c_int;
                } else if compare(str1 as *mut libc::c_char, NOINFO, 0 as libc::c_int)
                    != 0
                {
                    info_window = 0 as libc::c_int;
                } else if compare(str1 as *mut libc::c_char, MARGINS, 0 as libc::c_int)
                    != 0
                {
                    observ_margins = 1 as libc::c_int;
                } else if compare(str1 as *mut libc::c_char, NOMARGINS, 0 as libc::c_int)
                    != 0
                {
                    observ_margins = 0 as libc::c_int;
                } else if compare(
                    str1 as *mut libc::c_char,
                    AUTOFORMAT,
                    0 as libc::c_int,
                ) != 0
                {
                    auto_format = 1 as libc::c_int;
                    observ_margins = 1 as libc::c_int;
                } else if compare(
                    str1 as *mut libc::c_char,
                    NOAUTOFORMAT,
                    0 as libc::c_int,
                ) != 0
                {
                    auto_format = 0 as libc::c_int;
                } else if compare(str1 as *mut libc::c_char, Echo, 0 as libc::c_int) != 0
                {
                    str1 = next_word(str1);
                    if *str1 as libc::c_int != '\0' as i32 {
                        echo_string(str1 as *mut libc::c_char);
                    }
                } else if compare(
                    str1 as *mut libc::c_char,
                    PRINTCOMMAND,
                    0 as libc::c_int,
                ) != 0
                {
                    str1 = next_word(str1);
                    print_command = malloc(
                        (strlen(str1 as *const libc::c_char))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_uchar;
                    strcpy(
                        print_command as *mut libc::c_char,
                        str1 as *const libc::c_char,
                    );
                } else if compare(
                    str1 as *mut libc::c_char,
                    RIGHTMARGIN,
                    0 as libc::c_int,
                ) != 0
                {
                    str1 = next_word(str1);
                    if *str1 as libc::c_int >= '0' as i32
                        && *str1 as libc::c_int <= '9' as i32
                    {
                        temp_int = atoi(str1 as *const libc::c_char);
                        if temp_int > 0 as libc::c_int {
                            right_margin = temp_int;
                        }
                    }
                } else if compare(str1 as *mut libc::c_char, HIGHLIGHT, 0 as libc::c_int)
                    != 0
                {
                    nohighlight = 0 as libc::c_int;
                } else if compare(
                    str1 as *mut libc::c_char,
                    NOHIGHLIGHT,
                    0 as libc::c_int,
                ) != 0
                {
                    nohighlight = 1 as libc::c_int;
                } else if compare(str1 as *mut libc::c_char, EIGHTBIT, 0 as libc::c_int)
                    != 0
                {
                    eightbit = 1 as libc::c_int;
                } else if compare(
                    str1 as *mut libc::c_char,
                    NOEIGHTBIT,
                    0 as libc::c_int,
                ) != 0
                {
                    eightbit = 0 as libc::c_int;
                    ee_chinese = 0 as libc::c_int;
                } else if compare(
                    str1 as *mut libc::c_char,
                    EMACS_string,
                    0 as libc::c_int,
                ) != 0
                {
                    emacs_keys_mode = 1 as libc::c_int;
                } else if compare(
                    str1 as *mut libc::c_char,
                    NOEMACS_string,
                    0 as libc::c_int,
                ) != 0
                {
                    emacs_keys_mode = 0 as libc::c_int;
                } else if compare(
                    str1 as *mut libc::c_char,
                    chinese_cmd,
                    0 as libc::c_int,
                ) != 0
                {
                    ee_chinese = 1 as libc::c_int;
                    eightbit = 1 as libc::c_int;
                } else if compare(
                    str1 as *mut libc::c_char,
                    nochinese_cmd,
                    0 as libc::c_int,
                ) != 0
                {
                    ee_chinese = 0 as libc::c_int;
                }
            }
            fclose(init_file);
        }
        counter += 1;
        counter;
    }
    free(string as *mut libc::c_void);
    free(home as *mut libc::c_void);
    string = getenv(b"LANG\0" as *const u8 as *const libc::c_char) as *mut libc::c_uchar;
    if !string.is_null() {
        if strcmp(
            string as *const libc::c_char,
            b"zh_TW.big5\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            ee_chinese = 1 as libc::c_int;
            eightbit = 1 as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dump_ee_conf() {
    let mut init_file: *mut FILE = 0 as *mut FILE;
    let mut old_init_file: *mut FILE = 0 as *mut FILE;
    let mut file_name: *mut libc::c_char = b".init.ee\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut home_dir: *mut libc::c_char = b"~/.init.ee\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut buffer: [libc::c_char; 512] = [0; 512];
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: libc::c_int = 0;
    let mut option: libc::c_int = 0 as libc::c_int;
    if restrict_mode() != 0 {
        return;
    }
    option = menu_op(config_dump_menu.as_mut_ptr());
    werase(com_win);
    wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
    if option == 0 as libc::c_int {
        wprintw(
            com_win,
            b"%s\0" as *const u8 as *const libc::c_char,
            conf_not_saved_msg,
        );
        wrefresh(com_win);
        return;
    } else if option == 2 as libc::c_int {
        file_name = resolve_name(home_dir);
    }
    if stat(file_name, &mut buf) != -(1 as libc::c_int) {
        sprintf(
            buffer.as_mut_ptr(),
            b"%s.old\0" as *const u8 as *const libc::c_char,
            file_name,
        );
        unlink(buffer.as_mut_ptr());
        link(file_name, buffer.as_mut_ptr());
        unlink(file_name);
        old_init_file = fopen(
            buffer.as_mut_ptr(),
            b"r\0" as *const u8 as *const libc::c_char,
        );
    }
    init_file = fopen(file_name, b"w\0" as *const u8 as *const libc::c_char);
    if init_file.is_null() {
        wprintw(com_win, b"%s\0" as *const u8 as *const libc::c_char, conf_dump_err_msg);
        wrefresh(com_win);
        return;
    }
    if !old_init_file.is_null() {
        loop {
            string = fgets(buffer.as_mut_ptr(), 512 as libc::c_int, old_init_file);
            if string.is_null() {
                break;
            }
            length = strlen(string) as libc::c_int;
            *string
                .offset(
                    (length - 1 as libc::c_int) as isize,
                ) = '\0' as i32 as libc::c_char;
            if unique_test(string, init_strings.as_mut_ptr()) == 1 as libc::c_int {
                if compare(string, Echo, 0 as libc::c_int) != 0 {
                    fprintf(
                        init_file,
                        b"%s\n\0" as *const u8 as *const libc::c_char,
                        string,
                    );
                }
            } else {
                fprintf(
                    init_file,
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    string,
                );
            }
        }
        fclose(old_init_file);
    }
    fprintf(
        init_file,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        if case_sen != 0 { CASE } else { NOCASE },
    );
    fprintf(
        init_file,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        if expand_tabs != 0 { EXPAND } else { NOEXPAND },
    );
    fprintf(
        init_file,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        if info_window != 0 { INFO } else { NOINFO },
    );
    fprintf(
        init_file,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        if observ_margins != 0 { MARGINS } else { NOMARGINS },
    );
    fprintf(
        init_file,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        if auto_format != 0 { AUTOFORMAT } else { NOAUTOFORMAT },
    );
    fprintf(
        init_file,
        b"%s %s\n\0" as *const u8 as *const libc::c_char,
        PRINTCOMMAND,
        print_command,
    );
    fprintf(
        init_file,
        b"%s %d\n\0" as *const u8 as *const libc::c_char,
        RIGHTMARGIN,
        right_margin,
    );
    fprintf(
        init_file,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        if nohighlight != 0 { NOHIGHLIGHT } else { HIGHLIGHT },
    );
    fprintf(
        init_file,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        if eightbit != 0 { EIGHTBIT } else { NOEIGHTBIT },
    );
    fprintf(
        init_file,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        if emacs_keys_mode != 0 { EMACS_string } else { NOEMACS_string },
    );
    fprintf(
        init_file,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        if ee_chinese != 0 { chinese_cmd } else { nochinese_cmd },
    );
    fclose(init_file);
    wprintw(com_win, conf_dump_success_msg, file_name);
    wrefresh(com_win);
    if option == 2 as libc::c_int && file_name != home_dir {
        free(file_name as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn echo_string(mut string: *mut libc::c_char) {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut Counter: libc::c_int = 0;
    temp = string;
    while *temp as libc::c_int != '\0' as i32 {
        if *temp as libc::c_int == '\\' as i32 {
            temp = temp.offset(1);
            temp;
            if *temp as libc::c_int == 'n' as i32 {
                putchar('\n' as i32);
            } else if *temp as libc::c_int == 't' as i32 {
                putchar('\t' as i32);
            } else if *temp as libc::c_int == 'b' as i32 {
                putchar('\u{8}' as i32);
            } else if *temp as libc::c_int == 'r' as i32 {
                putchar('\r' as i32);
            } else if *temp as libc::c_int == 'f' as i32 {
                putchar('\u{c}' as i32);
            } else if *temp as libc::c_int == 'e' as i32
                || *temp as libc::c_int == 'E' as i32
            {
                putchar('\u{1b}' as i32);
            } else if *temp as libc::c_int == '\\' as i32 {
                putchar('\\' as i32);
            } else if *temp as libc::c_int == '\'' as i32 {
                putchar('\'' as i32);
            } else if *temp as libc::c_int >= '0' as i32
                && *temp as libc::c_int <= '9' as i32
            {
                Counter = 0 as libc::c_int;
                while *temp as libc::c_int >= '0' as i32
                    && *temp as libc::c_int <= '9' as i32
                {
                    Counter = 8 as libc::c_int * Counter
                        + (*temp as libc::c_int - '0' as i32);
                    temp = temp.offset(1);
                    temp;
                }
                putchar(Counter);
                temp = temp.offset(-1);
                temp;
            }
            temp = temp.offset(1);
            temp;
        } else {
            putchar(*temp as libc::c_int);
            temp = temp.offset(1);
            temp;
        }
    }
    fflush(stdout);
}
#[no_mangle]
pub unsafe extern "C" fn spell_op() {
    if restrict_mode() != 0 {
        return;
    }
    top();
    insert_line(0 as libc::c_int);
    insert_line(0 as libc::c_int);
    top();
    command(shell_echo_msg);
    adv_line();
    wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
    wprintw(com_win, b"%s\0" as *const u8 as *const libc::c_char, spell_in_prog_msg);
    wrefresh(com_win);
    command(b"<>!spell\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn ispell_op() {
    let mut template: [libc::c_char; 128] = [0; 128];
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut string: [libc::c_char; 256] = [0; 256];
    let mut fd: libc::c_int = 0;
    if restrict_mode() != 0 {
        return;
    }
    sprintf(
        template.as_mut_ptr(),
        b"/tmp/ee.XXXXXXXX\0" as *const u8 as *const libc::c_char,
    );
    fd = mkstemp(template.as_mut_ptr());
    if fd < 0 as libc::c_int {
        wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
        wprintw(com_win, create_file_fail_msg, name);
        wrefresh(com_win);
        return;
    }
    close(fd);
    if write_file(name, 0 as libc::c_int) != 0 {
        sprintf(
            string.as_mut_ptr(),
            b"ispell %s\0" as *const u8 as *const libc::c_char,
            name,
        );
        sh_command(string.as_mut_ptr());
        delete_text();
        tmp_file = name;
        recv_file = 1 as libc::c_int;
        check_fp();
        unlink(name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn first_word_len(mut test_line: *mut text) -> libc::c_int {
    let mut counter: libc::c_int = 0;
    let mut pnt: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if test_line.is_null() {
        return 0 as libc::c_int;
    }
    pnt = (*test_line).line;
    if pnt.is_null() || *pnt as libc::c_int == '\0' as i32
        || *pnt as libc::c_int == '.' as i32 || *pnt as libc::c_int == '>' as i32
    {
        return 0 as libc::c_int;
    }
    if *pnt as libc::c_int == ' ' as i32 || *pnt as libc::c_int == '\t' as i32 {
        pnt = next_word(pnt);
    }
    if *pnt as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    counter = 0 as libc::c_int;
    while *pnt as libc::c_int != '\0' as i32
        && (*pnt as libc::c_int != ' ' as i32 && *pnt as libc::c_int != '\t' as i32)
    {
        pnt = pnt.offset(1);
        pnt;
        counter += 1;
        counter;
    }
    while *pnt as libc::c_int != '\0' as i32
        && (*pnt as libc::c_int == ' ' as i32 || *pnt as libc::c_int == '\t' as i32)
    {
        pnt = pnt.offset(1);
        pnt;
        counter += 1;
        counter;
    }
    return counter;
}
#[no_mangle]
pub unsafe extern "C" fn Auto_Format() {
    let mut string_count: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut temp_case: libc::c_int = 0;
    let mut word_len: libc::c_int = 0;
    let mut temp_dwl: libc::c_int = 0;
    let mut tmp_d_line_length: libc::c_int = 0;
    let mut leave_loop: libc::c_int = 0 as libc::c_int;
    let mut status: libc::c_int = 0;
    let mut counter: libc::c_int = 0;
    let mut not_blank: libc::c_char = 0;
    let mut line: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp_srchstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp_dword: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut temp_d_char: [libc::c_uchar; 3] = [0; 3];
    let mut tmp_d_line: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    temp_d_char[0 as libc::c_int as usize] = *d_char.offset(0 as libc::c_int as isize);
    temp_d_char[1 as libc::c_int as usize] = *d_char.offset(1 as libc::c_int as isize);
    temp_d_char[2 as libc::c_int as usize] = *d_char.offset(2 as libc::c_int as isize);
    if observ_margins == 0 || Blank_Line(curr_line) != 0 {
        return;
    }
    tmp_d_line = d_line;
    tmp_d_line_length = (*dlt_line).line_length;
    d_line = 0 as *mut libc::c_uchar;
    auto_format = 0 as libc::c_int;
    offset = position;
    if position != 1 as libc::c_int
        && (*point as libc::c_int == ' ' as i32 || *point as libc::c_int == '\t' as i32
            || position == (*curr_line).line_length
            || *point as libc::c_int == '\0' as i32)
    {
        prev_word();
    }
    temp_dword = d_word;
    temp_dwl = d_wrd_len;
    d_wrd_len = 0 as libc::c_int;
    d_word = 0 as *mut libc::c_uchar;
    temp_case = case_sen;
    case_sen = 1 as libc::c_int;
    tmp_srchstr = srch_str;
    srch_str = malloc(
        (1 as libc::c_int + (*curr_line).line_length - position) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    temp2 = srch_str;
    if *point as libc::c_int == ' ' as i32 || *point as libc::c_int == '\t' as i32 {
        adv_word();
    }
    offset -= position;
    counter = position;
    temp1 = point;
    line = temp1;
    while *temp1 as libc::c_int != '\0' as i32 && *temp1 as libc::c_int != ' ' as i32
        && *temp1 as libc::c_int != '\t' as i32 && counter < (*curr_line).line_length
    {
        *temp2 = *temp1;
        temp2 = temp2.offset(1);
        temp2;
        temp1 = temp1.offset(1);
        temp1;
        counter += 1;
        counter;
    }
    *temp2 = '\0' as i32 as libc::c_uchar;
    if position != 1 as libc::c_int {
        bol();
    }
    while Blank_Line((*curr_line).prev_line) == 0 {
        bol();
    }
    string_count = 0 as libc::c_int;
    status = 1 as libc::c_int;
    while line != point && status != 0 {
        status = search(0 as libc::c_int);
        string_count += 1;
        string_count;
    }
    if position != 1 as libc::c_int {
        bol();
    }
    while Blank_Line((*curr_line).prev_line) == 0 {
        bol();
    }
    counter = 0 as libc::c_int;
    while leave_loop == 0 {
        if position != (*curr_line).line_length {
            eol();
        }
        left(1 as libc::c_int);
        if *point as libc::c_int != ' ' as i32 {
            right(1 as libc::c_int);
            insert(' ' as i32);
        } else {
            right(1 as libc::c_int);
        }
        not_blank = 0 as libc::c_int as libc::c_char;
        while !((*curr_line).next_line).is_null()
            && {
                word_len = first_word_len((*curr_line).next_line);
                word_len > 0 as libc::c_int
            } && scr_pos + word_len < right_margin
        {
            adv_line();
            if *point as libc::c_int == ' ' as i32
                || *point as libc::c_int == '\t' as i32
            {
                adv_word();
            }
            del_word();
            if position != 1 as libc::c_int {
                bol();
            }
            if Blank_Line(curr_line) != 0
                && *((*curr_line).line).offset(0 as libc::c_int as isize) as libc::c_int
                    != '.' as i32
                && *((*curr_line).line).offset(0 as libc::c_int as isize) as libc::c_int
                    != '>' as i32
            {
                del_line();
                not_blank = 0 as libc::c_int as libc::c_char;
            } else {
                not_blank = 1 as libc::c_int as libc::c_char;
            }
            left(1 as libc::c_int);
            undel_word();
            eol();
            left(1 as libc::c_int);
            if *point as libc::c_int != ' ' as i32 {
                right(1 as libc::c_int);
                insert(' ' as i32);
            } else {
                right(1 as libc::c_int);
            }
        }
        while right_margin <= scr_pos {
            prev_word();
            if position != 1 as libc::c_int {
                del_word();
                if Blank_Line((*curr_line).next_line) != 0 {
                    insert_line(1 as libc::c_int);
                } else {
                    adv_line();
                }
                if *point as libc::c_int == ' ' as i32
                    || *point as libc::c_int == '\t' as i32
                {
                    adv_word();
                }
                undel_word();
                not_blank = 1 as libc::c_int as libc::c_char;
                if position != 1 as libc::c_int {
                    bol();
                }
                left(1 as libc::c_int);
            }
        }
        if Blank_Line((*curr_line).next_line) == 0 || not_blank as libc::c_int != 0 {
            adv_line();
            counter += 1;
            counter;
        } else {
            leave_loop = 1 as libc::c_int;
        }
    }
    if position != 1 as libc::c_int {
        bol();
    }
    loop {
        let fresh0 = counter;
        counter = counter - 1;
        if !(fresh0 > 0 as libc::c_int || Blank_Line((*curr_line).prev_line) == 0) {
            break;
        }
        bol();
    }
    status = 1 as libc::c_int;
    while status != 0 && string_count > 0 as libc::c_int {
        status = search(0 as libc::c_int);
        string_count -= 1;
        string_count;
    }
    while offset > 0 as libc::c_int {
        offset -= 1;
        offset;
        right(1 as libc::c_int);
    }
    if string_count > 0 as libc::c_int && offset < 0 as libc::c_int {
        while offset < 0 as libc::c_int {
            offset += 1;
            offset;
            left(1 as libc::c_int);
        }
    }
    if !d_word.is_null() {
        free(d_word as *mut libc::c_void);
    }
    d_word = temp_dword;
    d_wrd_len = temp_dwl;
    case_sen = temp_case;
    free(srch_str as *mut libc::c_void);
    srch_str = tmp_srchstr;
    *d_char.offset(0 as libc::c_int as isize) = temp_d_char[0 as libc::c_int as usize];
    *d_char.offset(1 as libc::c_int as isize) = temp_d_char[1 as libc::c_int as usize];
    *d_char.offset(2 as libc::c_int as isize) = temp_d_char[2 as libc::c_int as usize];
    auto_format = 1 as libc::c_int;
    (*dlt_line).line_length = tmp_d_line_length;
    d_line = tmp_d_line;
    formatted = 1 as libc::c_int;
    midscreen(scr_vert, point);
}
#[no_mangle]
pub unsafe extern "C" fn modes_op() {
    let mut ret_value: libc::c_int = 0;
    let mut counter: libc::c_int = 0;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        sprintf(
            modes_menu[1 as libc::c_int as usize].item_string,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            mode_strings[1 as libc::c_int as usize],
            if expand_tabs != 0 { ON } else { OFF },
        );
        sprintf(
            modes_menu[2 as libc::c_int as usize].item_string,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            mode_strings[2 as libc::c_int as usize],
            if case_sen != 0 { ON } else { OFF },
        );
        sprintf(
            modes_menu[3 as libc::c_int as usize].item_string,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            mode_strings[3 as libc::c_int as usize],
            if observ_margins != 0 { ON } else { OFF },
        );
        sprintf(
            modes_menu[4 as libc::c_int as usize].item_string,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            mode_strings[4 as libc::c_int as usize],
            if auto_format != 0 { ON } else { OFF },
        );
        sprintf(
            modes_menu[5 as libc::c_int as usize].item_string,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            mode_strings[5 as libc::c_int as usize],
            if eightbit != 0 { ON } else { OFF },
        );
        sprintf(
            modes_menu[6 as libc::c_int as usize].item_string,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            mode_strings[6 as libc::c_int as usize],
            if info_window != 0 { ON } else { OFF },
        );
        sprintf(
            modes_menu[7 as libc::c_int as usize].item_string,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            mode_strings[7 as libc::c_int as usize],
            if emacs_keys_mode != 0 { ON } else { OFF },
        );
        sprintf(
            modes_menu[8 as libc::c_int as usize].item_string,
            b"%s %d\0" as *const u8 as *const libc::c_char,
            mode_strings[8 as libc::c_int as usize],
            right_margin,
        );
        sprintf(
            modes_menu[9 as libc::c_int as usize].item_string,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            mode_strings[9 as libc::c_int as usize],
            if ee_chinese != 0 { ON } else { OFF },
        );
        ret_value = menu_op(modes_menu.as_mut_ptr());
        match ret_value {
            1 => {
                expand_tabs = (expand_tabs == 0) as libc::c_int;
            }
            2 => {
                case_sen = (case_sen == 0) as libc::c_int;
            }
            3 => {
                observ_margins = (observ_margins == 0) as libc::c_int;
            }
            4 => {
                auto_format = (auto_format == 0) as libc::c_int;
                if auto_format != 0 {
                    observ_margins = 1 as libc::c_int;
                }
            }
            5 => {
                eightbit = (eightbit == 0) as libc::c_int;
                if eightbit == 0 {
                    ee_chinese = 0 as libc::c_int;
                }
                redraw();
                wnoutrefresh(text_win);
            }
            6 => {
                if info_window != 0 {
                    no_info_window();
                } else {
                    create_info_window();
                }
            }
            7 => {
                emacs_keys_mode = (emacs_keys_mode == 0) as libc::c_int;
                if info_window != 0 {
                    paint_info_win();
                }
            }
            8 => {
                string = get_string(margin_prompt, 1 as libc::c_int);
                if !string.is_null() {
                    counter = atoi(string);
                    if counter > 0 as libc::c_int {
                        right_margin = counter;
                    }
                    free(string as *mut libc::c_void);
                }
            }
            9 => {
                ee_chinese = (ee_chinese == 0) as libc::c_int;
                if ee_chinese != 0 as libc::c_int {
                    eightbit = 1 as libc::c_int;
                }
                redraw();
            }
            _ => {}
        }
        if !(ret_value != 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn is_in_string(
    mut string: *mut libc::c_char,
    mut substring: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut full: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sub: *mut libc::c_char = 0 as *mut libc::c_char;
    sub = substring;
    while !sub.is_null() && *sub as libc::c_int != '\0' as i32 {
        full = string;
        while !full.is_null() && *full as libc::c_int != '\0' as i32 {
            if *sub as libc::c_int == *full as libc::c_int {
                return full;
            }
            full = full.offset(1);
            full;
        }
        sub = sub.offset(1);
        sub;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn resolve_name(mut name: *mut libc::c_char) -> *mut libc::c_char {
    let mut long_buffer: [libc::c_char; 1024] = [0; 1024];
    let mut short_buffer: [libc::c_char; 128] = [0; 128];
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start_of_var: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut offset: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut counter: libc::c_int = 0;
    let mut user: *mut passwd = 0 as *mut passwd;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32 {
        if *name.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            index = getuid() as libc::c_int;
            user = getpwuid(index as __uid_t);
            slash = name.offset(1 as libc::c_int as isize);
        } else {
            slash = strchr(name, '/' as i32);
            if slash.is_null() {
                return name;
            }
            *slash = '\0' as i32 as libc::c_char;
            user = getpwnam(name.offset(1 as libc::c_int as isize));
            *slash = '/' as i32 as libc::c_char;
        }
        if user.is_null() {
            return name;
        }
        buffer = malloc(
            (strlen((*user).pw_dir))
                .wrapping_add(strlen(slash))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(buffer, (*user).pw_dir);
        strcat(buffer, slash);
    } else {
        buffer = name;
    }
    if !(is_in_string(
        buffer,
        b"$\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .is_null()
    {
        tmp = buffer;
        index = 0 as libc::c_int;
        while *tmp as libc::c_int != '\0' as i32 && index < 1024 as libc::c_int {
            while *tmp as libc::c_int != '\0' as i32 && *tmp as libc::c_int != '$' as i32
                && index < 1024 as libc::c_int
            {
                long_buffer[index as usize] = *tmp;
                tmp = tmp.offset(1);
                tmp;
                index += 1;
                index;
            }
            if *tmp as libc::c_int == '$' as i32 && index < 1024 as libc::c_int {
                counter = 0 as libc::c_int;
                start_of_var = tmp;
                tmp = tmp.offset(1);
                tmp;
                if *tmp as libc::c_int == '{' as i32 {
                    tmp = tmp.offset(1);
                    tmp;
                    while *tmp as libc::c_int != '\0' as i32
                        && *tmp as libc::c_int != '}' as i32
                        && counter < 128 as libc::c_int
                    {
                        short_buffer[counter as usize] = *tmp;
                        counter += 1;
                        counter;
                        tmp = tmp.offset(1);
                        tmp;
                    }
                    if *tmp as libc::c_int == '}' as i32 {
                        tmp = tmp.offset(1);
                        tmp;
                    }
                } else {
                    while *tmp as libc::c_int != '\0' as i32
                        && *tmp as libc::c_int != '/' as i32
                        && *tmp as libc::c_int != '$' as i32
                        && counter < 128 as libc::c_int
                    {
                        short_buffer[counter as usize] = *tmp;
                        counter += 1;
                        counter;
                        tmp = tmp.offset(1);
                        tmp;
                    }
                }
                short_buffer[counter as usize] = '\0' as i32 as libc::c_char;
                slash = getenv(short_buffer.as_mut_ptr());
                if !slash.is_null() {
                    offset = strlen(slash) as libc::c_int;
                    if offset + index < 1024 as libc::c_int {
                        strcpy(
                            &mut *long_buffer.as_mut_ptr().offset(index as isize),
                            slash,
                        );
                    }
                    index += offset;
                } else {
                    while start_of_var != tmp && index < 1024 as libc::c_int {
                        long_buffer[index as usize] = *start_of_var;
                        start_of_var = start_of_var.offset(1);
                        start_of_var;
                        index += 1;
                        index;
                    }
                }
            }
        }
        if index == 1024 as libc::c_int {
            return buffer
        } else {
            long_buffer[index as usize] = '\0' as i32 as libc::c_char;
        }
        if name != buffer {
            free(buffer as *mut libc::c_void);
        }
        buffer = malloc((index + 1 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        strcpy(buffer, long_buffer.as_mut_ptr());
    }
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn restrict_mode() -> libc::c_int {
    if restricted == 0 {
        return 0 as libc::c_int;
    }
    wmove(com_win, 0 as libc::c_int, 0 as libc::c_int);
    wprintw(com_win, b"%s\0" as *const u8 as *const libc::c_char, restricted_msg);
    wclrtoeol(com_win);
    wrefresh(com_win);
    clear_com_win = 1 as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn unique_test(
    mut string: *mut libc::c_char,
    mut list: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut counter: libc::c_int = 0;
    let mut num_match: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    num_match = 0 as libc::c_int;
    counter = 0 as libc::c_int;
    while !(*list.offset(counter as isize)).is_null() {
        result = compare(string, *list.offset(counter as isize), 0 as libc::c_int);
        if result != 0 {
            num_match += 1;
            num_match;
        }
        counter += 1;
        counter;
    }
    return num_match;
}
#[no_mangle]
pub unsafe extern "C" fn catgetlocal(
    mut number: libc::c_int,
    mut string: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut temp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp2: *mut libc::c_char = 0 as *mut libc::c_char;
    temp1 = catgets(catalog, 1 as libc::c_int, number, string);
    if temp1 != string {
        temp2 = malloc((strlen(temp1)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        strcpy(temp2, temp1);
        temp1 = temp2;
    }
    return temp1;
}
#[no_mangle]
pub unsafe extern "C" fn strings_init() {
    let mut counter: libc::c_int = 0;
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    catalog = catopen(b"ee\0" as *const u8 as *const libc::c_char, 1 as libc::c_int);
    modes_menu[0 as libc::c_int as usize]
        .item_string = catgetlocal(
        1 as libc::c_int,
        b"modes menu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    mode_strings[1 as libc::c_int
        as usize] = catgetlocal(
        2 as libc::c_int,
        b"tabs to spaces       \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    mode_strings[2 as libc::c_int
        as usize] = catgetlocal(
        3 as libc::c_int,
        b"case sensitive search\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    mode_strings[3 as libc::c_int
        as usize] = catgetlocal(
        4 as libc::c_int,
        b"margins observed     \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    mode_strings[4 as libc::c_int
        as usize] = catgetlocal(
        5 as libc::c_int,
        b"auto-paragraph format\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    mode_strings[5 as libc::c_int
        as usize] = catgetlocal(
        6 as libc::c_int,
        b"eightbit characters  \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    mode_strings[6 as libc::c_int
        as usize] = catgetlocal(
        7 as libc::c_int,
        b"info window          \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    mode_strings[8 as libc::c_int
        as usize] = catgetlocal(
        8 as libc::c_int,
        b"right margin         \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    leave_menu[0 as libc::c_int as usize]
        .item_string = catgetlocal(
        9 as libc::c_int,
        b"leave menu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    leave_menu[1 as libc::c_int as usize]
        .item_string = catgetlocal(
        10 as libc::c_int,
        b"save changes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    leave_menu[2 as libc::c_int as usize]
        .item_string = catgetlocal(
        11 as libc::c_int,
        b"no save\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    file_menu[0 as libc::c_int as usize]
        .item_string = catgetlocal(
        12 as libc::c_int,
        b"file menu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    file_menu[1 as libc::c_int as usize]
        .item_string = catgetlocal(
        13 as libc::c_int,
        b"read a file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    file_menu[2 as libc::c_int as usize]
        .item_string = catgetlocal(
        14 as libc::c_int,
        b"write a file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    file_menu[3 as libc::c_int as usize]
        .item_string = catgetlocal(
        15 as libc::c_int,
        b"save file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    file_menu[4 as libc::c_int as usize]
        .item_string = catgetlocal(
        16 as libc::c_int,
        b"print editor contents\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    search_menu[0 as libc::c_int as usize]
        .item_string = catgetlocal(
        17 as libc::c_int,
        b"search menu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    search_menu[1 as libc::c_int as usize]
        .item_string = catgetlocal(
        18 as libc::c_int,
        b"search for ...\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    search_menu[2 as libc::c_int as usize]
        .item_string = catgetlocal(
        19 as libc::c_int,
        b"search\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    spell_menu[0 as libc::c_int as usize]
        .item_string = catgetlocal(
        20 as libc::c_int,
        b"spell menu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    spell_menu[1 as libc::c_int as usize]
        .item_string = catgetlocal(
        21 as libc::c_int,
        b"use 'spell'\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    spell_menu[2 as libc::c_int as usize]
        .item_string = catgetlocal(
        22 as libc::c_int,
        b"use 'ispell'\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    misc_menu[0 as libc::c_int as usize]
        .item_string = catgetlocal(
        23 as libc::c_int,
        b"miscellaneous menu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    misc_menu[1 as libc::c_int as usize]
        .item_string = catgetlocal(
        24 as libc::c_int,
        b"format paragraph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    misc_menu[2 as libc::c_int as usize]
        .item_string = catgetlocal(
        25 as libc::c_int,
        b"shell command\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    misc_menu[3 as libc::c_int as usize]
        .item_string = catgetlocal(
        26 as libc::c_int,
        b"check spelling\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    main_menu[0 as libc::c_int as usize]
        .item_string = catgetlocal(
        27 as libc::c_int,
        b"main menu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    main_menu[1 as libc::c_int as usize]
        .item_string = catgetlocal(
        28 as libc::c_int,
        b"leave editor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    main_menu[2 as libc::c_int as usize]
        .item_string = catgetlocal(
        29 as libc::c_int,
        b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    main_menu[3 as libc::c_int as usize]
        .item_string = catgetlocal(
        30 as libc::c_int,
        b"file operations\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    main_menu[4 as libc::c_int as usize]
        .item_string = catgetlocal(
        31 as libc::c_int,
        b"redraw screen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    main_menu[5 as libc::c_int as usize]
        .item_string = catgetlocal(
        32 as libc::c_int,
        b"settings\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    main_menu[6 as libc::c_int as usize]
        .item_string = catgetlocal(
        33 as libc::c_int,
        b"search\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    main_menu[7 as libc::c_int as usize]
        .item_string = catgetlocal(
        34 as libc::c_int,
        b"miscellaneous\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[0 as libc::c_int
        as usize] = catgetlocal(
        35 as libc::c_int,
        b"Control keys:                                                              \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[1 as libc::c_int
        as usize] = catgetlocal(
        36 as libc::c_int,
        b"^a ascii code           ^i tab                  ^r right                   \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[2 as libc::c_int
        as usize] = catgetlocal(
        37 as libc::c_int,
        b"^b bottom of text       ^j newline              ^t top of text             \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[3 as libc::c_int
        as usize] = catgetlocal(
        38 as libc::c_int,
        b"^c command              ^k delete char          ^u up                      \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[4 as libc::c_int
        as usize] = catgetlocal(
        39 as libc::c_int,
        b"^d down                 ^l left                 ^v undelete word           \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[5 as libc::c_int
        as usize] = catgetlocal(
        40 as libc::c_int,
        b"^e search prompt        ^m newline              ^w delete word             \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[6 as libc::c_int
        as usize] = catgetlocal(
        41 as libc::c_int,
        b"^f undelete char        ^n next page            ^x search                  \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[7 as libc::c_int
        as usize] = catgetlocal(
        42 as libc::c_int,
        b"^g begin of line        ^o end of line          ^y delete line             \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[8 as libc::c_int
        as usize] = catgetlocal(
        43 as libc::c_int,
        b"^h backspace            ^p prev page            ^z undelete line           \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[9 as libc::c_int
        as usize] = catgetlocal(
        44 as libc::c_int,
        b"^[ (escape) menu        ESC-Enter: exit ee                                 \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[10 as libc::c_int
        as usize] = catgetlocal(
        45 as libc::c_int,
        b"                                                                           \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[11 as libc::c_int
        as usize] = catgetlocal(
        46 as libc::c_int,
        b"Commands:                                                                  \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[12 as libc::c_int
        as usize] = catgetlocal(
        47 as libc::c_int,
        b"help    : get this info                 file    : print file name          \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[13 as libc::c_int
        as usize] = catgetlocal(
        48 as libc::c_int,
        b"read    : read a file                   char    : ascii code of char       \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[14 as libc::c_int
        as usize] = catgetlocal(
        49 as libc::c_int,
        b"write   : write a file                  case    : case sensitive search    \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[15 as libc::c_int
        as usize] = catgetlocal(
        50 as libc::c_int,
        b"exit    : leave and save                nocase  : case insensitive search  \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[16 as libc::c_int
        as usize] = catgetlocal(
        51 as libc::c_int,
        b"quit    : leave, no save                !cmd    : execute \"cmd\" in shell   \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[17 as libc::c_int
        as usize] = catgetlocal(
        52 as libc::c_int,
        b"line    : display line #                0-9     : go to line \"#\"           \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[18 as libc::c_int
        as usize] = catgetlocal(
        53 as libc::c_int,
        b"expand  : expand tabs                   noexpand: do not expand tabs         \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[19 as libc::c_int
        as usize] = catgetlocal(
        54 as libc::c_int,
        b"                                                                             \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[20 as libc::c_int
        as usize] = catgetlocal(
        55 as libc::c_int,
        b"  ee [+#] [-i] [-e] [-h] [file(s)]                                            \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    help_text[21 as libc::c_int
        as usize] = catgetlocal(
        56 as libc::c_int,
        b"+# :go to line #  -i :no info window  -e : don't expand tabs  -h :no highlight\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    control_keys[0 as libc::c_int
        as usize] = catgetlocal(
        57 as libc::c_int,
        b"^[ (escape) menu  ^e search prompt  ^y delete line    ^u up     ^p prev page  \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    control_keys[1 as libc::c_int
        as usize] = catgetlocal(
        58 as libc::c_int,
        b"^a ascii code     ^x search         ^z undelete line  ^d down   ^n next page  \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    control_keys[2 as libc::c_int
        as usize] = catgetlocal(
        59 as libc::c_int,
        b"^b bottom of text ^g begin of line  ^w delete word    ^l left                 \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    control_keys[3 as libc::c_int
        as usize] = catgetlocal(
        60 as libc::c_int,
        b"^t top of text    ^o end of line    ^v undelete word  ^r right                \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    control_keys[4 as libc::c_int
        as usize] = catgetlocal(
        61 as libc::c_int,
        b"^c command        ^k delete char    ^f undelete char      ESC-Enter: exit ee  \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    command_strings[0 as libc::c_int
        as usize] = catgetlocal(
        62 as libc::c_int,
        b"help : get help info  |file  : print file name         |line : print line # \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    command_strings[1 as libc::c_int
        as usize] = catgetlocal(
        63 as libc::c_int,
        b"read : read a file    |char  : ascii code of char      |0-9 : go to line \"#\"\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    command_strings[2 as libc::c_int
        as usize] = catgetlocal(
        64 as libc::c_int,
        b"write: write a file   |case  : case sensitive search   |exit : leave and save \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    command_strings[3 as libc::c_int
        as usize] = catgetlocal(
        65 as libc::c_int,
        b"!cmd : shell \"cmd\"    |nocase: ignore case in search   |quit : leave, no save\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    command_strings[4 as libc::c_int
        as usize] = catgetlocal(
        66 as libc::c_int,
        b"expand: expand tabs   |noexpand: do not expand tabs                           \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    com_win_message = catgetlocal(
        67 as libc::c_int,
        b"    press Escape (^[) for menu\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    no_file_string = catgetlocal(
        68 as libc::c_int,
        b"no file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    ascii_code_str = catgetlocal(
        69 as libc::c_int,
        b"ascii code: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    printer_msg_str = catgetlocal(
        70 as libc::c_int,
        b"sending contents of buffer to \"%s\" \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    command_str = catgetlocal(
        71 as libc::c_int,
        b"command: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    file_write_prompt_str = catgetlocal(
        72 as libc::c_int,
        b"name of file to write: \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    file_read_prompt_str = catgetlocal(
        73 as libc::c_int,
        b"name of file to read: \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    char_str = catgetlocal(
        74 as libc::c_int,
        b"character = %d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    unkn_cmd_str = catgetlocal(
        75 as libc::c_int,
        b"unknown command \"%s\"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    non_unique_cmd_msg = catgetlocal(
        76 as libc::c_int,
        b"entered command is not unique\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    line_num_str = catgetlocal(
        77 as libc::c_int,
        b"line %d  \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    line_len_str = catgetlocal(
        78 as libc::c_int,
        b"length = %d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    current_file_str = catgetlocal(
        79 as libc::c_int,
        b"current file is \"%s\" \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    usage0 = catgetlocal(
        80 as libc::c_int,
        b"usage: %s [-i] [-e] [-h] [+line_number] [file(s)]\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    usage1 = catgetlocal(
        81 as libc::c_int,
        b"       -i   turn off info window\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    usage2 = catgetlocal(
        82 as libc::c_int,
        b"       -e   do not convert tabs to spaces\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    usage3 = catgetlocal(
        83 as libc::c_int,
        b"       -h   do not use highlighting\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    file_is_dir_msg = catgetlocal(
        84 as libc::c_int,
        b"file \"%s\" is a directory\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    new_file_msg = catgetlocal(
        85 as libc::c_int,
        b"new file \"%s\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cant_open_msg = catgetlocal(
        86 as libc::c_int,
        b"can't open \"%s\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    open_file_msg = catgetlocal(
        87 as libc::c_int,
        b"file \"%s\", %d lines\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    file_read_fin_msg = catgetlocal(
        88 as libc::c_int,
        b"finished reading file \"%s\"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    reading_file_msg = catgetlocal(
        89 as libc::c_int,
        b"reading file \"%s\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    read_only_msg = catgetlocal(
        90 as libc::c_int,
        b", read only\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    file_read_lines_msg = catgetlocal(
        91 as libc::c_int,
        b"file \"%s\", %d lines\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    save_file_name_prompt = catgetlocal(
        92 as libc::c_int,
        b"enter name of file: \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    file_not_saved_msg = catgetlocal(
        93 as libc::c_int,
        b"no filename entered: file not saved\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    changes_made_prompt = catgetlocal(
        94 as libc::c_int,
        b"changes have been made, are you sure? (y/n [n]) \0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    yes_char = catgetlocal(
        95 as libc::c_int,
        b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    file_exists_prompt = catgetlocal(
        96 as libc::c_int,
        b"file already exists, overwrite? (y/n) [n] \0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    create_file_fail_msg = catgetlocal(
        97 as libc::c_int,
        b"unable to create file \"%s\"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    writing_file_msg = catgetlocal(
        98 as libc::c_int,
        b"writing file \"%s\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    file_written_msg = catgetlocal(
        99 as libc::c_int,
        b"\"%s\" %d lines, %d characters\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    searching_msg = catgetlocal(
        100 as libc::c_int,
        b"           ...searching\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    str_not_found_msg = catgetlocal(
        101 as libc::c_int,
        b"string \"%s\" not found\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    search_prompt_str = catgetlocal(
        102 as libc::c_int,
        b"search for: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    exec_err_msg = catgetlocal(
        103 as libc::c_int,
        b"could not exec %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    continue_msg = catgetlocal(
        104 as libc::c_int,
        b"press return to continue \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    menu_cancel_msg = catgetlocal(
        105 as libc::c_int,
        b"press Esc to cancel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    menu_size_err_msg = catgetlocal(
        106 as libc::c_int,
        b"menu too large for window\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    press_any_key_msg = catgetlocal(
        107 as libc::c_int,
        b"press any key to continue \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    shell_prompt = catgetlocal(
        108 as libc::c_int,
        b"shell command: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    formatting_msg = catgetlocal(
        109 as libc::c_int,
        b"...formatting paragraph...\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    shell_echo_msg = catgetlocal(
        110 as libc::c_int,
        b"<!echo 'list of unrecognized words'; echo -=-=-=-=-=-\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    spell_in_prog_msg = catgetlocal(
        111 as libc::c_int,
        b"sending contents of edit buffer to 'spell'\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    margin_prompt = catgetlocal(
        112 as libc::c_int,
        b"right margin is: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    restricted_msg = catgetlocal(
        113 as libc::c_int,
        b"restricted mode: unable to perform requested operation\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    ON = catgetlocal(
        114 as libc::c_int,
        b"ON\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    OFF = catgetlocal(
        115 as libc::c_int,
        b"OFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    HELP = catgetlocal(
        116 as libc::c_int,
        b"HELP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    WRITE = catgetlocal(
        117 as libc::c_int,
        b"WRITE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    READ = catgetlocal(
        118 as libc::c_int,
        b"READ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    LINE = catgetlocal(
        119 as libc::c_int,
        b"LINE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    FILE_str = catgetlocal(
        120 as libc::c_int,
        b"FILE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    CHARACTER = catgetlocal(
        121 as libc::c_int,
        b"CHARACTER\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    REDRAW = catgetlocal(
        122 as libc::c_int,
        b"REDRAW\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    RESEQUENCE = catgetlocal(
        123 as libc::c_int,
        b"RESEQUENCE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    AUTHOR = catgetlocal(
        124 as libc::c_int,
        b"AUTHOR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    VERSION = catgetlocal(
        125 as libc::c_int,
        b"VERSION\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    CASE = catgetlocal(
        126 as libc::c_int,
        b"CASE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    NOCASE = catgetlocal(
        127 as libc::c_int,
        b"NOCASE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    EXPAND = catgetlocal(
        128 as libc::c_int,
        b"EXPAND\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    NOEXPAND = catgetlocal(
        129 as libc::c_int,
        b"NOEXPAND\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Exit_string = catgetlocal(
        130 as libc::c_int,
        b"EXIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    QUIT_string = catgetlocal(
        131 as libc::c_int,
        b"QUIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    INFO = catgetlocal(
        132 as libc::c_int,
        b"INFO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    NOINFO = catgetlocal(
        133 as libc::c_int,
        b"NOINFO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    MARGINS = catgetlocal(
        134 as libc::c_int,
        b"MARGINS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    NOMARGINS = catgetlocal(
        135 as libc::c_int,
        b"NOMARGINS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    AUTOFORMAT = catgetlocal(
        136 as libc::c_int,
        b"AUTOFORMAT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    NOAUTOFORMAT = catgetlocal(
        137 as libc::c_int,
        b"NOAUTOFORMAT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Echo = catgetlocal(
        138 as libc::c_int,
        b"ECHO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    PRINTCOMMAND = catgetlocal(
        139 as libc::c_int,
        b"PRINTCOMMAND\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    RIGHTMARGIN = catgetlocal(
        140 as libc::c_int,
        b"RIGHTMARGIN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    HIGHLIGHT = catgetlocal(
        141 as libc::c_int,
        b"HIGHLIGHT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    NOHIGHLIGHT = catgetlocal(
        142 as libc::c_int,
        b"NOHIGHLIGHT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    EIGHTBIT = catgetlocal(
        143 as libc::c_int,
        b"EIGHTBIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    NOEIGHTBIT = catgetlocal(
        144 as libc::c_int,
        b"NOEIGHTBIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    mode_strings[7 as libc::c_int
        as usize] = catgetlocal(
        145 as libc::c_int,
        b"emacs key bindings   \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    emacs_help_text[0 as libc::c_int as usize] = help_text[0 as libc::c_int as usize];
    emacs_help_text[1 as libc::c_int
        as usize] = catgetlocal(
        146 as libc::c_int,
        b"^a beginning of line    ^i tab                  ^r restore word            \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    emacs_help_text[2 as libc::c_int
        as usize] = catgetlocal(
        147 as libc::c_int,
        b"^b back 1 char          ^j undel char           ^t top of text             \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    emacs_help_text[3 as libc::c_int
        as usize] = catgetlocal(
        148 as libc::c_int,
        b"^c command              ^k delete line          ^u bottom of text          \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    emacs_help_text[4 as libc::c_int
        as usize] = catgetlocal(
        149 as libc::c_int,
        b"^d delete char          ^l undelete line        ^v next page               \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    emacs_help_text[5 as libc::c_int
        as usize] = catgetlocal(
        150 as libc::c_int,
        b"^e end of line          ^m newline              ^w delete word             \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    emacs_help_text[6 as libc::c_int
        as usize] = catgetlocal(
        151 as libc::c_int,
        b"^f forward 1 char       ^n next line            ^x search                  \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    emacs_help_text[7 as libc::c_int
        as usize] = catgetlocal(
        152 as libc::c_int,
        b"^g go back 1 page       ^o ascii char insert    ^y search prompt           \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    emacs_help_text[8 as libc::c_int
        as usize] = catgetlocal(
        153 as libc::c_int,
        b"^h backspace            ^p prev line            ^z next word               \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    emacs_help_text[9 as libc::c_int as usize] = help_text[9 as libc::c_int as usize];
    emacs_help_text[10 as libc::c_int as usize] = help_text[10 as libc::c_int as usize];
    emacs_help_text[11 as libc::c_int as usize] = help_text[11 as libc::c_int as usize];
    emacs_help_text[12 as libc::c_int as usize] = help_text[12 as libc::c_int as usize];
    emacs_help_text[13 as libc::c_int as usize] = help_text[13 as libc::c_int as usize];
    emacs_help_text[14 as libc::c_int as usize] = help_text[14 as libc::c_int as usize];
    emacs_help_text[15 as libc::c_int as usize] = help_text[15 as libc::c_int as usize];
    emacs_help_text[16 as libc::c_int as usize] = help_text[16 as libc::c_int as usize];
    emacs_help_text[17 as libc::c_int as usize] = help_text[17 as libc::c_int as usize];
    emacs_help_text[18 as libc::c_int as usize] = help_text[18 as libc::c_int as usize];
    emacs_help_text[19 as libc::c_int as usize] = help_text[19 as libc::c_int as usize];
    emacs_help_text[20 as libc::c_int as usize] = help_text[20 as libc::c_int as usize];
    emacs_help_text[21 as libc::c_int as usize] = help_text[21 as libc::c_int as usize];
    emacs_control_keys[0 as libc::c_int
        as usize] = catgetlocal(
        154 as libc::c_int,
        b"^[ (escape) menu ^y search prompt ^k delete line   ^p prev li     ^g prev page\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    emacs_control_keys[1 as libc::c_int
        as usize] = catgetlocal(
        155 as libc::c_int,
        b"^o ascii code    ^x search        ^l undelete line ^n next li     ^v next page\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    emacs_control_keys[2 as libc::c_int
        as usize] = catgetlocal(
        156 as libc::c_int,
        b"^u end of file   ^a begin of line ^w delete word   ^b back 1 char ^z next word\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    emacs_control_keys[3 as libc::c_int
        as usize] = catgetlocal(
        157 as libc::c_int,
        b"^t top of text   ^e end of line   ^r restore word  ^f forward char            \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    emacs_control_keys[4 as libc::c_int
        as usize] = catgetlocal(
        158 as libc::c_int,
        b"^c command       ^d delete char   ^j undelete char              ESC-Enter: exit\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    EMACS_string = catgetlocal(
        159 as libc::c_int,
        b"EMACS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    NOEMACS_string = catgetlocal(
        160 as libc::c_int,
        b"NOEMACS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    usage4 = catgetlocal(
        161 as libc::c_int,
        b"       +#   put cursor at line #\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    conf_dump_err_msg = catgetlocal(
        162 as libc::c_int,
        b"unable to open .init.ee for writing, no configuration saved!\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    conf_dump_success_msg = catgetlocal(
        163 as libc::c_int,
        b"ee configuration saved in file %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    modes_menu[10 as libc::c_int as usize]
        .item_string = catgetlocal(
        164 as libc::c_int,
        b"save editor configuration\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    config_dump_menu[0 as libc::c_int as usize]
        .item_string = catgetlocal(
        165 as libc::c_int,
        b"save ee configuration\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    config_dump_menu[1 as libc::c_int as usize]
        .item_string = catgetlocal(
        166 as libc::c_int,
        b"save in current directory\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    config_dump_menu[2 as libc::c_int as usize]
        .item_string = catgetlocal(
        167 as libc::c_int,
        b"save in home directory\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    conf_not_saved_msg = catgetlocal(
        168 as libc::c_int,
        b"ee configuration not saved\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    ree_no_file_msg = catgetlocal(
        169 as libc::c_int,
        b"must specify a file when invoking ree\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    menu_too_lrg_msg = catgetlocal(
        180 as libc::c_int,
        b"menu too large for window\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    more_above_str = catgetlocal(
        181 as libc::c_int,
        b"^^more^^\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    more_below_str = catgetlocal(
        182 as libc::c_int,
        b"VVmoreVV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    mode_strings[9 as libc::c_int
        as usize] = catgetlocal(
        183 as libc::c_int,
        b"16 bit characters    \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    chinese_cmd = catgetlocal(
        184 as libc::c_int,
        b"16BIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nochinese_cmd = catgetlocal(
        185 as libc::c_int,
        b"NO16BIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    commands[0 as libc::c_int as usize] = HELP;
    commands[1 as libc::c_int as usize] = WRITE;
    commands[2 as libc::c_int as usize] = READ;
    commands[3 as libc::c_int as usize] = LINE;
    commands[4 as libc::c_int as usize] = FILE_str;
    commands[5 as libc::c_int as usize] = REDRAW;
    commands[6 as libc::c_int as usize] = RESEQUENCE;
    commands[7 as libc::c_int as usize] = AUTHOR;
    commands[8 as libc::c_int as usize] = VERSION;
    commands[9 as libc::c_int as usize] = CASE;
    commands[10 as libc::c_int as usize] = NOCASE;
    commands[11 as libc::c_int as usize] = EXPAND;
    commands[12 as libc::c_int as usize] = NOEXPAND;
    commands[13 as libc::c_int as usize] = Exit_string;
    commands[14 as libc::c_int as usize] = QUIT_string;
    commands[15 as libc::c_int
        as usize] = b"<\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[16 as libc::c_int
        as usize] = b">\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[17 as libc::c_int
        as usize] = b"!\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[18 as libc::c_int
        as usize] = b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[19 as libc::c_int
        as usize] = b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[20 as libc::c_int
        as usize] = b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[21 as libc::c_int
        as usize] = b"3\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[22 as libc::c_int
        as usize] = b"4\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[23 as libc::c_int
        as usize] = b"5\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[24 as libc::c_int
        as usize] = b"6\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[25 as libc::c_int
        as usize] = b"7\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[26 as libc::c_int
        as usize] = b"8\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[27 as libc::c_int
        as usize] = b"9\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    commands[28 as libc::c_int as usize] = CHARACTER;
    commands[29 as libc::c_int as usize] = chinese_cmd;
    commands[30 as libc::c_int as usize] = nochinese_cmd;
    commands[31 as libc::c_int as usize] = 0 as *mut libc::c_char;
    init_strings[0 as libc::c_int as usize] = CASE;
    init_strings[1 as libc::c_int as usize] = NOCASE;
    init_strings[2 as libc::c_int as usize] = EXPAND;
    init_strings[3 as libc::c_int as usize] = NOEXPAND;
    init_strings[4 as libc::c_int as usize] = INFO;
    init_strings[5 as libc::c_int as usize] = NOINFO;
    init_strings[6 as libc::c_int as usize] = MARGINS;
    init_strings[7 as libc::c_int as usize] = NOMARGINS;
    init_strings[8 as libc::c_int as usize] = AUTOFORMAT;
    init_strings[9 as libc::c_int as usize] = NOAUTOFORMAT;
    init_strings[10 as libc::c_int as usize] = Echo;
    init_strings[11 as libc::c_int as usize] = PRINTCOMMAND;
    init_strings[12 as libc::c_int as usize] = RIGHTMARGIN;
    init_strings[13 as libc::c_int as usize] = HIGHLIGHT;
    init_strings[14 as libc::c_int as usize] = NOHIGHLIGHT;
    init_strings[15 as libc::c_int as usize] = EIGHTBIT;
    init_strings[16 as libc::c_int as usize] = NOEIGHTBIT;
    init_strings[17 as libc::c_int as usize] = EMACS_string;
    init_strings[18 as libc::c_int as usize] = NOEMACS_string;
    init_strings[19 as libc::c_int as usize] = chinese_cmd;
    init_strings[20 as libc::c_int as usize] = nochinese_cmd;
    init_strings[21 as libc::c_int as usize] = 0 as *mut libc::c_char;
    counter = 1 as libc::c_int;
    while counter < 10 as libc::c_int {
        modes_menu[counter as usize]
            .item_string = malloc(80 as libc::c_int as libc::c_ulong)
            as *mut libc::c_char;
        counter += 1;
        counter;
    }
    catclose(catalog);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
