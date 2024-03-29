class PoFile: ...
class PoMessage: ...
class PoMessageIterator: ...
class PoFilePos: ...

def sum_as_string(a: int, b: int) -> str: ...

def po_file_read(filename: str) -> PoFile: ...
def po_file_write(file: PoFile, filename: str) -> None: ...
def po_message_check_format(message: PoMessage) -> None: ...

def po_file_create() -> PoFile: ...
def po_file_read_v3(filename: str) -> PoFile: ...
def po_file_write_v2(file: PoFile, filename: str) -> None: ...
def po_file_free(file: PoFile) -> None: ...
def po_file_domains(file: PoFile) -> list[str]: ...
def po_file_domain_header(file: PoFile, domain: str) -> str: ...
def po_header_field(header: str, field: str) -> str: ...
def po_header_set_field(header: str, field: str, value: str) -> str: ...
def po_message_iterator(file: PoFile, domain: str) -> PoMessageIterator: ...
def po_message_iterator_free(iterator: PoMessageIterator) -> None: ...
def po_next_message(iterator: PoMessageIterator) -> PoMessage | None: ...
def po_message_insert(iterator: PoMessageIterator, message: PoMessage) -> None: ...
def po_message_create() -> PoMessage: ...
def po_message_msgctxt(message: PoMessage) -> str: ...
def po_message_set_msgctxt(message: PoMessage, msgctxt: str) -> None: ...
def po_message_msgid(message: PoMessage) -> str: ...
def po_message_set_msgid(message: PoMessage, msgid: str) -> None: ...
def po_message_msgid_plural(message: PoMessage) -> str: ...
def po_message_set_msgid_plural(message: PoMessage, msgid_plural: str) -> None: ...
def po_message_msgstr(message: PoMessage) -> str: ...
def po_message_set_msgstr(message: PoMessage, msgstr: str) -> None: ...
def po_message_msgstr_plural(message: PoMessage, index: int) -> str: ...
def po_message_set_msgstr_plural(message: PoMessage, index: int, msgstr: str) -> None: ...
def po_message_comments(message: PoMessage) -> str: ...
def po_message_set_comments(message: PoMessage, comments: str) -> None: ...
def po_message_extracted_comments(message: PoMessage) -> str: ...
def po_message_set_extracted_comments(message: PoMessage, comments: str) -> None: ...
def po_message_filepos(message: PoMessage, i: int) -> PoFilePos: ...
def po_message_remove_filepos(message: PoMessage, i: int) -> None: ...
def po_message_add_filepos(message: PoMessage, file: str, start_line: int) -> None: ...
def po_message_prev_msgctxt(message: PoMessage) -> str: ...
def po_message_set_prev_msgctxt(message: PoMessage, prev_msgctxt: str) -> None: ...
def po_message_prev_msgid(message: PoMessage) -> str: ...
def po_message_set_prev_msgid(message: PoMessage, prev_msgid: str) -> None: ...
def po_message_prev_msgid_plural(message: PoMessage) -> str: ...
def po_message_set_prev_msgid_plural(message: PoMessage, prev_msgid_plural: str) -> None: ...
def po_message_is_obsolete(message: PoMessage) -> bool: ...
def po_message_set_obsolete(message: PoMessage, obsolete: bool) -> None: ...
def po_message_is_fuzzy(message: PoMessage) -> bool: ...
def po_message_set_fuzzy(message: PoMessage, fuzzy: bool) -> None: ...
def po_message_is_format(message: PoMessage, format_type: str) -> bool: ...
def po_message_set_format(message: PoMessage, format_type: str, value: bool) -> None: ...
def po_message_is_range(message: PoMessage, min: int, max: int) -> bool: ...
def po_message_set_range(message: PoMessage, min: int, max: int) -> None: ...
def po_filepos_file(filepos: PoFilePos) -> str: ...
def po_filepos_start_line(filepos: PoFilePos) -> int: ...
def po_format_list() -> list[str]: ...
def po_format_pretty_name(format_type: str) -> str: ...
def po_file_check_all(file: PoFile) -> None: ...
def po_message_check_all(message: PoMessage, iterator: PoMessageIterator) -> None: ...
def po_message_check_format_v2(message: PoMessage) -> None: ...
