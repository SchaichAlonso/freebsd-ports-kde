--- src/vendor/openssl-sys/src/err.rs.orig	2018-09-16 20:29:19 UTC
+++ src/vendor/openssl-sys/src/err.rs
@@ -0,0 +1,50 @@
+use libc::*;
+
+pub const ERR_TXT_MALLOCED: c_int = 0x01;
+pub const ERR_TXT_STRING: c_int = 0x02;
+
+pub const ERR_LIB_PEM: c_int = 9;
+
+pub fn ERR_GET_LIB(l: c_ulong) -> c_int {
+    ((l >> 24) & 0x0FF) as c_int
+}
+
+pub fn ERR_GET_FUNC(l: c_ulong) -> c_int {
+    ((l >> 12) & 0xFFF) as c_int
+}
+
+pub fn ERR_GET_REASON(l: c_ulong) -> c_int {
+    (l & 0xFFF) as c_int
+}
+
+#[repr(C)]
+pub struct ERR_STRING_DATA {
+    pub error: c_ulong,
+    pub string: *const c_char,
+}
+
+extern "C" {
+    pub fn ERR_put_error(lib: c_int, func: c_int, reason: c_int, file: *const c_char, line: c_int);
+    pub fn ERR_set_error_data(data: *mut c_char, flags: c_int);
+
+    pub fn ERR_get_error() -> c_ulong;
+    pub fn ERR_get_error_line_data(
+        file: *mut *const c_char,
+        line: *mut c_int,
+        data: *mut *const c_char,
+        flags: *mut c_int,
+    ) -> c_ulong;
+    pub fn ERR_peek_last_error() -> c_ulong;
+    pub fn ERR_clear_error();
+    pub fn ERR_lib_error_string(err: c_ulong) -> *const c_char;
+    pub fn ERR_func_error_string(err: c_ulong) -> *const c_char;
+    pub fn ERR_reason_error_string(err: c_ulong) -> *const c_char;
+    #[cfg(ossl110)]
+    pub fn ERR_load_strings(lib: c_int, str: *mut ERR_STRING_DATA) -> c_int;
+    #[cfg(not(ossl110))]
+    pub fn ERR_load_strings(lib: c_int, str: *mut ERR_STRING_DATA);
+    #[cfg(not(ossl110))]
+    pub fn ERR_load_crypto_strings();
+
+    pub fn ERR_get_next_error_library() -> c_int;
+}
