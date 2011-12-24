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

type sqlite_stmt = obj {
  fn step() -> int;
  fn reset() -> int;
  fn get_int(i: int) -> int;
  fn get_text(i: int) -> str;
};

type sqlite_dbh = obj {
  fn exec(sql: str) -> int;
  fn prepare(sql: str, &_tail: option::t<str>) -> (sqlite_stmt, int);
  fn get_errmsg() -> str;
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
  fn sqlite3_column_int64(sth: *_stmt, icol: int) -> i64;
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

    fn get_int(i: int) -> int {
      let stmt = st._stmt;
      ret _sqlite::sqlite3_column_int64(stmt, i) as int;
    }

    fn get_text(i: int) -> str unsafe {
      let stmt = st._stmt;
      ret str::str_from_cstr( _sqlite::sqlite3_column_text(stmt, i) );
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
  }

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
    res = dbh.exec("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT);");
    assert res == SQLITE_OK;
  }

  #[test]
  fn prepare_insert_stmt() {
    let (dbh, res) = sqlite_open("test.sqlite3");
    assert res == SQLITE_OK;
    let (sth, res) = dbh.prepare("INSERT OR IGNORE INTO test (id) VALUES (1);", none);
    assert res == SQLITE_OK;
    assert sth.step() == SQLITE_DONE;
  }

  #[test]
  fn prepare_select_stmt() {
    let (dbh, res) = sqlite_open("test.sqlite3");
    assert res == SQLITE_OK;
    let (sth, res) = dbh.prepare("SELECT id FROM test;", none);
    assert res == SQLITE_OK;
    assert sth.step() == SQLITE_ROW;
    assert sth.get_int(0) == 1;
    assert sth.step() == SQLITE_DONE;
  }
}