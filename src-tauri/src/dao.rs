pub mod imageview_dao {
    use chrono::prelude::*;
    use std::path::Path;
    // use serde::ser::{Serialize, SerializeStruct, Serializer};
    use serde_with::serde_as;

    pub struct ImageViewDao {
        conn: sqlite::Connection,
    }

    #[serde_as]
    #[derive(serde::Serialize)]
    pub struct ImagesMeta {
        path: String,
        title: String,
        author: String,
        intro: String,
        cover: String,
        create_time: i64,
        update_time: i64,
    }

    impl ImagesMeta {
        pub fn from(row: &[sqlite::Value]) -> Self {
            ImagesMeta {
                path: String::from(row[0].as_string().unwrap()),
                title: String::from(row[1].as_string().unwrap()),
                author: String::from(row[2].as_string().unwrap()),
                intro: String::from(row[3].as_string().unwrap()),
                cover: String::from(row[4].as_string().unwrap()),
                create_time: row[5].as_integer().unwrap(),
                update_time: row[6].as_integer().unwrap(),
            }
        }
    }

    #[serde_as]
    #[derive(serde::Serialize)]
    pub struct ImagesMetaList {
        pub list: Vec<ImagesMeta>,
    }

    impl ImageViewDao {
        pub fn new<T: AsRef<Path>>(path: T) -> Self {
            ImageViewDao {
                conn: sqlite::open(path).unwrap(),
            }
        }

        pub fn init_table(&self) {
            self.conn.execute(
                "
                CREATE TABLE IF NOT EXISTS imagesmeta (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    path TEXT,
                    title VARCHAR(128),
                    author VARCHAR(128),
                    intro TEXT,
                    cover TEXT,
                    create_time BIGINT NOT NULL,
                    update_time BIGINT NOT NULL
                );
                CREATE INDEX IF NOT EXISTS 'imagesmeta_author' ON 'imagesmeta' (`author`);
                CREATE INDEX IF NOT EXISTS 'imagesmeta_create_time' ON 'imagesmeta' (`create_time`);
                CREATE INDEX IF NOT EXISTS 'imagesmeta_update_time' ON 'imagesmeta' (`update_time`);
                "
            ).unwrap();
        }

        pub fn add_images_meta(
            &self, 
            path: &str, 
            title: &str, 
            author: &str, 
            intro: &str,
            cover: &str,
        ) {
            let dt = Local::now();
            let timestamp = dt.timestamp() as i64;

            let mut statement = self.conn.prepare(
                "
                INSERT INTO imagesmeta (path, title, author, intro, cover, create_time, update_time)
                VALUES (:path, :title, :author, :intro, :cover, :create_time, :update_time);
                "
            ).unwrap();
            statement.bind_by_name(":path", path).unwrap();
            statement.bind_by_name(":title", title).unwrap();
            statement.bind_by_name(":author", author).unwrap();
            statement.bind_by_name(":intro", intro).unwrap();
            statement.bind_by_name(":cover", cover).unwrap();
            statement.bind_by_name(":create_time", timestamp).unwrap();
            statement.bind_by_name(":update_time", timestamp).unwrap();
            statement.next().unwrap();
        }

        pub fn get_images_meta_list(&self, page: i64, page_size: i64) -> Result<ImagesMetaList, String> {
            let mut cursor = self.conn
                .prepare(
                    "
                    SELECT * FROM imagesmeta ORDER BY update_time desc LIMIT ? OFFSET ?;
                    "
                )
                .unwrap()
                .into_cursor();
            cursor.bind(&[
                sqlite::Value::Integer(page_size as i64),
                sqlite::Value::Integer(((page - 1) * page_size) as i64),
            ]).unwrap();

            let mut resp: ImagesMetaList = ImagesMetaList { list: vec![] };
            while let Some(row) = cursor.next().unwrap() {
                resp.list.push(ImagesMeta::from(row));
            }
            Ok(resp)
        }
    }

}

