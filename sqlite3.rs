use core::libc::*;
use types::*;

#[link_args="-lsqlite3"]
pub extern {
  fn sqlite3_open(path: *c_char, hnd: **dbh) -> ResultCode;
  fn sqlite3_close(dbh: *dbh) -> ResultCode;
  fn sqlite3_errmsg(dbh: *dbh) -> *c_char;
  fn sqlite3_changes(dbh: *dbh) -> c_int;
  fn sqlite3_last_insert_rowid(dbh: *dbh) -> i64;
  fn sqlite3_complete(sql: *c_char) -> c_int;

  fn sqlite3_prepare_v2(
    hnd: *dbh,
    sql: *c_char,
    sql_len: c_int,
    shnd: **stmt,
    tail: **c_char
  ) -> ResultCode;

  fn sqlite3_exec(dbh: *dbh, sql: *c_char, cb: *_notused, d: *_notused, err: **c_char) -> ResultCode;

  fn sqlite3_step(sth: *stmt) -> ResultCode;
  fn sqlite3_reset(sth: *stmt) -> ResultCode;
  fn sqlite3_finalize(sth: *stmt) -> ResultCode;
  fn sqlite3_clear_bindings(sth: *stmt) -> ResultCode;

  fn sqlite3_column_name(sth: *stmt, icol: c_int) -> *c_char;
  fn sqlite3_column_type(sth: *stmt, icol: c_int) -> c_int;
  fn sqlite3_data_count(sth: *stmt) -> c_int;
  fn sqlite3_column_bytes(sth: *stmt, icol: c_int) -> c_int;
  fn sqlite3_column_blob(sth: *stmt, icol: c_int) -> *u8;

  fn sqlite3_column_text(sth: *stmt, icol: c_int) -> *c_char;
  fn sqlite3_column_double(sth: *stmt, icol: c_int) -> float;
  fn sqlite3_column_int(sth: *stmt, icol: c_int) -> c_int;

  fn sqlite3_bind_blob(sth: *stmt, icol: c_int, buf: *u8, buflen: c_int, d: c_int) -> ResultCode;
  fn sqlite3_bind_text(sth: *stmt, icol: c_int, buf: *c_char, buflen: c_int, d: c_int) -> ResultCode;
  fn sqlite3_bind_null(sth: *stmt, icol: c_int) -> ResultCode;
  fn sqlite3_bind_int(sth: *stmt, icol: c_int, v: c_int) -> ResultCode;
  fn sqlite3_bind_double(sth: *stmt, icol: c_int, value: float) -> ResultCode;
  fn sqlite3_bind_parameter_index(sth: *stmt, name: *c_char) -> c_int;

  fn sqlite3_busy_timeout(dbh: *dbh, ms: c_int) -> ResultCode;
}
