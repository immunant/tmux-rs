extern crate libc;
extern "C" {
    pub type event_base;
    pub type screen_titles;
    pub type format_job_tree;
    pub type input_ctx;
    pub type hooks;
    pub type _IO_FILE_plus;
    pub type tmuxproc;
    pub type tty_code;
    pub type options;
    pub type tmuxpeer;
    pub type args_entry;
    pub type format_tree;
    pub type environ;
    pub type bufferevent_ops;
    pub type evbuffer;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char,
                   _: libc::c_ulong) -> libc::c_int;
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
    static mut _sys_nerr: libc::c_int;
    #[no_mangle]
    static _sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn strtonum(_: *const libc::c_char, _: libc::c_longlong,
                _: libc::c_longlong, _: *mut *const libc::c_char)
     -> libc::c_longlong;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
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
pub struct unnamed_0 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    ev_io: unnamed_13,
    ev_signal: unnamed_39,
}
pub const _ISpunct: unnamed_18 = 4;
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_10,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub type unnamed_4 = libc::c_uint;
pub const TTY_VT100: unnamed_6 = 0;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_5 {
    offset: u_int,
    data: unnamed_15,
}
pub type size_t = libc::c_ulong;
pub type unnamed_6 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_30,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_7 {
    ev_next_with_common_timeout: unnamed_16,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_0,
}
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
pub struct unnamed_11 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const _ISlower: unnamed_18 = 512;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
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
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub type __off64_t = libc::c_long;
pub const _ISspace: unnamed_18 = 8192;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub ev_io_next: unnamed_2,
    pub ev_timeout: timeval,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTY_VT101: unnamed_6 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
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
    pub term_type: unnamed_6,
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
pub struct unnamed_16 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const _ISprint: unnamed_18 = 16384;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub type unnamed_18 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub type uint32_t = libc::c_uint;
pub const _IScntrl: unnamed_18 = 2;
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
    pub entry: unnamed_24,
    pub tree_entry: unnamed_20,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_8,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
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
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub type options_table_scope = libc::c_uint;
pub type u_char = __u_char;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub type tcflag_t = libc::c_uint;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_5,
}
pub const TTY_VT102: unnamed_6 = 2;
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
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type unnamed_23 = libc::c_uint;
pub const LINE_SEL_NONE: unnamed_36 = 0;
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
    pub entry: unnamed_19,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub type speed_t = libc::c_uint;
pub type __u_char = libc::c_uchar;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_3,
    pub entry: unnamed_34,
}
pub type pid_t = __pid_t;
pub const LINE_SEL_RIGHT_LEFT: unnamed_36 = 2;
pub const _ISxdigit: unnamed_18 = 4096;
pub const _ISupper: unnamed_18 = 256;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub type cmd_retval = libc::c_int;
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const _ISblank: unnamed_18 = 1;
pub type options_table_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const _ISdigit: unnamed_18 = 2048;
pub const TTY_VT220: unnamed_6 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
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
pub const JOB_DEAD: unnamed_4 = 1;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const TTY_VT420: unnamed_6 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub type cmdq_type = libc::c_uint;
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
    pub winlinks: unnamed_22,
    pub entry: unnamed_11,
}
pub const _ISalpha: unnamed_18 = 1024;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
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
    pub message_log: unnamed_9,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_23,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_25,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub type uint16_t = libc::c_ushort;
pub const PROMPT_COMMAND: unnamed_23 = 1;
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
    pub entry: unnamed_38,
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
pub struct unnamed_27 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_21,
}
pub type bitstr_t = libc::c_uchar;
pub const _ISgraph: unnamed_18 = 32768;
pub const JOB_CLOSED: unnamed_4 = 2;
pub type __u_short = libc::c_ushort;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
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
pub type _IO_lock_t = ();
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
    pub gentry: unnamed_33,
    pub entry: unnamed_32,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const TTY_UNKNOWN: unnamed_6 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const JOB_RUNNING: unnamed_4 = 0;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub type key_code = libc::c_ulonglong;
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
pub struct unnamed_30 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub const PROMPT_ENTRY: unnamed_23 = 0;
pub type __pid_t = libc::c_int;
pub type uint8_t = libc::c_uchar;
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
pub struct unnamed_31 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_4,
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
    pub entry: unnamed_37,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_17,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const _ISalnum: unnamed_18 = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_28,
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
pub struct event {
    pub ev_active_next: unnamed_14,
    pub ev_next: unnamed_35,
    pub ev_timeout_pos: unnamed_7,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_1,
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
pub const LINE_SEL_LEFT_RIGHT: unnamed_36 = 1;
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
    pub wentry: unnamed_31,
    pub sentry: unnamed_12,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type unnamed_36 = libc::c_uint;
pub type u_int = __u_int;
pub type cmd_find_type = libc::c_uint;
pub const TTY_VT320: unnamed_6 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub ev_signal_next: unnamed_26,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub type __suseconds_t = libc::c_long;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[no_mangle]
pub unsafe extern "C" fn colour_find_rgb(mut r: u_char, mut g: u_char,
                                         mut b: u_char) -> libc::c_int {
    static mut q2c: [libc::c_int; 6] =
        unsafe { [0i32, 95i32, 135i32, 175i32, 215i32, 255i32] };
    let mut qr: libc::c_int = 0;
    let mut qg: libc::c_int = 0;
    let mut qb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut cg: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut grey_avg: libc::c_int = 0;
    let mut grey_idx: libc::c_int = 0;
    let mut grey: libc::c_int = 0;
    qr = colour_to_6cube(r as libc::c_int);
    cr = q2c[qr as usize];
    qg = colour_to_6cube(g as libc::c_int);
    cg = q2c[qg as usize];
    qb = colour_to_6cube(b as libc::c_int);
    cb = q2c[qb as usize];
    if cr == r as libc::c_int && cg == g as libc::c_int &&
           cb == b as libc::c_int {
        return 16i32 + 36i32 * qr + 6i32 * qg + qb | 16777216i32
    } else {
        grey_avg =
            (r as libc::c_int + g as libc::c_int + b as libc::c_int) / 3i32;
        if grey_avg > 238i32 {
            grey_idx = 23i32
        } else { grey_idx = (grey_avg - 3i32) / 10i32 }
        grey = 8i32 + 10i32 * grey_idx;
        d =
            colour_dist_sq(cr, cg, cb, r as libc::c_int, g as libc::c_int,
                           b as libc::c_int);
        if colour_dist_sq(grey, grey, grey, r as libc::c_int,
                          g as libc::c_int, b as libc::c_int) < d {
            idx = 232i32 + grey_idx
        } else { idx = 16i32 + 36i32 * qr + 6i32 * qg + qb }
        return idx | 16777216i32
    };
}
unsafe extern "C" fn colour_dist_sq(mut R: libc::c_int, mut G: libc::c_int,
                                    mut B: libc::c_int, mut r: libc::c_int,
                                    mut g: libc::c_int, mut b: libc::c_int)
 -> libc::c_int {
    return (R - r) * (R - r) + (G - g) * (G - g) + (B - b) * (B - b);
}
unsafe extern "C" fn colour_to_6cube(mut v: libc::c_int) -> libc::c_int {
    if v < 48i32 {
        return 0i32
    } else if v < 114i32 { return 1i32 } else { return (v - 35i32) / 40i32 };
}
#[no_mangle]
pub unsafe extern "C" fn colour_join_rgb(mut r: u_char, mut g: u_char,
                                         mut b: u_char) -> libc::c_int {
    return (r as libc::c_int & 255i32) << 16i32 |
               (g as libc::c_int & 255i32) << 8i32 | b as libc::c_int & 255i32
               | 33554432i32;
}
#[no_mangle]
pub unsafe extern "C" fn colour_split_rgb(mut c: libc::c_int,
                                          mut r: *mut u_char,
                                          mut g: *mut u_char,
                                          mut b: *mut u_char) -> () {
    *r = (c >> 16i32 & 255i32) as u_char;
    *g = (c >> 8i32 & 255i32) as u_char;
    *b = (c & 255i32) as u_char;
}
#[no_mangle]
pub unsafe extern "C" fn colour_tostring(mut c: libc::c_int)
 -> *const libc::c_char {
    static mut s: [libc::c_char; 32] = unsafe { [0; 32] };
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    if 0 != c & 33554432i32 {
        colour_split_rgb(c, &mut r as *mut u_char, &mut g as *mut u_char,
                         &mut b as *mut u_char);
        xsnprintf(s.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong,
                  b"#%02x%02x%02x\x00" as *const u8 as *const libc::c_char,
                  r as libc::c_int, g as libc::c_int, b as libc::c_int);
        return s.as_mut_ptr()
    } else if 0 != c & 16777216i32 {
        xsnprintf(s.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong,
                  b"colour%u\x00" as *const u8 as *const libc::c_char,
                  c & 255i32);
        return s.as_mut_ptr()
    } else {
        match c {
            0 => { return b"black\x00" as *const u8 as *const libc::c_char }
            1 => { return b"red\x00" as *const u8 as *const libc::c_char }
            2 => { return b"green\x00" as *const u8 as *const libc::c_char }
            3 => { return b"yellow\x00" as *const u8 as *const libc::c_char }
            4 => { return b"blue\x00" as *const u8 as *const libc::c_char }
            5 => { return b"magenta\x00" as *const u8 as *const libc::c_char }
            6 => { return b"cyan\x00" as *const u8 as *const libc::c_char }
            7 => { return b"white\x00" as *const u8 as *const libc::c_char }
            8 => { return b"default\x00" as *const u8 as *const libc::c_char }
            90 => {
                return b"brightblack\x00" as *const u8 as *const libc::c_char
            }
            91 => {
                return b"brightred\x00" as *const u8 as *const libc::c_char
            }
            92 => {
                return b"brightgreen\x00" as *const u8 as *const libc::c_char
            }
            93 => {
                return b"brightyellow\x00" as *const u8 as *const libc::c_char
            }
            94 => {
                return b"brightblue\x00" as *const u8 as *const libc::c_char
            }
            95 => {
                return b"brightmagenta\x00" as *const u8 as
                           *const libc::c_char
            }
            96 => {
                return b"brightcyan\x00" as *const u8 as *const libc::c_char
            }
            97 => {
                return b"brightwhite\x00" as *const u8 as *const libc::c_char
            }
            _ => { return 0 as *const libc::c_char }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn colour_fromstring(mut s: *const libc::c_char)
 -> libc::c_int {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    if *s as libc::c_int == 35 && strlen(s) == 7i32 as libc::c_ulong {
        cp = s.offset(1isize);
        while 0 !=
                  *(*__ctype_b_loc()).offset(*cp as u_char as libc::c_int as
                                                 isize) as libc::c_int &
                      _ISxdigit as libc::c_int as libc::c_ushort as
                          libc::c_int {
            cp = cp.offset(1isize)
        }
        if *cp as libc::c_int != 0 {
            return 1i32.wrapping_neg()
        } else {
            n =
                sscanf(s.offset(1isize),
                       b"%2hhx%2hhx%2hhx\x00" as *const u8 as
                           *const libc::c_char, &mut r as *mut u_char,
                       &mut g as *mut u_char, &mut b as *mut u_char);
            if n != 3i32 {
                return 1i32.wrapping_neg()
            } else { return colour_join_rgb(r, g, b) }
        }
    } else if strncasecmp(s,
                          b"colour\x00" as *const u8 as *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 7]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong))
                  == 0i32 {
        n =
            strtonum(s.offset(::std::mem::size_of::<[libc::c_char; 7]>() as
                                  libc::c_ulong as isize).offset(-1isize),
                     0i32 as libc::c_longlong, 255i32 as libc::c_longlong,
                     &mut errstr as *mut *const libc::c_char) as libc::c_int;
        if errstr != 0 as *mut libc::c_void as *const libc::c_char {
            return 1i32.wrapping_neg()
        } else { return n | 16777216i32 }
    } else if strcasecmp(s, b"black\x00" as *const u8 as *const libc::c_char)
                  == 0i32 ||
                  strcmp(s, b"0\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 0i32
    } else if strcasecmp(s, b"red\x00" as *const u8 as *const libc::c_char) ==
                  0i32 ||
                  strcmp(s, b"1\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 1i32
    } else if strcasecmp(s, b"green\x00" as *const u8 as *const libc::c_char)
                  == 0i32 ||
                  strcmp(s, b"2\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 2i32
    } else if strcasecmp(s, b"yellow\x00" as *const u8 as *const libc::c_char)
                  == 0i32 ||
                  strcmp(s, b"3\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 3i32
    } else if strcasecmp(s, b"blue\x00" as *const u8 as *const libc::c_char)
                  == 0i32 ||
                  strcmp(s, b"4\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 4i32
    } else if strcasecmp(s,
                         b"magenta\x00" as *const u8 as *const libc::c_char)
                  == 0i32 ||
                  strcmp(s, b"5\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 5i32
    } else if strcasecmp(s, b"cyan\x00" as *const u8 as *const libc::c_char)
                  == 0i32 ||
                  strcmp(s, b"6\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 6i32
    } else if strcasecmp(s, b"white\x00" as *const u8 as *const libc::c_char)
                  == 0i32 ||
                  strcmp(s, b"7\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 7i32
    } else if strcasecmp(s,
                         b"default\x00" as *const u8 as *const libc::c_char)
                  == 0i32 ||
                  strcmp(s, b"8\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 8i32
    } else if strcasecmp(s,
                         b"brightblack\x00" as *const u8 as
                             *const libc::c_char) == 0i32 ||
                  strcmp(s, b"90\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 90i32
    } else if strcasecmp(s,
                         b"brightred\x00" as *const u8 as *const libc::c_char)
                  == 0i32 ||
                  strcmp(s, b"91\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 91i32
    } else if strcasecmp(s,
                         b"brightgreen\x00" as *const u8 as
                             *const libc::c_char) == 0i32 ||
                  strcmp(s, b"92\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 92i32
    } else if strcasecmp(s,
                         b"brightyellow\x00" as *const u8 as
                             *const libc::c_char) == 0i32 ||
                  strcmp(s, b"93\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 93i32
    } else if strcasecmp(s,
                         b"brightblue\x00" as *const u8 as
                             *const libc::c_char) == 0i32 ||
                  strcmp(s, b"94\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 94i32
    } else if strcasecmp(s,
                         b"brightmagenta\x00" as *const u8 as
                             *const libc::c_char) == 0i32 ||
                  strcmp(s, b"95\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 95i32
    } else if strcasecmp(s,
                         b"brightcyan\x00" as *const u8 as
                             *const libc::c_char) == 0i32 ||
                  strcmp(s, b"96\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 96i32
    } else if strcasecmp(s,
                         b"brightwhite\x00" as *const u8 as
                             *const libc::c_char) == 0i32 ||
                  strcmp(s, b"97\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        return 97i32
    } else { return 1i32.wrapping_neg() };
}
#[no_mangle]
pub unsafe extern "C" fn colour_256to16(mut c: u_char) -> u_char {
    static mut table: [u_char; 256] =
        unsafe {
            [0i32 as u_char, 1i32 as u_char, 2i32 as u_char, 3i32 as u_char,
             4i32 as u_char, 5i32 as u_char, 6i32 as u_char, 7i32 as u_char,
             8i32 as u_char, 9i32 as u_char, 10i32 as u_char, 11i32 as u_char,
             12i32 as u_char, 13i32 as u_char, 14i32 as u_char,
             15i32 as u_char, 0i32 as u_char, 4i32 as u_char, 4i32 as u_char,
             4i32 as u_char, 12i32 as u_char, 12i32 as u_char, 2i32 as u_char,
             6i32 as u_char, 4i32 as u_char, 4i32 as u_char, 12i32 as u_char,
             12i32 as u_char, 2i32 as u_char, 2i32 as u_char, 6i32 as u_char,
             4i32 as u_char, 12i32 as u_char, 12i32 as u_char, 2i32 as u_char,
             2i32 as u_char, 2i32 as u_char, 6i32 as u_char, 12i32 as u_char,
             12i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             10i32 as u_char, 10i32 as u_char, 14i32 as u_char,
             12i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             10i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             14i32 as u_char, 1i32 as u_char, 5i32 as u_char, 4i32 as u_char,
             4i32 as u_char, 12i32 as u_char, 12i32 as u_char, 3i32 as u_char,
             8i32 as u_char, 4i32 as u_char, 4i32 as u_char, 12i32 as u_char,
             12i32 as u_char, 2i32 as u_char, 2i32 as u_char, 6i32 as u_char,
             4i32 as u_char, 12i32 as u_char, 12i32 as u_char, 2i32 as u_char,
             2i32 as u_char, 2i32 as u_char, 6i32 as u_char, 12i32 as u_char,
             12i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             10i32 as u_char, 10i32 as u_char, 14i32 as u_char,
             12i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             10i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             14i32 as u_char, 1i32 as u_char, 1i32 as u_char, 5i32 as u_char,
             4i32 as u_char, 12i32 as u_char, 12i32 as u_char, 1i32 as u_char,
             1i32 as u_char, 5i32 as u_char, 4i32 as u_char, 12i32 as u_char,
             12i32 as u_char, 3i32 as u_char, 3i32 as u_char, 8i32 as u_char,
             4i32 as u_char, 12i32 as u_char, 12i32 as u_char, 2i32 as u_char,
             2i32 as u_char, 2i32 as u_char, 6i32 as u_char, 12i32 as u_char,
             12i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             10i32 as u_char, 10i32 as u_char, 14i32 as u_char,
             12i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             10i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             14i32 as u_char, 1i32 as u_char, 1i32 as u_char, 1i32 as u_char,
             5i32 as u_char, 12i32 as u_char, 12i32 as u_char, 1i32 as u_char,
             1i32 as u_char, 1i32 as u_char, 5i32 as u_char, 12i32 as u_char,
             12i32 as u_char, 1i32 as u_char, 1i32 as u_char, 1i32 as u_char,
             5i32 as u_char, 12i32 as u_char, 12i32 as u_char, 3i32 as u_char,
             3i32 as u_char, 3i32 as u_char, 7i32 as u_char, 12i32 as u_char,
             12i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             10i32 as u_char, 10i32 as u_char, 14i32 as u_char,
             12i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             10i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             14i32 as u_char, 9i32 as u_char, 9i32 as u_char, 9i32 as u_char,
             9i32 as u_char, 13i32 as u_char, 12i32 as u_char, 9i32 as u_char,
             9i32 as u_char, 9i32 as u_char, 9i32 as u_char, 13i32 as u_char,
             12i32 as u_char, 9i32 as u_char, 9i32 as u_char, 9i32 as u_char,
             9i32 as u_char, 13i32 as u_char, 12i32 as u_char, 9i32 as u_char,
             9i32 as u_char, 9i32 as u_char, 9i32 as u_char, 13i32 as u_char,
             12i32 as u_char, 11i32 as u_char, 11i32 as u_char,
             11i32 as u_char, 11i32 as u_char, 7i32 as u_char,
             12i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             10i32 as u_char, 10i32 as u_char, 10i32 as u_char,
             14i32 as u_char, 9i32 as u_char, 9i32 as u_char, 9i32 as u_char,
             9i32 as u_char, 9i32 as u_char, 13i32 as u_char, 9i32 as u_char,
             9i32 as u_char, 9i32 as u_char, 9i32 as u_char, 9i32 as u_char,
             13i32 as u_char, 9i32 as u_char, 9i32 as u_char, 9i32 as u_char,
             9i32 as u_char, 9i32 as u_char, 13i32 as u_char, 9i32 as u_char,
             9i32 as u_char, 9i32 as u_char, 9i32 as u_char, 9i32 as u_char,
             13i32 as u_char, 9i32 as u_char, 9i32 as u_char, 9i32 as u_char,
             9i32 as u_char, 9i32 as u_char, 13i32 as u_char, 11i32 as u_char,
             11i32 as u_char, 11i32 as u_char, 11i32 as u_char,
             11i32 as u_char, 15i32 as u_char, 0i32 as u_char, 0i32 as u_char,
             0i32 as u_char, 0i32 as u_char, 0i32 as u_char, 0i32 as u_char,
             8i32 as u_char, 8i32 as u_char, 8i32 as u_char, 8i32 as u_char,
             8i32 as u_char, 8i32 as u_char, 7i32 as u_char, 7i32 as u_char,
             7i32 as u_char, 7i32 as u_char, 7i32 as u_char, 7i32 as u_char,
             15i32 as u_char, 15i32 as u_char, 15i32 as u_char,
             15i32 as u_char, 15i32 as u_char, 15i32 as u_char]
        };
    return table[c as usize];
}

