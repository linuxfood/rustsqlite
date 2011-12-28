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

use std;
import std::map;
import result::{ok, err, success};

const SQLITE_OK         : int =  0;
const SQLITE_ERROR      : int =  1;
const SQLITE_INTERNAL   : int =  2;
const SQLITE_PERM       : int =  3;
const SQLITE_ABORT      : int =  4;
const SQLITE_BUSY       : int =  5;
const SQLITE_LOCKED     : int =  6;
const SQLITE_NOMEM      : int =  7;
const SQLITE_READONLY   : int =  8;
const SQLITE_INTERRUPT  : int =  9;
const SQLITE_IOERR      : int = 10;
const SQLITE_CORRUPT    : int = 11;
const SQLITE_NOTFOUND   : int = 12;
const SQLITE_FULL       : int = 13;
const SQLITE_CANTOPEN   : int = 14;
const SQLITE_PROTOCOL   : int = 15;
const SQLITE_EMPTY      : int = 16;
const SQLITE_SCHEMA     : int = 17;
const SQLITE_TOOBIG     : int = 18;
const SQLITE_CONSTRAINT : int = 19;
const SQLITE_MISMATCH   : int = 20;
const SQLITE_MISUSE     : int = 21;
const SQLITE_NOLFS      : int = 22;
const SQLITE_AUTH       : int = 23;
const SQLITE_FORMAT     : int = 24;
const SQLITE_RANGE      : int = 25;
const SQLITE_NOTADB     : int = 26;
const SQLITE_ROW        : int = 100;
const SQLITE_DONE       : int = 101;

tag sqlite_bind_arg {
  text(str);
  number(float);
  integer(int);
  blob([u8]);
  null();
}

tag sqlite_column_type {
  sqlite_integer;
  sqlite_float;
  sqlite_text;
  sqlite_blob;
  sqlite_null;
}

type sqlite_result<T> = result::t<T, int>;
tag sqlite_row_result {
  row(map::hashmap<str, sqlite_bind_arg>);
  done();
}

type sqlite_stmt = obj {
  fn step() -> int;
  fn step_row() -> sqlite_result<sqlite_row_result>;
  fn reset() -> int;
  fn clear_bindings() -> int;

  fn get_num(i: int) -> float;
  fn get_int(i: int) -> int;
  fn get_bytes(i: int) -> int;
  fn get_blob(i: int) -> [u8];
  fn get_text(i: int) -> str;

  fn get_column_count() -> int;
  fn get_column_name(i: int) -> str;
  fn get_column_type(i: int) -> sqlite_column_type;
  fn get_column_names() -> [str];

  fn get_bind_index(name: str) -> int;

  fn bind_param(i: int, value: sqlite_bind_arg) -> int;
  fn bind_params(values: [sqlite_bind_arg]) -> int;
};


type sqlite_dbh = obj {
  fn get_errmsg() -> str;
  fn prepare(sql: str, &_tail: option::t<str>) -> sqlite_result<sqlite_stmt>;
  fn exec(sql: str) -> sqlite_result<int>;
  fn get_changes() -> int;
  fn get_last_insert_rowid() -> i64;

  fn set_busy_timeout(ms: int) -> int;
};


#[nolink] #[link_args = "sqlite3.c"]
native mod _sqlite {
  type _dbh;
  type _stmt;

  type _notused;

  fn sqlite3_open(path: str::sbuf, hnd: **_dbh) -> int;
  fn sqlite3_close(dbh: *_dbh) -> int;
  fn sqlite3_errmsg(dbh: *_dbh) -> str::sbuf;
  fn sqlite3_changes(dbh: *_dbh) -> int;
  fn sqlite3_last_insert_rowid(dbh: *_dbh) -> i64;

  fn sqlite3_prepare_v2(
    hnd: *_dbh,
    sql: str::sbuf,
    sql_len: int,
    shnd: **_stmt,
    tail: *str::sbuf
  ) -> int;

  fn sqlite3_exec(dbh: *_dbh, sql: str::sbuf, cb: *_notused, d: *_notused, err: *str::sbuf) -> int;

  fn sqlite3_step(sth: *_stmt) -> int;
  fn sqlite3_reset(sth: *_stmt) -> int;
  fn sqlite3_finalize(sth: *_stmt) -> int;
  fn sqlite3_clear_bindings(sth: *_stmt) -> int;

  fn sqlite3_column_name(sth: *_stmt, icol: int) -> str::sbuf;
  fn sqlite3_column_type(sth: *_stmt, icol: int) -> int;
  fn sqlite3_data_count(sth: *_stmt) -> int;
  fn sqlite3_column_bytes(sth: *_stmt, icol: int) -> int;
  fn sqlite3_column_blob(sth: *_stmt, icol: int) -> str::sbuf;

  fn sqlite3_column_text(sth: *_stmt, icol: int) -> str::sbuf;
  fn sqlite3_column_double(sth: *_stmt, icol: int) -> float;
  fn sqlite3_column_int(sth: *_stmt, icol: int) -> int;

  fn sqlite3_bind_blob(sth: *_stmt, icol: int, buf: str::sbuf, buflen: int, d: int) -> int;
  fn sqlite3_bind_text(sth: *_stmt, icol: int, buf: str::sbuf, buflen: int, d: int) -> int;
  fn sqlite3_bind_null(sth: *_stmt, icol: int) -> int;
  fn sqlite3_bind_int(sth: *_stmt, icol: int, v: int) -> int;
  fn sqlite3_bind_double(sth: *_stmt, icol: int, value: float) -> int;
  fn sqlite3_bind_parameter_index(sth: *_stmt, name: str::sbuf) -> int;

  fn sqlite3_busy_timeout(dbh: *_dbh, ms: int) -> int;

}

resource _sqlite_dbh(dbh: *_sqlite::_dbh) {
  log(debug, ("freeing dbh resource: ", dbh));
  _sqlite::sqlite3_close(dbh);
}

resource _sqlite_stmt(stmt: *_sqlite::_stmt) {
  log(debug, ("freeing stmt resource: ", stmt));
  _sqlite::sqlite3_finalize(stmt);
}

fn sqlite_open(path: str) -> sqlite_result<sqlite_dbh> {
  type sqliteState = {
    _dbh: *_sqlite::_dbh,
    _res: _sqlite_dbh
  };

  type sqliteStmtState = {
    _stmt: *_sqlite::_stmt,
    _res: _sqlite_stmt
  };

  obj sqlite_stmt(st: sqliteStmtState) {
    fn reset() -> int {
      let stmt = st._stmt;
      ret _sqlite::sqlite3_reset(stmt);
    }

    fn clear_bindings() -> int {
      let stmt = st._stmt;
      ret _sqlite::sqlite3_clear_bindings(stmt);
    }

    fn step() -> int {
      let stmt = st._stmt;
      ret _sqlite::sqlite3_step(stmt);
    }

    fn step_row() -> sqlite_result<sqlite_row_result> {
      let is_row = self.step();
      if is_row == SQLITE_ROW {
        let column_cnt = self.get_column_count();
        let i = 0;
        let sqlrow = map::new_str_hash::<sqlite_bind_arg>();
        while( i < column_cnt ) {
          let name = self.get_column_name(i);
          alt self.get_column_type(i) {
            sqlite_integer. { sqlrow.insert(name, integer(self.get_int(i))); }
            sqlite_float.   { sqlrow.insert(name, number(self.get_num(i))); }
            sqlite_text.    { sqlrow.insert(name, text(self.get_text(i))); }
            sqlite_blob.    { sqlrow.insert(name, blob(self.get_blob(i))); }
            sqlite_null.    { sqlrow.insert(name, null); }
          }
          i += 1;
        }

        ret ok(row(sqlrow));
      }
      else if is_row == SQLITE_DONE {
        ret ok(done);
      }
      ret err(is_row);
    }

    fn get_bytes(i: int) -> int {
      let stmt = st._stmt;
      ret _sqlite::sqlite3_column_bytes(stmt, i);
    }

    fn get_blob(i: int) -> [u8] unsafe {
      let stmt = st._stmt;
      let len  = self.get_bytes(i);
      let bytes : [u8] = [];
      bytes = vec::unsafe::from_buf(
        _sqlite::sqlite3_column_blob(stmt, i),
        len as uint
      );
      ret bytes
    }

    fn get_int(i: int) -> int {
      let stmt = st._stmt;
      ret _sqlite::sqlite3_column_int(stmt, i);
    }

    fn get_num(i: int) -> float {
      let stmt = st._stmt;
      ret _sqlite::sqlite3_column_double(stmt, i);
    }

    fn get_text(i: int) -> str unsafe {
      let stmt = st._stmt;
      ret str::str_from_cstr( _sqlite::sqlite3_column_text(stmt, i) );
    }

    fn get_bind_index(name: str) -> int {
      let stmt = st._stmt;
      ret str::as_buf(name, { |_name|
        _sqlite::sqlite3_bind_parameter_index(stmt, _name)
      });
    }

    fn get_column_count() -> int {
      let stmt = st._stmt;
      ret _sqlite::sqlite3_data_count(stmt);
    }

    fn get_column_name(i: int) -> str unsafe {
      let stmt = st._stmt;
      ret str::str_from_cstr( _sqlite::sqlite3_column_name(stmt, i) );
    }

    fn get_column_type(i: int) -> sqlite_column_type {
      let stmt = st._stmt;
      let ct = _sqlite::sqlite3_column_type(stmt, i);
      let res = sqlite_null;
      alt ct {
        1 /* SQLITE_INTEGER */ { res = sqlite_integer; }
        2 /* SQLITE_FLOAT   */ { res = sqlite_float; }
        3 /* SQLITE_TEXT    */ { res = sqlite_text; }
        4 /* SQLITE_BLOB    */ { res = sqlite_blob; }
        5 /* SQLITE_NULL    */ { res = sqlite_null; }
        _ { fail #fmt("sqlite internal error: Got an unknown column type (%d) back from the library.", ct); }
      }
      ret res;
    }

    fn get_column_names() -> [str] {
      let cnt  = self.get_column_count();
      let i    = 0;
      let r    = [];
      while(i < cnt){
        vec::push(r, self.get_column_name(i));
        i += 1;
      }
      ret r;
    }

    fn bind_params(values: [sqlite_bind_arg]) -> int {
      let i = 0i;
      let r = 0i;
      for v in values {
        r += self.bind_param(i, v);
        i += 1;
      }
      ret r;
    }

    fn bind_param(i: int, value: sqlite_bind_arg) -> int unsafe {
      let stmt = st._stmt;
      let r = 0i;
      alt value {

        text(v) {
          let l = str::byte_len(v);
          r = str::as_buf(v, { |_v|
            // FIXME: -1 means: SQLITE_TRANSIENT, so this interface will do lots
            //        of copying when binding text or blob values.
            _sqlite::sqlite3_bind_text(stmt, i, _v, l as int, -1)
          });
        }

        blob(v) {
          let l = vec::len(v);
          // FIXME: -1 means: SQLITE_TRANSIENT, so this interface will do lots
          //        of copying when binding text or blob values.
          r = _sqlite::sqlite3_bind_blob(stmt, i, vec::unsafe::to_ptr(v), l as int, -1);
        }

        integer(v) {
          r = _sqlite::sqlite3_bind_int(stmt, i, v);
        }

        number(v) {
          r = _sqlite::sqlite3_bind_double(stmt, i, v);
        }

        null {
          r = _sqlite::sqlite3_bind_null(stmt, i);
        }

      }

      ret r;
    }
  };

  obj sqlite_dbh(st: sqliteState) {
    fn get_errmsg() -> str unsafe {
      ret str::str_from_cstr(_sqlite::sqlite3_errmsg(st._dbh));
    }

    fn get_changes() -> int {
      let dbh = st._dbh;
      ret _sqlite::sqlite3_changes(dbh);
    }

    fn get_last_insert_rowid() -> i64 {
      let dbh = st._dbh;
      ret _sqlite::sqlite3_last_insert_rowid(dbh);
    }

    fn prepare(sql: str, &_tail: option::t<str>) -> sqlite_result<sqlite_stmt> {
      let new_stmt : *_sqlite::_stmt = ptr::null();
      let dbh = st._dbh;
      let r : int = str::as_buf(sql, { |_sql|
        _sqlite::sqlite3_prepare_v2( dbh, _sql, str::byte_len(sql) as int, ptr::addr_of(new_stmt), ptr::null())
      });
      if r == SQLITE_OK {
        ret ok(sqlite_stmt({ _stmt: new_stmt, _res: _sqlite_stmt(new_stmt) }));
      }
      ret err(r);
    }

    fn exec(sql: str) -> sqlite_result<int> {
      let dbh = st._dbh;
      let r : int = str::as_buf(sql, { |_sql|
        _sqlite::sqlite3_exec(dbh, _sql, ptr::null(), ptr::null(), ptr::null())
      });
      if r == SQLITE_OK {
        ret ok(r);
      }
      ret err(r);
    }

    fn set_busy_timeout(ms: int) -> int {
      let dbh = st._dbh;
      ret _sqlite::sqlite3_busy_timeout(dbh, ms);
    }
  };

  let new_dbh : *_sqlite::_dbh = ptr::null();
  let r : int = str::as_buf(path, { |_path|
    _sqlite::sqlite3_open(_path, ptr::addr_of(new_dbh))
  });
  if r != SQLITE_OK {
    ret err(r);
  }
  //#debug("created new dbh: %s", new_dbh);
  ret ok(sqlite_dbh({ _dbh: new_dbh, _res: _sqlite_dbh(new_dbh) }));
}

#[cfg(test)]
mod tests {

  fn checked_prepare(dbh: sqlite_dbh, sql: str) -> sqlite_stmt {
    alt dbh.prepare(sql, none) {
      ok(s) { ret s; }
      err(x) { fail #fmt("sqlite error: \"%s\" (%d)", dbh.get_errmsg(), x); }
    }
  }

  fn checked_open() -> sqlite_dbh {
    let dbh = sqlite_open(":memory:");
    check success(dbh);
    ret result::get(dbh);
  }

  fn checked_exec(dbh: sqlite_dbh, sql: str) {
    let r = dbh.exec(sql);
    check success(r);
  }

  #[test]
  fn open_db() {
    checked_open();
  }

  #[test]
  fn exec_create_tbl() {
    let dbh = checked_open();
    checked_exec(dbh, "BEGIN; CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT); COMMIT;");
  }

  #[test]
  fn prepare_insert_stmt() {
    let dbh = checked_open();

    checked_exec(dbh, "BEGIN; CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT); COMMIT;");
    let sth = checked_prepare(dbh, "INSERT OR IGNORE INTO test (id) VALUES (1)");
    let res = sth.step();
    #error("prepare_insert_stmt step res: %d", res);
  }

  #[test]
  fn prepare_select_stmt() {
    let dbh = checked_open();

    checked_exec(dbh,
      "BEGIN;
      CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT);
      INSERT OR IGNORE INTO test (id) VALUES (1);
      COMMIT;"
    );

    let sth = checked_prepare(dbh, "SELECT id FROM test WHERE id = 1;");
    assert sth.step() == SQLITE_ROW;
    assert sth.get_int(0) == 1;
    assert sth.step() == SQLITE_DONE;
  }

  #[test]
  fn prepared_stmt_bind() {
    let dbh = checked_open();

    checked_exec(dbh, "BEGIN; CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT); COMMIT;");

    checked_exec(dbh,
      "INSERT OR IGNORE INTO test (id) VALUES(2);"
    + "INSERT OR IGNORE INTO test (id) VALUES(3);"
    + "INSERT OR IGNORE INTO test (id) VALUES(4);"
    );
    let sth = checked_prepare(dbh, "SELECT id FROM test WHERE id > ? AND id < ?");
    assert sth.bind_param(1, integer(2)) == SQLITE_OK;
    assert sth.bind_param(2, integer(4)) == SQLITE_OK;

    assert sth.step() == SQLITE_ROW;
    assert sth.get_num(0) as int == 3;
  }

  #[test]
  fn column_names() {
    let dbh = checked_open();

    checked_exec(dbh,
      "BEGIN;
      CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT);
      INSERT OR IGNORE INTO test (id, v) VALUES(1, 'leeeee');
      COMMIT;
      "
    );
    let sth = checked_prepare(dbh, "SELECT * FROM test");
    assert sth.step() == SQLITE_ROW;
    assert sth.get_column_names() == ["id", "v"];
  }

  #[test] #[should_fail]
  fn failed_prepare() {
    let dbh = checked_open();

    checked_exec(dbh,
      "BEGIN;
      CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT);
      INSERT OR IGNORE INTO test (id, v) VALUES(1, 'leeeee');
      COMMIT;
      "
    );
    let _sth = checked_prepare(dbh, "SELECT q FRO test");
  }

  #[test]
  fn bind_param_index() {
    let dbh = checked_open();

    checked_exec(dbh,
      "BEGIN;
      CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT);
      INSERT OR IGNORE INTO test (id, v) VALUES(1, 'leeeee');
      COMMIT;
      "
    );
    let sth = checked_prepare(dbh, "SELECT * FROM test WHERE v=:Name");
    assert sth.get_bind_index(":Name") == 1;
  }

  #[test]
  fn last_insert_id() {
    let dbh = checked_open();
    checked_exec(dbh,
      "
      BEGIN;
      CREATE TABLE IF NOT EXISTS test (v TEXT);
      INSERT OR IGNORE INTO test (v) VALUES ('This is a really long string.');
      COMMIT;
      "
    );
    #error("last insert_id: %s", u64::str(dbh.get_last_insert_rowid() as u64));
    assert dbh.get_last_insert_rowid() == 1_i64;
  }

  #[test]
  fn step_row_basics() {
    let dbh = checked_open();
    checked_exec(dbh,
      "
      BEGIN;
      CREATE TABLE IF NOT EXISTS test (id INTEGER, k TEXT, v REAL);
      INSERT OR IGNORE INTO test (id, k, v) VALUES(1, 'pi', 3.1415);
      INSERT OR IGNORE INTO test (id, k, v) VALUES(2, 'e', 2.17);
      INSERT OR IGNORE INTO test (id, k, v) VALUES(3, 'o', 1.618);
      COMMIT;
      "
    );
    let sth = checked_prepare(dbh, "SELECT * FROM test WHERE id=2");
    let r = sth.step_row();
    check success(r);
    alt r {
      ok(row(x)) {
        assert x.get("id") == integer(2);
        assert x.get("k")  == text("e");
        assert x.get("v")  == number(2.17);
      }
      ok(done) {
        fail("didnt get even one row back.");
      }
    }
  }
}

