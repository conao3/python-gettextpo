from __future__ import annotations
import tempfile
from typing import Generator

from . import gettextpo


class PoFile:
    def __init__(self, po_file: gettextpo.PoFile):
        self._po_file = po_file

    @classmethod
    def create(cls) -> PoFile:
        return cls(gettextpo.po_file_create())

    @classmethod
    def read(cls, filename: str) -> PoFile:
        return cls(gettextpo.po_file_read(filename))

    @classmethod
    def load(cls, contents: str) -> PoFile:
        with tempfile.NamedTemporaryFile() as f:
            f.write(contents.encode())
            f.flush()
            return cls.read(f.name)

    def domains(self) -> list[str]:
        return gettextpo.po_file_domains(self._po_file)

    def domain_header(self, domain: str) -> str:
        return gettextpo.po_file_domain_header(self._po_file, domain)

    def messages(self, domain: str) -> Generator[gettextpo.PoMessage, None, None]:
        iterator = gettextpo.po_message_iterator(self._po_file, domain)

        while (res := gettextpo.po_next_message(iterator)) is not None:
            yield res

        gettextpo.po_message_iterator_free(iterator)

        raise StopIteration

    def __del__(self):
        gettextpo.po_file_free(self._po_file)


class PoHeader:
    def __init__(self, header: str):
        self._header = header

    def __getitem__(self, field: str) -> str:
        return gettextpo.po_header_field(self._header, field)

    def __setitem__(self, field: str, value: str) -> None:
        self._header = gettextpo.po_header_set_field(self._header, field, value)

    def render(self) -> str:
        return self._header
