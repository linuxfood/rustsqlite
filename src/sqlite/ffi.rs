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

use std::libc::*;
use types::*;

#[link(name = "sqlite3")]
extern {
    pub fn sqlite3_open(path: *c_char, hnd: **dbh) -> ResultCode;
    pub fn sqlite3_close(dbh: *dbh) -> ResultCode;
    pub fn sqlite3_errmsg(dbh: *dbh) -> *c_char;
    pub fn sqlite3_changes(dbh: *dbh) -> c_int;
    pub fn sqlite3_last_insert_rowid(dbh: *dbh) -> i64;
    pub fn sqlite3_complete(sql: *c_char) -> c_int;

    pub fn sqlite3_prepare_v2(
        hnd: *dbh,
        sql: *c_char,
        sql_len: c_int,
        shnd: **stmt,
        tail: **c_char
    ) -> ResultCode;

    pub fn sqlite3_exec(dbh: *dbh, sql: *c_char, cb: *_notused, d: *_notused, err: **c_char) -> ResultCode;

    pub fn sqlite3_step(sth: *stmt) -> ResultCode;
    pub fn sqlite3_reset(sth: *stmt) -> ResultCode;
    pub fn sqlite3_finalize(sth: *stmt) -> ResultCode;
    pub fn sqlite3_clear_bindings(sth: *stmt) -> ResultCode;

    pub fn sqlite3_column_name(sth: *stmt, icol: c_int) -> *c_char;
    pub fn sqlite3_column_type(sth: *stmt, icol: c_int) -> c_int;
    pub fn sqlite3_data_count(sth: *stmt) -> c_int;
    pub fn sqlite3_column_bytes(sth: *stmt, icol: c_int) -> c_int;
    pub fn sqlite3_column_blob(sth: *stmt, icol: c_int) -> *u8;

    pub fn sqlite3_column_text(sth: *stmt, icol: c_int) -> *c_char;
    pub fn sqlite3_column_double(sth: *stmt, icol: c_int) -> f64;
    pub fn sqlite3_column_int(sth: *stmt, icol: c_int) -> c_int;

    pub fn sqlite3_bind_blob(sth: *stmt, icol: c_int, buf: *u8, buflen: c_int, d: *c_void) -> ResultCode;
    pub fn sqlite3_bind_text(sth: *stmt, icol: c_int, buf: *c_char, buflen: c_int, d: *c_void) -> ResultCode;
    pub fn sqlite3_bind_null(sth: *stmt, icol: c_int) -> ResultCode;
    pub fn sqlite3_bind_int(sth: *stmt, icol: c_int, v: c_int) -> ResultCode;
    pub fn sqlite3_bind_double(sth: *stmt, icol: c_int, value: f64) -> ResultCode;
    pub fn sqlite3_bind_parameter_index(sth: *stmt, name: *c_char) -> c_int;

    pub fn sqlite3_busy_timeout(dbh: *dbh, ms: c_int) -> ResultCode;
}
