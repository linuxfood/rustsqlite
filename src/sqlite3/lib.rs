#![crate_id="sqlite3#0.1"]
#![crate_type = "lib"]
#![feature(globs, phase)]
#[phase(syntax, link)] extern crate log;
extern crate debug;

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

extern crate libc;

pub use cursor::*;
pub use database::*;
use ffi::*;
pub use types::*;
use std::ptr;

pub mod cursor;
pub mod database;
mod ffi;

#[allow(non_camel_case_types)]
pub mod types;



/// Determines whether an SQL statement is complete.
/// See http://www.sqlite.org/c3ref/complete.html
pub fn sqlite_complete(sql: &str) -> SqliteResult<bool> {
    let r = sql.with_c_str( { |_sql|
        unsafe {
            sqlite3_complete(_sql)
        }
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


/// Opens a new database connection.
/// `path` can either be a filesystem path or ":memory:".
/// See http://www.sqlite.org/c3ref/open.html
pub fn open(path: &str) -> SqliteResult<Database> {
    let dbh = ptr::null();
    let r = path.with_c_str( |_path| {
        unsafe {
            sqlite3_open(_path, &dbh)
        }
    });
    if r != SQLITE_OK {
        unsafe {
            sqlite3_close(dbh);
        }
        Err(r)
    } else {
        debug!("`open()`: dbh={:?}", dbh);
        Ok(database_with_handle(dbh))
    }
}

#[cfg(test)]
mod tests {
    use cursor::*;
    use database::*;
    use super::*;
    use types::*;

    fn checked_prepare<'db>(database: &'db Database, sql: &str) -> Cursor<'db> {
        match database.prepare(sql, &None) {
            Ok(s)  => s,
            Err(x) => fail!(format!("sqlite error: \"{}\" ({:?})", database.get_errmsg(), x)),
        }
    }

    fn checked_open() -> Database {
        match open(":memory:") {
            Ok(database) => database,
            Err(ref e) => fail!(e.to_str()),
        }
    }

    fn checked_exec(database: &Database, sql: &str) {
        match database.exec(sql) {
            Ok(..) => {}
            Err(x) => fail!(format!("sqlite error: \"{}\" ({:?})", database.get_errmsg(), x)),
        }
    }

    #[test]
    fn open_db() {
        checked_open();
    }

    #[test]
    fn exec_create_tbl() {
        let database = checked_open();
        checked_exec(&database, "BEGIN; CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT); COMMIT;");
    }

    #[test]
    fn prepare_insert_stmt() {
        let database = checked_open();

        checked_exec(&database, "BEGIN; CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT); COMMIT;");
        let sth = checked_prepare(&database, "INSERT OR IGNORE INTO test (id) VALUES (1)");
        let res = sth.step();
        debug!("test `prepare_insert_stmt`: res={:?}", res);
    }

    #[test]
    fn prepare_select_stmt() {
        let database = checked_open();

        checked_exec(&database,
            "BEGIN;
            CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT);
            INSERT OR IGNORE INTO test (id) VALUES (1);
            COMMIT;"
        );

        let sth = checked_prepare(&database, "SELECT id FROM test WHERE id = 1;");
        assert!(sth.step() == SQLITE_ROW);
        assert!(sth.get_int(0) == 1);
        assert!(sth.step() == SQLITE_DONE);
    }

    #[test]
    fn prepare_select_stmt_blob() {
        let database = checked_open();

        checked_exec(&database,
            "BEGIN;
            CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT);
            INSERT OR IGNORE INTO test (id, v) VALUES (1, x'00123456789abcdeff');
            COMMIT;"
        );

        let sth = checked_prepare(&database, "SELECT v FROM test WHERE id = 1;");
        assert!(sth.step() == SQLITE_ROW);
        assert!(sth.get_blob(0) == vec!(0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xff));
        assert!(sth.step() == SQLITE_DONE);
    }

    #[test]
    fn prepared_stmt_bind_int() {
        let database = checked_open();

        checked_exec(&database, "BEGIN; CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT); COMMIT;");

        checked_exec(&database,
            "INSERT OR IGNORE INTO test (id) VALUES(2);
                INSERT OR IGNORE INTO test (id) VALUES(3);
                INSERT OR IGNORE INTO test (id) VALUES(4);"
        );
        let sth = checked_prepare(&database, "SELECT id FROM test WHERE id > ? AND id < ?");
        assert!(sth.bind_param(1, &Integer(2)) == SQLITE_OK);
        assert!(sth.bind_param(2, &Integer(4)) == SQLITE_OK);

        assert!(sth.step() == SQLITE_ROW);
        assert!(sth.get_f64(0) as int == 3);
    }

    #[test]
    fn prepared_stmt_bind_i64() {
        let database = checked_open();

        checked_exec(&database, "BEGIN; CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT); COMMIT;");

        checked_exec(&database,
            "INSERT OR IGNORE INTO test (id) VALUES(0);
             INSERT OR IGNORE INTO test (id) VALUES(1234567890123456);"
        );
        let sth = checked_prepare(&database, "SELECT id FROM test WHERE id > ?");
        assert!(sth.bind_param(1, &Integer64(1234567890120000)) == SQLITE_OK);

        assert!(sth.step() == SQLITE_ROW);
        assert!(sth.get_i64(0) == 1234567890123456);
    }

    #[test]
    fn prepared_stmt_bind_text() {
        let database = checked_open();

        checked_exec(&database, "BEGIN; CREATE TABLE IF NOT EXISTS test (name text); COMMIT;");

        let sth = checked_prepare(&database, "INSERT INTO test (name) VALUES (?)");

        assert!(sth.bind_param(1, &Text("test".to_string())) == SQLITE_OK);
    }

    #[test]
    fn prepared_stmt_bind_static_text() {
        let database = checked_open();

        checked_exec(&database, "BEGIN; CREATE TABLE IF NOT EXISTS test (name text); COMMIT;");

        let sth = checked_prepare(&database, "INSERT INTO test (name) VALUES (?)");

        assert!(sth.bind_param(1, &StaticText("test")) == SQLITE_OK);
    }

    #[test]
    fn column_names() {
        let database = checked_open();

        checked_exec(&database,
            "BEGIN;
                CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT);
                INSERT OR IGNORE INTO test (id, v) VALUES(1, 'leeeee');
                COMMIT;"
        );
        let sth = checked_prepare(&database, "SELECT * FROM test");
        assert!(sth.step() == SQLITE_ROW);
        assert!(sth.get_column_names() == vec!("id".to_string(), "v".to_string()));
    }

    #[test]
    #[should_fail]
    fn failed_prepare() {
        let database = checked_open();

        checked_exec(&database,
            "BEGIN;
                CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT);
                INSERT OR IGNORE INTO test (id, v) VALUES(1, 'leeeee');
                COMMIT;"
        );
        let _sth = checked_prepare(&database, "SELECT q FRO test");
    }

    #[test]
    fn bind_param_index() {
        let database = checked_open();

        checked_exec(&database,
            "BEGIN;
                CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT);
                INSERT OR IGNORE INTO test (id, v) VALUES(1, 'leeeee');
                COMMIT;"
        );
        let sth = checked_prepare(&database, "SELECT * FROM test WHERE v=:Name");
        assert!(sth.get_bind_index(":Name") == 1);
    }

    #[test]
    fn last_insert_id() {
        let database = checked_open();
        checked_exec(&database,
            "
            BEGIN;
            CREATE TABLE IF NOT EXISTS test (v TEXT);
            INSERT OR IGNORE INTO test (v) VALUES ('This is a really long string.');
            COMMIT;
            "
        );
        debug!("test `last insert_id`: {}", (database.get_last_insert_rowid() as u64).to_str() );
        assert!(database.get_last_insert_rowid() == 1_i64);
    }

    #[test]
    fn step_row_basics() {
        let database = checked_open();
        checked_exec(&database,
            "
            BEGIN;
            CREATE TABLE IF NOT EXISTS test (id INTEGER, k TEXT, v REAL);
            INSERT OR IGNORE INTO test (id, k, v) VALUES(1, 'pi', 3.1415);
            INSERT OR IGNORE INTO test (id, k, v) VALUES(2, 'e', 2.17);
            INSERT OR IGNORE INTO test (id, k, v) VALUES(3, 'o', 1.618);
            COMMIT;
            "
        );
        let sth = checked_prepare(&database, "SELECT * FROM test WHERE id=2");
        let r = sth.step_row();
        let possible_row = r.unwrap();
        match possible_row {
            Some(x) => {
                let mut x = x;
                assert!(x.pop(&"id".to_string()) == Some(Integer(2)));
                assert!(x.pop(&"k".to_string())  == Some(Text("e".to_string())));
                assert!(x.pop(&"v".to_string())  == Some(Float64(2.17)));
            }
            None => {
                fail!("didnt get even one row back.");
            }
        }
    }

    #[test]
    fn check_complete_sql() {
        let r1 = sqlite_complete("SELECT * FROM");
        let r2 = sqlite_complete("SELECT * FROM bob;");
        assert!(is_ok_and(r1, false));
        assert!(is_ok_and(r2, true));

        fn is_ok_and(r: SqliteResult<bool>, v: bool) -> bool {
            assert!(r.is_ok());
            return r.unwrap() == v;
        }
    }
}

