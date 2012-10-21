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
#[forbid(deprecated_pattern)];
#[forbid(deprecated_mode)];

extern mod std;
use libc::*;
use send_map::linear::LinearMap;

pub enum sqlite_result_code {
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

impl sqlite_result_code: to_str::ToStr {
  pure fn to_str() -> ~str {
    match self {
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

impl sqlite_result_code : cmp::Eq {
   pure fn eq(other: &sqlite_result_code) -> bool { self as int == *other as int }
   pure fn ne(other: &sqlite_result_code) -> bool { !self.eq(other) }
}

pub enum sqlite_bind_arg {
  text(~str),
  number(float),
  integer(int),
  blob(~[u8]),
  null(),
}

impl sqlite_bind_arg : cmp::Eq {
  pure fn eq(other: &sqlite_bind_arg) -> bool {
    match self {
      text(copy ss) =>
        match *other {
          text(copy sss) => ss == sss,
          _        => false
         },
      number(copy ff) =>
        match *other {
          number(copy fff) => ff == fff,
          _          => false
        },
      integer(copy ii) =>
        match *other {
          integer(copy iii) => ii == iii,
           _           => false
        },
      blob(copy bb) =>
        match *other {
          blob(copy bbb) => bb == bbb,
          _        => false
        },
      null() =>
        match *other {
          null() => true,
          _     => false
        },
    }
  }

  pure fn ne(other: &sqlite_bind_arg) -> bool { !self.eq(other) }
}

pub enum sqlite_column_type {
  sqlite_integer,
  sqlite_float,
  sqlite_text,
  sqlite_blob,
  sqlite_null,
}

pub type sqlite_result<T> = Result<T, sqlite_result_code>;

type RowMap = LinearMap<~str, sqlite_bind_arg>;

pub enum sqlite_row_result {
  row(RowMap),
  done,
}

pub struct sqlite_dbh {
  priv _dbh: *_dbh,

  drop {
    debug!("freeing dbh resource: %?", self._dbh);
    sqlite3::sqlite3_close(self._dbh);
  }
}

impl sqlite_dbh {
  fn get_errmsg(&self) -> ~str unsafe {
    str::raw::from_c_str(sqlite3::sqlite3_errmsg(self._dbh))
  }

  fn prepare(&self, sql: &str, _tail: &Option<&str>) -> sqlite_result<sqlite_stmt> {
    let new_stmt = ptr::null();
    let mut r = str::as_c_str(sql, |_sql| {
        sqlite3::sqlite3_prepare_v2(self._dbh, _sql, str::len(sql) as c_int, ptr::addr_of(&new_stmt), ptr::null())
    });
    if r == SQLITE_OK {
      debug!("created new stmt: %?", new_stmt);
      Ok(sqlite_stmt { stmt: new_stmt })
    } else {
      Err(r)
    }
  }
  fn exec(&self, sql: &str) -> sqlite_result<sqlite_result_code> {
    let mut r = SQLITE_ERROR;
    str::as_c_str(sql, |_sql| {
      r = sqlite3::sqlite3_exec(self._dbh, _sql, ptr::null(), ptr::null(), ptr::null())
    });

    if r == SQLITE_OK { Ok(r) } else { Err(r) }
  }
  fn get_changes(&self) -> int {
    sqlite3::sqlite3_changes(self._dbh) as int
  }
  fn get_last_insert_rowid(&self) -> i64 {
    sqlite3::sqlite3_last_insert_rowid(self._dbh)
  }

  fn set_busy_timeout(&self, ms: int) -> sqlite_result_code {
    sqlite3::sqlite3_busy_timeout(self._dbh, ms as c_int)
  }
}

enum _dbh {}
enum _stmt {}

enum _notused {}

extern mod sqlite3 {
  fn sqlite3_open(path: *c_char, hnd: **_dbh) -> sqlite_result_code;
  fn sqlite3_close(dbh: *_dbh) -> sqlite_result_code;
  fn sqlite3_errmsg(dbh: *_dbh) -> *c_char;
  fn sqlite3_changes(dbh: *_dbh) -> c_int;
  fn sqlite3_last_insert_rowid(dbh: *_dbh) -> i64;
  fn sqlite3_complete(sql: *c_char) -> c_int;

  fn sqlite3_prepare_v2(
    hnd: *_dbh,
    sql: *c_char,
    sql_len: c_int,
    shnd: **_stmt,
    tail: **c_char
  ) -> sqlite_result_code;

  fn sqlite3_exec(dbh: *_dbh, sql: *c_char, cb: *_notused, d: *_notused, err: **c_char) -> sqlite_result_code;

  fn sqlite3_step(sth: *_stmt) -> sqlite_result_code;
  fn sqlite3_reset(sth: *_stmt) -> sqlite_result_code;
  fn sqlite3_finalize(sth: *_stmt) -> sqlite_result_code;
  fn sqlite3_clear_bindings(sth: *_stmt) -> sqlite_result_code;

  fn sqlite3_column_name(sth: *_stmt, icol: c_int) -> *c_char;
  fn sqlite3_column_type(sth: *_stmt, icol: c_int) -> c_int;
  fn sqlite3_data_count(sth: *_stmt) -> c_int;
  fn sqlite3_column_bytes(sth: *_stmt, icol: c_int) -> c_int;
  fn sqlite3_column_blob(sth: *_stmt, icol: c_int) -> *u8;

  fn sqlite3_column_text(sth: *_stmt, icol: c_int) -> *c_char;
  fn sqlite3_column_double(sth: *_stmt, icol: c_int) -> float;
  fn sqlite3_column_int(sth: *_stmt, icol: c_int) -> c_int;

  fn sqlite3_bind_blob(sth: *_stmt, icol: c_int, buf: *u8, buflen: c_int, d: c_int) -> sqlite_result_code;
  fn sqlite3_bind_text(sth: *_stmt, icol: c_int, buf: *c_char, buflen: c_int, d: c_int) -> sqlite_result_code;
  fn sqlite3_bind_null(sth: *_stmt, icol: c_int) -> sqlite_result_code;
  fn sqlite3_bind_int(sth: *_stmt, icol: c_int, v: c_int) -> sqlite_result_code;
  fn sqlite3_bind_double(sth: *_stmt, icol: c_int, value: float) -> sqlite_result_code;
  fn sqlite3_bind_parameter_index(sth: *_stmt, name: *c_char) -> c_int;

  fn sqlite3_busy_timeout(dbh: *_dbh, ms: c_int) -> sqlite_result_code;

}

struct sqlite_stmt {
  priv stmt: *_stmt,

  drop {
    log(debug, (~"freeing stmt resource: ", self.stmt));
    sqlite3::sqlite3_finalize(self.stmt);
  }
}

fn sqlite_complete(sql: &str) -> sqlite_result<bool> {
  let r = str::as_c_str(sql, { |_sql|
    sqlite3::sqlite3_complete(_sql)
  }) as int;
  if r == SQLITE_NOMEM as int {
    return Err(SQLITE_NOMEM);
  }
  else if r == 1 {
    return Ok(true);
  }
  else {
    return Ok(false);
  }
}

impl sqlite_stmt {
  fn reset(&self) -> sqlite_result_code {
    sqlite3::sqlite3_reset(self.stmt)
  }

  fn clear_bindings(&self) -> sqlite_result_code {
    sqlite3::sqlite3_clear_bindings(self.stmt)
  }

  fn step(&self) -> sqlite_result_code {
    sqlite3::sqlite3_step(self.stmt)
  }

  fn step_row(&self) -> sqlite_result<sqlite_row_result> {
    let is_row: sqlite_result_code = self.step();
    if is_row == SQLITE_ROW {
      let column_cnt = self.get_column_count();
      let mut i = 0;
      let mut sqlrow = LinearMap();
      while( i < column_cnt ) {
        let name = self.get_column_name(i);
        let coltype = self.get_column_type(i);
        let res = match coltype {
          sqlite_integer => sqlrow.insert(name, integer(self.get_int(i))),
          sqlite_float   => sqlrow.insert(name, number(self.get_num(i))),
          sqlite_text    => sqlrow.insert(name, text(self.get_text(i))),
          sqlite_blob    => sqlrow.insert(name, blob(self.get_blob(i))),
          sqlite_null    => sqlrow.insert(name, null),
        };
        if res == false {
          fail ~"Couldn't insert a value into the map for sqlrow!";
        }
        i += 1;
      }

      Ok(row(sqlrow))
    }
    else if is_row == SQLITE_DONE {
      Ok(done)
    } else {
      Err(is_row)
    }
  }

  fn get_bytes(&self, i: int) -> int {
    sqlite3::sqlite3_column_bytes(self.stmt, i as c_int) as int
  }

  fn get_blob(&self, i: int) -> ~[u8] unsafe {
    let len  = self.get_bytes(i);
    let mut bytes = vec::raw::from_buf_raw(
      sqlite3::sqlite3_column_blob(self.stmt, i as c_int),
      len as uint
    );
    return bytes;
  }

  fn get_int(&self, i: int) -> int {
    return sqlite3::sqlite3_column_int(self.stmt, i as c_int) as int;
  }

  fn get_num(&self, i: int) -> float {
    return sqlite3::sqlite3_column_double(self.stmt, i as c_int);
  }

  fn get_text(&self, i: int) -> ~str unsafe {
    return str::raw::from_c_str( sqlite3::sqlite3_column_text(self.stmt, i as c_int) );
  }

  fn get_bind_index(&self, name: &str) -> int {
    let stmt = self.stmt;
    do str::as_c_str(name) |namebuf| {
      sqlite3::sqlite3_bind_parameter_index(stmt, namebuf) as int
    }
  }

  fn get_column_count(&self) -> int {
    return sqlite3::sqlite3_data_count(self.stmt) as int;
  }

  fn get_column_name(&self, i: int) -> ~str unsafe {
    return str::raw::from_c_str( sqlite3::sqlite3_column_name(self.stmt, i as c_int) );
  }

  fn get_column_type(&self, i: int) -> sqlite_column_type {
    let ct = sqlite3::sqlite3_column_type(self.stmt, i as c_int) as int;
    let mut res = match ct {
      1 /* SQLITE_INTEGER */ => sqlite_integer,
      2 /* SQLITE_FLOAT   */ => sqlite_float,
      3 /* SQLITE_TEXT    */ => sqlite_text,
      4 /* SQLITE_BLOB    */ => sqlite_blob,
      5 /* SQLITE_NULL    */ => sqlite_null,
      _ => fail #fmt("sqlite internal error: Got an unknown column type (%d) back from the library.", ct),
    };
    return res;
  }

  fn get_column_names(&self) -> ~[~str] {
    let cnt  = self.get_column_count();
    let mut i    = 0;
    let mut r    = ~[];
    while(i < cnt){
      vec::push(&mut r, self.get_column_name(i));
      i += 1;
    }
    return r;
  }

  fn bind_params(&self, values: &[sqlite_bind_arg]) -> sqlite_result_code {
    let mut i = 0i;
    for values.each |v| {
      let r = self.bind_param(i, v);
      if r != SQLITE_OK {
        return r;
      }
      i += 1;
    }
    return SQLITE_OK;
  }

  fn bind_param(&self, i: int, value: &sqlite_bind_arg) -> sqlite_result_code unsafe {
    let mut r = match *value {
      text(copy v) => {
        let l = str::len(v);
        str::as_c_str(v, |_v| {
          // FIXME: -1 means: SQLITE_TRANSIENT, so this interface will do lots
          //        of copying when binding text or blob values.
          sqlite3::sqlite3_bind_text(self.stmt, i as c_int, _v, l as c_int, -1 as c_int)
        })
      }

      blob(copy v) => {
        let l = vec::len(v);
        // FIXME: -1 means: SQLITE_TRANSIENT, so this interface will do lots
        //        of copying when binding text or blob values.
        sqlite3::sqlite3_bind_blob(self.stmt, i as c_int, vec::raw::to_ptr(v), l as c_int, -1 as c_int)
      }

      integer(copy v) => { sqlite3::sqlite3_bind_int(self.stmt, i as c_int, v as c_int) }

      number(copy v) => { sqlite3::sqlite3_bind_double(self.stmt, i as c_int, v) }

      null => { sqlite3::sqlite3_bind_null(self.stmt, i as c_int) }

    };

    return r;
  }
}

pub fn sqlite_open(path: &str) -> sqlite_result<sqlite_dbh> {
  let new_dbh : *_dbh = ptr::null();
  let r = str::as_c_str(path, |_path| {
    sqlite3::sqlite3_open(_path, ptr::addr_of(&new_dbh))
  });
  if r != SQLITE_OK {
    Err(r)
  } else {
    debug!("created new dbh: %?", new_dbh);
    Ok(sqlite_dbh { _dbh: new_dbh })
  }
}

#[cfg(test)]
mod tests {

  fn checked_prepare(dbh: sqlite_dbh, sql: &str) -> sqlite_stmt {
    match dbh.prepare(sql, &None) {
      Ok(move s)  => { s }
      Err(x) => { fail fmt!("sqlite error: \"%s\" (%?)", dbh.get_errmsg(), x); }
    }
  }

  fn checked_open() -> sqlite_dbh {
    match sqlite_open(":memory:") {
      Ok(move dbh) => dbh,
      Err(e) => fail e.to_str(),
    }
  }

  fn checked_exec(dbh: &sqlite_dbh, sql: &str) {
    let r = dbh.exec(sql);
    assert r.is_ok();
  }

  #[test]
  fn open_db() {
    checked_open();
  }

  #[test]
  fn exec_create_tbl() {
    let dbh = checked_open();
    checked_exec(&dbh, &"BEGIN; CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT); COMMIT;");
  }

  #[test]
  fn prepare_insert_stmt() {
    let dbh = checked_open();

    checked_exec(&dbh, &"BEGIN; CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT); COMMIT;");
    let sth = checked_prepare(dbh, &"INSERT OR IGNORE INTO test (id) VALUES (1)");
    let res = sth.step();
    debug!("prepare_insert_stmt step res: %?", res);
  }

  #[test]
  fn prepare_select_stmt() {
    let dbh = checked_open();

    checked_exec(&dbh,
      &"BEGIN;
      CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT);
      INSERT OR IGNORE INTO test (id) VALUES (1);
      COMMIT;"
    );

    let sth = checked_prepare(dbh, &"SELECT id FROM test WHERE id = 1;");
    assert sth.step() == SQLITE_ROW;
    assert sth.get_int(0) == 1;
    assert sth.step() == SQLITE_DONE;
  }

  #[test]
  fn prepared_stmt_bind() {
    let dbh = checked_open();

    checked_exec(&dbh, &"BEGIN; CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT); COMMIT;");

    checked_exec(&dbh,
      &"INSERT OR IGNORE INTO test (id) VALUES(2);
        INSERT OR IGNORE INTO test (id) VALUES(3);
        INSERT OR IGNORE INTO test (id) VALUES(4);"
    );
    let sth = checked_prepare(dbh, &"SELECT id FROM test WHERE id > ? AND id < ?");
    assert sth.bind_param(1, &integer(2)) == SQLITE_OK;
    assert sth.bind_param(2, &integer(4)) == SQLITE_OK;

    assert sth.step() == SQLITE_ROW;
    assert sth.get_num(0) as int == 3;
  }

  #[test]
  fn column_names() {
    let dbh = checked_open();

    checked_exec(&dbh,
      &"BEGIN;
        CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT);
        INSERT OR IGNORE INTO test (id, v) VALUES(1, 'leeeee');
        COMMIT;"
    );
    let sth = checked_prepare(dbh, &"SELECT * FROM test");
    assert sth.step() == SQLITE_ROW;
    assert sth.get_column_names() == ~[~"id", ~"v"];
  }

  #[test]
  #[should_fail]
  fn failed_prepare() {
    let dbh = checked_open();

    checked_exec(&dbh,
      &"BEGIN;
        CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT);
        INSERT OR IGNORE INTO test (id, v) VALUES(1, 'leeeee');
        COMMIT;"
    );
    let _sth = checked_prepare(dbh, &"SELECT q FRO test");
  }

  #[test]
  fn bind_param_index() {
    let dbh = checked_open();

    checked_exec(&dbh,
      &"BEGIN;
        CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT);
        INSERT OR IGNORE INTO test (id, v) VALUES(1, 'leeeee');
        COMMIT;"
    );
    let sth = checked_prepare(dbh, &"SELECT * FROM test WHERE v=:Name");
    assert sth.get_bind_index(&":Name") == 1;
  }

  #[test]
  fn last_insert_id() {
    let dbh = checked_open();
    checked_exec(&dbh,
      &"
      BEGIN;
      CREATE TABLE IF NOT EXISTS test (v TEXT);
      INSERT OR IGNORE INTO test (v) VALUES ('This is a really long string.');
      COMMIT;
      "
    );
    debug!("last insert_id: %s", u64::str(dbh.get_last_insert_rowid() as u64));
    assert dbh.get_last_insert_rowid() == 1_i64;
  }

  #[test]
  fn step_row_basics() {
    let dbh = checked_open();
    checked_exec(&dbh,
      &"
      BEGIN;
      CREATE TABLE IF NOT EXISTS test (id INTEGER, k TEXT, v REAL);
      INSERT OR IGNORE INTO test (id, k, v) VALUES(1, 'pi', 3.1415);
      INSERT OR IGNORE INTO test (id, k, v) VALUES(2, 'e', 2.17);
      INSERT OR IGNORE INTO test (id, k, v) VALUES(3, 'o', 1.618);
      COMMIT;
      "
    );
    let sth = checked_prepare(dbh, &"SELECT * FROM test WHERE id=2");
    let r = sth.step_row();
    let possible_row = result::unwrap(r);
    match move possible_row {
      row(move x) => {
        let mut x = x;
        assert x.pop(&~"id") == Some(integer(2));
        assert x.pop(&~"k")  == Some(text(~"e"));
        assert x.pop(&~"v")  == Some(number(2.17));
      }
      done => {
        fail(~"didnt get even one row back.");
      }
    }
  }

  #[test]
  fn check_complete_sql() {
    let r1 = sqlite_complete(&"SELECT * FROM");
    let r2 = sqlite_complete(&"SELECT * FROM bob;");
    assert is_ok_and(r1, false);
    assert is_ok_and(r2, true);

    fn is_ok_and(r: sqlite_result<bool>, v: bool) -> bool {
      assert r.is_ok();
      return r.get() == v;
    }
  }
}

