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

use ffi::*;
use std::libc::*;
use std::hashmap::HashMap;
use std::str;
use std::vec;
use types::*;

/// The database cursor.
pub struct Cursor {
  priv stmt: *stmt,
}

pub fn cursor_with_statement(stmt: *stmt) -> Cursor {
  Cursor { stmt: stmt }
}

impl Drop for Cursor {
  /// Deletes a prepared SQL statement.
  /// See http://www.sqlite.org/c3ref/finalize.html
  fn drop(&self) {
    debug!("freeing stmt resource: %?", self.stmt);
    unsafe {
      sqlite3_finalize(self.stmt);
    }
  }
}

impl Cursor {

  /// Resets a prepared SQL statement, but does not reset its bindings.
  /// See http://www.sqlite.org/c3ref/reset.html
  pub fn reset(&self) -> ResultCode {
    unsafe {
      sqlite3_reset(self.stmt)
    }
  }

  /// Resets all bindings on a prepared SQL statement.
  /// See http://www.sqlite.org/c3ref/clear_bindings.html
  pub fn clear_bindings(&self) -> ResultCode {
    unsafe {
      sqlite3_clear_bindings(self.stmt)
    }
  }

  /// Evaluates a prepared SQL statement one ore more times.
  /// See http://www.sqlite.org/c3ref/step.html
  pub fn step(&self) -> ResultCode {
    unsafe {
      sqlite3_step(self.stmt)
    }
  }

  /// 
  pub fn step_row(&self) -> SqliteResult<Option<RowMap>> {
    let is_row: ResultCode = self.step();
    if is_row == SQLITE_ROW {
      let column_cnt = self.get_column_count();
      let mut i = 0;
      let mut sqlrow = HashMap::new();
      while( i < column_cnt ) {
        let name = self.get_column_name(i);
        let coltype = self.get_column_type(i);
        let res = match coltype {
          SQLITE_INTEGER => sqlrow.insert(name, Integer(self.get_int(i))),
          SQLITE_FLOAT   => sqlrow.insert(name, Number(self.get_num(i))),
          SQLITE_TEXT    => sqlrow.insert(name, Text(self.get_text(i))),
          SQLITE_BLOB    => sqlrow.insert(name, Blob(self.get_blob(i))),
          SQLITE_NULL    => sqlrow.insert(name, Null),
        };
        if res == false {
          fail!("Couldn't insert a value into the map for sqlrow!");
        }
        i += 1;
      }

      Ok(Some(sqlrow))
    }
    else if is_row == SQLITE_DONE {
      Ok(None)
    } else {
      Err(is_row)
    }
  }

  /// 
  /// See http://www.sqlite.org/c3ref/column_blob.html
  pub fn get_bytes(&self, i: int) -> int {
    unsafe {
      sqlite3_column_bytes(self.stmt, i as c_int) as int
    }
  }

  /// 
  /// See http://www.sqlite.org/c3ref/column_blob.html
  pub fn get_blob(&self, i: int) -> ~[u8] {
    let len  = self.get_bytes(i);
    unsafe {
      let bytes = vec::raw::from_buf_raw(
        sqlite3_column_blob(self.stmt, i as c_int),
        len as uint
      );
      return bytes;
    }
  }

  /// 
  /// See http://www.sqlite.org/c3ref/column_blob.html
  pub fn get_int(&self, i: int) -> int {
    unsafe {
      return sqlite3_column_int(self.stmt, i as c_int) as int;
    }
  }

  /// 
  /// See http://www.sqlite.org/c3ref/column_blob.html
  pub fn get_num(&self, i: int) -> float {
    unsafe {
      return sqlite3_column_double(self.stmt, i as c_int);
    }
  }

  /// 
  /// See http://www.sqlite.org/c3ref/column_blob.html
  pub fn get_text(&self, i: int) -> ~str {
    unsafe {
      return str::raw::from_c_str( sqlite3_column_text(self.stmt, i as c_int) );
    }
  }

  /// 
  /// See http://www.sqlite.org/c3ref/bind_parameter_index.html
  pub fn get_bind_index(&self, name: &str) -> int {
    let stmt = self.stmt;
    do name.to_c_str().with_ref() |namebuf| {
      unsafe {
        sqlite3_bind_parameter_index(stmt, namebuf) as int
      }
    }
  }

  /// Returns the number of columns in a result set.
  /// See http://www.sqlite.org/c3ref/data_count.html
  pub fn get_column_count(&self) -> int {
    unsafe {
      return sqlite3_data_count(self.stmt) as int;
    }
  }

  /// Returns the name of the column with index `i` in the result set.
  /// See http://www.sqlite.org/c3ref/column_name.html
  pub fn get_column_name(&self, i: int) -> ~str {
    unsafe {
      return str::raw::from_c_str( sqlite3_column_name(self.stmt, i as c_int) );
    }
  }

  /// Returns the type of the column with index `i` in the result set.
  /// See http://www.sqlite.org/c3ref/column_blob.html
  pub fn get_column_type(&self, i: int) -> ColumnType {
    let ct;
    unsafe {
      ct = sqlite3_column_type(self.stmt, i as c_int) as int;
    }
    let res = match ct {
      1 /* SQLITE_INTEGER */ => SQLITE_INTEGER,
      2 /* SQLITE_FLOAT   */ => SQLITE_FLOAT,
      3 /* SQLITE_TEXT    */ => SQLITE_TEXT,
      4 /* SQLITE_BLOB    */ => SQLITE_BLOB,
      5 /* SQLITE_NULL    */ => SQLITE_NULL,
      _ => fail!(fmt!("sqlite internal error: Got an unknown column type (%d) back from the library.", ct)),
    };
    return res;
  }

  /// Returns the names of all columns in the result set.
  pub fn get_column_names(&self) -> ~[~str] {
    let cnt  = self.get_column_count();
    let mut i    = 0;
    let mut r    = ~[];
    while(i < cnt){
      r.push(self.get_column_name(i));
      i += 1;
    }
    return r;
  }

  /// 
  pub fn bind_params(&self, values: &[BindArg]) -> ResultCode {
    let mut i = 0i;
    for v in values.iter() {
      let r = self.bind_param(i, v);
      if r != SQLITE_OK {
        return r;
      }
      i += 1;
    }
    return SQLITE_OK;
  }

  /// 
  /// See http://www.sqlite.org/c3ref/bind_blob.html
  pub fn bind_param(&self, i: int, value: &BindArg) -> ResultCode {
    let r = match *value {
      Text(ref v) => {
        let l = v.len();
        (*v).to_c_str().with_ref( |_v| {
          // FIXME: -1 means: SQLITE_TRANSIENT, so this interface will do lots
          //        of copying when binding text or blob values.
          unsafe {
            sqlite3_bind_text(self.stmt, i as c_int, _v, l as c_int, -1 as c_int)
          }
        })
      }

      Blob(ref v) => {
        let l = v.len();
        // FIXME: -1 means: SQLITE_TRANSIENT, so this interface will do lots
        //        of copying when binding text or blob values.
        unsafe {
          sqlite3_bind_blob(self.stmt, i as c_int, vec::raw::to_ptr(*v), l as c_int, -1 as c_int)
        }
      }

      Integer(ref v) => { unsafe { sqlite3_bind_int(self.stmt, i as c_int, *v as c_int) } }

      Number(ref v) => { unsafe { sqlite3_bind_double(self.stmt, i as c_int, *v) } }

      Null => { unsafe { sqlite3_bind_null(self.stmt, i as c_int) } }

    };

    return r;
  }
}
