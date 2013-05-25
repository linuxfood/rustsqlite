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
use std::libc::*;
use types::*;

/// The database connection.
pub struct Database {
  priv dbh: *dbh,
}

pub fn database_with_handle(dbh: *dbh) -> Database {
  Database { dbh: dbh }
}

impl Drop for Database {
  /// Closes the database connection.
  /// See http://www.sqlite.org/c3ref/close.html
  fn finalize(&self) {
    debug!("freeing dbh resource: %?", self.dbh);
    unsafe {
      sqlite3_close(self.dbh);
    }
  }
}

pub impl Database {

  /// Returns the error message of the the most recent call.
  /// See http://www.sqlite.org/c3ref/errcode.html
  unsafe fn get_errmsg(&self) -> ~str {
    str::raw::from_c_str(sqlite3_errmsg(self.dbh))
  }

  /// Prepares/compiles an SQL statement.
  /// See http://www.sqlite.org/c3ref/prepare.html
  fn prepare(&self, sql: &str, _tail: &Option<&str>) -> SqliteResult<Cursor> {
    let new_stmt = ptr::null();
    let r = str::as_c_str(sql, |_sql| {
      unsafe {
        sqlite3_prepare_v2(self.dbh, _sql, str::len(sql) as c_int, &new_stmt, ptr::null())
      }
    });
    if r == SQLITE_OK {
      debug!("created new stmt: %?", new_stmt);
      Ok( cursor_with_statement(new_stmt))
    } else {
      Err(r)
    }
  }

  /// Executes an SQL statement.
  /// See http://www.sqlite.org/c3ref/exec.html
  fn exec(&self, sql: &str) -> SqliteResult<bool> {
    let mut r = SQLITE_ERROR;
    str::as_c_str(sql, |_sql| {
      unsafe {
        r = sqlite3_exec(self.dbh, _sql, ptr::null(), ptr::null(), ptr::null())
      }
    });

    if r == SQLITE_OK { Ok(true) } else { Err(r) }
  }

  /// Returns the number of modified/inserted/deleted rows by the most recent
  /// call.
  /// See http://www.sqlite.org/c3ref/changes.html
  fn get_changes(&self) -> int {
    unsafe {
      sqlite3_changes(self.dbh) as int
    }
  }

  /// Returns the ID of the last inserted row.
  /// See http://www.sqlite.org/c3ref/last_insert_rowid.html
  fn get_last_insert_rowid(&self) -> i64 {
    unsafe {
      sqlite3_last_insert_rowid(self.dbh)
    }
  }

  /// Sets a busy timeout.
  /// See http://www.sqlite.org/c3ref/busy_timeout.html
  fn set_busy_timeout(&self, ms: int) -> ResultCode {
    unsafe {
      sqlite3_busy_timeout(self.dbh, ms as c_int)
    }
  }
}
