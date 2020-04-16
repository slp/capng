#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type capng_act_t = u32;
pub type capng_type_t = u32;
pub type capng_select_t = u32;
pub type capng_results_t = i32;
pub type capng_print_t = u32;
pub type capng_flags_t = u32;

#[link(name = "cap-ng")]
extern "C" {
    pub fn capng_clear(set: capng_select_t);
    pub fn capng_fill(set: capng_select_t);
    pub fn capng_setpid(pid: ::std::os::raw::c_int);
    pub fn capng_get_caps_process() -> ::std::os::raw::c_int;
    pub fn capng_update(
        action: capng_act_t,
        type_: capng_type_t,
        capability: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn capng_apply(set: capng_select_t) -> ::std::os::raw::c_int;
    pub fn capng_lock() -> ::std::os::raw::c_int;
    pub fn capng_change_id(
        uid: ::std::os::raw::c_int,
        gid: ::std::os::raw::c_int,
        flag: capng_flags_t,
    ) -> ::std::os::raw::c_int;
    pub fn capng_get_caps_fd(fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn capng_apply_caps_fd(fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn capng_have_capabilities(set: capng_select_t) -> capng_results_t;
    pub fn capng_have_permitted_capabilities() -> capng_results_t;
    pub fn capng_have_capability(
        which: capng_type_t,
        capability: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn capng_print_caps_numeric(
        where_: capng_print_t,
        set: capng_select_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn capng_print_caps_text(
        where_: capng_print_t,
        which: capng_type_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn capng_name_to_capability(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn capng_capability_to_name(
        capability: ::std::os::raw::c_uint,
    ) -> *const ::std::os::raw::c_char;
    pub fn capng_save_state() -> *mut ::std::os::raw::c_void;
    pub fn capng_restore_state(state: *mut *mut ::std::os::raw::c_void);
}
