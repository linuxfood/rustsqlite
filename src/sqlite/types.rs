/*
** Copyright (c) 2011, Brian Smith <brian@linuxfood.net>
** All rights reserved.
**
** Redistribution and use in source and binary forms, with or without
** modification, are permitted provided that the following conditions are met:
**
**   * Redistributions of source code must retain the above copyright notice,
**     this list of conditions and the following disclaimer.
**
**   * Redistributions in binary form must reproduce the above copyright notice,
**     this list of conditions and the following disclaimer in the documentation
**     and/or other materials provided with the distribution.
**
**   * Neither the name of Brian Smith nor the names of its contributors
**     may be used to endorse or promote products derived from this software
**     without specific prior written permission.
**
** THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
** AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
** IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
** ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
** LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
** CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
** SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
** INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
** CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
** ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
** POSSIBILITY OF SUCH DAMAGE.
*/

use std::hashmap::HashMap;
use std::to_str;

#[deriving(Eq)]
#[repr(C)]
pub enum ResultCode {
    SQLITE_OK         =  0,
    SQLITE_ERROR      =  1,
    SQLITE_INTERNAL   =  2,
    SQLITE_PERM       =  3,
    SQLITE_ABORT      =  4,
    SQLITE_BUSY       =  5,
    SQLITE_LOCKED     =  6,
    SQLITE_NOMEM      =  7,
    SQLITE_READONLY   =  8,
    SQLITE_INTERRUPT  =  9,
    SQLITE_IOERR      = 10,
    SQLITE_CORRUPT    = 11,
    SQLITE_NOTFOUND   = 12,
    SQLITE_FULL       = 13,
    SQLITE_CANTOPEN   = 14,
    SQLITE_PROTOCOL   = 15,
    SQLITE_EMPTY      = 16,
    SQLITE_SCHEMA     = 17,
    SQLITE_TOOBIG     = 18,
    SQLITE_CONSTRAINT = 19,
    SQLITE_MISMATCH   = 20,
    SQLITE_MISUSE     = 21,
    SQLITE_NOLFS      = 22,
    SQLITE_AUTH       = 23,
    SQLITE_FORMAT     = 24,
    SQLITE_RANGE      = 25,
    SQLITE_NOTADB     = 26,
    SQLITE_ROW        = 100,
    SQLITE_DONE       = 101,
}

impl to_str::ToStr for ResultCode {
    fn to_str(&self) -> ~str {
        match *self {
            SQLITE_OK => ~"Ok",
            SQLITE_ERROR => ~"SQLITE_ERROR",
            SQLITE_INTERNAL => ~"SQLITE_INTERNAL",
            SQLITE_PERM => ~"SQLITE_PERM",
            SQLITE_ABORT => ~"SQLITE_ABORT",
            SQLITE_BUSY => ~"SQLITE_BUSY",
            SQLITE_LOCKED => ~"SQLITE_LOCKED",
            SQLITE_NOMEM => ~"SQLITE_NOMEM",
            SQLITE_READONLY => ~"SQLITE_READONLY",
            SQLITE_INTERRUPT => ~"SQLITE_INTERRUPT",
            SQLITE_IOERR => ~"SQLITE_IOERR",
            SQLITE_CORRUPT => ~"SQLITE_CORRUPT",
            SQLITE_NOTFOUND => ~"SQLITE_NOTFOUND",
            SQLITE_FULL => ~"SQLITE_FULL",
            SQLITE_CANTOPEN => ~"SQLITE_CANTOPEN",
            SQLITE_PROTOCOL => ~"SQLITE_PROTOCOL",
            SQLITE_EMPTY => ~"SQLITE_EMPTY",
            SQLITE_SCHEMA => ~"SQLITE_SCHEMA",
            SQLITE_TOOBIG => ~"SQLITE_TOOBIG",
            SQLITE_CONSTRAINT => ~"SQLITE_CONSTRAINT",
            SQLITE_MISMATCH => ~"SQLITE_MISMATCH",
            SQLITE_MISUSE => ~"SQLITE_MISUSE",
            SQLITE_NOLFS => ~"SQLITE_NOLFS",
            SQLITE_AUTH => ~"SQLITE_AUTH",
            SQLITE_FORMAT => ~"SQLITE_FORMAT",
            SQLITE_RANGE => ~"SQLITE_RANGE",
            SQLITE_NOTADB => ~"SQLITE_NOTADB",
            SQLITE_ROW => ~"SQLITE_ROW",
            SQLITE_DONE => ~"SQLITE_DONE",
        }
    }
}

#[deriving(Eq)]
pub enum BindArg {
    Text(~str),
    Number(f64),
    Integer(int),
    Integer64(i64),
    Blob(~[u8]),
    Null,
}

pub enum ColumnType {
    SQLITE_INTEGER,
    SQLITE_FLOAT,
    SQLITE_TEXT,
    SQLITE_BLOB,
    SQLITE_NULL,
}

pub type SqliteResult<T> = Result<T, ResultCode>;

pub type RowMap = HashMap<~str, BindArg>;

pub enum dbh {}
pub enum stmt {}
pub enum _notused {}
