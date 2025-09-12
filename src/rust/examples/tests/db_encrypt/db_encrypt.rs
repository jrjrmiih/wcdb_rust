use wcdb_derive::WCDBTableCoding;

#[derive(WCDBTableCoding)]
pub struct TableForEncrypt {
    #[WCDBField]
    id: i64,
    #[WCDBField]
    name: String,
    #[WCDBField]
    desc: String,
}

impl TableForEncrypt {
    pub fn new() -> Self {
        TableForEncrypt {
            id: 0,
            name: "testName".to_string(),
            desc: "myDesc".to_string(),
        }
    }
}

#[cfg(test)]
pub mod db_encrypt_test {
    use crate::db_encrypt::db_encrypt::{
        DbTableForEncrypt, TableForEncrypt, DB_TABLE_FOR_ENCRYPT_INSTANCE,
    };
    use wcdb::core::database::Database;
    use wcdb::core::handle_orm_operation::HandleORMOperationTrait;

    // wcdb 加密
    #[test]
    pub fn test_encrypt_wcdb() {
        let db_name = "test_encrypt_wcdb.db";
        let table_name = "myTableName";
        let db_path = format!("./target/tmp/{}", db_name);
        let db = Database::new(db_path.as_str(), None);
        db.remove_files().unwrap();
        let password = "123";
        let key = password.as_bytes().to_vec();
        db.set_cipher_key(&key, None, None);
        db.create_table(table_name, &*DB_TABLE_FOR_ENCRYPT_INSTANCE)
            .unwrap();

        check(&db, table_name);
    }

    fn check(db: &Database, table_name: &str) {
        let mut obj = TableForEncrypt::new();
        obj.id = 1234;
        db.insert_object(obj, DbTableForEncrypt::all_fields(), "myTableName")
            .unwrap();

        let select_objs = db
            .get_all_objects(
                DbTableForEncrypt::all_fields(),
                "myTableName",
                None,
                None,
                None,
                None,
            )
            .unwrap();

        assert_eq!(select_objs.len(), 1);
    }

    // 融云加密
    #[test]
    pub fn test_encrypt_rong() {
        let db_name = "test_encrypt_rong.db";
        let table_name = "myTableName";
        let db_path = format!("./target/tmp/{}", db_name);
        let db = Database::new(db_path.as_str(), None);
        db.remove_files().unwrap();
        let password = "123";
        let key = format!("{:x}", md5::compute(password.as_bytes()));
        let key = key.as_bytes().to_vec();
        db.set_cipher_rc_key(&key, None, None);

        db.create_table(table_name, &*DB_TABLE_FOR_ENCRYPT_INSTANCE)
            .unwrap();

        check(&db, table_name);
    }
}
