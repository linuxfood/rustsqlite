use core::libc::*;
use cursor::*;
use sqlite3::*;
use types::*;

/// The database connection.
pub struct Database {
  /*priv*/ dbh: *dbh,
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
      Ok(Cursor { stmt: new_stmt })
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
