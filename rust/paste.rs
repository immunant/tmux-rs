#![feature ( libc )]
#![feature ( i128_type )]
#![feature ( const_ptr_null )]
#![feature ( offset_to )]
#![feature ( const_ptr_null_mut )]
#![feature ( extern_types )]
#![feature ( asm )]
#![allow ( non_upper_case_globals )]
#![allow ( non_camel_case_types )]
#![allow ( non_snake_case )]
#![allow ( dead_code )]
#![allow ( mutable_transmutes )]
#![allow ( unused_mut )]
extern crate libc;
extern "C" {
    pub type options;
    pub type evbuffer;
    pub type environ;
    pub type tty_code;
    pub type input_ctx;
    pub type _IO_FILE_plus;
    pub type tmuxproc;
    pub type tmuxpeer;
    pub type hooks;
    pub type args_entry;
    pub type screen_titles;
    pub type format_tree;
    pub type bufferevent_ops;
    pub type event_base;
    pub type format_job_tree;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    static mut __tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut __daylight: libc::c_int;
    #[no_mangle]
    static mut __timezone: libc::c_long;
    #[no_mangle]
    static mut tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut daylight: libc::c_int;
    #[no_mangle]
    static mut timezone: libc::c_long;
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
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> libc::c_ulong;
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
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, ...)
     -> libc::c_int;
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
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    fn utf8_strvis(_: *mut libc::c_char, _: *const libc::c_char, _: size_t,
                   _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
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
}
pub type unnamed = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_19,
}
pub type unnamed_0 = libc::c_uint;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub type options_table_scope = libc::c_uint;
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
    pub alerts_entry: unnamed_29,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_37,
    pub entry: unnamed_3,
}
pub const TTY_UNKNOWN: unnamed_0 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub type u_short = __u_short;
pub type _IO_lock_t = ();
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
    pub term_type: unnamed_0,
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
pub struct unnamed_2 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type uint32_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
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
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct paste_buffer {
    pub data: *mut libc::c_char,
    pub size: size_t,
    pub name: *mut libc::c_char,
    pub created: time_t,
    pub automatic: libc::c_int,
    pub order: u_int,
    pub name_entry: unnamed_38,
    pub time_entry: unnamed_20,
}
pub const TTY_VT102: unnamed_0 = 2;
pub type cmdq_type = libc::c_uint;
pub type key_code = libc::c_ulonglong;
pub type bitstr_t = libc::c_uchar;
pub type cmd_retval = libc::c_int;
pub const PROMPT_COMMAND: unnamed_33 = 1;
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
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_7,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub const TTY_VT100: unnamed_0 = 0;
pub const LINE_SEL_LEFT_RIGHT: unnamed_36 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_6 {
    offset: u_int,
    data: unnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub const JOB_DEAD: unnamed = 1;
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
    pub entry: unnamed_14,
    pub wentry: unnamed_4,
    pub sentry: unnamed_23,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_8,
    pub ev_next: unnamed_2,
    pub ev_timeout_pos: unnamed_26,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_28,
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
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
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
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const PROMPT_ENTRY: unnamed_33 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub type u_char = __u_char;
pub type uint16_t = libc::c_ushort;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const TTY_VT420: unnamed_0 = 5;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_12,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
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
    pub gentry: unnamed_35,
    pub entry: unnamed_40,
}
pub type pid_t = __pid_t;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
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
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type __u_int = libc::c_uint;
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub rbe_left: *mut paste_buffer,
    pub rbe_right: *mut paste_buffer,
    pub rbe_parent: *mut paste_buffer,
    pub rbe_color: libc::c_int,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
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
    pub entry: unnamed_21,
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
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_24,
    pub entry: unnamed_15,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub ev_signal_next: unnamed_31,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_16,
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
pub struct paste_time_tree {
    pub rbh_root: *mut paste_buffer,
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
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
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub type u_int = __u_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_25 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const TTY_VT320: unnamed_0 = 4;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_6,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_9,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_26 {
    ev_next_with_common_timeout: unnamed_32,
    min_heap_idx: libc::c_int,
}
pub const LINE_SEL_NONE: unnamed_36 = 0;
pub const TTY_VT101: unnamed_0 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub type options_table_type = libc::c_uint;
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed,
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
    pub entry: unnamed_17,
}
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
pub union unnamed_28 {
    ev_io: unnamed_30,
    ev_signal: unnamed_22,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_36,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const JOB_CLOSED: unnamed = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub ev_io_next: unnamed_18,
    pub ev_timeout: timeval,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_36 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_5,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
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
    pub entry: unnamed_10,
    pub tree_entry: unnamed_11,
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
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub type cc_t = libc::c_uchar;
pub type layout_type = libc::c_uint;
pub const JOB_RUNNING: unnamed = 0;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
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
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_25,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type unnamed_33 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub type unnamed_36 = libc::c_uint;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const TTY_VT220: unnamed_0 = 3;
pub type tcflag_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub rbe_left: *mut paste_buffer,
    pub rbe_right: *mut paste_buffer,
    pub rbe_parent: *mut paste_buffer,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct paste_name_tree {
    pub rbh_root: *mut paste_buffer,
}
pub type __off64_t = libc::c_long;
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
    pub prompt_mode: unnamed_33,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_27,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_40 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type __pid_t = libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn paste_buffer_name(mut pb: *mut paste_buffer)
 -> *const libc::c_char {
    return (*pb).name;
}
#[no_mangle]
pub unsafe extern "C" fn paste_buffer_order(mut pb: *mut paste_buffer)
 -> u_int {
    return (*pb).order;
}
#[no_mangle]
pub unsafe extern "C" fn paste_buffer_created(mut pb: *mut paste_buffer)
 -> time_t {
    return (*pb).created;
}
#[no_mangle]
pub unsafe extern "C" fn paste_buffer_data(mut pb: *mut paste_buffer,
                                           mut size: *mut size_t)
 -> *const libc::c_char {
    if size != 0 as *mut libc::c_void as *mut size_t { *size = (*pb).size }
    return (*pb).data;
}
#[no_mangle]
pub unsafe extern "C" fn paste_walk(mut pb: *mut paste_buffer)
 -> *mut paste_buffer {
    if pb == 0 as *mut libc::c_void as *mut paste_buffer {
        return paste_time_tree_RB_MINMAX(&mut paste_by_time as
                                             *mut paste_time_tree,
                                         1i32.wrapping_neg())
    } else { return paste_time_tree_RB_NEXT(pb) };
}
unsafe extern "C" fn paste_time_tree_RB_NEXT(mut elm: *mut paste_buffer)
 -> *mut paste_buffer {
    if !(*elm).time_entry.rbe_right.is_null() {
        elm = (*elm).time_entry.rbe_right;
        while !(*elm).time_entry.rbe_left.is_null() {
            elm = (*elm).time_entry.rbe_left
        }
    } else if !(*elm).time_entry.rbe_parent.is_null() &&
                  elm == (*(*elm).time_entry.rbe_parent).time_entry.rbe_left {
        elm = (*elm).time_entry.rbe_parent
    } else {
        while !(*elm).time_entry.rbe_parent.is_null() &&
                  elm == (*(*elm).time_entry.rbe_parent).time_entry.rbe_right
              {
            elm = (*elm).time_entry.rbe_parent
        }
        elm = (*elm).time_entry.rbe_parent
    }
    return elm;
}
static mut paste_by_time: paste_time_tree =
    unsafe {
        paste_time_tree{rbh_root:
                            0 as *const paste_buffer as *mut paste_buffer,}
    };
unsafe extern "C" fn paste_time_tree_RB_MINMAX(mut head: *mut paste_time_tree,
                                               mut val: libc::c_int)
 -> *mut paste_buffer {
    let mut tmp: *mut paste_buffer = (*head).rbh_root;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).time_entry.rbe_left
        } else { tmp = (*tmp).time_entry.rbe_right }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn paste_get_top(mut name: *mut *const libc::c_char)
 -> *mut paste_buffer {
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    pb =
        paste_time_tree_RB_MINMAX(&mut paste_by_time as *mut paste_time_tree,
                                  1i32.wrapping_neg());
    if pb == 0 as *mut libc::c_void as *mut paste_buffer {
        return 0 as *mut paste_buffer
    } else {
        if name != 0 as *mut libc::c_void as *mut *const libc::c_char {
            *name = (*pb).name
        }
        return pb
    };
}
#[no_mangle]
pub unsafe extern "C" fn paste_get_name(mut name: *const libc::c_char)
 -> *mut paste_buffer {
    let mut pbfind: paste_buffer =
        paste_buffer{data: 0 as *mut libc::c_char,
                     size: 0,
                     name: 0 as *mut libc::c_char,
                     created: 0,
                     automatic: 0,
                     order: 0,
                     name_entry:
                         unnamed_38{rbe_left: 0 as *mut paste_buffer,
                                    rbe_right: 0 as *mut paste_buffer,
                                    rbe_parent: 0 as *mut paste_buffer,
                                    rbe_color: 0,},
                     time_entry:
                         unnamed_20{rbe_left: 0 as *mut paste_buffer,
                                    rbe_right: 0 as *mut paste_buffer,
                                    rbe_parent: 0 as *mut paste_buffer,
                                    rbe_color: 0,},};
    if name == 0 as *mut libc::c_void as *const libc::c_char ||
           *name as libc::c_int == 0 {
        return 0 as *mut paste_buffer
    } else {
        pbfind.name = name as *mut libc::c_char;
        return paste_name_tree_RB_FIND(&mut paste_by_name as
                                           *mut paste_name_tree,
                                       &mut pbfind as *mut paste_buffer)
    };
}
static mut paste_by_name: paste_name_tree =
    unsafe {
        paste_name_tree{rbh_root:
                            0 as *const paste_buffer as *mut paste_buffer,}
    };
unsafe extern "C" fn paste_name_tree_RB_FIND(mut head: *mut paste_name_tree,
                                             mut elm: *mut paste_buffer)
 -> *mut paste_buffer {
    let mut tmp: *mut paste_buffer = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = paste_cmp_names(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).name_entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).name_entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut paste_buffer }
    };
}
unsafe extern "C" fn paste_cmp_names(mut a: *const paste_buffer,
                                     mut b: *const paste_buffer)
 -> libc::c_int {
    return strcmp((*a).name, (*b).name);
}
#[no_mangle]
pub unsafe extern "C" fn paste_free(mut pb: *mut paste_buffer) -> () {
    paste_name_tree_RB_REMOVE(&mut paste_by_name as *mut paste_name_tree, pb);
    paste_time_tree_RB_REMOVE(&mut paste_by_time as *mut paste_time_tree, pb);
    if 0 != (*pb).automatic {
        paste_num_automatic = paste_num_automatic.wrapping_sub(1)
    }
    free((*pb).data as *mut libc::c_void);
    free((*pb).name as *mut libc::c_void);
    free(pb as *mut libc::c_void);
}
static mut paste_num_automatic: u_int = unsafe { 0 };
unsafe extern "C" fn paste_time_tree_RB_REMOVE(mut head: *mut paste_time_tree,
                                               mut elm: *mut paste_buffer)
 -> *mut paste_buffer {
    let mut current_block: u64;
    let mut child: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut old: *mut paste_buffer = elm;
    let mut color: libc::c_int = 0;
    if (*elm).time_entry.rbe_left ==
           0 as *mut libc::c_void as *mut paste_buffer {
        child = (*elm).time_entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).time_entry.rbe_right ==
                  0 as *mut libc::c_void as *mut paste_buffer {
        child = (*elm).time_entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut paste_buffer = 0 as *mut paste_buffer;
        elm = (*elm).time_entry.rbe_right;
        loop  {
            left = (*elm).time_entry.rbe_left;
            if left.is_null() { break ; }
            elm = left
        }
        child = (*elm).time_entry.rbe_right;
        parent = (*elm).time_entry.rbe_parent;
        color = (*elm).time_entry.rbe_color;
        if !child.is_null() { (*child).time_entry.rbe_parent = parent }
        if !parent.is_null() {
            if (*parent).time_entry.rbe_left == elm {
                (*parent).time_entry.rbe_left = child
            } else { (*parent).time_entry.rbe_right = child }
            while 0 != 0i32 { }
        } else { (*head).rbh_root = child }
        if (*elm).time_entry.rbe_parent == old { parent = elm }
        (*elm).time_entry = (*old).time_entry;
        if !(*old).time_entry.rbe_parent.is_null() {
            if (*(*old).time_entry.rbe_parent).time_entry.rbe_left == old {
                (*(*old).time_entry.rbe_parent).time_entry.rbe_left = elm
            } else {
                (*(*old).time_entry.rbe_parent).time_entry.rbe_right = elm
            }
            while 0 != 0i32 { }
        } else { (*head).rbh_root = elm }
        (*(*old).time_entry.rbe_left).time_entry.rbe_parent = elm;
        if !(*old).time_entry.rbe_right.is_null() {
            (*(*old).time_entry.rbe_right).time_entry.rbe_parent = elm
        }
        if !parent.is_null() {
            left = parent;
            loop  {
                if 0 != 0i32 { continue ; }
                left = (*left).time_entry.rbe_parent;
                if left.is_null() { break ; }
            }
            current_block = 12771512870825406329;
        } else { current_block = 12771512870825406329; }
    }
    match current_block {
        9386390421034826751 => {
            parent = (*elm).time_entry.rbe_parent;
            color = (*elm).time_entry.rbe_color;
            if !child.is_null() { (*child).time_entry.rbe_parent = parent }
            if !parent.is_null() {
                if (*parent).time_entry.rbe_left == elm {
                    (*parent).time_entry.rbe_left = child
                } else { (*parent).time_entry.rbe_right = child }
                while 0 != 0i32 { }
            } else { (*head).rbh_root = child }
        }
        _ => { }
    }
    if color == 0i32 { paste_time_tree_RB_REMOVE_COLOR(head, parent, child); }
    return old;
}
unsafe extern "C" fn paste_time_tree_RB_REMOVE_COLOR(mut head:
                                                         *mut paste_time_tree,
                                                     mut parent:
                                                         *mut paste_buffer,
                                                     mut elm:
                                                         *mut paste_buffer)
 -> () {
    let mut current_block: u64;
    let mut tmp: *mut paste_buffer = 0 as *mut paste_buffer;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut paste_buffer ||
                      (*elm).time_entry.rbe_color == 0i32) &&
                     elm != (*head).rbh_root) {
                current_block = 11174649648027449784;
                break ;
            }
            if (*parent).time_entry.rbe_left == elm {
                tmp = (*parent).time_entry.rbe_right;
                if (*tmp).time_entry.rbe_color == 1i32 {
                    current_block = 17179679302217393232;
                } else { current_block = 14155750587950065367; }
                loop  {
                    match current_block {
                        14155750587950065367 => {
                            if ((*tmp).time_entry.rbe_left ==
                                    0 as *mut libc::c_void as
                                        *mut paste_buffer ||
                                    (*(*tmp).time_entry.rbe_left).time_entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).time_entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut paste_buffer ||
                                        (*(*tmp).time_entry.rbe_right).time_entry.rbe_color
                                            == 0i32) {
                                (*tmp).time_entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).time_entry.rbe_parent;
                                break ;
                            } else if (*tmp).time_entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut paste_buffer ||
                                          (*(*tmp).time_entry.rbe_right).time_entry.rbe_color
                                              == 0i32 {
                                current_block = 15976848397966268834;
                                break 's_4 ;
                            } else {
                                current_block = 7149356873433890176;
                                break 's_4 ;
                            }
                        }
                        _ => {
                            (*tmp).time_entry.rbe_color = 0i32;
                            (*parent).time_entry.rbe_color = 1i32;
                            if 0 != 0i32 {
                                current_block = 17179679302217393232;
                                continue ;
                            }
                            's_30:
                                loop  {
                                    tmp = (*parent).time_entry.rbe_right;
                                    (*parent).time_entry.rbe_right =
                                        (*tmp).time_entry.rbe_left;
                                    if !(*parent).time_entry.rbe_right.is_null()
                                       {
                                        (*(*tmp).time_entry.rbe_left).time_entry.rbe_parent
                                            = parent
                                    }
                                    while 0 != 0i32 { }
                                    (*tmp).time_entry.rbe_parent =
                                        (*parent).time_entry.rbe_parent;
                                    if !(*tmp).time_entry.rbe_parent.is_null()
                                       {
                                        if parent ==
                                               (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                                           {
                                            (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                                                = tmp
                                        } else {
                                            (*(*parent).time_entry.rbe_parent).time_entry.rbe_right
                                                = tmp
                                        }
                                    } else { (*head).rbh_root = tmp }
                                    (*tmp).time_entry.rbe_left = parent;
                                    (*parent).time_entry.rbe_parent = tmp;
                                    while 0 != 0i32 { }
                                    if !(*tmp).time_entry.rbe_parent.is_null()
                                       {
                                        current_block = 11050875288958768710;
                                    } else {
                                        current_block = 15240798224410183470;
                                    }
                                    loop  {
                                        match current_block {
                                            11050875288958768710 => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        11050875288958768710;
                                                } else {
                                                    current_block =
                                                        15240798224410183470;
                                                }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_30 ; }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).time_entry.rbe_right;
                            current_block = 14155750587950065367;
                        }
                    }
                }
            } else {
                tmp = (*parent).time_entry.rbe_left;
                if (*tmp).time_entry.rbe_color == 1i32 {
                    current_block = 6450597802325118133;
                } else { current_block = 7746103178988627676; }
                loop  {
                    match current_block {
                        7746103178988627676 => {
                            if ((*tmp).time_entry.rbe_left ==
                                    0 as *mut libc::c_void as
                                        *mut paste_buffer ||
                                    (*(*tmp).time_entry.rbe_left).time_entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).time_entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut paste_buffer ||
                                        (*(*tmp).time_entry.rbe_right).time_entry.rbe_color
                                            == 0i32) {
                                (*tmp).time_entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).time_entry.rbe_parent;
                                break ;
                            } else if (*tmp).time_entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut paste_buffer ||
                                          (*(*tmp).time_entry.rbe_left).time_entry.rbe_color
                                              == 0i32 {
                                current_block = 13826291924415791078;
                                break 's_4 ;
                            } else {
                                current_block = 5892776923941496671;
                                break 's_4 ;
                            }
                        }
                        _ => {
                            (*tmp).time_entry.rbe_color = 0i32;
                            (*parent).time_entry.rbe_color = 1i32;
                            if 0 != 0i32 {
                                current_block = 6450597802325118133;
                                continue ;
                            }
                            's_210:
                                loop  {
                                    tmp = (*parent).time_entry.rbe_left;
                                    (*parent).time_entry.rbe_left =
                                        (*tmp).time_entry.rbe_right;
                                    if !(*parent).time_entry.rbe_left.is_null()
                                       {
                                        (*(*tmp).time_entry.rbe_right).time_entry.rbe_parent
                                            = parent
                                    }
                                    while 0 != 0i32 { }
                                    (*tmp).time_entry.rbe_parent =
                                        (*parent).time_entry.rbe_parent;
                                    if !(*tmp).time_entry.rbe_parent.is_null()
                                       {
                                        if parent ==
                                               (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                                           {
                                            (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                                                = tmp
                                        } else {
                                            (*(*parent).time_entry.rbe_parent).time_entry.rbe_right
                                                = tmp
                                        }
                                    } else { (*head).rbh_root = tmp }
                                    (*tmp).time_entry.rbe_right = parent;
                                    (*parent).time_entry.rbe_parent = tmp;
                                    while 0 != 0i32 { }
                                    if !(*tmp).time_entry.rbe_parent.is_null()
                                       {
                                        current_block = 16738040538446813684;
                                    } else {
                                        current_block = 17784502470059252271;
                                    }
                                    loop  {
                                        match current_block {
                                            17784502470059252271 => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_210 ; }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        16738040538446813684;
                                                } else {
                                                    current_block =
                                                        17784502470059252271;
                                                }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).time_entry.rbe_left;
                            current_block = 7746103178988627676;
                        }
                    }
                }
            }
        }
    match current_block {
        13826291924415791078 => {
            let mut oright: *mut paste_buffer = 0 as *mut paste_buffer;
            oright = (*tmp).time_entry.rbe_right;
            if !oright.is_null() { (*oright).time_entry.rbe_color = 0i32 }
            (*tmp).time_entry.rbe_color = 1i32;
            's_276:
                loop  {
                    oright = (*tmp).time_entry.rbe_right;
                    (*tmp).time_entry.rbe_right =
                        (*oright).time_entry.rbe_left;
                    if !(*tmp).time_entry.rbe_right.is_null() {
                        (*(*oright).time_entry.rbe_left).time_entry.rbe_parent
                            = tmp
                    }
                    while 0 != 0i32 { }
                    (*oright).time_entry.rbe_parent =
                        (*tmp).time_entry.rbe_parent;
                    if !(*oright).time_entry.rbe_parent.is_null() {
                        if tmp ==
                               (*(*tmp).time_entry.rbe_parent).time_entry.rbe_left
                           {
                            (*(*tmp).time_entry.rbe_parent).time_entry.rbe_left
                                = oright
                        } else {
                            (*(*tmp).time_entry.rbe_parent).time_entry.rbe_right
                                = oright
                        }
                    } else { (*head).rbh_root = oright }
                    (*oright).time_entry.rbe_left = tmp;
                    (*tmp).time_entry.rbe_parent = oright;
                    while 0 != 0i32 { }
                    if !(*oright).time_entry.rbe_parent.is_null() {
                        current_block = 3392087639489470149;
                    } else { current_block = 1854459640724737493; }
                    loop  {
                        match current_block {
                            1854459640724737493 => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_276 ; }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    current_block = 3392087639489470149;
                                } else {
                                    current_block = 1854459640724737493;
                                }
                            }
                        }
                    }
                }
            tmp = (*parent).time_entry.rbe_left;
            current_block = 5892776923941496671;
        }
        15976848397966268834 => {
            let mut oleft: *mut paste_buffer = 0 as *mut paste_buffer;
            oleft = (*tmp).time_entry.rbe_left;
            if !oleft.is_null() { (*oleft).time_entry.rbe_color = 0i32 }
            (*tmp).time_entry.rbe_color = 1i32;
            's_96:
                loop  {
                    oleft = (*tmp).time_entry.rbe_left;
                    (*tmp).time_entry.rbe_left =
                        (*oleft).time_entry.rbe_right;
                    if !(*tmp).time_entry.rbe_left.is_null() {
                        (*(*oleft).time_entry.rbe_right).time_entry.rbe_parent
                            = tmp
                    }
                    while 0 != 0i32 { }
                    (*oleft).time_entry.rbe_parent =
                        (*tmp).time_entry.rbe_parent;
                    if !(*oleft).time_entry.rbe_parent.is_null() {
                        if tmp ==
                               (*(*tmp).time_entry.rbe_parent).time_entry.rbe_left
                           {
                            (*(*tmp).time_entry.rbe_parent).time_entry.rbe_left
                                = oleft
                        } else {
                            (*(*tmp).time_entry.rbe_parent).time_entry.rbe_right
                                = oleft
                        }
                    } else { (*head).rbh_root = oleft }
                    (*oleft).time_entry.rbe_right = tmp;
                    (*tmp).time_entry.rbe_parent = oleft;
                    while 0 != 0i32 { }
                    if !(*oleft).time_entry.rbe_parent.is_null() {
                        current_block = 2232869372362427478;
                    } else { current_block = 15904375183555213903; }
                    loop  {
                        match current_block {
                            2232869372362427478 => {
                                if 0 != 0i32 {
                                    current_block = 2232869372362427478;
                                } else {
                                    current_block = 15904375183555213903;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_96 ; }
                            }
                        }
                    }
                }
            tmp = (*parent).time_entry.rbe_right;
            current_block = 7149356873433890176;
        }
        _ => { }
    }
    match current_block {
        7149356873433890176 => {
            (*tmp).time_entry.rbe_color = (*parent).time_entry.rbe_color;
            (*parent).time_entry.rbe_color = 0i32;
            if !(*tmp).time_entry.rbe_right.is_null() {
                (*(*tmp).time_entry.rbe_right).time_entry.rbe_color = 0i32
            }
            's_148:
                loop  {
                    tmp = (*parent).time_entry.rbe_right;
                    (*parent).time_entry.rbe_right =
                        (*tmp).time_entry.rbe_left;
                    if !(*parent).time_entry.rbe_right.is_null() {
                        (*(*tmp).time_entry.rbe_left).time_entry.rbe_parent =
                            parent
                    }
                    while 0 != 0i32 { }
                    (*tmp).time_entry.rbe_parent =
                        (*parent).time_entry.rbe_parent;
                    if !(*tmp).time_entry.rbe_parent.is_null() {
                        if parent ==
                               (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                           {
                            (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                                = tmp
                        } else {
                            (*(*parent).time_entry.rbe_parent).time_entry.rbe_right
                                = tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).time_entry.rbe_left = parent;
                    (*parent).time_entry.rbe_parent = tmp;
                    while 0 != 0i32 { }
                    if !(*tmp).time_entry.rbe_parent.is_null() {
                        current_block = 6450636197030046351;
                    } else { current_block = 16924917904204750491; }
                    loop  {
                        match current_block {
                            6450636197030046351 => {
                                if 0 != 0i32 {
                                    current_block = 6450636197030046351;
                                } else {
                                    current_block = 16924917904204750491;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_148 ; }
                            }
                        }
                    }
                }
            elm = (*head).rbh_root
        }
        5892776923941496671 => {
            (*tmp).time_entry.rbe_color = (*parent).time_entry.rbe_color;
            (*parent).time_entry.rbe_color = 0i32;
            if !(*tmp).time_entry.rbe_left.is_null() {
                (*(*tmp).time_entry.rbe_left).time_entry.rbe_color = 0i32
            }
            's_328:
                loop  {
                    tmp = (*parent).time_entry.rbe_left;
                    (*parent).time_entry.rbe_left =
                        (*tmp).time_entry.rbe_right;
                    if !(*parent).time_entry.rbe_left.is_null() {
                        (*(*tmp).time_entry.rbe_right).time_entry.rbe_parent =
                            parent
                    }
                    while 0 != 0i32 { }
                    (*tmp).time_entry.rbe_parent =
                        (*parent).time_entry.rbe_parent;
                    if !(*tmp).time_entry.rbe_parent.is_null() {
                        if parent ==
                               (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                           {
                            (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                                = tmp
                        } else {
                            (*(*parent).time_entry.rbe_parent).time_entry.rbe_right
                                = tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).time_entry.rbe_right = parent;
                    (*parent).time_entry.rbe_parent = tmp;
                    while 0 != 0i32 { }
                    if !(*tmp).time_entry.rbe_parent.is_null() {
                        current_block = 13910774313357589740;
                    } else { current_block = 13707613154239713890; }
                    loop  {
                        match current_block {
                            13910774313357589740 => {
                                if 0 != 0i32 {
                                    current_block = 13910774313357589740;
                                } else {
                                    current_block = 13707613154239713890;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_328 ; }
                            }
                        }
                    }
                }
            elm = (*head).rbh_root
        }
        _ => { }
    }
    if !elm.is_null() { (*elm).time_entry.rbe_color = 0i32 };
}
unsafe extern "C" fn paste_name_tree_RB_REMOVE(mut head: *mut paste_name_tree,
                                               mut elm: *mut paste_buffer)
 -> *mut paste_buffer {
    let mut current_block: u64;
    let mut child: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut old: *mut paste_buffer = elm;
    let mut color: libc::c_int = 0;
    if (*elm).name_entry.rbe_left ==
           0 as *mut libc::c_void as *mut paste_buffer {
        child = (*elm).name_entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).name_entry.rbe_right ==
                  0 as *mut libc::c_void as *mut paste_buffer {
        child = (*elm).name_entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut paste_buffer = 0 as *mut paste_buffer;
        elm = (*elm).name_entry.rbe_right;
        loop  {
            left = (*elm).name_entry.rbe_left;
            if left.is_null() { break ; }
            elm = left
        }
        child = (*elm).name_entry.rbe_right;
        parent = (*elm).name_entry.rbe_parent;
        color = (*elm).name_entry.rbe_color;
        if !child.is_null() { (*child).name_entry.rbe_parent = parent }
        if !parent.is_null() {
            if (*parent).name_entry.rbe_left == elm {
                (*parent).name_entry.rbe_left = child
            } else { (*parent).name_entry.rbe_right = child }
            while 0 != 0i32 { }
        } else { (*head).rbh_root = child }
        if (*elm).name_entry.rbe_parent == old { parent = elm }
        (*elm).name_entry = (*old).name_entry;
        if !(*old).name_entry.rbe_parent.is_null() {
            if (*(*old).name_entry.rbe_parent).name_entry.rbe_left == old {
                (*(*old).name_entry.rbe_parent).name_entry.rbe_left = elm
            } else {
                (*(*old).name_entry.rbe_parent).name_entry.rbe_right = elm
            }
            while 0 != 0i32 { }
        } else { (*head).rbh_root = elm }
        (*(*old).name_entry.rbe_left).name_entry.rbe_parent = elm;
        if !(*old).name_entry.rbe_right.is_null() {
            (*(*old).name_entry.rbe_right).name_entry.rbe_parent = elm
        }
        if !parent.is_null() {
            left = parent;
            loop  {
                if 0 != 0i32 { continue ; }
                left = (*left).name_entry.rbe_parent;
                if left.is_null() { break ; }
            }
            current_block = 11508873959027919857;
        } else { current_block = 11508873959027919857; }
    }
    match current_block {
        9386390421034826751 => {
            parent = (*elm).name_entry.rbe_parent;
            color = (*elm).name_entry.rbe_color;
            if !child.is_null() { (*child).name_entry.rbe_parent = parent }
            if !parent.is_null() {
                if (*parent).name_entry.rbe_left == elm {
                    (*parent).name_entry.rbe_left = child
                } else { (*parent).name_entry.rbe_right = child }
                while 0 != 0i32 { }
            } else { (*head).rbh_root = child }
        }
        _ => { }
    }
    if color == 0i32 { paste_name_tree_RB_REMOVE_COLOR(head, parent, child); }
    return old;
}
unsafe extern "C" fn paste_name_tree_RB_REMOVE_COLOR(mut head:
                                                         *mut paste_name_tree,
                                                     mut parent:
                                                         *mut paste_buffer,
                                                     mut elm:
                                                         *mut paste_buffer)
 -> () {
    let mut current_block: u64;
    let mut tmp: *mut paste_buffer = 0 as *mut paste_buffer;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut paste_buffer ||
                      (*elm).name_entry.rbe_color == 0i32) &&
                     elm != (*head).rbh_root) {
                current_block = 11174649648027449784;
                break ;
            }
            if (*parent).name_entry.rbe_left == elm {
                tmp = (*parent).name_entry.rbe_right;
                if (*tmp).name_entry.rbe_color == 1i32 {
                    current_block = 17179679302217393232;
                } else { current_block = 14155750587950065367; }
                loop  {
                    match current_block {
                        14155750587950065367 => {
                            if ((*tmp).name_entry.rbe_left ==
                                    0 as *mut libc::c_void as
                                        *mut paste_buffer ||
                                    (*(*tmp).name_entry.rbe_left).name_entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).name_entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut paste_buffer ||
                                        (*(*tmp).name_entry.rbe_right).name_entry.rbe_color
                                            == 0i32) {
                                (*tmp).name_entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).name_entry.rbe_parent;
                                break ;
                            } else if (*tmp).name_entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut paste_buffer ||
                                          (*(*tmp).name_entry.rbe_right).name_entry.rbe_color
                                              == 0i32 {
                                current_block = 15976848397966268834;
                                break 's_4 ;
                            } else {
                                current_block = 7149356873433890176;
                                break 's_4 ;
                            }
                        }
                        _ => {
                            (*tmp).name_entry.rbe_color = 0i32;
                            (*parent).name_entry.rbe_color = 1i32;
                            if 0 != 0i32 {
                                current_block = 17179679302217393232;
                                continue ;
                            }
                            's_30:
                                loop  {
                                    tmp = (*parent).name_entry.rbe_right;
                                    (*parent).name_entry.rbe_right =
                                        (*tmp).name_entry.rbe_left;
                                    if !(*parent).name_entry.rbe_right.is_null()
                                       {
                                        (*(*tmp).name_entry.rbe_left).name_entry.rbe_parent
                                            = parent
                                    }
                                    while 0 != 0i32 { }
                                    (*tmp).name_entry.rbe_parent =
                                        (*parent).name_entry.rbe_parent;
                                    if !(*tmp).name_entry.rbe_parent.is_null()
                                       {
                                        if parent ==
                                               (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                                           {
                                            (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                                                = tmp
                                        } else {
                                            (*(*parent).name_entry.rbe_parent).name_entry.rbe_right
                                                = tmp
                                        }
                                    } else { (*head).rbh_root = tmp }
                                    (*tmp).name_entry.rbe_left = parent;
                                    (*parent).name_entry.rbe_parent = tmp;
                                    while 0 != 0i32 { }
                                    if !(*tmp).name_entry.rbe_parent.is_null()
                                       {
                                        current_block = 11050875288958768710;
                                    } else {
                                        current_block = 15240798224410183470;
                                    }
                                    loop  {
                                        match current_block {
                                            15240798224410183470 => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_30 ; }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        11050875288958768710;
                                                } else {
                                                    current_block =
                                                        15240798224410183470;
                                                }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).name_entry.rbe_right;
                            current_block = 14155750587950065367;
                        }
                    }
                }
            } else {
                tmp = (*parent).name_entry.rbe_left;
                if (*tmp).name_entry.rbe_color == 1i32 {
                    current_block = 6450597802325118133;
                } else { current_block = 7746103178988627676; }
                loop  {
                    match current_block {
                        7746103178988627676 => {
                            if ((*tmp).name_entry.rbe_left ==
                                    0 as *mut libc::c_void as
                                        *mut paste_buffer ||
                                    (*(*tmp).name_entry.rbe_left).name_entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).name_entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut paste_buffer ||
                                        (*(*tmp).name_entry.rbe_right).name_entry.rbe_color
                                            == 0i32) {
                                (*tmp).name_entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).name_entry.rbe_parent;
                                break ;
                            } else if (*tmp).name_entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut paste_buffer ||
                                          (*(*tmp).name_entry.rbe_left).name_entry.rbe_color
                                              == 0i32 {
                                current_block = 13826291924415791078;
                                break 's_4 ;
                            } else {
                                current_block = 5892776923941496671;
                                break 's_4 ;
                            }
                        }
                        _ => {
                            (*tmp).name_entry.rbe_color = 0i32;
                            (*parent).name_entry.rbe_color = 1i32;
                            if 0 != 0i32 {
                                current_block = 6450597802325118133;
                                continue ;
                            }
                            's_210:
                                loop  {
                                    tmp = (*parent).name_entry.rbe_left;
                                    (*parent).name_entry.rbe_left =
                                        (*tmp).name_entry.rbe_right;
                                    if !(*parent).name_entry.rbe_left.is_null()
                                       {
                                        (*(*tmp).name_entry.rbe_right).name_entry.rbe_parent
                                            = parent
                                    }
                                    while 0 != 0i32 { }
                                    (*tmp).name_entry.rbe_parent =
                                        (*parent).name_entry.rbe_parent;
                                    if !(*tmp).name_entry.rbe_parent.is_null()
                                       {
                                        if parent ==
                                               (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                                           {
                                            (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                                                = tmp
                                        } else {
                                            (*(*parent).name_entry.rbe_parent).name_entry.rbe_right
                                                = tmp
                                        }
                                    } else { (*head).rbh_root = tmp }
                                    (*tmp).name_entry.rbe_right = parent;
                                    (*parent).name_entry.rbe_parent = tmp;
                                    while 0 != 0i32 { }
                                    if !(*tmp).name_entry.rbe_parent.is_null()
                                       {
                                        current_block = 16738040538446813684;
                                    } else {
                                        current_block = 17784502470059252271;
                                    }
                                    loop  {
                                        match current_block {
                                            17784502470059252271 => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_210 ; }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        16738040538446813684;
                                                } else {
                                                    current_block =
                                                        17784502470059252271;
                                                }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).name_entry.rbe_left;
                            current_block = 7746103178988627676;
                        }
                    }
                }
            }
        }
    match current_block {
        15976848397966268834 => {
            let mut oleft: *mut paste_buffer = 0 as *mut paste_buffer;
            oleft = (*tmp).name_entry.rbe_left;
            if !oleft.is_null() { (*oleft).name_entry.rbe_color = 0i32 }
            (*tmp).name_entry.rbe_color = 1i32;
            's_96:
                loop  {
                    oleft = (*tmp).name_entry.rbe_left;
                    (*tmp).name_entry.rbe_left =
                        (*oleft).name_entry.rbe_right;
                    if !(*tmp).name_entry.rbe_left.is_null() {
                        (*(*oleft).name_entry.rbe_right).name_entry.rbe_parent
                            = tmp
                    }
                    while 0 != 0i32 { }
                    (*oleft).name_entry.rbe_parent =
                        (*tmp).name_entry.rbe_parent;
                    if !(*oleft).name_entry.rbe_parent.is_null() {
                        if tmp ==
                               (*(*tmp).name_entry.rbe_parent).name_entry.rbe_left
                           {
                            (*(*tmp).name_entry.rbe_parent).name_entry.rbe_left
                                = oleft
                        } else {
                            (*(*tmp).name_entry.rbe_parent).name_entry.rbe_right
                                = oleft
                        }
                    } else { (*head).rbh_root = oleft }
                    (*oleft).name_entry.rbe_right = tmp;
                    (*tmp).name_entry.rbe_parent = oleft;
                    while 0 != 0i32 { }
                    if !(*oleft).name_entry.rbe_parent.is_null() {
                        current_block = 2232869372362427478;
                    } else { current_block = 15904375183555213903; }
                    loop  {
                        match current_block {
                            15904375183555213903 => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_96 ; }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    current_block = 2232869372362427478;
                                } else {
                                    current_block = 15904375183555213903;
                                }
                            }
                        }
                    }
                }
            tmp = (*parent).name_entry.rbe_right;
            current_block = 7149356873433890176;
        }
        13826291924415791078 => {
            let mut oright: *mut paste_buffer = 0 as *mut paste_buffer;
            oright = (*tmp).name_entry.rbe_right;
            if !oright.is_null() { (*oright).name_entry.rbe_color = 0i32 }
            (*tmp).name_entry.rbe_color = 1i32;
            's_276:
                loop  {
                    oright = (*tmp).name_entry.rbe_right;
                    (*tmp).name_entry.rbe_right =
                        (*oright).name_entry.rbe_left;
                    if !(*tmp).name_entry.rbe_right.is_null() {
                        (*(*oright).name_entry.rbe_left).name_entry.rbe_parent
                            = tmp
                    }
                    while 0 != 0i32 { }
                    (*oright).name_entry.rbe_parent =
                        (*tmp).name_entry.rbe_parent;
                    if !(*oright).name_entry.rbe_parent.is_null() {
                        if tmp ==
                               (*(*tmp).name_entry.rbe_parent).name_entry.rbe_left
                           {
                            (*(*tmp).name_entry.rbe_parent).name_entry.rbe_left
                                = oright
                        } else {
                            (*(*tmp).name_entry.rbe_parent).name_entry.rbe_right
                                = oright
                        }
                    } else { (*head).rbh_root = oright }
                    (*oright).name_entry.rbe_left = tmp;
                    (*tmp).name_entry.rbe_parent = oright;
                    while 0 != 0i32 { }
                    if !(*oright).name_entry.rbe_parent.is_null() {
                        current_block = 3392087639489470149;
                    } else { current_block = 1854459640724737493; }
                    loop  {
                        match current_block {
                            3392087639489470149 => {
                                if 0 != 0i32 {
                                    current_block = 3392087639489470149;
                                } else {
                                    current_block = 1854459640724737493;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_276 ; }
                            }
                        }
                    }
                }
            tmp = (*parent).name_entry.rbe_left;
            current_block = 5892776923941496671;
        }
        _ => { }
    }
    match current_block {
        7149356873433890176 => {
            (*tmp).name_entry.rbe_color = (*parent).name_entry.rbe_color;
            (*parent).name_entry.rbe_color = 0i32;
            if !(*tmp).name_entry.rbe_right.is_null() {
                (*(*tmp).name_entry.rbe_right).name_entry.rbe_color = 0i32
            }
            's_148:
                loop  {
                    tmp = (*parent).name_entry.rbe_right;
                    (*parent).name_entry.rbe_right =
                        (*tmp).name_entry.rbe_left;
                    if !(*parent).name_entry.rbe_right.is_null() {
                        (*(*tmp).name_entry.rbe_left).name_entry.rbe_parent =
                            parent
                    }
                    while 0 != 0i32 { }
                    (*tmp).name_entry.rbe_parent =
                        (*parent).name_entry.rbe_parent;
                    if !(*tmp).name_entry.rbe_parent.is_null() {
                        if parent ==
                               (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                           {
                            (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                                = tmp
                        } else {
                            (*(*parent).name_entry.rbe_parent).name_entry.rbe_right
                                = tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).name_entry.rbe_left = parent;
                    (*parent).name_entry.rbe_parent = tmp;
                    while 0 != 0i32 { }
                    if !(*tmp).name_entry.rbe_parent.is_null() {
                        current_block = 6450636197030046351;
                    } else { current_block = 16924917904204750491; }
                    loop  {
                        match current_block {
                            6450636197030046351 => {
                                if 0 != 0i32 {
                                    current_block = 6450636197030046351;
                                } else {
                                    current_block = 16924917904204750491;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_148 ; }
                            }
                        }
                    }
                }
            elm = (*head).rbh_root
        }
        5892776923941496671 => {
            (*tmp).name_entry.rbe_color = (*parent).name_entry.rbe_color;
            (*parent).name_entry.rbe_color = 0i32;
            if !(*tmp).name_entry.rbe_left.is_null() {
                (*(*tmp).name_entry.rbe_left).name_entry.rbe_color = 0i32
            }
            's_328:
                loop  {
                    tmp = (*parent).name_entry.rbe_left;
                    (*parent).name_entry.rbe_left =
                        (*tmp).name_entry.rbe_right;
                    if !(*parent).name_entry.rbe_left.is_null() {
                        (*(*tmp).name_entry.rbe_right).name_entry.rbe_parent =
                            parent
                    }
                    while 0 != 0i32 { }
                    (*tmp).name_entry.rbe_parent =
                        (*parent).name_entry.rbe_parent;
                    if !(*tmp).name_entry.rbe_parent.is_null() {
                        if parent ==
                               (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                           {
                            (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                                = tmp
                        } else {
                            (*(*parent).name_entry.rbe_parent).name_entry.rbe_right
                                = tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).name_entry.rbe_right = parent;
                    (*parent).name_entry.rbe_parent = tmp;
                    while 0 != 0i32 { }
                    if !(*tmp).name_entry.rbe_parent.is_null() {
                        current_block = 13910774313357589740;
                    } else { current_block = 13707613154239713890; }
                    loop  {
                        match current_block {
                            13910774313357589740 => {
                                if 0 != 0i32 {
                                    current_block = 13910774313357589740;
                                } else {
                                    current_block = 13707613154239713890;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_328 ; }
                            }
                        }
                    }
                }
            elm = (*head).rbh_root
        }
        _ => { }
    }
    if !elm.is_null() { (*elm).name_entry.rbe_color = 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn paste_add(mut data: *mut libc::c_char,
                                   mut size: size_t) -> () {
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut pb1: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut limit: u_int = 0;
    if size == 0i32 as libc::c_ulong {
        free(data as *mut libc::c_void);
        return
    } else {
        limit =
            options_get_number(global_options,
                               b"buffer-limit\x00" as *const u8 as
                                   *const libc::c_char) as u_int;
        pb =
            paste_time_tree_RB_MINMAX(&mut paste_by_time as
                                          *mut paste_time_tree, 1i32);
        while pb != 0 as *mut libc::c_void as *mut paste_buffer &&
                  { pb1 = paste_time_tree_RB_PREV(pb); 0 != 1i32 } {
            if paste_num_automatic < limit { break ; }
            if 0 != (*pb).automatic { paste_free(pb); }
            pb = pb1
        }
        pb =
            xmalloc(::std::mem::size_of::<paste_buffer>() as libc::c_ulong) as
                *mut paste_buffer;
        (*pb).name = 0 as *mut libc::c_char;
        loop  {
            free((*pb).name as *mut libc::c_void);
            xasprintf(&mut (*pb).name as *mut *mut libc::c_char,
                      b"buffer%04u\x00" as *const u8 as *const libc::c_char,
                      paste_next_index);
            paste_next_index = paste_next_index.wrapping_add(1);
            if !(paste_get_name((*pb).name) !=
                     0 as *mut libc::c_void as *mut paste_buffer) {
                break ;
            }
        }
        (*pb).data = data;
        (*pb).size = size;
        (*pb).automatic = 1i32;
        paste_num_automatic = paste_num_automatic.wrapping_add(1);
        (*pb).created = time(0 as *mut time_t);
        let fresh0 = paste_next_order;
        paste_next_order = paste_next_order.wrapping_add(1);
        (*pb).order = fresh0;
        paste_name_tree_RB_INSERT(&mut paste_by_name as *mut paste_name_tree,
                                  pb);
        paste_time_tree_RB_INSERT(&mut paste_by_time as *mut paste_time_tree,
                                  pb);
        return;
    };
}
unsafe extern "C" fn paste_time_tree_RB_INSERT(mut head: *mut paste_time_tree,
                                               mut elm: *mut paste_buffer)
 -> *mut paste_buffer {
    let mut tmp: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = paste_cmp_times(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).time_entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).time_entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).time_entry.rbe_parent = parent;
        (*elm).time_entry.rbe_right = 0 as *mut paste_buffer;
        (*elm).time_entry.rbe_left = (*elm).time_entry.rbe_right;
        (*elm).time_entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut paste_buffer {
        if comp < 0i32 {
            (*parent).time_entry.rbe_left = elm
        } else { (*parent).time_entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    paste_time_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut paste_buffer;
}
unsafe extern "C" fn paste_time_tree_RB_INSERT_COLOR(mut head:
                                                         *mut paste_time_tree,
                                                     mut elm:
                                                         *mut paste_buffer)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut gparent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut tmp: *mut paste_buffer = 0 as *mut paste_buffer;
    loop  {
        parent = (*elm).time_entry.rbe_parent;
        if !(!parent.is_null() && (*parent).time_entry.rbe_color == 1i32) {
            break ;
        }
        gparent = (*parent).time_entry.rbe_parent;
        if parent == (*gparent).time_entry.rbe_left {
            tmp = (*gparent).time_entry.rbe_right;
            if !tmp.is_null() && (*tmp).time_entry.rbe_color == 1i32 {
                (*tmp).time_entry.rbe_color = 0i32;
                loop  {
                    (*parent).time_entry.rbe_color = 0i32;
                    (*gparent).time_entry.rbe_color = 1i32;
                    if !(0 != 0i32) { break ; }
                }
                elm = gparent
            } else {
                if (*parent).time_entry.rbe_right == elm {
                    current_block = 7351195479953500246;
                } else { current_block = 4956146061682418353; }
                's_87:
                    loop  {
                        match current_block {
                            7351195479953500246 => {
                                tmp = (*parent).time_entry.rbe_right;
                                (*parent).time_entry.rbe_right =
                                    (*tmp).time_entry.rbe_left;
                                if !(*parent).time_entry.rbe_right.is_null() {
                                    (*(*tmp).time_entry.rbe_left).time_entry.rbe_parent
                                        = parent
                                }
                                while 0 != 0i32 { }
                                (*tmp).time_entry.rbe_parent =
                                    (*parent).time_entry.rbe_parent;
                                if !(*tmp).time_entry.rbe_parent.is_null() {
                                    if parent ==
                                           (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                                       {
                                        (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                                            = tmp
                                    } else {
                                        (*(*parent).time_entry.rbe_parent).time_entry.rbe_right
                                            = tmp
                                    }
                                } else { (*head).rbh_root = tmp }
                                (*tmp).time_entry.rbe_left = parent;
                                (*parent).time_entry.rbe_parent = tmp;
                                while 0 != 0i32 { }
                                if !(*tmp).time_entry.rbe_parent.is_null() {
                                    current_block = 10048703153582371463;
                                } else {
                                    current_block = 10879442775620481940;
                                }
                                loop  {
                                    match current_block {
                                        10879442775620481940 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    7351195479953500246;
                                                continue 's_87 ;
                                            } else { break ; }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    10048703153582371463;
                                            } else {
                                                current_block =
                                                    10879442775620481940;
                                            }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4956146061682418353;
                            }
                            _ => {
                                (*parent).time_entry.rbe_color = 0i32;
                                (*gparent).time_entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4956146061682418353;
                                } else { break ; }
                            }
                        }
                    }
                's_95:
                    loop  {
                        tmp = (*gparent).time_entry.rbe_left;
                        (*gparent).time_entry.rbe_left =
                            (*tmp).time_entry.rbe_right;
                        if !(*gparent).time_entry.rbe_left.is_null() {
                            (*(*tmp).time_entry.rbe_right).time_entry.rbe_parent
                                = gparent
                        }
                        while 0 != 0i32 { }
                        (*tmp).time_entry.rbe_parent =
                            (*gparent).time_entry.rbe_parent;
                        if !(*tmp).time_entry.rbe_parent.is_null() {
                            if gparent ==
                                   (*(*gparent).time_entry.rbe_parent).time_entry.rbe_left
                               {
                                (*(*gparent).time_entry.rbe_parent).time_entry.rbe_left
                                    = tmp
                            } else {
                                (*(*gparent).time_entry.rbe_parent).time_entry.rbe_right
                                    = tmp
                            }
                        } else { (*head).rbh_root = tmp }
                        (*tmp).time_entry.rbe_right = gparent;
                        (*gparent).time_entry.rbe_parent = tmp;
                        while 0 != 0i32 { }
                        if !(*tmp).time_entry.rbe_parent.is_null() {
                            current_block = 6669252993407410313;
                        } else { current_block = 5948590327928692120; }
                        loop  {
                            match current_block {
                                5948590327928692120 => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_95 ; }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        current_block = 6669252993407410313;
                                    } else {
                                        current_block = 5948590327928692120;
                                    }
                                }
                            }
                        }
                    }
            }
        } else {
            tmp = (*gparent).time_entry.rbe_left;
            if !tmp.is_null() && (*tmp).time_entry.rbe_color == 1i32 {
                (*tmp).time_entry.rbe_color = 0i32;
                loop  {
                    (*parent).time_entry.rbe_color = 0i32;
                    (*gparent).time_entry.rbe_color = 1i32;
                    if !(0 != 0i32) { break ; }
                }
                elm = gparent
            } else {
                if (*parent).time_entry.rbe_left == elm {
                    current_block = 652864300344834934;
                } else { current_block = 4567019141635105728; }
                's_162:
                    loop  {
                        match current_block {
                            4567019141635105728 => {
                                (*parent).time_entry.rbe_color = 0i32;
                                (*gparent).time_entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4567019141635105728;
                                } else { break ; }
                            }
                            _ => {
                                tmp = (*parent).time_entry.rbe_left;
                                (*parent).time_entry.rbe_left =
                                    (*tmp).time_entry.rbe_right;
                                if !(*parent).time_entry.rbe_left.is_null() {
                                    (*(*tmp).time_entry.rbe_right).time_entry.rbe_parent
                                        = parent
                                }
                                while 0 != 0i32 { }
                                (*tmp).time_entry.rbe_parent =
                                    (*parent).time_entry.rbe_parent;
                                if !(*tmp).time_entry.rbe_parent.is_null() {
                                    if parent ==
                                           (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                                       {
                                        (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                                            = tmp
                                    } else {
                                        (*(*parent).time_entry.rbe_parent).time_entry.rbe_right
                                            = tmp
                                    }
                                } else { (*head).rbh_root = tmp }
                                (*tmp).time_entry.rbe_right = parent;
                                (*parent).time_entry.rbe_parent = tmp;
                                while 0 != 0i32 { }
                                if !(*tmp).time_entry.rbe_parent.is_null() {
                                    current_block = 980989089337379490;
                                } else {
                                    current_block = 5494826135382683477;
                                }
                                loop  {
                                    match current_block {
                                        5494826135382683477 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    652864300344834934;
                                                continue 's_162 ;
                                            } else { break ; }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    980989089337379490;
                                            } else {
                                                current_block =
                                                    5494826135382683477;
                                            }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4567019141635105728;
                            }
                        }
                    }
                's_219:
                    loop  {
                        tmp = (*gparent).time_entry.rbe_right;
                        (*gparent).time_entry.rbe_right =
                            (*tmp).time_entry.rbe_left;
                        if !(*gparent).time_entry.rbe_right.is_null() {
                            (*(*tmp).time_entry.rbe_left).time_entry.rbe_parent
                                = gparent
                        }
                        while 0 != 0i32 { }
                        (*tmp).time_entry.rbe_parent =
                            (*gparent).time_entry.rbe_parent;
                        if !(*tmp).time_entry.rbe_parent.is_null() {
                            if gparent ==
                                   (*(*gparent).time_entry.rbe_parent).time_entry.rbe_left
                               {
                                (*(*gparent).time_entry.rbe_parent).time_entry.rbe_left
                                    = tmp
                            } else {
                                (*(*gparent).time_entry.rbe_parent).time_entry.rbe_right
                                    = tmp
                            }
                        } else { (*head).rbh_root = tmp }
                        (*tmp).time_entry.rbe_left = gparent;
                        (*gparent).time_entry.rbe_parent = tmp;
                        while 0 != 0i32 { }
                        if !(*tmp).time_entry.rbe_parent.is_null() {
                            current_block = 11793792312832361944;
                        } else { current_block = 2543120759711851213; }
                        loop  {
                            match current_block {
                                2543120759711851213 => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_219 ; }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        current_block = 11793792312832361944;
                                    } else {
                                        current_block = 2543120759711851213;
                                    }
                                }
                            }
                        }
                    }
            }
        }
    }
    (*(*head).rbh_root).time_entry.rbe_color = 0i32;
}
unsafe extern "C" fn paste_cmp_times(mut a: *const paste_buffer,
                                     mut b: *const paste_buffer)
 -> libc::c_int {
    if (*a).order > (*b).order {
        return 1i32.wrapping_neg()
    } else if (*a).order < (*b).order { return 1i32 } else { return 0i32 };
}
unsafe extern "C" fn paste_name_tree_RB_INSERT(mut head: *mut paste_name_tree,
                                               mut elm: *mut paste_buffer)
 -> *mut paste_buffer {
    let mut tmp: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = paste_cmp_names(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).name_entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).name_entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).name_entry.rbe_parent = parent;
        (*elm).name_entry.rbe_right = 0 as *mut paste_buffer;
        (*elm).name_entry.rbe_left = (*elm).name_entry.rbe_right;
        (*elm).name_entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut paste_buffer {
        if comp < 0i32 {
            (*parent).name_entry.rbe_left = elm
        } else { (*parent).name_entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    paste_name_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut paste_buffer;
}
unsafe extern "C" fn paste_name_tree_RB_INSERT_COLOR(mut head:
                                                         *mut paste_name_tree,
                                                     mut elm:
                                                         *mut paste_buffer)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut gparent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut tmp: *mut paste_buffer = 0 as *mut paste_buffer;
    loop  {
        parent = (*elm).name_entry.rbe_parent;
        if !(!parent.is_null() && (*parent).name_entry.rbe_color == 1i32) {
            break ;
        }
        gparent = (*parent).name_entry.rbe_parent;
        if parent == (*gparent).name_entry.rbe_left {
            tmp = (*gparent).name_entry.rbe_right;
            if !tmp.is_null() && (*tmp).name_entry.rbe_color == 1i32 {
                (*tmp).name_entry.rbe_color = 0i32;
                loop  {
                    (*parent).name_entry.rbe_color = 0i32;
                    (*gparent).name_entry.rbe_color = 1i32;
                    if !(0 != 0i32) { break ; }
                }
                elm = gparent
            } else {
                if (*parent).name_entry.rbe_right == elm {
                    current_block = 7351195479953500246;
                } else { current_block = 4956146061682418353; }
                's_38:
                    loop  {
                        match current_block {
                            7351195479953500246 => {
                                tmp = (*parent).name_entry.rbe_right;
                                (*parent).name_entry.rbe_right =
                                    (*tmp).name_entry.rbe_left;
                                if !(*parent).name_entry.rbe_right.is_null() {
                                    (*(*tmp).name_entry.rbe_left).name_entry.rbe_parent
                                        = parent
                                }
                                while 0 != 0i32 { }
                                (*tmp).name_entry.rbe_parent =
                                    (*parent).name_entry.rbe_parent;
                                if !(*tmp).name_entry.rbe_parent.is_null() {
                                    if parent ==
                                           (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                                       {
                                        (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                                            = tmp
                                    } else {
                                        (*(*parent).name_entry.rbe_parent).name_entry.rbe_right
                                            = tmp
                                    }
                                } else { (*head).rbh_root = tmp }
                                (*tmp).name_entry.rbe_left = parent;
                                (*parent).name_entry.rbe_parent = tmp;
                                while 0 != 0i32 { }
                                if !(*tmp).name_entry.rbe_parent.is_null() {
                                    current_block = 10048703153582371463;
                                } else {
                                    current_block = 10879442775620481940;
                                }
                                loop  {
                                    match current_block {
                                        10879442775620481940 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    7351195479953500246;
                                                continue 's_38 ;
                                            } else { break ; }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    10048703153582371463;
                                            } else {
                                                current_block =
                                                    10879442775620481940;
                                            }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4956146061682418353;
                            }
                            _ => {
                                (*parent).name_entry.rbe_color = 0i32;
                                (*gparent).name_entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4956146061682418353;
                                } else { break ; }
                            }
                        }
                    }
                's_95:
                    loop  {
                        tmp = (*gparent).name_entry.rbe_left;
                        (*gparent).name_entry.rbe_left =
                            (*tmp).name_entry.rbe_right;
                        if !(*gparent).name_entry.rbe_left.is_null() {
                            (*(*tmp).name_entry.rbe_right).name_entry.rbe_parent
                                = gparent
                        }
                        while 0 != 0i32 { }
                        (*tmp).name_entry.rbe_parent =
                            (*gparent).name_entry.rbe_parent;
                        if !(*tmp).name_entry.rbe_parent.is_null() {
                            if gparent ==
                                   (*(*gparent).name_entry.rbe_parent).name_entry.rbe_left
                               {
                                (*(*gparent).name_entry.rbe_parent).name_entry.rbe_left
                                    = tmp
                            } else {
                                (*(*gparent).name_entry.rbe_parent).name_entry.rbe_right
                                    = tmp
                            }
                        } else { (*head).rbh_root = tmp }
                        (*tmp).name_entry.rbe_right = gparent;
                        (*gparent).name_entry.rbe_parent = tmp;
                        while 0 != 0i32 { }
                        if !(*tmp).name_entry.rbe_parent.is_null() {
                            current_block = 6669252993407410313;
                        } else { current_block = 5948590327928692120; }
                        loop  {
                            match current_block {
                                5948590327928692120 => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_95 ; }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        current_block = 6669252993407410313;
                                    } else {
                                        current_block = 5948590327928692120;
                                    }
                                }
                            }
                        }
                    }
            }
        } else {
            tmp = (*gparent).name_entry.rbe_left;
            if !tmp.is_null() && (*tmp).name_entry.rbe_color == 1i32 {
                (*tmp).name_entry.rbe_color = 0i32;
                loop  {
                    (*parent).name_entry.rbe_color = 0i32;
                    (*gparent).name_entry.rbe_color = 1i32;
                    if !(0 != 0i32) { break ; }
                }
                elm = gparent
            } else {
                if (*parent).name_entry.rbe_left == elm {
                    current_block = 652864300344834934;
                } else { current_block = 4567019141635105728; }
                's_211:
                    loop  {
                        match current_block {
                            4567019141635105728 => {
                                (*parent).name_entry.rbe_color = 0i32;
                                (*gparent).name_entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4567019141635105728;
                                } else { break ; }
                            }
                            _ => {
                                tmp = (*parent).name_entry.rbe_left;
                                (*parent).name_entry.rbe_left =
                                    (*tmp).name_entry.rbe_right;
                                if !(*parent).name_entry.rbe_left.is_null() {
                                    (*(*tmp).name_entry.rbe_right).name_entry.rbe_parent
                                        = parent
                                }
                                while 0 != 0i32 { }
                                (*tmp).name_entry.rbe_parent =
                                    (*parent).name_entry.rbe_parent;
                                if !(*tmp).name_entry.rbe_parent.is_null() {
                                    if parent ==
                                           (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                                       {
                                        (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                                            = tmp
                                    } else {
                                        (*(*parent).name_entry.rbe_parent).name_entry.rbe_right
                                            = tmp
                                    }
                                } else { (*head).rbh_root = tmp }
                                (*tmp).name_entry.rbe_right = parent;
                                (*parent).name_entry.rbe_parent = tmp;
                                while 0 != 0i32 { }
                                if !(*tmp).name_entry.rbe_parent.is_null() {
                                    current_block = 980989089337379490;
                                } else {
                                    current_block = 5494826135382683477;
                                }
                                loop  {
                                    match current_block {
                                        5494826135382683477 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    652864300344834934;
                                                continue 's_211 ;
                                            } else { break ; }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    980989089337379490;
                                            } else {
                                                current_block =
                                                    5494826135382683477;
                                            }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4567019141635105728;
                            }
                        }
                    }
                's_219:
                    loop  {
                        tmp = (*gparent).name_entry.rbe_right;
                        (*gparent).name_entry.rbe_right =
                            (*tmp).name_entry.rbe_left;
                        if !(*gparent).name_entry.rbe_right.is_null() {
                            (*(*tmp).name_entry.rbe_left).name_entry.rbe_parent
                                = gparent
                        }
                        while 0 != 0i32 { }
                        (*tmp).name_entry.rbe_parent =
                            (*gparent).name_entry.rbe_parent;
                        if !(*tmp).name_entry.rbe_parent.is_null() {
                            if gparent ==
                                   (*(*gparent).name_entry.rbe_parent).name_entry.rbe_left
                               {
                                (*(*gparent).name_entry.rbe_parent).name_entry.rbe_left
                                    = tmp
                            } else {
                                (*(*gparent).name_entry.rbe_parent).name_entry.rbe_right
                                    = tmp
                            }
                        } else { (*head).rbh_root = tmp }
                        (*tmp).name_entry.rbe_left = gparent;
                        (*gparent).name_entry.rbe_parent = tmp;
                        while 0 != 0i32 { }
                        if !(*tmp).name_entry.rbe_parent.is_null() {
                            current_block = 11793792312832361944;
                        } else { current_block = 2543120759711851213; }
                        loop  {
                            match current_block {
                                2543120759711851213 => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_219 ; }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        current_block = 11793792312832361944;
                                    } else {
                                        current_block = 2543120759711851213;
                                    }
                                }
                            }
                        }
                    }
            }
        }
    }
    (*(*head).rbh_root).name_entry.rbe_color = 0i32;
}
static mut paste_next_order: u_int = unsafe { 0 };
static mut paste_next_index: u_int = unsafe { 0 };
unsafe extern "C" fn paste_time_tree_RB_PREV(mut elm: *mut paste_buffer)
 -> *mut paste_buffer {
    if !(*elm).time_entry.rbe_left.is_null() {
        elm = (*elm).time_entry.rbe_left;
        while !(*elm).time_entry.rbe_right.is_null() {
            elm = (*elm).time_entry.rbe_right
        }
    } else if !(*elm).time_entry.rbe_parent.is_null() &&
                  elm == (*(*elm).time_entry.rbe_parent).time_entry.rbe_right
     {
        elm = (*elm).time_entry.rbe_parent
    } else {
        while !(*elm).time_entry.rbe_parent.is_null() &&
                  elm == (*(*elm).time_entry.rbe_parent).time_entry.rbe_left {
            elm = (*elm).time_entry.rbe_parent
        }
        elm = (*elm).time_entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn paste_rename(mut oldname: *const libc::c_char,
                                      mut newname: *const libc::c_char,
                                      mut cause: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut pb_new: *mut paste_buffer = 0 as *mut paste_buffer;
    if cause != 0 as *mut libc::c_void as *mut *mut libc::c_char {
        *cause = 0 as *mut libc::c_char
    }
    if oldname == 0 as *mut libc::c_void as *const libc::c_char ||
           *oldname as libc::c_int == 0 {
        if cause != 0 as *mut libc::c_void as *mut *mut libc::c_char {
            *cause =
                xstrdup(b"no buffer\x00" as *const u8 as *const libc::c_char)
        }
        return 1i32.wrapping_neg()
    } else if newname == 0 as *mut libc::c_void as *const libc::c_char ||
                  *newname as libc::c_int == 0 {
        if cause != 0 as *mut libc::c_void as *mut *mut libc::c_char {
            *cause =
                xstrdup(b"new name is empty\x00" as *const u8 as
                            *const libc::c_char)
        }
        return 1i32.wrapping_neg()
    } else {
        pb = paste_get_name(oldname);
        if pb == 0 as *mut libc::c_void as *mut paste_buffer {
            if cause != 0 as *mut libc::c_void as *mut *mut libc::c_char {
                xasprintf(cause,
                          b"no buffer %s\x00" as *const u8 as
                              *const libc::c_char, oldname);
            }
            return 1i32.wrapping_neg()
        } else {
            pb_new = paste_get_name(newname);
            if pb_new != 0 as *mut libc::c_void as *mut paste_buffer {
                if cause != 0 as *mut libc::c_void as *mut *mut libc::c_char {
                    xasprintf(cause,
                              b"buffer %s already exists\x00" as *const u8 as
                                  *const libc::c_char, newname);
                }
                return 1i32.wrapping_neg()
            } else {
                paste_name_tree_RB_REMOVE(&mut paste_by_name as
                                              *mut paste_name_tree, pb);
                free((*pb).name as *mut libc::c_void);
                (*pb).name = xstrdup(newname);
                if 0 != (*pb).automatic {
                    paste_num_automatic = paste_num_automatic.wrapping_sub(1)
                }
                (*pb).automatic = 0i32;
                paste_name_tree_RB_INSERT(&mut paste_by_name as
                                              *mut paste_name_tree, pb);
                return 0i32
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn paste_set(mut data: *mut libc::c_char,
                                   mut size: size_t,
                                   mut name: *const libc::c_char,
                                   mut cause: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut old: *mut paste_buffer = 0 as *mut paste_buffer;
    if cause != 0 as *mut libc::c_void as *mut *mut libc::c_char {
        *cause = 0 as *mut libc::c_char
    }
    if size == 0i32 as libc::c_ulong {
        free(data as *mut libc::c_void);
        return 0i32
    } else if name == 0 as *mut libc::c_void as *const libc::c_char {
        paste_add(data, size);
        return 0i32
    } else if *name as libc::c_int == 0 {
        if cause != 0 as *mut libc::c_void as *mut *mut libc::c_char {
            *cause =
                xstrdup(b"empty buffer name\x00" as *const u8 as
                            *const libc::c_char)
        }
        return 1i32.wrapping_neg()
    } else {
        pb =
            xmalloc(::std::mem::size_of::<paste_buffer>() as libc::c_ulong) as
                *mut paste_buffer;
        (*pb).name = xstrdup(name);
        (*pb).data = data;
        (*pb).size = size;
        (*pb).automatic = 0i32;
        let fresh1 = paste_next_order;
        paste_next_order = paste_next_order.wrapping_add(1);
        (*pb).order = fresh1;
        (*pb).created = time(0 as *mut time_t);
        old = paste_get_name(name);
        if old != 0 as *mut libc::c_void as *mut paste_buffer {
            paste_free(old);
        }
        paste_name_tree_RB_INSERT(&mut paste_by_name as *mut paste_name_tree,
                                  pb);
        paste_time_tree_RB_INSERT(&mut paste_by_time as *mut paste_time_tree,
                                  pb);
        return 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn paste_make_sample(mut pb: *mut paste_buffer)
 -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut used: size_t = 0;
    let flags: libc::c_int = 1i32 | 8i32 | 16i32;
    let width: size_t = 200i32 as size_t;
    len = (*pb).size;
    if len > width { len = width }
    buf =
        xreallocarray(0 as *mut libc::c_void, len, (4i32 + 4i32) as size_t) as
            *mut libc::c_char;
    used = utf8_strvis(buf, (*pb).data, len, flags) as size_t;
    if (*pb).size > width || used > width {
        strlcpy(buf.offset(width as isize),
                b"...\x00" as *const u8 as *const libc::c_char,
                4i32 as libc::c_ulong);
    }
    return buf;
}
unsafe extern "C" fn paste_name_tree_RB_NFIND(mut head: *mut paste_name_tree,
                                              mut elm: *mut paste_buffer)
 -> *mut paste_buffer {
    let mut tmp: *mut paste_buffer = (*head).rbh_root;
    let mut res: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = paste_cmp_names(elm, tmp);
            if comp < 0i32 {
                res = tmp;
                tmp = (*tmp).name_entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).name_entry.rbe_right
            } else { return tmp }
        } else { return res }
    };
}
unsafe extern "C" fn paste_name_tree_RB_NEXT(mut elm: *mut paste_buffer)
 -> *mut paste_buffer {
    if !(*elm).name_entry.rbe_right.is_null() {
        elm = (*elm).name_entry.rbe_right;
        while !(*elm).name_entry.rbe_left.is_null() {
            elm = (*elm).name_entry.rbe_left
        }
    } else if !(*elm).name_entry.rbe_parent.is_null() &&
                  elm == (*(*elm).name_entry.rbe_parent).name_entry.rbe_left {
        elm = (*elm).name_entry.rbe_parent
    } else {
        while !(*elm).name_entry.rbe_parent.is_null() &&
                  elm == (*(*elm).name_entry.rbe_parent).name_entry.rbe_right
              {
            elm = (*elm).name_entry.rbe_parent
        }
        elm = (*elm).name_entry.rbe_parent
    }
    return elm;
}
unsafe extern "C" fn paste_name_tree_RB_PREV(mut elm: *mut paste_buffer)
 -> *mut paste_buffer {
    if !(*elm).name_entry.rbe_left.is_null() {
        elm = (*elm).name_entry.rbe_left;
        while !(*elm).name_entry.rbe_right.is_null() {
            elm = (*elm).name_entry.rbe_right
        }
    } else if !(*elm).name_entry.rbe_parent.is_null() &&
                  elm == (*(*elm).name_entry.rbe_parent).name_entry.rbe_right
     {
        elm = (*elm).name_entry.rbe_parent
    } else {
        while !(*elm).name_entry.rbe_parent.is_null() &&
                  elm == (*(*elm).name_entry.rbe_parent).name_entry.rbe_left {
            elm = (*elm).name_entry.rbe_parent
        }
        elm = (*elm).name_entry.rbe_parent
    }
    return elm;
}
unsafe extern "C" fn paste_name_tree_RB_MINMAX(mut head: *mut paste_name_tree,
                                               mut val: libc::c_int)
 -> *mut paste_buffer {
    let mut tmp: *mut paste_buffer = (*head).rbh_root;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).name_entry.rbe_left
        } else { tmp = (*tmp).name_entry.rbe_right }
    }
    return parent;
}
unsafe extern "C" fn paste_time_tree_RB_FIND(mut head: *mut paste_time_tree,
                                             mut elm: *mut paste_buffer)
 -> *mut paste_buffer {
    let mut tmp: *mut paste_buffer = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = paste_cmp_times(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).time_entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).time_entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut paste_buffer }
    };
}
unsafe extern "C" fn paste_time_tree_RB_NFIND(mut head: *mut paste_time_tree,
                                              mut elm: *mut paste_buffer)
 -> *mut paste_buffer {
    let mut tmp: *mut paste_buffer = (*head).rbh_root;
    let mut res: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = paste_cmp_times(elm, tmp);
            if comp < 0i32 {
                res = tmp;
                tmp = (*tmp).time_entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).time_entry.rbe_right
            } else { return tmp }
        } else { return res }
    };
}

