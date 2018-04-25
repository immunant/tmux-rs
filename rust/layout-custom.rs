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
    pub type event_base;
    pub type screen_titles;
    pub type bufferevent_ops;
    pub type options;
    pub type _IO_FILE_plus;
    pub type args_entry;
    pub type evbuffer;
    pub type format_tree;
    pub type input_ctx;
    pub type tty_code;
    pub type tmuxproc;
    pub type hooks;
    pub type tmuxpeer;
    pub type environ;
    pub type format_job_tree;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
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
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, ...)
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
    fn notify_window(_: *const libc::c_char, _: *mut window) -> ();
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
    fn window_resize(_: *mut window, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn window_count_panes(_: *mut window) -> u_int;
    #[no_mangle]
    fn layout_count_cells(_: *mut layout_cell) -> u_int;
    #[no_mangle]
    fn layout_create_cell(_: *mut layout_cell) -> *mut layout_cell;
    #[no_mangle]
    fn layout_free_cell(_: *mut layout_cell) -> ();
    #[no_mangle]
    fn layout_print_cell(_: *mut layout_cell, _: *const libc::c_char,
                         _: u_int) -> ();
    #[no_mangle]
    fn layout_destroy_cell(_: *mut window, _: *mut layout_cell,
                           _: *mut *mut layout_cell) -> ();
    #[no_mangle]
    fn layout_make_leaf(_: *mut layout_cell, _: *mut window_pane) -> ();
    #[no_mangle]
    fn layout_fix_offsets(_: *mut layout_cell) -> ();
    #[no_mangle]
    fn layout_fix_panes(_: *mut window, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn layout_resize(_: *mut window, _: u_int, _: u_int) -> ();
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
pub type u_int = __u_int;
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
    pub entry: unnamed_27,
    pub wentry: unnamed_34,
    pub sentry: unnamed_10,
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
    pub entry: unnamed_22,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_28,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
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
    pub entry: unnamed_24,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type unnamed_6 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_23,
    pub entry: unnamed_11,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
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
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const _ISgraph: unnamed_13 = 32768;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_25,
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
pub union unnamed_8 {
    offset: u_int,
    data: unnamed_15,
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
pub struct unnamed_9 {
    pub ev_io_next: unnamed_31,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_30,
    pub ev_next: unnamed_37,
    pub ev_timeout_pos: unnamed_16,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_19,
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
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
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
pub const JOB_RUNNING: unnamed_25 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type cmd_find_type = libc::c_uint;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
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
    pub message_log: unnamed_3,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_6,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_4,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
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
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
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
pub const _ISxdigit: unnamed_13 = 4096;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const _ISalnum: unnamed_13 = 8;
pub const TTY_VT100: unnamed_21 = 0;
pub const JOB_DEAD: unnamed_25 = 1;
pub const PROMPT_COMMAND: unnamed_6 = 1;
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub type unnamed_13 = libc::c_uint;
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
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
pub type size_t = libc::c_ulong;
pub type uint8_t = libc::c_uchar;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const _ISspace: unnamed_13 = 8192;
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_16 {
    ev_next_with_common_timeout: unnamed_5,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub ev_signal_next: unnamed_38,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_19 {
    ev_io: unnamed_9,
    ev_signal: unnamed_18,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type layout_type = libc::c_uint;
pub type unnamed_21 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_26,
}
pub const _ISblank: unnamed_13 = 1;
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
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const PROMPT_ENTRY: unnamed_6 = 0;
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
    pub entry: unnamed_35,
    pub tree_entry: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_0,
}
pub type cmdq_type = libc::c_uint;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_8,
}
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub const TTY_VT320: unnamed_21 = 4;
pub const _ISdigit: unnamed_13 = 2048;
pub const CMDQ_CALLBACK: cmdq_type = 1;
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
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_33,
}
pub type unnamed_25 = libc::c_uint;
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type u_char = __u_char;
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
    pub alerts_entry: unnamed_2,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_12,
    pub entry: unnamed,
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
pub const TTY_VT102: unnamed_21 = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_7,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
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
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type tcflag_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const _ISalpha: unnamed_13 = 1024;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type speed_t = libc::c_uint;
pub const TTY_VT420: unnamed_21 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const TTY_VT101: unnamed_21 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_36 = 1;
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_28 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
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
pub struct grid {
    pub flags: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub hscrolled: u_int,
    pub hsize: u_int,
    pub hlimit: u_int,
    pub linedata: *mut grid_line,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub type __pid_t = libc::c_int;
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
    pub gentry: unnamed_32,
    pub entry: unnamed_20,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const _ISupper: unnamed_13 = 256;
pub const _ISprint: unnamed_13 = 16384;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
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
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_36 = 2;
pub const TTY_UNKNOWN: unnamed_21 = 6;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub type key_code = libc::c_ulonglong;
pub type bitstr_t = libc::c_uchar;
pub const _IScntrl: unnamed_13 = 2;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub type pid_t = __pid_t;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_14,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_39,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub type options_table_type = libc::c_uint;
pub const _ISlower: unnamed_13 = 512;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type u_short = __u_short;
pub const _ISpunct: unnamed_13 = 4;
pub const TTY_VT220: unnamed_21 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub type unnamed_36 = libc::c_uint;
pub type __off_t = libc::c_long;
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
    pub term_type: unnamed_21,
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
pub const JOB_CLOSED: unnamed_25 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type time_t = __time_t;
pub const LINE_SEL_NONE: unnamed_36 = 0;
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
pub struct unnamed_39 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[no_mangle]
pub unsafe extern "C" fn layout_dump(mut root: *mut layout_cell)
 -> *mut libc::c_char {
    let mut layout: [libc::c_char; 8192] = [0; 8192];
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    *layout.as_mut_ptr() = 0 as libc::c_char;
    if layout_append(root, layout.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 8192]>() as
                         libc::c_ulong) != 0i32 {
        return 0 as *mut libc::c_char
    } else {
        xasprintf(&mut out as *mut *mut libc::c_char,
                  b"%04hx,%s\x00" as *const u8 as *const libc::c_char,
                  layout_checksum(layout.as_mut_ptr()) as libc::c_int,
                  layout.as_mut_ptr());
        return out
    };
}
unsafe extern "C" fn layout_checksum(mut layout: *const libc::c_char)
 -> u_short {
    let mut csum: u_short = 0;
    csum = 0i32 as u_short;
    while *layout as libc::c_int != 0 {
        csum =
            ((csum as libc::c_int >> 1i32) +
                 ((csum as libc::c_int & 1i32) << 15i32)) as u_short;
        csum = (csum as libc::c_int + *layout as libc::c_int) as u_short;
        layout = layout.offset(1isize)
    }
    return csum;
}
unsafe extern "C" fn layout_append(mut lc: *mut layout_cell,
                                   mut buf: *mut libc::c_char,
                                   mut len: size_t) -> libc::c_int {
    let mut current_block: u64;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    let mut tmplen: size_t = 0;
    let mut brackets: *const libc::c_char =
        b"][\x00" as *const u8 as *const libc::c_char;
    if len == 0i32 as libc::c_ulong {
        return 1i32.wrapping_neg()
    } else {
        if (*lc).wp != 0 as *mut libc::c_void as *mut window_pane {
            tmplen =
                xsnprintf(tmp.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong,
                          b"%ux%u,%u,%u,%u\x00" as *const u8 as
                              *const libc::c_char, (*lc).sx, (*lc).sy,
                          (*lc).xoff, (*lc).yoff, (*(*lc).wp).id) as size_t
        } else {
            tmplen =
                xsnprintf(tmp.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong,
                          b"%ux%u,%u,%u\x00" as *const u8 as
                              *const libc::c_char, (*lc).sx, (*lc).sy,
                          (*lc).xoff, (*lc).yoff) as size_t
        }
        if tmplen >
               (::std::mem::size_of::<[libc::c_char; 64]>() as
                    libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) {
            return 1i32.wrapping_neg()
        } else if strlcat(buf, tmp.as_mut_ptr(), len) >= len {
            return 1i32.wrapping_neg()
        } else {
            match (*lc).type_0 as libc::c_uint {
                0 => {
                    brackets = b"}{\x00" as *const u8 as *const libc::c_char;
                    current_block = 13217455834649296980;
                }
                1 => { current_block = 13217455834649296980; }
                2 | _ => { current_block = 820271813250567934; }
            }
            match current_block {
                13217455834649296980 => {
                    if strlcat(buf,
                               &*brackets.offset(1isize) as
                                   *const libc::c_char, len) >= len {
                        return 1i32.wrapping_neg()
                    } else {
                        lcchild =
                            (*(&mut (*lc).cells as
                                   *mut layout_cells)).tqh_first;
                        loop  {
                            if lcchild !=
                                   0 as *mut libc::c_void as *mut layout_cell
                               {
                                if layout_append(lcchild, buf, len) != 0i32 {
                                    return 1i32.wrapping_neg()
                                } else if strlcat(buf,
                                                  b",\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  len) >= len {
                                    return 1i32.wrapping_neg()
                                } else { lcchild = (*lcchild).entry.tqe_next }
                            } else {
                                *buf.offset(strlen(buf).wrapping_sub(1i32 as
                                                                         libc::c_ulong)
                                                as isize) =
                                    *brackets.offset(0isize);
                                break ;
                            }
                        }
                    }
                }
                _ => { }
            }
            return 0i32
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_parse(mut w: *mut window,
                                      mut layout: *const libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut npanes: u_int = 0;
    let mut ncells: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut csum: u_short = 0;
    if sscanf(layout, b"%hx,\x00" as *const u8 as *const libc::c_char,
              &mut csum as *mut u_short) != 1i32 {
        return 1i32.wrapping_neg()
    } else {
        layout = layout.offset(5isize);
        if csum as libc::c_int != layout_checksum(layout) as libc::c_int {
            return 1i32.wrapping_neg()
        } else {
            lc =
                layout_construct(0 as *mut layout_cell,
                                 &mut layout as *mut *const libc::c_char);
            if lc == 0 as *mut libc::c_void as *mut layout_cell {
                return 1i32.wrapping_neg()
            } else {
                if *layout as libc::c_int != 0 {
                    current_block = 15104755292470106589;
                } else { current_block = 820271813250567934; }
                loop  {
                    match current_block {
                        820271813250567934 => {
                            npanes = window_count_panes(w);
                            ncells = layout_count_cells(lc);
                            if npanes > ncells {
                                current_block = 15104755292470106589;
                                continue ;
                            }
                            if npanes == ncells { break ; }
                            lcchild = layout_find_bottomright(lc);
                            layout_destroy_cell(w, lcchild,
                                                &mut lc as
                                                    *mut *mut layout_cell);
                            current_block = 820271813250567934;
                        }
                        _ => {
                            layout_free_cell(lc);
                            return 1i32.wrapping_neg()
                        }
                    }
                }
                sx = (*w).sx;
                sy = (*w).sy;
                window_resize(w, (*lc).sx, (*lc).sy);
                layout_free_cell((*w).layout_root);
                (*w).layout_root = lc;
                wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
                layout_assign(&mut wp as *mut *mut window_pane, lc);
                layout_fix_offsets(lc);
                layout_fix_panes(w, (*lc).sx, (*lc).sy);
                layout_resize(w, sx, sy);
                window_resize(w, sx, sy);
                layout_print_cell(lc,
                                  (*::std::mem::transmute::<&[u8; 13],
                                                            &[libc::c_char; 13]>(b"layout_parse\x00")).as_ptr(),
                                  0i32 as u_int);
                notify_window(b"window-layout-changed\x00" as *const u8 as
                                  *const libc::c_char, w);
                return 0i32
            }
        }
    };
}
unsafe extern "C" fn layout_assign(mut wp: *mut *mut window_pane,
                                   mut lc: *mut layout_cell) -> () {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    match (*lc).type_0 as libc::c_uint {
        2 => {
            layout_make_leaf(lc, *wp);
            *wp = (**wp).entry.tqe_next;
            return
        }
        0 | 1 => {
            lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
            while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
                layout_assign(wp, lcchild);
                lcchild = (*lcchild).entry.tqe_next
            }
            return
        }
        _ => { return; }
    };
}
unsafe extern "C" fn layout_find_bottomright(mut lc: *mut layout_cell)
 -> *mut layout_cell {
    if (*lc).type_0 as libc::c_uint ==
           LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        return lc
    } else {
        lc =
            *(*((*(&mut (*lc).cells as *mut layout_cells)).tqh_last as
                    *mut layout_cells)).tqh_last;
        return layout_find_bottomright(lc)
    };
}
unsafe extern "C" fn layout_construct(mut lcparent: *mut layout_cell,
                                      mut layout: *mut *const libc::c_char)
 -> *mut layout_cell {
    let mut current_block: u64;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    let mut saved: *const libc::c_char = 0 as *const libc::c_char;
    if 0 ==
           *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as
                                          isize) as libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int {
        return 0 as *mut layout_cell
    } else if sscanf(*layout,
                     b"%ux%u,%u,%u\x00" as *const u8 as *const libc::c_char,
                     &mut sx as *mut u_int, &mut sy as *mut u_int,
                     &mut xoff as *mut u_int, &mut yoff as *mut u_int) != 4i32
     {
        return 0 as *mut layout_cell
    } else {
        while 0 !=
                  *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int
                                                 as isize) as libc::c_int &
                      _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
              {
            *layout = (*layout).offset(1isize)
        }
        if **layout as libc::c_int != 120 {
            return 0 as *mut layout_cell
        } else {
            *layout = (*layout).offset(1isize);
            while 0 !=
                      *(*__ctype_b_loc()).offset(**layout as u_char as
                                                     libc::c_int as isize) as
                          libc::c_int &
                          _ISdigit as libc::c_int as libc::c_ushort as
                              libc::c_int {
                *layout = (*layout).offset(1isize)
            }
            if **layout as libc::c_int != 44 {
                return 0 as *mut layout_cell
            } else {
                *layout = (*layout).offset(1isize);
                while 0 !=
                          *(*__ctype_b_loc()).offset(**layout as u_char as
                                                         libc::c_int as isize)
                              as libc::c_int &
                              _ISdigit as libc::c_int as libc::c_ushort as
                                  libc::c_int {
                    *layout = (*layout).offset(1isize)
                }
                if **layout as libc::c_int != 44 {
                    return 0 as *mut layout_cell
                } else {
                    *layout = (*layout).offset(1isize);
                    while 0 !=
                              *(*__ctype_b_loc()).offset(**layout as u_char as
                                                             libc::c_int as
                                                             isize) as
                                  libc::c_int &
                                  _ISdigit as libc::c_int as libc::c_ushort as
                                      libc::c_int {
                        *layout = (*layout).offset(1isize)
                    }
                    if **layout as libc::c_int == 44 {
                        saved = *layout;
                        *layout = (*layout).offset(1isize);
                        while 0 !=
                                  *(*__ctype_b_loc()).offset(**layout as
                                                                 u_char as
                                                                 libc::c_int
                                                                 as isize) as
                                      libc::c_int &
                                      _ISdigit as libc::c_int as
                                          libc::c_ushort as libc::c_int {
                            *layout = (*layout).offset(1isize)
                        }
                        if **layout as libc::c_int == 120 { *layout = saved }
                    }
                    lc = layout_create_cell(lcparent);
                    (*lc).sx = sx;
                    (*lc).sy = sy;
                    (*lc).xoff = xoff;
                    (*lc).yoff = yoff;
                    match **layout as libc::c_int {
                        44 | 125 | 93 | 0 => { return lc }
                        123 => {
                            (*lc).type_0 = LAYOUT_LEFTRIGHT;
                            current_block = 12599329904712511516;
                        }
                        91 => {
                            (*lc).type_0 = LAYOUT_TOPBOTTOM;
                            current_block = 12599329904712511516;
                        }
                        _ => { current_block = 12183639489562779793; }
                    }
                    loop  {
                        match current_block {
                            12183639489562779793 => {
                                layout_free_cell(lc);
                                return 0 as *mut layout_cell
                            }
                            _ => {
                                *layout = (*layout).offset(1isize);
                                lcchild = layout_construct(lc, layout);
                                if lcchild ==
                                       0 as *mut libc::c_void as
                                           *mut layout_cell {
                                    current_block = 12183639489562779793;
                                    continue ;
                                }
                                loop  {
                                    (*lcchild).entry.tqe_next =
                                        0 as *mut layout_cell;
                                    (*lcchild).entry.tqe_prev =
                                        (*(&mut (*lc).cells as
                                               *mut layout_cells)).tqh_last;
                                    let ref mut fresh0 =
                                        *(*(&mut (*lc).cells as
                                                *mut layout_cells)).tqh_last;
                                    *fresh0 = lcchild;
                                    let ref mut fresh1 =
                                        (*(&mut (*lc).cells as
                                               *mut layout_cells)).tqh_last;
                                    *fresh1 =
                                        &mut (*lcchild).entry.tqe_next as
                                            *mut *mut layout_cell;
                                    if !(0 != 0i32) { break ; }
                                }
                                if **layout as libc::c_int == 44 {
                                    current_block = 12599329904712511516;
                                    continue ;
                                }
                                match (*lc).type_0 as libc::c_uint {
                                    0 => {
                                        if **layout as libc::c_int != 125 {
                                            current_block =
                                                12183639489562779793;
                                        } else { break ; }
                                    }
                                    1 => {
                                        if **layout as libc::c_int != 93 {
                                            current_block =
                                                12183639489562779793;
                                        } else { break ; }
                                    }
                                    _ => {
                                        current_block = 12183639489562779793;
                                    }
                                }
                            }
                        }
                    }
                    *layout = (*layout).offset(1isize);
                    return lc
                }
            }
        }
    };
}

