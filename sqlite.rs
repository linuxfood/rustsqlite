use std;

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

type sqlite_stmt = obj {
  fn step() -> int;
  fn reset() -> int;

  fn get_num(i: int) -> float;
  fn get_int(i: int) -> int;
  fn get_bytes(i: int) -> int;
  fn get_blob(i: int) -> [u8];
  fn get_text(i: int) -> str;

  fn bind_param(i: int, value: sqlite_bind_arg) -> int;
  fn bind_params(values: [sqlite_bind_arg]) -> int;
};


type sqlite_dbh = obj {
  fn get_errmsg() -> str;
  fn prepare(sql: str, &_tail: option::t<str>) -> (sqlite_stmt, int);
  fn exec(sql: str) -> int;
};


#[nolink] #[link_args = "sqlite3.c"]
native mod _sqlite {
  type _dbh;
  type _stmt;

  type _notused;

  fn sqlite3_open(path: str::sbuf, hnd: **_dbh) -> int;
  fn sqlite3_close(dbh: *_dbh) -> int;
  fn sqlite3_errmsg(dbh: *_dbh) -> str::sbuf;

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

}

resource _sqlite_dbh(dbh: *_sqlite::_dbh) {
  //#debug("freeing dbh resource %s", dbh);
  _sqlite::sqlite3_close(dbh);
}

resource _sqlite_stmt(stmt: *_sqlite::_stmt) {
  //#debug("freeing stmt resource %s", stmt);
  _sqlite::sqlite3_finalize(stmt);
}


fn sqlite_open(path: str) -> (sqlite_dbh, int) {
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
    fn step() -> int {
      let stmt = st._stmt;
      ret _sqlite::sqlite3_step(stmt);
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

    fn prepare(sql: str, &_tail: option::t<str>) -> (sqlite_stmt, int) {
      let new_stmt : *_sqlite::_stmt = ptr::null();
      let dbh = st._dbh;
      let r : int = str::as_buf(sql, { |_sql|
        _sqlite::sqlite3_prepare_v2( dbh, _sql, str::byte_len(sql) as int, ptr::addr_of(new_stmt), ptr::null())
      });
      //#debug("created new stmt: %s", new_stmt);
      ret (sqlite_stmt({ _stmt: new_stmt, _res: _sqlite_stmt(new_stmt) }), r);
    }

    fn exec(sql: str) -> int {
      let dbh = st._dbh;
      let r : int = str::as_buf(sql, { |_sql|
        _sqlite::sqlite3_exec(dbh, _sql, ptr::null(), ptr::null(), ptr::null())
      });
      ret r;
    }
  };

  let new_dbh : *_sqlite::_dbh = ptr::null();
  let r : int = str::as_buf(path, { |_path|
    _sqlite::sqlite3_open(_path, ptr::addr_of(new_dbh))
  });
  //#debug("created new dbh: %s", new_dbh);
  ret (sqlite_dbh({ _dbh: new_dbh, _res: _sqlite_dbh(new_dbh) }), r);
}

#[cfg(test)]
mod tests {

  #[test]
  fn open_db() {
    let (_dbh, res) = sqlite_open("test.sqlite3");
    assert res == SQLITE_OK;
  }

  #[test]
  fn exec_create_tbl() {
    let (dbh, res) = sqlite_open("test.sqlite3");
    assert res == SQLITE_OK;

    res = dbh.exec("BEGIN; CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT); COMMIT;");
    assert res == SQLITE_OK;
  }

  #[test]
  fn prepare_insert_stmt() {
    let (dbh, res) = sqlite_open("test.sqlite3");

    assert dbh.exec("BEGIN;") == SQLITE_OK;
    assert res == SQLITE_OK;
    let (sth, res) = dbh.prepare("INSERT OR IGNORE INTO test (id) VALUES (1)", none);
    #error("prepare_insert_stmt res: %d", res);
    assert res == SQLITE_OK;
    res = sth.step();
    #error("prepare_insert_stmt step res: %d", res);
    assert res == SQLITE_DONE;
  }

  #[test]
  fn prepare_select_stmt() {
    let (dbh, res) = sqlite_open("test.sqlite3");

    assert res == SQLITE_OK;
    let (sth, res) = dbh.prepare("SELECT id FROM test WHERE id = 1;", none);
    assert res == SQLITE_OK;
    assert sth.step() == SQLITE_ROW;
    assert sth.get_int(0) == 1;
    assert sth.step() == SQLITE_DONE;
  }

  #[test]
  fn prepared_stmt_bind() {
    let (dbh, res) = sqlite_open("test.sqlite3");

    assert res == SQLITE_OK;
    assert dbh.exec(
      "INSERT OR IGNORE INTO test (id) VALUES(2);"
    + "INSERT OR IGNORE INTO test (id) VALUES(3);"
    + "INSERT OR IGNORE INTO test (id) VALUES(4);"
    ) == SQLITE_OK;
    let (sth, res) = dbh.prepare("SELECT id FROM test WHERE id > ? AND id < ?", none);
    assert res == SQLITE_OK;
    assert sth.bind_param(1, integer(2)) == SQLITE_OK;
    assert sth.bind_param(2, integer(4)) == SQLITE_OK;

    assert sth.step() == SQLITE_ROW;
    assert sth.get_num(0) as int == 3;
  }
}