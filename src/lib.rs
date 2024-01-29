use std::{
    ffi::{CStr, CString},
    os,
};

use pyo3::prelude::*;

unsafe extern "C" fn my_xerror_handler(
    _severity: os::raw::c_int,
    _message: gettextpo::po_message_t,
    _filename: *const os::raw::c_char,
    _lineno: usize,
    _column: usize,
    _multiline_p: os::raw::c_int,
    message_text: *const os::raw::c_char,
) {
    let message_text = unsafe { CStr::from_ptr(message_text) };
    eprintln!("xerror_handler: {}", message_text.to_str().unwrap());
}

unsafe extern "C" fn my_error_handler(
    _severity: os::raw::c_int,
    _message1: gettextpo::po_message_t,
    _filename1: *const os::raw::c_char,
    _lineno1: usize,
    _column1: usize,
    _multiline_p1: os::raw::c_int,
    message_text1: *const os::raw::c_char,
    _message2: gettextpo::po_message_t,
    _filename2: *const os::raw::c_char,
    _lineno2: usize,
    _column2: usize,
    _multiline_p2: os::raw::c_int,
    _message_text2: *const os::raw::c_char,
) {
    let message_text = unsafe { CStr::from_ptr(message_text1) };
    eprintln!("error_handler: {}", message_text.to_str().unwrap());
}

const XERROR_HANDLER: gettextpo::po_xerror_handler = gettextpo::po_xerror_handler {
    xerror: Some(my_xerror_handler),
    xerror2: Some(my_error_handler),
};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass(unsendable)]
struct PoFile(gettextpo::po_file_t);

#[pyclass(unsendable)]
struct PoMessage(gettextpo::po_message_t);

#[pyclass(unsendable)]
struct PoMessageIterator(gettextpo::po_message_iterator_t);

#[pyclass(unsendable)]
struct PoFilePos(gettextpo::po_filepos_t);

use po_file_read_v3 as po_file_read;
use po_file_write_v2 as po_file_write;
use po_message_check_format_v2 as po_message_check_format;

#[pyfunction]
fn po_file_create() -> PyResult<PoFile> {
    unsafe { Ok(PoFile(gettextpo::po_file_create())) }
}

#[pyfunction]
fn po_file_read_v3(filename: &str) -> PyResult<PoFile> {
    unsafe {
        let filename = CString::new(filename).unwrap();
        Ok(PoFile(gettextpo::po_file_read_v3(
            filename.as_ptr(),
            &XERROR_HANDLER,
        )))
    }
}

#[pyfunction]
fn po_file_write_v2(file: &PoFile, filename: &str) -> PyResult<()> {
    unsafe {
        let filename = CString::new(filename).unwrap();
        gettextpo::po_file_write_v2(file.0, filename.as_ptr(), &XERROR_HANDLER);
    }
    Ok(())
}

#[pyfunction]
fn po_file_free(file: &PoFile) -> PyResult<()> {
    unsafe {
        gettextpo::po_file_free(file.0);
    }
    Ok(())
}

#[pyfunction]
fn po_file_domains(file: &PoFile) -> PyResult<Vec<String>> {
    let domains = unsafe { gettextpo::po_file_domains(file.0) };
    let mut result = Vec::new();
    let mut i = 0;
    loop {
        let item = unsafe { *domains.offset(i) };
        if item.is_null() {
            break;
        }
        result.push(unsafe { CStr::from_ptr(item).to_str().unwrap().to_string() });
        i += 1;
    }
    Ok(result)
}

#[pyfunction]
fn po_file_domain_header(file: &PoFile, domain: &str) -> PyResult<String> {
    unsafe {
        let domain = CString::new(domain).unwrap();
        let header = gettextpo::po_file_domain_header(file.0, domain.as_ptr());
        Ok(CStr::from_ptr(header).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_header_field(header: &str, field: &str) -> PyResult<String> {
    unsafe {
        let header = CString::new(header).unwrap();
        let field = CString::new(field).unwrap();
        let value = gettextpo::po_header_field(header.as_ptr(), field.as_ptr());
        Ok(CStr::from_ptr(value).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_header_set_field(header: &str, field: &str, value: &str) -> PyResult<String> {
    unsafe {
        let header = CString::new(header).unwrap();
        let field = CString::new(field).unwrap();
        let value = CString::new(value).unwrap();
        let value = gettextpo::po_header_set_field(header.as_ptr(), field.as_ptr(), value.as_ptr());
        Ok(CStr::from_ptr(value).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_message_iterator(file: &PoFile, domain: &str) -> PyResult<PoMessageIterator> {
    unsafe {
        let domain = CString::new(domain).unwrap();
        Ok(PoMessageIterator(gettextpo::po_message_iterator(
            file.0,
            domain.as_ptr(),
        )))
    }
}

#[pyfunction]
fn po_message_iterator_free(iterator: &PoMessageIterator) -> PyResult<()> {
    unsafe {
        gettextpo::po_message_iterator_free(iterator.0);
    }
    Ok(())
}

#[pyfunction]
fn po_next_message(iterator: &PoMessageIterator) -> PyResult<PoMessage> {
    unsafe { Ok(PoMessage(gettextpo::po_next_message(iterator.0))) }
}

#[pyfunction]
fn po_message_insert(iterator: &PoMessageIterator, message: &PoMessage) -> PyResult<()> {
    unsafe {
        gettextpo::po_message_insert(iterator.0, message.0);
    }
    Ok(())
}

#[pyfunction]
fn po_message_create() -> PyResult<PoMessage> {
    unsafe { Ok(PoMessage(gettextpo::po_message_create())) }
}

#[pyfunction]
fn po_message_msgctxt(message: &PoMessage) -> PyResult<String> {
    unsafe {
        let msgctxt = gettextpo::po_message_msgctxt(message.0);
        Ok(CStr::from_ptr(msgctxt).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_message_set_msgctxt(message: &PoMessage, msgctxt: &str) -> PyResult<()> {
    unsafe {
        let msgctxt = CString::new(msgctxt).unwrap();
        gettextpo::po_message_set_msgctxt(message.0, msgctxt.as_ptr());
    }
    Ok(())
}

#[pyfunction]
fn po_message_msgid(message: &PoMessage) -> PyResult<String> {
    unsafe {
        let msgid = gettextpo::po_message_msgid(message.0);
        Ok(CStr::from_ptr(msgid).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_message_set_msgid(message: &PoMessage, msgid: &str) -> PyResult<()> {
    unsafe {
        let msgid = CString::new(msgid).unwrap();
        gettextpo::po_message_set_msgid(message.0, msgid.as_ptr());
    }
    Ok(())
}

#[pyfunction]
fn po_message_msgid_plural(message: &PoMessage) -> PyResult<String> {
    unsafe {
        let msgid_plural = gettextpo::po_message_msgid_plural(message.0);
        Ok(CStr::from_ptr(msgid_plural).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_message_set_msgid_plural(message: &PoMessage, msgid_plural: &str) -> PyResult<()> {
    unsafe {
        let msgid_plural = CString::new(msgid_plural).unwrap();
        gettextpo::po_message_set_msgid_plural(message.0, msgid_plural.as_ptr());
    }
    Ok(())
}

#[pyfunction]
fn po_message_msgstr(message: &PoMessage) -> PyResult<String> {
    unsafe {
        let msgstr = gettextpo::po_message_msgstr(message.0);
        Ok(CStr::from_ptr(msgstr).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_message_set_msgstr(message: &PoMessage, msgstr: &str) -> PyResult<()> {
    unsafe {
        let msgstr = CString::new(msgstr).unwrap();
        gettextpo::po_message_set_msgstr(message.0, msgstr.as_ptr());
    }
    Ok(())
}

#[pyfunction]
fn po_message_msgstr_plural(message: &PoMessage, index: i32) -> PyResult<String> {
    unsafe {
        let msgstr = gettextpo::po_message_msgstr_plural(message.0, index);
        Ok(CStr::from_ptr(msgstr).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_message_set_msgstr_plural(message: &PoMessage, index: i32, msgstr: &str) -> PyResult<()> {
    unsafe {
        let msgstr = CString::new(msgstr).unwrap();
        gettextpo::po_message_set_msgstr_plural(message.0, index, msgstr.as_ptr());
    }
    Ok(())
}

#[pyfunction]
fn po_message_comments(message: &PoMessage) -> PyResult<String> {
    unsafe {
        let comments = gettextpo::po_message_comments(message.0);
        Ok(CStr::from_ptr(comments).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_message_set_comments(message: &PoMessage, comments: &str) -> PyResult<()> {
    unsafe {
        let comments = CString::new(comments).unwrap();
        gettextpo::po_message_set_comments(message.0, comments.as_ptr());
    }
    Ok(())
}

#[pyfunction]
fn po_message_extracted_comments(message: &PoMessage) -> PyResult<String> {
    unsafe {
        let comments = gettextpo::po_message_extracted_comments(message.0);
        Ok(CStr::from_ptr(comments).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_message_set_extracted_comments(message: &PoMessage, comments: &str) -> PyResult<()> {
    unsafe {
        let comments = CString::new(comments).unwrap();
        gettextpo::po_message_set_extracted_comments(message.0, comments.as_ptr());
    }
    Ok(())
}

#[pyfunction]
fn po_message_filepos(message: &PoMessage, i: i32) -> PyResult<PoFilePos> {
    unsafe {
        let filepos = gettextpo::po_message_filepos(message.0, i);
        Ok(PoFilePos(filepos))
    }
}

#[pyfunction]
fn po_message_remove_filepos(message: &PoMessage, i: i32) -> PyResult<()> {
    unsafe {
        gettextpo::po_message_remove_filepos(message.0, i);
    }
    Ok(())
}

#[pyfunction]
fn po_message_add_filepos(message: &PoMessage, file: &str, start_line: usize) -> PyResult<()> {
    unsafe {
        let file = CString::new(file).unwrap();
        gettextpo::po_message_add_filepos(message.0, file.as_ptr(), start_line);
    }
    Ok(())
}

#[pyfunction]
fn po_message_prev_msgctxt(message: &PoMessage) -> PyResult<String> {
    unsafe {
        let msgctxt = gettextpo::po_message_prev_msgctxt(message.0);
        Ok(CStr::from_ptr(msgctxt).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_message_set_prev_msgctxt(message: &PoMessage, prev_msgctxt: &str) -> PyResult<()> {
    unsafe {
        let prev_msgctxt = CString::new(prev_msgctxt).unwrap();
        gettextpo::po_message_set_prev_msgctxt(message.0, prev_msgctxt.as_ptr());
    }
    Ok(())
}

#[pyfunction]
fn po_message_prev_msgid(message: &PoMessage) -> PyResult<String> {
    unsafe {
        let prev_msgid = gettextpo::po_message_prev_msgid(message.0);
        Ok(CStr::from_ptr(prev_msgid).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_message_set_prev_msgid(message: &PoMessage, prev_msgid: &str) -> PyResult<()> {
    unsafe {
        let prev_msgid = CString::new(prev_msgid).unwrap();
        gettextpo::po_message_set_prev_msgid(message.0, prev_msgid.as_ptr());
    }
    Ok(())
}

#[pyfunction]
fn po_message_prev_msgid_plural(message: &PoMessage) -> PyResult<String> {
    unsafe {
        let prev_msgid_plural = gettextpo::po_message_prev_msgid_plural(message.0);
        Ok(CStr::from_ptr(prev_msgid_plural)
            .to_str()
            .unwrap()
            .to_string())
    }
}

#[pyfunction]
fn po_message_set_prev_msgid_plural(message: &PoMessage, prev_msgid_plural: &str) -> PyResult<()> {
    unsafe {
        let prev_msgid_plural = CString::new(prev_msgid_plural).unwrap();
        gettextpo::po_message_set_prev_msgid_plural(message.0, prev_msgid_plural.as_ptr());
    }
    Ok(())
}

#[pyfunction]
fn po_message_is_obsolete(message: &PoMessage) -> PyResult<bool> {
    unsafe {
        let obsolete = gettextpo::po_message_is_obsolete(message.0);
        Ok(obsolete != 0)
    }
}

#[pyfunction]
fn po_message_set_obsolete(message: &PoMessage, obsolete: bool) -> PyResult<()> {
    unsafe {
        gettextpo::po_message_set_obsolete(message.0, obsolete as i32);
    }
    Ok(())
}

#[pyfunction]
fn po_message_is_fuzzy(message: &PoMessage) -> PyResult<bool> {
    unsafe {
        let fuzzy = gettextpo::po_message_is_fuzzy(message.0);
        Ok(fuzzy != 0)
    }
}

#[pyfunction]
fn po_message_set_fuzzy(message: &PoMessage, fuzzy: bool) -> PyResult<()> {
    unsafe {
        gettextpo::po_message_set_fuzzy(message.0, fuzzy as i32);
    }
    Ok(())
}

#[pyfunction]
fn po_message_is_format(message: &PoMessage, format_type: &str) -> PyResult<bool> {
    unsafe {
        let format_type = CString::new(format_type).unwrap();
        let format = gettextpo::po_message_is_format(message.0, format_type.as_ptr());
        Ok(format != 0)
    }
}

#[pyfunction]
fn po_message_set_format(message: &PoMessage, format_type: &str, value: bool) -> PyResult<()> {
    unsafe {
        let format_type = CString::new(format_type).unwrap();
        gettextpo::po_message_set_format(message.0, format_type.as_ptr(), value as i32);
    }
    Ok(())
}

#[pyfunction]
fn po_message_is_range(message: &PoMessage, min: i32, max: i32) -> PyResult<bool> {
    unsafe {
        let range = gettextpo::po_message_is_range(message.0, min as *mut _, max as *mut _);
        Ok(range != 0)
    }
}

#[pyfunction]
fn po_message_set_range(message: &PoMessage, min: i32, max: i32) -> PyResult<()> {
    unsafe {
        gettextpo::po_message_set_range(message.0, min, max);
    }
    Ok(())
}

#[pyfunction]
fn po_filepos_file(filepos: &PoFilePos) -> PyResult<String> {
    unsafe {
        let file = gettextpo::po_filepos_file(filepos.0);
        Ok(CStr::from_ptr(file).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_filepos_start_line(filepos: &PoFilePos) -> PyResult<usize> {
    unsafe {
        let start_line = gettextpo::po_filepos_start_line(filepos.0);
        Ok(start_line)
    }
}

#[pyfunction]
fn po_format_list() -> PyResult<Vec<String>> {
    unsafe {
        let list = gettextpo::po_format_list();
        let mut result = Vec::new();
        let mut i = 0;
        loop {
            let item = *list.offset(i);
            if item.is_null() {
                break;
            }
            result.push(CStr::from_ptr(item).to_str().unwrap().to_string());
            i += 1;
        }
        Ok(result)
    }
}

#[pyfunction]
fn po_format_pretty_name(format_type: &str) -> PyResult<String> {
    unsafe {
        let format_type = CString::new(format_type).unwrap();
        let pretty_name = gettextpo::po_format_pretty_name(format_type.as_ptr());
        Ok(CStr::from_ptr(pretty_name).to_str().unwrap().to_string())
    }
}

#[pyfunction]
fn po_file_check_all(file: &PoFile) -> PyResult<()> {
    unsafe {
        gettextpo::po_file_check_all(file.0, &XERROR_HANDLER);
    }
    Ok(())
}

#[pyfunction]
fn po_message_check_all(message: &PoMessage, iterator: &PoMessageIterator) -> PyResult<()> {
    unsafe {
        gettextpo::po_message_check_all(message.0, iterator.0, &XERROR_HANDLER);
    }
    Ok(())
}

#[pyfunction]
fn po_message_check_format_v2(message: &PoMessage) -> PyResult<()> {
    unsafe {
        gettextpo::po_message_check_format_v2(message.0, &XERROR_HANDLER);
    }
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _gettextpo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    m.add_function(wrap_pyfunction!(po_file_read, m)?)?;
    m.add_function(wrap_pyfunction!(po_file_write, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_check_format, m)?)?;

    m.add_function(wrap_pyfunction!(po_file_create, m)?)?;
    m.add_function(wrap_pyfunction!(po_file_create, m)?)?;
    m.add_function(wrap_pyfunction!(po_file_read_v3, m)?)?;
    m.add_function(wrap_pyfunction!(po_file_write_v2, m)?)?;
    m.add_function(wrap_pyfunction!(po_file_free, m)?)?;
    m.add_function(wrap_pyfunction!(po_file_domains, m)?)?;
    m.add_function(wrap_pyfunction!(po_file_domain_header, m)?)?;
    m.add_function(wrap_pyfunction!(po_header_field, m)?)?;
    m.add_function(wrap_pyfunction!(po_header_set_field, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_iterator, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_iterator_free, m)?)?;
    m.add_function(wrap_pyfunction!(po_next_message, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_insert, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_create, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_msgctxt, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_msgctxt, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_msgid, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_msgid, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_msgid_plural, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_msgid_plural, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_msgstr, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_msgstr, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_msgstr_plural, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_msgstr_plural, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_comments, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_comments, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_extracted_comments, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_extracted_comments, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_filepos, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_remove_filepos, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_add_filepos, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_prev_msgctxt, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_prev_msgctxt, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_prev_msgid, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_prev_msgid, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_prev_msgid_plural, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_prev_msgid_plural, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_is_obsolete, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_obsolete, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_is_fuzzy, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_fuzzy, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_is_format, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_format, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_is_range, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_set_range, m)?)?;
    m.add_function(wrap_pyfunction!(po_filepos_file, m)?)?;
    m.add_function(wrap_pyfunction!(po_filepos_start_line, m)?)?;
    m.add_function(wrap_pyfunction!(po_format_list, m)?)?;
    m.add_function(wrap_pyfunction!(po_format_pretty_name, m)?)?;
    m.add_function(wrap_pyfunction!(po_file_check_all, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_check_all, m)?)?;
    m.add_function(wrap_pyfunction!(po_message_check_format_v2, m)?)?;
    Ok(())
}
