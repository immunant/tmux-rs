extern crate libc;
extern "C" {
    pub type tty_code;
    pub type tmuxproc;
    pub type event_base;
    pub type options;
    pub type environ;
    pub type hooks;
    pub type format_job_tree;
    pub type screen_titles;
    pub type bufferevent_ops;
    pub type format_tree;
    pub type _IO_FILE_plus;
    pub type args_entry;
    pub type input_ctx;
    pub type tmuxpeer;
    pub type evbuffer;
    #[no_mangle]
    fn strtod(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char)
     -> libc::c_double;
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn strtoll(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
               __base: libc::c_int) -> libc::c_longlong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __rawmemchr(__s: *const libc::c_void, __c: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __builtin_strchr(_: *const libc::c_char, _: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn __builtin_constant_p() -> libc::c_int;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
     -> libc::c_int;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    static in6addr_loopback: in6_addr;
    #[no_mangle]
    static mut _IO_2_1_stdin_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stdout_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stderr_: _IO_FILE_plus;
    #[no_mangle]
    fn __uflow(_: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn __overflow(_: *mut _IO_FILE, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn _IO_getc(__fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE_0;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE_0;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE_0;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: *mut __va_list_tag)
     -> libc::c_int;
    #[no_mangle]
    fn __builtin_expect(_: libc::c_long, _: libc::c_long) -> libc::c_long;
    #[no_mangle]
    fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
                  __delimiter: libc::c_int, __stream: *mut FILE) -> __ssize_t;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    static mut _sys_nerr: libc::c_int;
    #[no_mangle]
    static _sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn __btowc_alias(__c: libc::c_int) -> wint_t;
    #[no_mangle]
    fn __wctob_alias(__c: wint_t) -> libc::c_int;
    #[no_mangle]
    fn mbrtowc(__pwc: *mut wchar_t, __s: *const libc::c_char, __n: size_t,
               __p: *mut mbstate_t) -> size_t;
    #[no_mangle]
    fn __mbrlen(__s: *const libc::c_char, __n: size_t, __ps: *mut mbstate_t)
     -> size_t;
    #[no_mangle]
    static mut BSDopterr: libc::c_int;
    #[no_mangle]
    static mut BSDoptind: libc::c_int;
    #[no_mangle]
    static mut BSDoptopt: libc::c_int;
    #[no_mangle]
    static mut BSDoptreset: libc::c_int;
    #[no_mangle]
    static mut BSDoptarg: *mut libc::c_char;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut global_hooks: *mut hooks;
    #[no_mangle]
    static mut global_options: *mut options;
    #[no_mangle]
    static mut global_s_options: *mut options;
    #[no_mangle]
    static mut global_w_options: *mut options;
    #[no_mangle]
    static mut global_environ: *mut environ;
    #[no_mangle]
    static mut start_time: timeval;
    #[no_mangle]
    static mut socket_path: *const libc::c_char;
    #[no_mangle]
    static mut shell_command: *const libc::c_char;
    #[no_mangle]
    static mut ptm_fd: libc::c_int;
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn notify_client(_: *const libc::c_char, _: *mut client) -> ();
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn environ_update(_: *mut options, _: *mut environ, _: *mut environ)
     -> ();
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_find_target(_: *mut cmd_find_state, _: *mut cmdq_item,
                       _: *const libc::c_char, _: cmd_find_type,
                       _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_client(_: *mut cmdq_item, _: *const libc::c_char,
                       _: libc::c_int) -> *mut client;
    #[no_mangle]
    fn cmd_find_from_session(_: *mut cmd_find_state, _: *mut session,
                             _: libc::c_int) -> ();
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    fn key_bindings_get_table(_: *const libc::c_char, _: libc::c_int)
     -> *mut key_table;
    #[no_mangle]
    fn key_bindings_unref_table(_: *mut key_table) -> ();
    #[no_mangle]
    fn alerts_check_session(_: *mut session) -> ();
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn server_client_set_key_table(_: *mut client, _: *const libc::c_char)
     -> ();
    #[no_mangle]
    fn server_redraw_client(_: *mut client) -> ();
    #[no_mangle]
    fn server_check_unattached() -> ();
    #[no_mangle]
    fn status_timer_start(_: *mut client) -> ();
    #[no_mangle]
    fn recalculate_sizes() -> ();
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn window_set_active_pane(_: *mut window, _: *mut window_pane)
     -> libc::c_int;
    #[no_mangle]
    static window_buffer_mode: window_mode;
    #[no_mangle]
    static window_tree_mode: window_mode;
    #[no_mangle]
    static window_clock_mode: window_mode;
    #[no_mangle]
    static window_clock_table: [[[libc::c_char; 5]; 5]; 14];
    #[no_mangle]
    static window_client_mode: window_mode;
    #[no_mangle]
    static window_copy_mode: window_mode;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    static mut session_groups: session_groups;
    #[no_mangle]
    fn session_alive(_: *mut session) -> libc::c_int;
    #[no_mangle]
    fn session_update_activity(_: *mut session, _: *mut timeval) -> ();
    #[no_mangle]
    fn session_next_session(_: *mut session) -> *mut session;
    #[no_mangle]
    fn session_previous_session(_: *mut session) -> *mut session;
    #[no_mangle]
    fn session_set_current(_: *mut session, _: *mut winlink) -> libc::c_int;
    #[no_mangle]
    fn __builtin_strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
}
pub type pid_t = __pid_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_33,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub ev_signal_next: unnamed_22,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub type __mbstate_t = mbstate_t;
pub type FILE = _IO_FILE_0;
pub type unnamed_0 = libc::c_uint;
pub type __u_char = libc::c_uchar;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_9,
}
pub const PROMPT_COMMAND: unnamed_37 = 1;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_26,
}
pub type _IO_FILE = _IO_FILE_0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window {
    pub id: u_int,
    pub name: *mut libc::c_char,
    pub name_event: event,
    pub name_time: timeval,
    pub alerts_timer: event,
    pub activity_time: timeval,
    pub active: *mut window_pane,
    pub last: *mut window_pane,
    pub panes: window_panes,
    pub lastlayout: libc::c_int,
    pub layout_root: *mut layout_cell,
    pub saved_layout_root: *mut layout_cell,
    pub old_layout: *mut libc::c_char,
    pub sx: u_int,
    pub sy: u_int,
    pub flags: libc::c_int,
    pub alerts_queued: libc::c_int,
    pub alerts_entry: unnamed_21,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_27,
    pub entry: unnamed_13,
}
pub const TTY_VT420: unnamed_15 = 5;
pub const JOB_RUNNING: unnamed_19 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub type wint_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type options_table_type = libc::c_uint;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink {
    pub idx: libc::c_int,
    pub session: *mut session,
    pub window: *mut window,
    pub status_width: size_t,
    pub status_cell: grid_cell,
    pub status_text: *mut libc::c_char,
    pub flags: libc::c_int,
    pub entry: unnamed_3,
    pub wentry: unnamed_38,
    pub sentry: unnamed_28,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type tcflag_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub const TTY_VT100: unnamed_15 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_8,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_item {
    pub name: *const libc::c_char,
    pub queue: *mut cmdq_list,
    pub next: *mut cmdq_item,
    pub client: *mut client,
    pub type_0: cmdq_type,
    pub group: u_int,
    pub number: u_int,
    pub time: time_t,
    pub flags: libc::c_int,
    pub shared: *mut cmdq_shared,
    pub source: cmd_find_state,
    pub target: cmd_find_state,
    pub cmdlist: *mut cmd_list,
    pub cmd: *mut cmd,
    pub cb: cmdq_cb,
    pub data: *mut libc::c_void,
    pub entry: unnamed_39,
}
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE_0,
    pub _pos: libc::c_int,
}
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_31,
}
pub type wchar_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cell {
    pub type_0: layout_type,
    pub parent: *mut layout_cell,
    pub sx: u_int,
    pub sy: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub wp: *mut window_pane,
    pub cells: layout_cells,
    pub entry: unnamed_20,
}
pub const LINE_SEL_NONE: unnamed_0 = 0;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_9 {
    offset: u_int,
    data: unnamed_2,
}
pub type options_table_scope = libc::c_uint;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct mouse_event {
    pub valid: libc::c_int,
    pub key: key_code,
    pub statusat: libc::c_int,
    pub x: u_int,
    pub y: u_int,
    pub b: u_int,
    pub lx: u_int,
    pub ly: u_int,
    pub lb: u_int,
    pub s: libc::c_int,
    pub w: libc::c_int,
    pub wp: libc::c_int,
    pub sgr_type: u_int,
    pub sgr_b: u_int,
}
pub type cmdq_type = libc::c_uint;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_36,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub type key_code = libc::c_ulonglong;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_find_state {
    pub flags: libc::c_int,
    pub current: *mut cmd_find_state,
    pub s: *mut session,
    pub wl: *mut winlink,
    pub w: *mut window,
    pub wp: *mut window_pane,
    pub idx: libc::c_int,
}
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const TTY_UNKNOWN: unnamed_15 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_line {
    pub cellused: u_int,
    pub cellsize: u_int,
    pub celldata: *mut grid_cell_entry,
    pub extdsize: u_int,
    pub extddata: *mut grid_cell,
    pub flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_12 {
    ev_io: unnamed_14,
    ev_signal: unnamed,
}
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub ev_io_next: unnamed_1,
    pub ev_timeout: timeval,
}
pub type unnamed_15 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type uint8_t = libc::c_uchar;
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_32,
    pub ev_next: unnamed_4,
    pub ev_timeout_pos: unnamed_25,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_12,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_flags: libc::c_short,
    pub ev_pri: uint8_t,
    pub ev_closure: uint8_t,
    pub ev_timeout: timeval,
    pub ev_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_short,
                                                 _: *mut libc::c_void) -> ()>,
    pub ev_arg: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_17 {
    __wch: libc::c_uint,
    __wchb: [libc::c_char; 4],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_23,
}
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_FILE_0 {
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
    pub _chain: *mut _IO_FILE_0,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub type layout_type = libc::c_uint;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_19,
    pub flags: libc::c_int,
    pub cmd: *mut libc::c_char,
    pub pid: pid_t,
    pub status: libc::c_int,
    pub fd: libc::c_int,
    pub event: *mut bufferevent,
    pub updatecb: job_update_cb,
    pub completecb: job_complete_cb,
    pub freecb: job_free_cb,
    pub data: *mut libc::c_void,
    pub entry: unnamed_7,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: unnamed_17,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid {
    pub flags: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub hscrolled: u_int,
    pub hsize: u_int,
    pub hlimit: u_int,
    pub linedata: *mut grid_line,
}
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_0,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub type unnamed_19 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub const TTY_VT101: unnamed_15 = 1;
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub const JOB_CLOSED: unnamed_19 = 2;
pub type __timezone_ptr_t = *mut timezone;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const JOB_DEAD: unnamed_19 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty {
    pub client: *mut client,
    pub sx: u_int,
    pub sy: u_int,
    pub cx: u_int,
    pub cy: u_int,
    pub cstyle: u_int,
    pub ccolour: *mut libc::c_char,
    pub mode: libc::c_int,
    pub rlower: u_int,
    pub rupper: u_int,
    pub rleft: u_int,
    pub rright: u_int,
    pub fd: libc::c_int,
    pub event_in: event,
    pub in_0: *mut evbuffer,
    pub event_out: event,
    pub out: *mut evbuffer,
    pub timer: event,
    pub discarded: size_t,
    pub tio: termios,
    pub cell: grid_cell,
    pub last_wp: libc::c_int,
    pub last_cell: grid_cell,
    pub flags: libc::c_int,
    pub term: *mut tty_term,
    pub term_name: *mut libc::c_char,
    pub term_flags: libc::c_int,
    pub term_type: unnamed_15,
    pub mouse: mouse_event,
    pub mouse_drag_flag: libc::c_int,
    pub mouse_drag_update: Option<unsafe extern "C" fn(_: *mut client,
                                                       _: *mut mouse_event)
                                      -> ()>,
    pub mouse_drag_release: Option<unsafe extern "C" fn(_: *mut client,
                                                        _: *mut mouse_event)
                                       -> ()>,
    pub key_timer: event,
    pub key_tree: *mut tty_key,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct client {
    pub name: *const libc::c_char,
    pub peer: *mut tmuxpeer,
    pub queue: cmdq_list,
    pub pid: pid_t,
    pub fd: libc::c_int,
    pub event: event,
    pub retval: libc::c_int,
    pub creation_time: timeval,
    pub activity_time: timeval,
    pub environ: *mut environ,
    pub jobs: *mut format_job_tree,
    pub title: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub term: *mut libc::c_char,
    pub ttyname: *mut libc::c_char,
    pub tty: tty,
    pub written: size_t,
    pub discarded: size_t,
    pub redraw: size_t,
    pub stdin_callback: Option<unsafe extern "C" fn(_: *mut client,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_void)
                                   -> ()>,
    pub stdin_callback_data: *mut libc::c_void,
    pub stdin_data: *mut evbuffer,
    pub stdin_closed: libc::c_int,
    pub stdout_data: *mut evbuffer,
    pub stderr_data: *mut evbuffer,
    pub repeat_timer: event,
    pub click_timer: event,
    pub click_button: u_int,
    pub status: status_line,
    pub flags: libc::c_int,
    pub keytable: *mut key_table,
    pub identify_timer: event,
    pub identify_callback: Option<unsafe extern "C" fn(_: *mut client,
                                                       _: *mut window_pane)
                                      -> ()>,
    pub identify_callback_data: *mut libc::c_void,
    pub message_string: *mut libc::c_char,
    pub message_timer: event,
    pub message_next: u_int,
    pub message_log: unnamed_34,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_37,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_10,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const TTY_VT320: unnamed_15 = 4;
pub type __socklen_t = libc::c_uint;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
    pub __cmsg_data: [libc::c_uchar; 0],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_6,
    pub entry: unnamed_16,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_25 {
    ev_next_with_common_timeout: unnamed_24,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_mode {
    pub name: *const libc::c_char,
    pub init: Option<unsafe extern "C" fn(_: *mut window_pane,
                                          _: *mut cmd_find_state,
                                          _: *mut args) -> *mut screen>,
    pub free: Option<unsafe extern "C" fn(_: *mut window_pane) -> ()>,
    pub resize: Option<unsafe extern "C" fn(_: *mut window_pane, _: u_int,
                                            _: u_int) -> ()>,
    pub key: Option<unsafe extern "C" fn(_: *mut window_pane, _: *mut client,
                                         _: *mut session, _: key_code,
                                         _: *mut mouse_event) -> ()>,
    pub key_table: Option<unsafe extern "C" fn(_: *mut window_pane)
                              -> *const libc::c_char>,
    pub command: Option<unsafe extern "C" fn(_: *mut window_pane,
                                             _: *mut client, _: *mut session,
                                             _: *mut args,
                                             _: *mut mouse_event) -> ()>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type speed_t = libc::c_uint;
pub type cmd_retval = libc::c_int;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type socklen_t = __socklen_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_0 = 2;
pub const TTY_VT220: unnamed_15 = 3;
pub type u_int = __u_int;
pub const PROMPT_ENTRY: unnamed_37 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub type bitstr_t = libc::c_uchar;
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session {
    pub id: u_int,
    pub name: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub creation_time: timeval,
    pub last_attached_time: timeval,
    pub activity_time: timeval,
    pub last_activity_time: timeval,
    pub lock_timer: event,
    pub sx: u_int,
    pub sy: u_int,
    pub curw: *mut winlink,
    pub lastw: winlink_stack,
    pub windows: winlinks,
    pub statusat: libc::c_int,
    pub hooks: *mut hooks,
    pub options: *mut options,
    pub flags: libc::c_int,
    pub attached: u_int,
    pub tio: *mut termios,
    pub environ: *mut environ,
    pub references: libc::c_int,
    pub gentry: unnamed_30,
    pub entry: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_5,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct options_table_entry {
    pub name: *const libc::c_char,
    pub type_0: options_table_type,
    pub scope: options_table_scope,
    pub minimum: u_int,
    pub maximum: u_int,
    pub choices: *mut *const libc::c_char,
    pub default_str: *const libc::c_char,
    pub default_num: libc::c_longlong,
    pub separator: *const libc::c_char,
    pub style: *const libc::c_char,
}
pub type uint32_t = libc::c_uint;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_33 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const LINE_SEL_LEFT_RIGHT: unnamed_0 = 1;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const TTY_VT102: unnamed_15 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane {
    pub id: u_int,
    pub active_point: u_int,
    pub window: *mut window,
    pub layout_cell: *mut layout_cell,
    pub saved_layout_cell: *mut layout_cell,
    pub sx: u_int,
    pub sy: u_int,
    pub osx: u_int,
    pub osy: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub flags: libc::c_int,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub shell: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub pid: pid_t,
    pub tty: [libc::c_char; 32],
    pub status: libc::c_int,
    pub fd: libc::c_int,
    pub event: *mut bufferevent,
    pub resize_timer: event,
    pub ictx: *mut input_ctx,
    pub colgc: grid_cell,
    pub palette: *mut libc::c_int,
    pub pipe_fd: libc::c_int,
    pub pipe_event: *mut bufferevent,
    pub pipe_off: size_t,
    pub screen: *mut screen,
    pub base: screen,
    pub status_screen: screen,
    pub status_size: size_t,
    pub saved_cx: u_int,
    pub saved_cy: u_int,
    pub saved_grid: *mut grid,
    pub saved_cell: grid_cell,
    pub mode: *const window_mode,
    pub modedata: *mut libc::c_void,
    pub modetimer: event,
    pub modelast: time_t,
    pub modeprefix: u_int,
    pub searchstr: *mut libc::c_char,
    pub entry: unnamed_18,
    pub tree_entry: unnamed_35,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct bufferevent {
    pub ev_base: *mut event_base,
    pub be_ops: *const bufferevent_ops,
    pub ev_read: event,
    pub ev_write: event,
    pub input: *mut evbuffer,
    pub output: *mut evbuffer,
    pub wm_read: event_watermark,
    pub wm_write: event_watermark,
    pub readcb: bufferevent_data_cb,
    pub writecb: bufferevent_data_cb,
    pub errorcb: bufferevent_event_cb,
    pub cbarg: *mut libc::c_void,
    pub timeout_read: timeval,
    pub timeout_write: timeval,
    pub enabled: libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen {
    pub title: *mut libc::c_char,
    pub titles: *mut screen_titles,
    pub grid: *mut grid,
    pub cx: u_int,
    pub cy: u_int,
    pub cstyle: u_int,
    pub ccolour: *mut libc::c_char,
    pub rupper: u_int,
    pub rlower: u_int,
    pub mode: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: screen_sel,
}
pub type unnamed_37 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type __ssize_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_11,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
unsafe extern "C" fn gnu_dev_major(mut __dev: libc::c_ulonglong)
 -> libc::c_uint {
    return (__dev >> 8i32 & 4095i32 as libc::c_ulonglong |
                ((__dev >> 32i32) as libc::c_uint & !4095i32 as libc::c_uint)
                    as libc::c_ulonglong) as libc::c_uint;
}
unsafe extern "C" fn gnu_dev_minor(mut __dev: libc::c_ulonglong)
 -> libc::c_uint {
    return (__dev & 255i32 as libc::c_ulonglong |
                ((__dev >> 12i32) as libc::c_uint & !255i32 as libc::c_uint)
                    as libc::c_ulonglong) as libc::c_uint;
}
unsafe extern "C" fn gnu_dev_makedev(mut __major: libc::c_uint,
                                     mut __minor: libc::c_uint)
 -> libc::c_ulonglong {
    return (__minor & 255i32 as libc::c_uint |
                (__major & 4095i32 as libc::c_uint) << 8i32) as
               libc::c_ulonglong |
               ((__minor & !255i32 as libc::c_uint) as libc::c_ulonglong) <<
                   12i32 |
               ((__major & !4095i32 as libc::c_uint) as libc::c_ulonglong) <<
                   32i32;
}
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10i32) as libc::c_int;
}
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10i32);
}
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char)
 -> libc::c_longlong {
    return strtoll(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                   10i32);
}
unsafe extern "C" fn bsearch(mut __key: *const libc::c_void,
                             mut __base: *const libc::c_void,
                             mut __nmemb: size_t, mut __size: size_t,
                             mut __compar: __compar_fn_t)
 -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0i32 as size_t;
    __u = __nmemb;
    loop  {
        if __l < __u {
            __idx = __l.wrapping_add(__u).wrapping_div(2i32 as libc::c_ulong);
            __p =
                (__base as
                     *const libc::c_char).offset(__idx.wrapping_mul(__size) as
                                                     isize) as
                    *mut libc::c_void;
            __comparison =
                __compar.expect("non-null function pointer")(__key, __p);
            if __comparison < 0i32 {
                __u = __idx
            } else if __comparison > 0i32 {
                __l = __idx.wrapping_add(1i32 as libc::c_ulong)
            } else { return __p as *mut libc::c_void }
        } else { return 0 as *mut libc::c_void }
    };
}
unsafe extern "C" fn __strcspn_c1(mut __s: *const libc::c_char,
                                  mut __reject: libc::c_int) -> size_t {
    let mut __result: size_t = 0i32 as size_t;
    while *__s.offset(__result as isize) as libc::c_int != 0 &&
              *__s.offset(__result as isize) as libc::c_int != __reject {
        __result = __result.wrapping_add(1)
    }
    return __result;
}
unsafe extern "C" fn __strcspn_c2(mut __s: *const libc::c_char,
                                  mut __reject1: libc::c_int,
                                  mut __reject2: libc::c_int) -> size_t {
    let mut __result: size_t = 0i32 as size_t;
    while *__s.offset(__result as isize) as libc::c_int != 0 &&
              *__s.offset(__result as isize) as libc::c_int != __reject1 &&
              *__s.offset(__result as isize) as libc::c_int != __reject2 {
        __result = __result.wrapping_add(1)
    }
    return __result;
}
unsafe extern "C" fn __strcspn_c3(mut __s: *const libc::c_char,
                                  mut __reject1: libc::c_int,
                                  mut __reject2: libc::c_int,
                                  mut __reject3: libc::c_int) -> size_t {
    let mut __result: size_t = 0i32 as size_t;
    while *__s.offset(__result as isize) as libc::c_int != 0 &&
              *__s.offset(__result as isize) as libc::c_int != __reject1 &&
              *__s.offset(__result as isize) as libc::c_int != __reject2 &&
              *__s.offset(__result as isize) as libc::c_int != __reject3 {
        __result = __result.wrapping_add(1)
    }
    return __result;
}
unsafe extern "C" fn __strspn_c1(mut __s: *const libc::c_char,
                                 mut __accept: libc::c_int) -> size_t {
    let mut __result: size_t = 0i32 as size_t;
    while *__s.offset(__result as isize) as libc::c_int == __accept {
        __result = __result.wrapping_add(1)
    }
    return __result;
}
unsafe extern "C" fn __strspn_c2(mut __s: *const libc::c_char,
                                 mut __accept1: libc::c_int,
                                 mut __accept2: libc::c_int) -> size_t {
    let mut __result: size_t = 0i32 as size_t;
    while *__s.offset(__result as isize) as libc::c_int == __accept1 ||
              *__s.offset(__result as isize) as libc::c_int == __accept2 {
        __result = __result.wrapping_add(1)
    }
    return __result;
}
unsafe extern "C" fn __strspn_c3(mut __s: *const libc::c_char,
                                 mut __accept1: libc::c_int,
                                 mut __accept2: libc::c_int,
                                 mut __accept3: libc::c_int) -> size_t {
    let mut __result: size_t = 0i32 as size_t;
    while *__s.offset(__result as isize) as libc::c_int == __accept1 ||
              *__s.offset(__result as isize) as libc::c_int == __accept2 ||
              *__s.offset(__result as isize) as libc::c_int == __accept3 {
        __result = __result.wrapping_add(1)
    }
    return __result;
}
unsafe extern "C" fn __strpbrk_c2(mut __s: *const libc::c_char,
                                  mut __accept1: libc::c_int,
                                  mut __accept2: libc::c_int)
 -> *mut libc::c_char {
    while *__s as libc::c_int != 0 && *__s as libc::c_int != __accept1 &&
              *__s as libc::c_int != __accept2 {
        __s = __s.offset(1isize)
    }
    return if *__s as libc::c_int == 0 {
               0 as *mut libc::c_char
           } else { __s as size_t as *mut libc::c_char };
}
unsafe extern "C" fn __strpbrk_c3(mut __s: *const libc::c_char,
                                  mut __accept1: libc::c_int,
                                  mut __accept2: libc::c_int,
                                  mut __accept3: libc::c_int)
 -> *mut libc::c_char {
    while *__s as libc::c_int != 0 && *__s as libc::c_int != __accept1 &&
              *__s as libc::c_int != __accept2 &&
              *__s as libc::c_int != __accept3 {
        __s = __s.offset(1isize)
    }
    return if *__s as libc::c_int == 0 {
               0 as *mut libc::c_char
           } else { __s as size_t as *mut libc::c_char };
}
unsafe extern "C" fn __strtok_r_1c(mut __s: *mut libc::c_char,
                                   mut __sep: libc::c_char,
                                   mut __nextp: *mut *mut libc::c_char)
 -> *mut libc::c_char {
    let mut __result: *mut libc::c_char = 0 as *mut libc::c_char;
    if __s == 0 as *mut libc::c_void as *mut libc::c_char { __s = *__nextp }
    while *__s as libc::c_int == __sep as libc::c_int {
        __s = __s.offset(1isize)
    }
    __result = 0 as *mut libc::c_char;
    if *__s as libc::c_int != 0 {
        let fresh0 = __s;
        __s = __s.offset(1);
        __result = fresh0;
        while *__s as libc::c_int != 0 {
            let fresh1 = __s;
            __s = __s.offset(1);
            if !(*fresh1 as libc::c_int == __sep as libc::c_int) {
                continue ;
            }
            *__s.offset(1i32.wrapping_neg() as isize) = 0 as libc::c_char;
            break ;
        }
    }
    *__nextp = __s;
    return __result;
}
unsafe extern "C" fn __strsep_1c(mut __s: *mut *mut libc::c_char,
                                 mut __reject: libc::c_char)
 -> *mut libc::c_char {
    let mut __retval: *mut libc::c_char = *__s;
    if __retval != 0 as *mut libc::c_void as *mut libc::c_char &&
           {
               *__s =
                   if 0 !=
                          Some(__builtin_constant_p).expect("non-null function pointer")(__reject)
                          &&
                          0 ==
                              Some(__builtin_constant_p).expect("non-null function pointer")(__retval)
                          && __reject as libc::c_int == 0 {
                       __rawmemchr(__retval as *const libc::c_void,
                                   __reject as libc::c_int) as
                           *mut libc::c_char
                   } else {
                       Some(__builtin_strchr).expect("non-null function pointer")(__retval,
                                                                                  __reject
                                                                                      as
                                                                                      libc::c_int)
                   };
               *__s != 0 as *mut libc::c_void as *mut libc::c_char
           } {
        let fresh2 = *__s;
        *__s = (*__s).offset(1);
        *fresh2 = 0 as libc::c_char
    }
    return __retval;
}
unsafe extern "C" fn __strsep_2c(mut __s: *mut *mut libc::c_char,
                                 mut __reject1: libc::c_char,
                                 mut __reject2: libc::c_char)
 -> *mut libc::c_char {
    let mut __retval: *mut libc::c_char = *__s;
    if __retval != 0 as *mut libc::c_void as *mut libc::c_char {
        let mut __cp: *mut libc::c_char = __retval;
        loop  {
            if *__cp as libc::c_int == 0 {
                __cp = 0 as *mut libc::c_char;
                break ;
            } else if *__cp as libc::c_int == __reject1 as libc::c_int ||
                          *__cp as libc::c_int == __reject2 as libc::c_int {
                let fresh3 = __cp;
                __cp = __cp.offset(1);
                *fresh3 = 0 as libc::c_char;
                break ;
            } else { __cp = __cp.offset(1isize) }
        }
        *__s = __cp
    }
    return __retval;
}
unsafe extern "C" fn __strsep_3c(mut __s: *mut *mut libc::c_char,
                                 mut __reject1: libc::c_char,
                                 mut __reject2: libc::c_char,
                                 mut __reject3: libc::c_char)
 -> *mut libc::c_char {
    let mut __retval: *mut libc::c_char = *__s;
    if __retval != 0 as *mut libc::c_void as *mut libc::c_char {
        let mut __cp: *mut libc::c_char = __retval;
        loop  {
            if *__cp as libc::c_int == 0 {
                __cp = 0 as *mut libc::c_char;
                break ;
            } else if *__cp as libc::c_int == __reject1 as libc::c_int ||
                          *__cp as libc::c_int == __reject2 as libc::c_int ||
                          *__cp as libc::c_int == __reject3 as libc::c_int {
                let fresh4 = __cp;
                __cp = __cp.offset(1);
                *fresh4 = 0 as libc::c_char;
                break ;
            } else { __cp = __cp.offset(1isize) }
        }
        *__s = __cp
    }
    return __retval;
}
unsafe extern "C" fn __cmsg_nxthdr(mut __mhdr: *mut msghdr,
                                   mut __cmsg: *mut cmsghdr) -> *mut cmsghdr {
    if (*__cmsg).cmsg_len < ::std::mem::size_of::<cmsghdr>() as libc::c_ulong
       {
        return 0 as *mut cmsghdr
    } else {
        __cmsg =
            (__cmsg as
                 *mut libc::c_uchar).offset(((*__cmsg).cmsg_len.wrapping_add(::std::mem::size_of::<size_t>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                                                 as
                                                                                                                 libc::c_ulong)
                                                 &
                                                 !(::std::mem::size_of::<size_t>()
                                                       as
                                                       libc::c_ulong).wrapping_sub(1i32
                                                                                       as
                                                                                       libc::c_ulong))
                                                as isize) as *mut cmsghdr;
        if __cmsg.offset(1isize) as *mut libc::c_uchar >
               ((*__mhdr).msg_control as
                    *mut libc::c_uchar).offset((*__mhdr).msg_controllen as
                                                   isize) ||
               (__cmsg as
                    *mut libc::c_uchar).offset(((*__cmsg).cmsg_len.wrapping_add(::std::mem::size_of::<size_t>()
                                                                                    as
                                                                                    libc::c_ulong).wrapping_sub(1i32
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                                                    &
                                                    !(::std::mem::size_of::<size_t>()
                                                          as
                                                          libc::c_ulong).wrapping_sub(1i32
                                                                                          as
                                                                                          libc::c_ulong))
                                                   as isize) >
                   ((*__mhdr).msg_control as
                        *mut libc::c_uchar).offset((*__mhdr).msg_controllen as
                                                       isize) {
            return 0 as *mut cmsghdr
        } else { return __cmsg }
    };
}
unsafe extern "C" fn vprintf(mut __fmt: *const libc::c_char,
                             mut __arg: *mut __va_list_tag) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg);
}
unsafe extern "C" fn getchar() -> libc::c_int { return _IO_getc(stdin); }
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if 0 !=
                  Some(__builtin_expect).expect("non-null function pointer")(((*__fp)._IO_read_ptr
                                                                                  >=
                                                                                  (*__fp)._IO_read_end)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_long,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_long)
              {
               __uflow(__fp)
           } else {
               let fresh5 = (*__fp)._IO_read_ptr;
               (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
               *(fresh5 as *mut libc::c_uchar) as libc::c_int
           };
}
unsafe extern "C" fn getchar_unlocked() -> libc::c_int {
    return if 0 !=
                  Some(__builtin_expect).expect("non-null function pointer")(((*stdin)._IO_read_ptr
                                                                                  >=
                                                                                  (*stdin)._IO_read_end)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_long,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_long)
              {
               __uflow(stdin)
           } else {
               let fresh6 = (*stdin)._IO_read_ptr;
               (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
               *(fresh6 as *mut libc::c_uchar) as libc::c_int
           };
}
unsafe extern "C" fn fgetc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if 0 !=
                  Some(__builtin_expect).expect("non-null function pointer")(((*__fp)._IO_read_ptr
                                                                                  >=
                                                                                  (*__fp)._IO_read_end)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_long,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_long)
              {
               __uflow(__fp)
           } else {
               let fresh7 = (*__fp)._IO_read_ptr;
               (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
               *(fresh7 as *mut libc::c_uchar) as libc::c_int
           };
}
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return _IO_putc(__c, stdout);
}
unsafe extern "C" fn fputc_unlocked(mut __c: libc::c_int,
                                    mut __stream: *mut FILE) -> libc::c_int {
    return if 0 !=
                  Some(__builtin_expect).expect("non-null function pointer")(((*__stream)._IO_write_ptr
                                                                                  >=
                                                                                  (*__stream)._IO_write_end)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_long,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_long)
              {
               __overflow(__stream, __c as libc::c_uchar as libc::c_int)
           } else {
               let fresh8 = (*__stream)._IO_write_ptr;
               (*__stream)._IO_write_ptr =
                   (*__stream)._IO_write_ptr.offset(1);
               *fresh8 = __c as libc::c_char;
               *fresh8 as libc::c_uchar as libc::c_int
           };
}
unsafe extern "C" fn putc_unlocked(mut __c: libc::c_int,
                                   mut __stream: *mut FILE) -> libc::c_int {
    return if 0 !=
                  Some(__builtin_expect).expect("non-null function pointer")(((*__stream)._IO_write_ptr
                                                                                  >=
                                                                                  (*__stream)._IO_write_end)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_long,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_long)
              {
               __overflow(__stream, __c as libc::c_uchar as libc::c_int)
           } else {
               let fresh9 = (*__stream)._IO_write_ptr;
               (*__stream)._IO_write_ptr =
                   (*__stream)._IO_write_ptr.offset(1);
               *fresh9 = __c as libc::c_char;
               *fresh9 as libc::c_uchar as libc::c_int
           };
}
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if 0 !=
                  Some(__builtin_expect).expect("non-null function pointer")(((*stdout)._IO_write_ptr
                                                                                  >=
                                                                                  (*stdout)._IO_write_end)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_long,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_long)
              {
               __overflow(stdout, __c as libc::c_uchar as libc::c_int)
           } else {
               let fresh10 = (*stdout)._IO_write_ptr;
               (*stdout)._IO_write_ptr = (*stdout)._IO_write_ptr.offset(1);
               *fresh10 = __c as libc::c_char;
               *fresh10 as libc::c_uchar as libc::c_int
           };
}
unsafe extern "C" fn getline(mut __lineptr: *mut *mut libc::c_char,
                             mut __n: *mut size_t, mut __stream: *mut FILE)
 -> __ssize_t {
    return __getdelim(__lineptr, __n, 10, __stream);
}
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 16i32 != 0i32) as libc::c_int;
}
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 32i32 != 0i32) as libc::c_int;
}
unsafe extern "C" fn btowc(mut __c: libc::c_int) -> wint_t {
    return if 0 !=
                  Some(__builtin_constant_p).expect("non-null function pointer")(__c)
                  && __c >= 0 && __c <= 127 {
               __c as wint_t
           } else { __btowc_alias(__c) };
}
unsafe extern "C" fn wctob(mut __wc: wint_t) -> libc::c_int {
    return if 0 !=
                  Some(__builtin_constant_p).expect("non-null function pointer")(__wc)
                  && __wc >= 0 as libc::c_uint && __wc <= 127 as libc::c_uint
              {
               __wc as libc::c_int
           } else { __wctob_alias(__wc) };
}
unsafe extern "C" fn mbrlen(mut __s: *const libc::c_char, mut __n: size_t,
                            mut __ps: *mut mbstate_t) -> size_t {
    return if __ps != 0 as *mut libc::c_void as *mut mbstate_t {
               mbrtowc(0 as *mut wchar_t, __s, __n, __ps)
           } else { __mbrlen(__s, __n, 0 as *mut mbstate_t) };
}
unsafe extern "C" fn cmd_switch_client_exec(mut self_0: *mut cmd,
                                            mut item: *mut cmdq_item)
 -> cmd_retval {
    let mut args: *mut args = (*self_0).args;
    let mut tflag: *const libc::c_char = args_get(args, 116 as u_char);
    let mut type_0: cmd_find_type = CMD_FIND_PANE;
    let mut flags: libc::c_int = 0;
    let mut c: *mut client = 0 as *mut client;
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut tablename: *const libc::c_char = 0 as *const libc::c_char;
    let mut table: *mut key_table = 0 as *mut key_table;
    c = cmd_find_client(item, args_get(args, 99 as u_char), 0i32);
    if c == 0 as *mut libc::c_void as *mut client {
        return CMD_RETURN_ERROR
    } else {
        if tflag != 0 as *mut libc::c_void as *const libc::c_char &&
               *tflag.offset({
                                 let mut __r0: libc::c_char = 0;
                                 let mut __r1: libc::c_char = 0;
                                 let mut __r2: libc::c_char = 0;
                                 if 0 !=
                                        Some(__builtin_constant_p).expect("non-null function pointer")(*::std::mem::transmute::<&[u8; 3],
                                                                                                                                &[libc::c_char; 3]>(b":.\x00"))
                                        &&
                                        ((b":.\x00" as *const u8 as
                                              *const libc::c_char).offset(1isize)
                                             as *const libc::c_void as
                                             size_t).wrapping_sub(b":.\x00" as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char
                                                                      as
                                                                      *const libc::c_void
                                                                      as
                                                                      size_t)
                                            == 1i32 as libc::c_ulong {
                                     if 0 !=
                                            Some(__builtin_constant_p).expect("non-null function pointer")(tflag)
                                            &&
                                            (tflag.offset(1isize) as
                                                 *const libc::c_void as
                                                 size_t).wrapping_sub(tflag as
                                                                          *const libc::c_void
                                                                          as
                                                                          size_t)
                                                == 1i32 as libc::c_ulong {
                                         Some(__builtin_strcspn).expect("non-null function pointer")(tflag,
                                                                                                     b":.\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char)
                                     } else {
                                         __r0 =
                                             *(b":.\x00" as *const u8 as
                                                   *const libc::c_char).offset(0isize);
                                         if __r0 as libc::c_int == 0 {
                                             strlen(tflag)
                                         } else {
                                             __r1 =
                                                 *(b":.\x00" as *const u8 as
                                                       *const libc::c_char).offset(1isize);
                                             if __r1 as libc::c_int == 0 {
                                                 __strcspn_c1(tflag,
                                                              __r0 as
                                                                  libc::c_int)
                                             } else {
                                                 __r2 =
                                                     *(b":.\x00" as *const u8
                                                           as
                                                           *const libc::c_char).offset(2isize);
                                                 if __r2 as libc::c_int == 0 {
                                                     __strcspn_c2(tflag,
                                                                  __r0 as
                                                                      libc::c_int,
                                                                  __r1 as
                                                                      libc::c_int)
                                                 } else if *(b":.\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char).offset(3isize)
                                                               as libc::c_int
                                                               == 0 {
                                                     __strcspn_c3(tflag,
                                                                  __r0 as
                                                                      libc::c_int,
                                                                  __r1 as
                                                                      libc::c_int,
                                                                  __r2 as
                                                                      libc::c_int)
                                                 } else {
                                                     Some(__builtin_strcspn).expect("non-null function pointer")(tflag,
                                                                                                                 b":.\x00"
                                                                                                                     as
                                                                                                                     *const u8
                                                                                                                     as
                                                                                                                     *const libc::c_char)
                                                 }
                                             }
                                         }
                                     }
                                 } else {
                                     Some(__builtin_strcspn).expect("non-null function pointer")(tflag,
                                                                                                 b":.\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char)
                                 }
                             } as isize) as libc::c_int != 0 {
            type_0 = CMD_FIND_PANE;
            flags = 0i32
        } else { type_0 = CMD_FIND_SESSION; flags = 1i32 }
        if cmd_find_target(&mut (*item).target as *mut cmd_find_state, item,
                           tflag, type_0, flags) != 0i32 {
            return CMD_RETURN_ERROR
        } else {
            s = (*item).target.s;
            wl = (*item).target.wl;
            wp = (*item).target.wp;
            if 0 != args_has(args, 114 as u_char) { (*c).flags ^= 2048i32 }
            tablename = args_get(args, 84 as u_char);
            if tablename != 0 as *mut libc::c_void as *const libc::c_char {
                table = key_bindings_get_table(tablename, 0i32);
                if table == 0 as *mut libc::c_void as *mut key_table {
                    cmdq_error(item,
                               b"table %s doesn\'t exist\x00" as *const u8 as
                                   *const libc::c_char, tablename);
                    return CMD_RETURN_ERROR
                } else {
                    (*table).references = (*table).references.wrapping_add(1);
                    key_bindings_unref_table((*c).keytable);
                    (*c).keytable = table;
                    return CMD_RETURN_NORMAL
                }
            } else {
                if 0 != args_has(args, 110 as u_char) {
                    s = session_next_session((*c).session);
                    if s == 0 as *mut libc::c_void as *mut session {
                        cmdq_error(item,
                                   b"can\'t find next session\x00" as
                                       *const u8 as *const libc::c_char);
                        return CMD_RETURN_ERROR
                    }
                } else if 0 != args_has(args, 112 as u_char) {
                    s = session_previous_session((*c).session);
                    if s == 0 as *mut libc::c_void as *mut session {
                        cmdq_error(item,
                                   b"can\'t find previous session\x00" as
                                       *const u8 as *const libc::c_char);
                        return CMD_RETURN_ERROR
                    }
                } else if 0 != args_has(args, 108 as u_char) {
                    if (*c).last_session !=
                           0 as *mut libc::c_void as *mut session &&
                           0 != session_alive((*c).last_session) {
                        s = (*c).last_session
                    } else { s = 0 as *mut session }
                    if s == 0 as *mut libc::c_void as *mut session {
                        cmdq_error(item,
                                   b"can\'t find last session\x00" as
                                       *const u8 as *const libc::c_char);
                        return CMD_RETURN_ERROR
                    }
                } else if (*item).client ==
                              0 as *mut libc::c_void as *mut client {
                    return CMD_RETURN_NORMAL
                } else if wl != 0 as *mut libc::c_void as *mut winlink {
                    if wp != 0 as *mut libc::c_void as *mut window_pane {
                        window_set_active_pane((*wp).window, wp);
                    }
                    session_set_current(s, wl);
                    cmd_find_from_session(&mut (*(*item).shared).current as
                                              *mut cmd_find_state, s, 0i32);
                }
                if 0 == args_has(args, 69 as u_char) {
                    environ_update((*s).options, (*c).environ, (*s).environ);
                }
                if (*c).session != 0 as *mut libc::c_void as *mut session &&
                       (*c).session != s {
                    (*c).last_session = (*c).session
                }
                (*c).session = s;
                if 0 != !(*(*item).shared).flags & 1i32 {
                    server_client_set_key_table(c, 0 as *const libc::c_char);
                }
                status_timer_start(c);
                notify_client(b"client-session-changed\x00" as *const u8 as
                                  *const libc::c_char, c);
                session_update_activity(s, 0 as *mut timeval);
                gettimeofday(&mut (*s).last_attached_time as *mut timeval,
                             0 as *mut timezone);
                recalculate_sizes();
                server_check_unattached();
                server_redraw_client(c);
                (*(*s).curw).flags &= !(1i32 | 2i32 | 4i32);
                alerts_check_session(s);
                return CMD_RETURN_NORMAL
            }
        }
    };
}
#[no_mangle]
pub static mut cmd_switch_client_entry: cmd_entry =
    unsafe {
        cmd_entry{name:
                      b"switch-client\x00" as *const u8 as
                          *const libc::c_char,
                  alias: b"switchc\x00" as *const u8 as *const libc::c_char,
                  args:
                      unnamed_36{template:
                                     b"lc:Enpt:rT:\x00" as *const u8 as
                                         *const libc::c_char,
                                 lower: 0i32,
                                 upper: 0i32,},
                  usage:
                      b"[-Elnpr] [-c target-client] [-t target-session] [-T key-table]\x00"
                          as *const u8 as *const libc::c_char,
                  source:
                      cmd_entry_flag{flag: 0,
                                     type_0: CMD_FIND_PANE,
                                     flags: 0,},
                  target:
                      cmd_entry_flag{flag: 0,
                                     type_0: CMD_FIND_PANE,
                                     flags: 0,},
                  flags: 2i32,
                  exec: Some(cmd_switch_client_exec),}
    };
