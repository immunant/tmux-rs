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
    pub type screen_write_collect_line;
    pub type screen_write_collect_item;
    pub type tmuxpeer;
    pub type mode_tree_item;
    pub type tty_code;
    pub type tmuxproc;
    pub type event_base;
    pub type paste_buffer;
    pub type mode_tree_data;
    pub type args_entry;
    pub type options;
    pub type format_tree;
    pub type hooks;
    pub type _IO_FILE_plus;
    pub type format_job_tree;
    pub type evbuffer;
    pub type input_ctx;
    pub type environ;
    pub type bufferevent_ops;
    pub type screen_titles;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t) -> ();
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn vis(_: *mut libc::c_char, _: libc::c_int, _: libc::c_int,
           _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn memmem(_: *const libc::c_void, _: size_t, _: *const libc::c_void,
              _: size_t) -> *mut libc::c_void;
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
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t)
     -> *mut libc::c_void;
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
    fn paste_buffer_name(_: *mut paste_buffer) -> *const libc::c_char;
    #[no_mangle]
    fn paste_buffer_order(_: *mut paste_buffer) -> u_int;
    #[no_mangle]
    fn paste_buffer_data(_: *mut paste_buffer, _: *mut size_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn paste_walk(_: *mut paste_buffer) -> *mut paste_buffer;
    #[no_mangle]
    fn paste_get_top(_: *mut *const libc::c_char) -> *mut paste_buffer;
    #[no_mangle]
    fn paste_get_name(_: *const libc::c_char) -> *mut paste_buffer;
    #[no_mangle]
    fn paste_free(_: *mut paste_buffer) -> ();
    #[no_mangle]
    fn format_true(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn format_create(_: *mut client, _: *mut cmdq_item, _: libc::c_int,
                     _: libc::c_int) -> *mut format_tree;
    #[no_mangle]
    fn format_free(_: *mut format_tree) -> ();
    #[no_mangle]
    fn format_expand(_: *mut format_tree, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn format_defaults_paste_buffer(_: *mut format_tree, _: *mut paste_buffer)
     -> ();
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
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
    fn screen_write_puts(_: *mut screen_write_ctx, _: *const grid_cell,
                         _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn window_pane_reset_mode(_: *mut window_pane) -> ();
    #[no_mangle]
    fn mode_tree_get_current(_: *mut mode_tree_data) -> *mut libc::c_void;
    #[no_mangle]
    fn mode_tree_each_tagged(_: *mut mode_tree_data, _: mode_tree_each_cb,
                             _: *mut client, _: key_code, _: libc::c_int)
     -> ();
    #[no_mangle]
    fn mode_tree_down(_: *mut mode_tree_data, _: libc::c_int) -> ();
    #[no_mangle]
    fn mode_tree_start(_: *mut window_pane, _: *mut args,
                       _: mode_tree_build_cb, _: mode_tree_draw_cb,
                       _: mode_tree_search_cb, _: *mut libc::c_void,
                       _: *mut *const libc::c_char, _: u_int,
                       _: *mut *mut screen) -> *mut mode_tree_data;
    #[no_mangle]
    fn mode_tree_zoom(_: *mut mode_tree_data, _: *mut args) -> ();
    #[no_mangle]
    fn mode_tree_build(_: *mut mode_tree_data) -> ();
    #[no_mangle]
    fn mode_tree_free(_: *mut mode_tree_data) -> ();
    #[no_mangle]
    fn mode_tree_resize(_: *mut mode_tree_data, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn mode_tree_add(_: *mut mode_tree_data, _: *mut mode_tree_item,
                     _: *mut libc::c_void, _: uint64_t,
                     _: *const libc::c_char, _: *const libc::c_char,
                     _: libc::c_int) -> *mut mode_tree_item;
    #[no_mangle]
    fn mode_tree_draw(_: *mut mode_tree_data) -> ();
    #[no_mangle]
    fn mode_tree_key(_: *mut mode_tree_data, _: *mut client, _: *mut key_code,
                     _: *mut mouse_event, _: *mut u_int, _: *mut u_int)
     -> libc::c_int;
    #[no_mangle]
    fn mode_tree_run_command(_: *mut client, _: *mut cmd_find_state,
                             _: *const libc::c_char, _: *const libc::c_char)
     -> ();
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
pub struct unnamed {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
pub struct unnamed_0 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub type key_code = libc::c_ulonglong;
pub type __pid_t = libc::c_int;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const LINE_SEL_NONE: unnamed_36 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_16,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const TTY_VT220: unnamed_26 = 3;
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_5,
    pub entry: unnamed_18,
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
    pub entry: unnamed_11,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_write_ctx {
    pub wp: *mut window_pane,
    pub s: *mut screen,
    pub item: *mut screen_write_collect_item,
    pub list: *mut screen_write_collect_line,
    pub scrolled: u_int,
    pub bg: u_int,
    pub cells: u_int,
    pub written: u_int,
    pub skipped: u_int,
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
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub ev_io_next: unnamed_21,
    pub ev_timeout: timeval,
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
pub struct event {
    pub ev_active_next: unnamed_13,
    pub ev_next: unnamed_12,
    pub ev_timeout_pos: unnamed_7,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_38,
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
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_7 {
    ev_next_with_common_timeout: unnamed_1,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_29,
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
    pub entry: unnamed_10,
}
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_24,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
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
    pub entry: unnamed_14,
    pub tree_entry: unnamed_3,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub type uint8_t = libc::c_uchar;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub type mode_tree_search_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: *const libc::c_char) -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
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
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const PROMPT_COMMAND: unnamed_15 = 1;
pub const LINE_SEL_RIGHT_LEFT: unnamed_36 = 2;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
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
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const WINDOW_BUFFER_BY_TIME: window_buffer_sort_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type unnamed_15 = libc::c_uint;
pub type cmd_retval = libc::c_int;
pub type uint32_t = libc::c_uint;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
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
    pub message_log: unnamed_37,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_15,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_20,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub struct in6_addr {
    pub __in6_u: unnamed_25,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const TTY_VT100: unnamed_26 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type mode_tree_draw_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: *mut screen_write_ctx, _: u_int, _: u_int)
               -> ()>;
pub type size_t = libc::c_ulong;
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
    pub term_type: unnamed_26,
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
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
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
pub struct unnamed_20 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_19,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_28,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub ev_signal_next: unnamed,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const JOB_CLOSED: unnamed_29 = 2;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_33,
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
    pub gentry: unnamed_31,
    pub entry: unnamed_23,
}
pub type mode_tree_each_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: *mut client, _: key_code) -> ()>;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
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
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub type u_short = __u_short;
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
    pub entry: unnamed_32,
}
pub type u_int = __u_int;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub type uint64_t = libc::c_ulong;
pub const LINE_SEL_LEFT_RIGHT: unnamed_36 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const TTY_UNKNOWN: unnamed_26 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_25 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type uint16_t = libc::c_ushort;
pub type speed_t = libc::c_uint;
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_buffer_modedata {
    pub data: *mut mode_tree_data,
    pub command: *mut libc::c_char,
    pub format: *mut libc::c_char,
    pub item_list: *mut *mut window_buffer_itemdata,
    pub item_size: u_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_27,
}
pub const TTY_VT320: unnamed_26 = 4;
pub type tcflag_t = libc::c_uint;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const PROMPT_ENTRY: unnamed_15 = 0;
pub type bitstr_t = libc::c_uchar;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const JOB_DEAD: unnamed_29 = 1;
pub type cmd_find_type = libc::c_uint;
pub type unnamed_26 = libc::c_uint;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type options_table_scope = libc::c_uint;
pub const WINDOW_BUFFER_BY_NAME: window_buffer_sort_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
    pub entry: unnamed_0,
    pub wentry: unnamed_35,
    pub sentry: unnamed_34,
}
pub type mode_tree_build_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: u_int,
                                _: *mut uint64_t, _: *const libc::c_char)
               -> ()>;
pub type pid_t = __pid_t;
pub const TTY_VT420: unnamed_26 = 5;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const TTY_VT101: unnamed_26 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_28 {
    offset: u_int,
    data: unnamed_9,
}
pub type unnamed_29 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
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
pub struct unnamed_31 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const WINDOW_BUFFER_BY_SIZE: window_buffer_sort_type = 2;
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
pub struct unnamed_32 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const TTY_VT102: unnamed_26 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_buffer_itemdata {
    pub name: *const libc::c_char,
    pub order: u_int,
    pub size: size_t,
}
pub type options_table_type = libc::c_uint;
pub type window_buffer_sort_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_4,
}
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type unnamed_36 = libc::c_uint;
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const JOB_RUNNING: unnamed_29 = 0;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
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
    pub alerts_entry: unnamed_17,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_8,
    pub entry: unnamed_30,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
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
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_38 {
    ev_io: unnamed_6,
    ev_signal: unnamed_22,
}
unsafe extern "C" fn window_buffer_init(mut wp: *mut window_pane,
                                        mut fs: *mut cmd_find_state,
                                        mut args: *mut args) -> *mut screen {
    let mut data: *mut window_buffer_modedata =
        0 as *mut window_buffer_modedata;
    let mut s: *mut screen = 0 as *mut screen;
    data =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<window_buffer_modedata>() as
                    libc::c_ulong) as *mut window_buffer_modedata;
    (*wp).modedata = data as *mut libc::c_void;
    if args == 0 as *mut libc::c_void as *mut args ||
           0 == args_has(args, 70 as u_char) {
        (*data).format =
            xstrdup(b"#{buffer_size} bytes (#{t:buffer_created})\x00" as
                        *const u8 as *const libc::c_char)
    } else { (*data).format = xstrdup(args_get(args, 70 as u_char)) }
    if args == 0 as *mut libc::c_void as *mut args || (*args).argc == 0i32 {
        (*data).command =
            xstrdup(b"paste-buffer -b \'%%\'\x00" as *const u8 as
                        *const libc::c_char)
    } else { (*data).command = xstrdup(*(*args).argv.offset(0isize)) }
    (*data).data =
        mode_tree_start(wp, args, Some(window_buffer_build),
                        Some(window_buffer_draw), Some(window_buffer_search),
                        data as *mut libc::c_void,
                        window_buffer_sort_list.as_mut_ptr(),
                        (::std::mem::size_of::<[*const libc::c_char; 3]>() as
                             libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                             as libc::c_ulong)
                            as u_int, &mut s as *mut *mut screen);
    mode_tree_zoom((*data).data, args);
    mode_tree_build((*data).data);
    mode_tree_draw((*data).data);
    return s;
}
static mut window_buffer_sort_list: [*const libc::c_char; 3] =
    unsafe {
        [b"time\x00" as *const u8 as *const libc::c_char,
         b"name\x00" as *const u8 as *const libc::c_char,
         b"size\x00" as *const u8 as *const libc::c_char]
    };
unsafe extern "C" fn window_buffer_search(mut modedata: *mut libc::c_void,
                                          mut itemdata: *mut libc::c_void,
                                          mut ss: *const libc::c_char)
 -> libc::c_int {
    let mut item: *mut window_buffer_itemdata =
        itemdata as *mut window_buffer_itemdata;
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut bufdata: *const libc::c_char = 0 as *const libc::c_char;
    let mut bufsize: size_t = 0;
    pb = paste_get_name((*item).name);
    if pb == 0 as *mut libc::c_void as *mut paste_buffer {
        return 0i32
    } else if strstr((*item).name, ss) !=
                  0 as *mut libc::c_void as *mut libc::c_char {
        return 1i32
    } else {
        bufdata = paste_buffer_data(pb, &mut bufsize as *mut size_t);
        return (memmem(bufdata as *const libc::c_void, bufsize,
                       ss as *const libc::c_void, strlen(ss)) !=
                    0 as *mut libc::c_void) as libc::c_int
    };
}
unsafe extern "C" fn window_buffer_draw(mut modedata: *mut libc::c_void,
                                        mut itemdata: *mut libc::c_void,
                                        mut ctx: *mut screen_write_ctx,
                                        mut sx: u_int, mut sy: u_int) -> () {
    let mut item: *mut window_buffer_itemdata =
        itemdata as *mut window_buffer_itemdata;
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut pdata: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut psize: size_t = 0;
    let mut at: size_t = 0;
    let mut i: u_int = 0;
    let mut cx: u_int = (*(*ctx).s).cx;
    let mut cy: u_int = (*(*ctx).s).cy;
    pb = paste_get_name((*item).name);
    if pb == 0 as *mut libc::c_void as *mut paste_buffer {
        return
    } else {
        end = paste_buffer_data(pb, &mut psize as *mut size_t);
        pdata = end;
        i = 0i32 as u_int;
        while i < sy {
            at = 0i32 as size_t;
            while end != pdata.offset(psize as isize) &&
                      *end as libc::c_int != 10 {
                if (::std::mem::size_of::<[libc::c_char; 1024]>() as
                        libc::c_ulong).wrapping_sub(at) >
                       5i32 as libc::c_ulong {
                    cp =
                        vis(line.as_mut_ptr().offset(at as isize),
                            *end as libc::c_int, 8i32 | 1i32, 0i32);
                    at =
                        line.as_mut_ptr().offset_to(cp).expect("bad offset_to")
                            as libc::c_long as size_t
                }
                end = end.offset(1isize)
            }
            if at > sx as libc::c_ulong { at = sx as size_t }
            line[at as usize] = 0 as libc::c_char;
            if *line.as_mut_ptr() as libc::c_int != 0 {
                screen_write_cursormove(ctx, cx, cy.wrapping_add(i));
                screen_write_puts(ctx, &grid_default_cell as *const grid_cell,
                                  b"%s\x00" as *const u8 as
                                      *const libc::c_char, line.as_mut_ptr());
            }
            if end == pdata.offset(psize as isize) { break ; }
            end = end.offset(1isize);
            i = i.wrapping_add(1)
        }
        return;
    };
}
unsafe extern "C" fn window_buffer_build(mut modedata: *mut libc::c_void,
                                         mut sort_type: u_int,
                                         mut tag: *mut uint64_t,
                                         mut filter: *const libc::c_char)
 -> () {
    let mut current_block: u64;
    let mut data: *mut window_buffer_modedata =
        modedata as *mut window_buffer_modedata;
    let mut item: *mut window_buffer_itemdata =
        0 as *mut window_buffer_itemdata;
    let mut i: u_int = 0;
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    i = 0i32 as u_int;
    while i < (*data).item_size {
        window_buffer_free_item(*(*data).item_list.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free((*data).item_list as *mut libc::c_void);
    (*data).item_list = 0 as *mut *mut window_buffer_itemdata;
    (*data).item_size = 0i32 as u_int;
    pb = 0 as *mut paste_buffer;
    loop  {
        pb = paste_walk(pb);
        if !(pb != 0 as *mut libc::c_void as *mut paste_buffer) { break ; }
        item = window_buffer_add_item(data);
        (*item).name = xstrdup(paste_buffer_name(pb));
        paste_buffer_data(pb, &mut (*item).size as *mut size_t);
        (*item).order = paste_buffer_order(pb)
    }
    match sort_type {
        1 => {
            qsort((*data).item_list as *mut libc::c_void,
                  (*data).item_size as size_t,
                  ::std::mem::size_of::<*mut window_buffer_itemdata>() as
                      libc::c_ulong, Some(window_buffer_cmp_name));
        }
        0 => {
            qsort((*data).item_list as *mut libc::c_void,
                  (*data).item_size as size_t,
                  ::std::mem::size_of::<*mut window_buffer_itemdata>() as
                      libc::c_ulong, Some(window_buffer_cmp_time));
        }
        2 => {
            qsort((*data).item_list as *mut libc::c_void,
                  (*data).item_size as size_t,
                  ::std::mem::size_of::<*mut window_buffer_itemdata>() as
                      libc::c_ulong, Some(window_buffer_cmp_size));
        }
        _ => { }
    }
    i = 0i32 as u_int;
    while i < (*data).item_size {
        item = *(*data).item_list.offset(i as isize);
        pb = paste_get_name((*item).name);
        if !(pb == 0 as *mut libc::c_void as *mut paste_buffer) {
            ft =
                format_create(0 as *mut client, 0 as *mut cmdq_item, 0i32,
                              0i32);
            format_defaults_paste_buffer(ft, pb);
            if filter != 0 as *mut libc::c_void as *const libc::c_char {
                cp = format_expand(ft, filter);
                if 0 == format_true(cp) {
                    free(cp as *mut libc::c_void);
                    format_free(ft);
                    current_block = 14523784380283086299;
                } else {
                    free(cp as *mut libc::c_void);
                    current_block = 13109137661213826276;
                }
            } else { current_block = 13109137661213826276; }
            match current_block {
                14523784380283086299 => { }
                _ => {
                    text = format_expand(ft, (*data).format);
                    mode_tree_add((*data).data, 0 as *mut mode_tree_item,
                                  item as *mut libc::c_void,
                                  (*item).order as uint64_t, (*item).name,
                                  text, 1i32.wrapping_neg());
                    free(text as *mut libc::c_void);
                    format_free(ft);
                }
            }
        }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn window_buffer_cmp_size(mut a0: *const libc::c_void,
                                            mut b0: *const libc::c_void)
 -> libc::c_int {
    let mut a: *const *const window_buffer_itemdata =
        a0 as *const *const window_buffer_itemdata;
    let mut b: *const *const window_buffer_itemdata =
        b0 as *const *const window_buffer_itemdata;
    if (**a).size > (**b).size {
        return 1i32.wrapping_neg()
    } else if (**a).size < (**b).size {
        return 1i32
    } else { return strcmp((**a).name, (**b).name) };
}
unsafe extern "C" fn window_buffer_cmp_time(mut a0: *const libc::c_void,
                                            mut b0: *const libc::c_void)
 -> libc::c_int {
    let mut a: *const *const window_buffer_itemdata =
        a0 as *const *const window_buffer_itemdata;
    let mut b: *const *const window_buffer_itemdata =
        b0 as *const *const window_buffer_itemdata;
    if (**a).order > (**b).order {
        return 1i32.wrapping_neg()
    } else if (**a).order < (**b).order {
        return 1i32
    } else { return strcmp((**a).name, (**b).name) };
}
unsafe extern "C" fn window_buffer_cmp_name(mut a0: *const libc::c_void,
                                            mut b0: *const libc::c_void)
 -> libc::c_int {
    let mut a: *const *const window_buffer_itemdata =
        a0 as *const *const window_buffer_itemdata;
    let mut b: *const *const window_buffer_itemdata =
        b0 as *const *const window_buffer_itemdata;
    return strcmp((**a).name, (**b).name);
}
unsafe extern "C" fn window_buffer_add_item(mut data:
                                                *mut window_buffer_modedata)
 -> *mut window_buffer_itemdata {
    let mut item: *mut window_buffer_itemdata =
        0 as *mut window_buffer_itemdata;
    (*data).item_list =
        xreallocarray((*data).item_list as *mut libc::c_void,
                      (*data).item_size.wrapping_add(1i32 as libc::c_uint) as
                          size_t,
                      ::std::mem::size_of::<*mut window_buffer_itemdata>() as
                          libc::c_ulong) as *mut *mut window_buffer_itemdata;
    let fresh0 = (*data).item_size;
    (*data).item_size = (*data).item_size.wrapping_add(1);
    let ref mut fresh1 = *(*data).item_list.offset(fresh0 as isize);
    *fresh1 =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<window_buffer_itemdata>() as
                    libc::c_ulong) as *mut window_buffer_itemdata;
    item = *fresh1;
    return item;
}
unsafe extern "C" fn window_buffer_free_item(mut item:
                                                 *mut window_buffer_itemdata)
 -> () {
    free((*item).name as *mut libc::c_void);
    free(item as *mut libc::c_void);
}
unsafe extern "C" fn window_buffer_free(mut wp: *mut window_pane) -> () {
    let mut data: *mut window_buffer_modedata =
        (*wp).modedata as *mut window_buffer_modedata;
    let mut i: u_int = 0;
    if data == 0 as *mut libc::c_void as *mut window_buffer_modedata {
        return
    } else {
        mode_tree_free((*data).data);
        i = 0i32 as u_int;
        while i < (*data).item_size {
            window_buffer_free_item(*(*data).item_list.offset(i as isize));
            i = i.wrapping_add(1)
        }
        free((*data).item_list as *mut libc::c_void);
        free((*data).format as *mut libc::c_void);
        free((*data).command as *mut libc::c_void);
        free(data as *mut libc::c_void);
        return;
    };
}
unsafe extern "C" fn window_buffer_resize(mut wp: *mut window_pane,
                                          mut sx: u_int, mut sy: u_int)
 -> () {
    let mut data: *mut window_buffer_modedata =
        (*wp).modedata as *mut window_buffer_modedata;
    mode_tree_resize((*data).data, sx, sy);
}
unsafe extern "C" fn window_buffer_key(mut wp: *mut window_pane,
                                       mut c: *mut client,
                                       mut s: *mut session, mut key: key_code,
                                       mut m: *mut mouse_event) -> () {
    let mut data: *mut window_buffer_modedata =
        (*wp).modedata as *mut window_buffer_modedata;
    let mut mtd: *mut mode_tree_data = (*data).data;
    let mut item: *mut window_buffer_itemdata =
        0 as *mut window_buffer_itemdata;
    let mut finished: libc::c_int = 0;
    finished =
        mode_tree_key(mtd, c, &mut key as *mut key_code, m, 0 as *mut u_int,
                      0 as *mut u_int);
    match key {
        100 => {
            item = mode_tree_get_current(mtd) as *mut window_buffer_itemdata;
            window_buffer_do_delete(data as *mut libc::c_void,
                                    item as *mut libc::c_void, c, key);
            mode_tree_build(mtd);
        }
        68 => {
            mode_tree_each_tagged(mtd, Some(window_buffer_do_delete), c, key,
                                  0i32);
            mode_tree_build(mtd);
        }
        80 => {
            mode_tree_each_tagged(mtd, Some(window_buffer_do_paste), c, key,
                                  0i32);
            finished = 1i32
        }
        112 | 13 => {
            item = mode_tree_get_current(mtd) as *mut window_buffer_itemdata;
            window_buffer_do_paste(data as *mut libc::c_void,
                                   item as *mut libc::c_void, c, key);
            finished = 1i32
        }
        _ => { }
    }
    if 0 != finished ||
           paste_get_top(0 as *mut *const libc::c_char) ==
               0 as *mut libc::c_void as *mut paste_buffer {
        window_pane_reset_mode(wp);
    } else { mode_tree_draw(mtd); (*wp).flags |= 1i32 };
}
unsafe extern "C" fn window_buffer_do_paste(mut modedata: *mut libc::c_void,
                                            mut itemdata: *mut libc::c_void,
                                            mut c: *mut client,
                                            mut key: key_code) -> () {
    let mut data: *mut window_buffer_modedata =
        modedata as *mut window_buffer_modedata;
    let mut item: *mut window_buffer_itemdata =
        itemdata as *mut window_buffer_itemdata;
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    pb = paste_get_name((*item).name);
    if pb != 0 as *mut libc::c_void as *mut paste_buffer {
        mode_tree_run_command(c, 0 as *mut cmd_find_state, (*data).command,
                              (*item).name);
    };
}
unsafe extern "C" fn window_buffer_do_delete(mut modedata: *mut libc::c_void,
                                             mut itemdata: *mut libc::c_void,
                                             mut c: *mut client,
                                             mut key: key_code) -> () {
    let mut data: *mut window_buffer_modedata =
        modedata as *mut window_buffer_modedata;
    let mut item: *mut window_buffer_itemdata =
        itemdata as *mut window_buffer_itemdata;
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    if item ==
           mode_tree_get_current((*data).data) as *mut window_buffer_itemdata
       {
        mode_tree_down((*data).data, 0i32);
    }
    pb = paste_get_name((*item).name);
    if pb != 0 as *mut libc::c_void as *mut paste_buffer { paste_free(pb); };
}

