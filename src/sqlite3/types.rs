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

use std::collections::HashMap;
use std::fmt;

#[deriving(PartialEq, Eq)]
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

impl fmt::Show for ResultCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write(match *self {
            SQLITE_OK => "Ok".as_bytes(),
            SQLITE_ERROR => "SQLITE_ERROR".as_bytes(),
            SQLITE_INTERNAL => "SQLITE_INTERNAL".as_bytes(),
            SQLITE_PERM => "SQLITE_PERM".as_bytes(),
            SQLITE_ABORT => "SQLITE_ABORT".as_bytes(),
            SQLITE_BUSY => "SQLITE_BUSY".as_bytes(),
            SQLITE_LOCKED => "SQLITE_LOCKED".as_bytes(),
            SQLITE_NOMEM => "SQLITE_NOMEM".as_bytes(),
            SQLITE_READONLY => "SQLITE_READONLY".as_bytes(),
            SQLITE_INTERRUPT => "SQLITE_INTERRUPT".as_bytes(),
            SQLITE_IOERR => "SQLITE_IOERR".as_bytes(),
            SQLITE_CORRUPT => "SQLITE_CORRUPT".as_bytes(),
            SQLITE_NOTFOUND => "SQLITE_NOTFOUND".as_bytes(),
            SQLITE_FULL => "SQLITE_FULL".as_bytes(),
            SQLITE_CANTOPEN => "SQLITE_CANTOPEN".as_bytes(),
            SQLITE_PROTOCOL => "SQLITE_PROTOCOL".as_bytes(),
            SQLITE_EMPTY => "SQLITE_EMPTY".as_bytes(),
            SQLITE_SCHEMA => "SQLITE_SCHEMA".as_bytes(),
            SQLITE_TOOBIG => "SQLITE_TOOBIG".as_bytes(),
            SQLITE_CONSTRAINT => "SQLITE_CONSTRAINT".as_bytes(),
            SQLITE_MISMATCH => "SQLITE_MISMATCH".as_bytes(),
            SQLITE_MISUSE => "SQLITE_MISUSE".as_bytes(),
            SQLITE_NOLFS => "SQLITE_NOLFS".as_bytes(),
            SQLITE_AUTH => "SQLITE_AUTH".as_bytes(),
            SQLITE_FORMAT => "SQLITE_FORMAT".as_bytes(),
            SQLITE_RANGE => "SQLITE_RANGE".as_bytes(),
            SQLITE_NOTADB => "SQLITE_NOTADB".as_bytes(),
            SQLITE_ROW => "SQLITE_ROW".as_bytes(),
            SQLITE_DONE => "SQLITE_DONE".as_bytes(),
        })
    }
}

#[deriving(PartialEq)]
pub enum BindArg {
    Text(String),
    StaticText(&'static str),
    Float64(f64),
    Integer(int),
    Integer64(i64),
    Blob(Vec<u8>),
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

pub type RowMap = HashMap<String, BindArg>;

pub enum dbh {}
pub enum stmt {}
pub enum _notused {}
