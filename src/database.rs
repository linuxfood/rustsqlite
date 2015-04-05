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

use cursor::*;
use ffi::*;
use libc::c_int;
use std::str;
use std::ptr;
use std::fmt;
use std::borrow::ToOwned;
use std::ffi::{CString, CStr};
use types::*;
use types::ResultCode::*;

/// The database connection.
///
/// SQLite database is `Send`able but not `Copy`able nor `Sync`able.
/// Consequently, it can be shared through `std::sync::Mutex` across tasks
/// (as it grants an exclusive access to the connection)
/// but cannot be shared through `std::sync::RWLock`.
pub struct Database {
    dbh: *mut dbh,
}

unsafe impl Send for Database {}

pub fn database_with_handle(dbh: *mut dbh) -> Database {
    Database { dbh: dbh }
}

impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Database dbh={:?}>", self.dbh)
    }
}

impl Drop for Database {
    /// Closes the database connection.
    /// See http://www.sqlite.org/c3ref/close.html
    fn drop(&mut self) {
        debug!("`Database.drop()`: self={:?}", *self);
        unsafe {
            sqlite3_close(self.dbh);
        }
    }
}

impl Database {

    /// Returns the error message of the the most recent call.
    /// See http://www.sqlite.org/c3ref/errcode.html
    pub fn get_errmsg(&self) -> String {
        unsafe {
            let msg = sqlite3_errmsg(self.dbh);
            str::from_utf8(CStr::from_ptr(msg).to_bytes()).unwrap().to_owned()
        }
    }

    /// Prepares/compiles an SQL statement.
    /// See http://www.sqlite.org/c3ref/prepare.html
    pub fn prepare<'db>(&'db self, sql: &str, _tail: &Option<&str>) -> SqliteResult<Cursor<'db>> {
        let sql = CString::new(sql.as_bytes()).unwrap();
        let mut new_stmt = ptr::null_mut();
        let r = unsafe {
            sqlite3_prepare_v2(self.dbh, sql.as_ptr(), sql.as_bytes().len() as c_int, &mut new_stmt, ptr::null_mut())
        };
        if r == SQLITE_OK {
            debug!("`Database.prepare()`: stmt={:?}", new_stmt);
            Ok( cursor_with_statement(new_stmt, &self.dbh))
        } else {
            Err(r)
        }
    }

    /// Executes an SQL statement.
    /// See http://www.sqlite.org/c3ref/exec.html
    pub fn exec(&mut self, sql: &str) -> SqliteResult<bool> {
        let sql = CString::new(sql.as_bytes()).unwrap();
        let r = unsafe {
            sqlite3_exec(self.dbh, sql.as_ptr(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut())
        };

        if r == SQLITE_OK { Ok(true) } else { Err(r) }
    }

    /// Returns the number of modified/inserted/deleted rows by the most recent
    /// call.
    /// See http://www.sqlite.org/c3ref/changes.html
    pub fn get_changes(&self) -> isize {
        unsafe {
            sqlite3_changes(self.dbh) as isize
        }
    }

    /// Returns the ID of the last inserted row.
    /// See http://www.sqlite.org/c3ref/last_insert_rowid.html
    pub fn get_last_insert_rowid(&self) -> i64 {
        unsafe {
            sqlite3_last_insert_rowid(self.dbh)
        }
    }

    /// Sets a busy timeout.
    /// See http://www.sqlite.org/c3ref/busy_timeout.html
    pub fn set_busy_timeout(&mut self, ms: isize) -> ResultCode {
        unsafe {
            sqlite3_busy_timeout(self.dbh, ms as c_int)
        }
    }
}
