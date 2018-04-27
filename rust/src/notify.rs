extern crate libc;

use client::{client, clients};
use session::session;
use window::window;

extern "C" {
    pub type screen_titles;
    pub type format_job_tree;
    pub type environ;
    pub type input_ctx;
    pub type hooks;
    pub type evbuffer;
    pub type args_entry;
    pub type format_tree;
    pub type options;
    pub type bufferevent_ops;
    pub type _IO_FILE_plus;
    pub type tmuxpeer;
    pub type tty_code;
    pub type tmuxproc;
    pub type event_base;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
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
    fn hooks_get(_: *mut session) -> *mut hooks;
    #[no_mangle]
    fn hooks_find(_: *mut hooks, _: *const libc::c_char) -> *mut hook;
    #[no_mangle]
    fn control_notify_input(_: *mut client, _: *mut window_pane,
                            _: *mut evbuffer) -> ();
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn cmdq_append(_: *mut client, _: *mut cmdq_item) -> ();
    #[no_mangle]
    fn session_remove_ref(_: *mut session, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn window_remove_ref(_: *mut window, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn server_client_unref(_: *mut client) -> ();
    #[no_mangle]
    fn cmdq_insert_after(_: *mut cmdq_item, _: *mut cmdq_item) -> ();
    #[no_mangle]
    fn cmdq_format(_: *mut cmdq_item, _: *const libc::c_char,
                   _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn cmdq_get_command(_: *mut cmd_list, _: *mut cmd_find_state,
                        _: *mut mouse_event, _: libc::c_int)
     -> *mut cmdq_item;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn cmd_find_copy_state(_: *mut cmd_find_state, _: *mut cmd_find_state)
     -> ();
    #[no_mangle]
    fn cmd_find_from_nothing(_: *mut cmd_find_state, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn cmd_find_valid_state(_: *mut cmd_find_state) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_empty_state(_: *mut cmd_find_state) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_clear_state(_: *mut cmd_find_state, _: libc::c_int) -> ();
    #[no_mangle]
    fn control_notify_session_window_changed(_: *mut session) -> ();
    #[no_mangle]
    fn control_notify_session_closed(_: *mut session) -> ();
    #[no_mangle]
    fn control_notify_session_created(_: *mut session) -> ();
    #[no_mangle]
    fn control_notify_session_renamed(_: *mut session) -> ();
    #[no_mangle]
    fn control_notify_client_session_changed(_: *mut client) -> ();
    #[no_mangle]
    fn control_notify_window_renamed(_: *mut window) -> ();
    #[no_mangle]
    fn control_notify_window_linked(_: *mut session, _: *mut window) -> ();
    #[no_mangle]
    fn control_notify_window_unlinked(_: *mut session, _: *mut window) -> ();
    #[no_mangle]
    fn control_notify_window_pane_changed(_: *mut window) -> ();
    #[no_mangle]
    fn control_notify_window_layout_changed(_: *mut window) -> ();
    #[no_mangle]
    fn control_notify_pane_mode_changed(_: libc::c_int) -> ();
    #[no_mangle]
    fn cmdq_get_callback1(_: *const libc::c_char, _: cmdq_cb,
                          _: *mut libc::c_void) -> *mut cmdq_item;
    #[no_mangle]
    fn session_add_ref(_: *mut session, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn window_add_ref(_: *mut window, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn cmd_find_from_client(_: *mut cmd_find_state, _: *mut client,
                            _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_from_session(_: *mut cmd_find_state, _: *mut session,
                             _: libc::c_int) -> ();
    #[no_mangle]
    fn session_alive(_: *mut session) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_from_winlink(_: *mut cmd_find_state, _: *mut winlink,
                             _: libc::c_int) -> ();
    #[no_mangle]
    fn cmd_find_from_session_window(_: *mut cmd_find_state, _: *mut session,
                                    _: *mut window, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn cmd_find_from_window(_: *mut cmd_find_state, _: *mut window,
                            _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_from_pane(_: *mut cmd_find_state, _: *mut window_pane,
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
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub type u_short = __u_short;
pub type tcflag_t = libc::c_uint;
pub type pid_t = __pid_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub rbe_left: *mut hook,
    pub rbe_right: *mut hook,
    pub rbe_parent: *mut hook,
    pub rbe_color: libc::c_int,
}
pub type cmd_retval = libc::c_int;
pub type u_char = __u_char;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub type layout_type = libc::c_uint;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct hook {
    pub name: *const libc::c_char,
    pub cmdlist: *mut cmd_list,
    pub entry: unnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_4 {
    offset: u_int,
    data: unnamed_28,
}
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub type cmd_find_type = libc::c_uint;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type cmdq_type = libc::c_uint;
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
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub ev_signal_next: unnamed_1,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
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
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_0,
    pub entry: unnamed_21,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
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
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub type _IO_lock_t = ();
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
    pub entry: unnamed_5,
}
pub const JOB_DEAD: unnamed_10 = 1;
pub type unnamed_10 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const PROMPT_COMMAND: unnamed_23 = 1;
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
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
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
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
    pub entry: unnamed_35,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_11 {
    ev_io: unnamed_24,
    ev_signal: unnamed_7,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_18,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const JOB_RUNNING: unnamed_10 = 0;
pub const PROMPT_ENTRY: unnamed_23 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const JOB_CLOSED: unnamed_10 = 2;
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
    pub entry: unnamed_14,
    pub tree_entry: unnamed_29,
}
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub type u_int = __u_int;
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
pub struct unnamed_12 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_17 {
    ev_next_with_common_timeout: unnamed_38,
    min_heap_idx: libc::c_int,
}
pub const TTY_VT320: unnamed_36 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_18 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const TTY_VT102: unnamed_36 = 2;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_30,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type options_table_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_22,
    pub ev_next: unnamed_32,
    pub ev_timeout_pos: unnamed_17,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_11,
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
pub struct unnamed_20 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
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
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type __suseconds_t = libc::c_long;
pub type unnamed_23 = libc::c_uint;
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
pub struct unnamed_24 {
    pub ev_io_next: unnamed_34,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_4,
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
pub struct unnamed_25 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_12,
}
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
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
    pub term_type: unnamed_36,
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
pub struct unnamed_28 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub type time_t = __time_t;
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
    pub entry: unnamed_8,
    pub wentry: unnamed_39,
    pub sentry: unnamed_9,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_30 = 2;
pub const LINE_SEL_NONE: unnamed_30 = 0;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub type uint32_t = libc::c_uint;
pub type unnamed_30 = libc::c_uint;
pub type key_code = libc::c_ulonglong;
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
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_15,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_31,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTY_UNKNOWN: unnamed_36 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_20,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_6,
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
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
pub const TTY_VT100: unnamed_36 = 0;
pub const TTY_VT220: unnamed_36 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_37,
}
pub type unnamed_36 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct notify_entry {
    pub name: *const libc::c_char,
    pub client: *mut client,
    pub session: *mut session,
    pub window: *mut window,
    pub pane: libc::c_int,
    pub fs: cmd_find_state,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_10,
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
    pub entry: unnamed_25,
}
pub type __pid_t = libc::c_int;
pub type uint16_t = libc::c_ushort;
pub type __off64_t = libc::c_long;
pub const LINE_SEL_LEFT_RIGHT: unnamed_30 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const TTY_VT420: unnamed_36 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type __off_t = libc::c_long;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const TTY_VT101: unnamed_36 = 1;
#[no_mangle]
pub unsafe extern "C" fn notify_input(mut wp: *mut window_pane,
                                      mut input: *mut evbuffer) -> () {
    let mut c: *mut client = 0 as *mut client;
    c = (*(&mut clients as *mut clients)).tqh_first;
    while c != 0 as *mut libc::c_void as *mut client {
        if 0 != (*c).flags & 8192i32 { control_notify_input(c, wp, input); }
        c = (*c).entry.tqe_next
    };
}
#[no_mangle]
pub unsafe extern "C" fn notify_client(mut name: *const libc::c_char,
                                       mut c: *mut client) -> () {
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current: 0 as *mut cmd_find_state,
                       s: 0 as *mut session,
                       wl: 0 as *mut winlink,
                       w: 0 as *mut window,
                       wp: 0 as *mut window_pane,
                       idx: 0,};
    cmd_find_from_client(&mut fs as *mut cmd_find_state, c, 0i32);
    notify_add(name, &mut fs as *mut cmd_find_state, c, 0 as *mut session,
               0 as *mut window, 0 as *mut window_pane);
}
unsafe extern "C" fn notify_add(mut name: *const libc::c_char,
                                mut fs: *mut cmd_find_state,
                                mut c: *mut client, mut s: *mut session,
                                mut w: *mut window, mut wp: *mut window_pane)
 -> () {
    let mut ne: *mut notify_entry = 0 as *mut notify_entry;
    let mut new_item: *mut cmdq_item = 0 as *mut cmdq_item;
    ne =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<notify_entry>() as libc::c_ulong) as
            *mut notify_entry;
    (*ne).name = xstrdup(name);
    (*ne).client = c;
    (*ne).session = s;
    (*ne).window = w;
    if wp != 0 as *mut libc::c_void as *mut window_pane {
        (*ne).pane = (*wp).id as libc::c_int
    } else { (*ne).pane = 1i32.wrapping_neg() }
    if c != 0 as *mut libc::c_void as *mut client { (*c).references += 1 }
    if s != 0 as *mut libc::c_void as *mut session {
        session_add_ref(s,
                        (*::std::mem::transmute::<&[u8; 11],
                                                  &[libc::c_char; 11]>(b"notify_add\x00")).as_ptr());
    }
    if w != 0 as *mut libc::c_void as *mut window {
        window_add_ref(w,
                       (*::std::mem::transmute::<&[u8; 11],
                                                 &[libc::c_char; 11]>(b"notify_add\x00")).as_ptr());
    }
    cmd_find_copy_state(&mut (*ne).fs as *mut cmd_find_state, fs);
    if (*ne).fs.s != 0 as *mut libc::c_void as *mut session {
        session_add_ref((*ne).fs.s,
                        (*::std::mem::transmute::<&[u8; 11],
                                                  &[libc::c_char; 11]>(b"notify_add\x00")).as_ptr());
    }
    new_item =
        cmdq_get_callback1(b"notify_callback\x00" as *const u8 as
                               *const libc::c_char, Some(notify_callback),
                           ne as *mut libc::c_void);
    cmdq_append(0 as *mut client, new_item);
}
unsafe extern "C" fn notify_callback(mut item: *mut cmdq_item,
                                     mut data: *mut libc::c_void)
 -> cmd_retval {
    let mut ne: *mut notify_entry = data as *mut notify_entry;
    log_debug(b"%s: %s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"notify_callback\x00")).as_ptr(),
              (*ne).name);
    if strcmp((*ne).name,
              b"pane-mode-changed\x00" as *const u8 as *const libc::c_char) ==
           0i32 {
        control_notify_pane_mode_changed((*ne).pane);
    }
    if strcmp((*ne).name,
              b"window-layout-changed\x00" as *const u8 as
                  *const libc::c_char) == 0i32 {
        control_notify_window_layout_changed((*ne).window);
    }
    if strcmp((*ne).name,
              b"window-pane-changed\x00" as *const u8 as *const libc::c_char)
           == 0i32 {
        control_notify_window_pane_changed((*ne).window);
    }
    if strcmp((*ne).name,
              b"window-unlinked\x00" as *const u8 as *const libc::c_char) ==
           0i32 {
        control_notify_window_unlinked((*ne).session, (*ne).window);
    }
    if strcmp((*ne).name,
              b"window-linked\x00" as *const u8 as *const libc::c_char) ==
           0i32 {
        control_notify_window_linked((*ne).session, (*ne).window);
    }
    if strcmp((*ne).name,
              b"window-renamed\x00" as *const u8 as *const libc::c_char) ==
           0i32 {
        control_notify_window_renamed((*ne).window);
    }
    if strcmp((*ne).name,
              b"client-session-changed\x00" as *const u8 as
                  *const libc::c_char) == 0i32 {
        control_notify_client_session_changed((*ne).client);
    }
    if strcmp((*ne).name,
              b"session-renamed\x00" as *const u8 as *const libc::c_char) ==
           0i32 {
        control_notify_session_renamed((*ne).session);
    }
    if strcmp((*ne).name,
              b"session-created\x00" as *const u8 as *const libc::c_char) ==
           0i32 {
        control_notify_session_created((*ne).session);
    }
    if strcmp((*ne).name,
              b"session-closed\x00" as *const u8 as *const libc::c_char) ==
           0i32 {
        control_notify_session_closed((*ne).session);
    }
    if strcmp((*ne).name,
              b"session-window-changed\x00" as *const u8 as
                  *const libc::c_char) == 0i32 {
        control_notify_session_window_changed((*ne).session);
    }
    notify_hook(item, ne);
    if (*ne).client != 0 as *mut libc::c_void as *mut client {
        server_client_unref((*ne).client);
    }
    if (*ne).session != 0 as *mut libc::c_void as *mut session {
        session_remove_ref((*ne).session,
                           (*::std::mem::transmute::<&[u8; 16],
                                                     &[libc::c_char; 16]>(b"notify_callback\x00")).as_ptr());
    }
    if (*ne).window != 0 as *mut libc::c_void as *mut window {
        window_remove_ref((*ne).window,
                          (*::std::mem::transmute::<&[u8; 16],
                                                    &[libc::c_char; 16]>(b"notify_callback\x00")).as_ptr());
    }
    if (*ne).fs.s != 0 as *mut libc::c_void as *mut session {
        session_remove_ref((*ne).fs.s,
                           (*::std::mem::transmute::<&[u8; 16],
                                                     &[libc::c_char; 16]>(b"notify_callback\x00")).as_ptr());
    }
    free((*ne).name as *mut libc::c_void);
    free(ne as *mut libc::c_void);
    return CMD_RETURN_NORMAL;
}
unsafe extern "C" fn notify_hook(mut item: *mut cmdq_item,
                                 mut ne: *mut notify_entry) -> () {
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current: 0 as *mut cmd_find_state,
                       s: 0 as *mut session,
                       wl: 0 as *mut winlink,
                       w: 0 as *mut window,
                       wp: 0 as *mut window_pane,
                       idx: 0,};
    let mut hook: *mut hook = 0 as *mut hook;
    let mut new_item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut s: *mut session = (*ne).session;
    let mut w: *mut window = (*ne).window;
    cmd_find_clear_state(&mut fs as *mut cmd_find_state, 0i32);
    if 0 != cmd_find_empty_state(&mut (*ne).fs as *mut cmd_find_state) ||
           0 == cmd_find_valid_state(&mut (*ne).fs as *mut cmd_find_state) {
        cmd_find_from_nothing(&mut fs as *mut cmd_find_state, 0i32);
    } else {
        cmd_find_copy_state(&mut fs as *mut cmd_find_state,
                            &mut (*ne).fs as *mut cmd_find_state);
    }
    hook = hooks_find(hooks_get(fs.s), (*ne).name);
    if hook == 0 as *mut libc::c_void as *mut hook {
        return
    } else {
        log_debug(b"notify hook %s\x00" as *const u8 as *const libc::c_char,
                  (*ne).name);
        new_item =
            cmdq_get_command((*hook).cmdlist, &mut fs as *mut cmd_find_state,
                             0 as *mut mouse_event, 4i32);
        cmdq_format(new_item, b"hook\x00" as *const u8 as *const libc::c_char,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    (*ne).name);
        if s != 0 as *mut libc::c_void as *mut session {
            cmdq_format(new_item,
                        b"hook_session\x00" as *const u8 as
                            *const libc::c_char,
                        b"$%u\x00" as *const u8 as *const libc::c_char,
                        (*s).id);
            cmdq_format(new_item,
                        b"hook_session_name\x00" as *const u8 as
                            *const libc::c_char,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        (*s).name);
        }
        if w != 0 as *mut libc::c_void as *mut window {
            cmdq_format(new_item,
                        b"hook_window\x00" as *const u8 as
                            *const libc::c_char,
                        b"@%u\x00" as *const u8 as *const libc::c_char,
                        (*w).id);
            cmdq_format(new_item,
                        b"hook_window_name\x00" as *const u8 as
                            *const libc::c_char,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        (*w).name);
        }
        if (*ne).pane != 1i32.wrapping_neg() {
            cmdq_format(new_item,
                        b"hook_pane\x00" as *const u8 as *const libc::c_char,
                        b"%%%d\x00" as *const u8 as *const libc::c_char,
                        (*ne).pane);
        }
        cmdq_insert_after(item, new_item);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn notify_session(mut name: *const libc::c_char,
                                        mut s: *mut session) -> () {
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current: 0 as *mut cmd_find_state,
                       s: 0 as *mut session,
                       wl: 0 as *mut winlink,
                       w: 0 as *mut window,
                       wp: 0 as *mut window_pane,
                       idx: 0,};
    if 0 != session_alive(s) {
        cmd_find_from_session(&mut fs as *mut cmd_find_state, s, 0i32);
    } else { cmd_find_from_nothing(&mut fs as *mut cmd_find_state, 0i32); }
    notify_add(name, &mut fs as *mut cmd_find_state, 0 as *mut client, s,
               0 as *mut window, 0 as *mut window_pane);
}
#[no_mangle]
pub unsafe extern "C" fn notify_winlink(mut name: *const libc::c_char,
                                        mut wl: *mut winlink) -> () {
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current: 0 as *mut cmd_find_state,
                       s: 0 as *mut session,
                       wl: 0 as *mut winlink,
                       w: 0 as *mut window,
                       wp: 0 as *mut window_pane,
                       idx: 0,};
    cmd_find_from_winlink(&mut fs as *mut cmd_find_state, wl, 0i32);
    notify_add(name, &mut fs as *mut cmd_find_state, 0 as *mut client,
               (*wl).session, (*wl).window, 0 as *mut window_pane);
}
#[no_mangle]
pub unsafe extern "C" fn notify_session_window(mut name: *const libc::c_char,
                                               mut s: *mut session,
                                               mut w: *mut window) -> () {
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current: 0 as *mut cmd_find_state,
                       s: 0 as *mut session,
                       wl: 0 as *mut winlink,
                       w: 0 as *mut window,
                       wp: 0 as *mut window_pane,
                       idx: 0,};
    cmd_find_from_session_window(&mut fs as *mut cmd_find_state, s, w, 0i32);
    notify_add(name, &mut fs as *mut cmd_find_state, 0 as *mut client, s, w,
               0 as *mut window_pane);
}
#[no_mangle]
pub unsafe extern "C" fn notify_window(mut name: *const libc::c_char,
                                       mut w: *mut window) -> () {
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current: 0 as *mut cmd_find_state,
                       s: 0 as *mut session,
                       wl: 0 as *mut winlink,
                       w: 0 as *mut window,
                       wp: 0 as *mut window_pane,
                       idx: 0,};
    cmd_find_from_window(&mut fs as *mut cmd_find_state, w, 0i32);
    notify_add(name, &mut fs as *mut cmd_find_state, 0 as *mut client,
               0 as *mut session, w, 0 as *mut window_pane);
}
#[no_mangle]
pub unsafe extern "C" fn notify_pane(mut name: *const libc::c_char,
                                     mut wp: *mut window_pane) -> () {
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current: 0 as *mut cmd_find_state,
                       s: 0 as *mut session,
                       wl: 0 as *mut winlink,
                       w: 0 as *mut window,
                       wp: 0 as *mut window_pane,
                       idx: 0,};
    cmd_find_from_pane(&mut fs as *mut cmd_find_state, wp, 0i32);
    notify_add(name, &mut fs as *mut cmd_find_state, 0 as *mut client,
               0 as *mut session, 0 as *mut window, wp);
}

