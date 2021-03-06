use std::os::raw::*;

pub enum CWebView {} // opaque type, only used in ffi pointers

type ErasedExternalInvokeFn = extern "system" fn(webview: *mut CWebView, arg: *const c_char);
type ErasedDispatchFn = extern "system" fn(webview: *mut CWebView, arg: *mut c_void);

extern {
	pub fn wrapper_webview_free(this: *mut CWebView);
	pub fn wrapper_webview_new(title: *const c_char, url: *const c_char, width: c_int, height: c_int, resizable: c_int, debug: c_int, external_invoke_cb: Option<ErasedExternalInvokeFn>, userdata: *mut c_void) -> *mut CWebView;
	pub fn webview_loop(this: *mut CWebView, blocking: c_int) -> c_int;
	pub fn webview_terminate(this: *mut CWebView);
	pub fn webview_exit(this: *mut CWebView);
	pub fn wrapper_webview_get_userdata(this: *mut CWebView) -> *mut c_void;
	pub fn webview_dispatch(this: *mut CWebView, f: Option<ErasedDispatchFn>, arg: *mut c_void);
	pub fn webview_eval(this: *mut CWebView, js: *const c_char) -> c_int;
	pub fn webview_inject_css(this: *mut CWebView, css: *const c_char) -> c_int;
	pub fn webview_set_fullscreen(this: *mut CWebView, fullscreen: c_int);
}