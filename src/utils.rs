use gio::ffi;
use glib::translate::*;

// Workaround to https://github.com/gtk-rs/gtk-rs-core/issues/1257
pub fn content_type_guess(
    filename: Option<impl AsRef<std::path::Path>>,
    data: Option<&[u8]>,
) -> (glib::GString, bool) {
    let data_size = data.map(|d| d.len()).unwrap_or(0);
    unsafe {
        let mut result_uncertain = std::mem::MaybeUninit::uninit();
        let ret = from_glib_full(ffi::g_content_type_guess(
            filename.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            data.to_glib_none().0,
            data_size as _,
            result_uncertain.as_mut_ptr(),
        ));
        (ret, from_glib(result_uncertain.assume_init()))
    }
}
