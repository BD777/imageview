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
        id: i64,
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
            // println!("row {:?}", row);
            ImagesMeta {
                id: row[0].as_integer().unwrap(),
                path: String::from(row[1].as_string().unwrap()),
                title: String::from(row[2].as_string().unwrap()),
                author: String::from(row[3].as_string().unwrap()),
                intro: String::from(row[4].as_string().unwrap()),
                cover: String::from(row[5].as_string().unwrap()),
                create_time: row[6].as_integer().unwrap(),
                update_time: row[7].as_integer().unwrap(),
            }
        }
    }

    #[serde_as]
    #[derive(serde::Serialize)]
    pub struct Pagination {
        pub current: i32,
        pub page_size: i32,
        pub total: i32,
    }

    #[serde_as]
    #[derive(serde::Serialize)]
    pub struct ImagesMetaList {
        pub list: Vec<ImagesMeta>,
        pub pagination: Pagination,
    }

    #[serde_as]
    #[derive(serde::Serialize)]
    pub struct BrowseSettings {
        meta_id: i64,
        browse_type: String,
        home_page: bool,
        current_path: String,
        current_index: i64,
    }

    impl BrowseSettings {
        pub fn from(row: &[sqlite::Value]) -> Self {
            BrowseSettings {
                meta_id: row[0].as_integer().unwrap(),
                browse_type: String::from(row[1].as_string().unwrap()),
                home_page: if (row[2].as_integer().unwrap()) > 0 { true } else { false },
                current_path: String::from(row[3].as_string().unwrap()),
                current_index: row[4].as_integer().unwrap(),
            }
        }
    }

    impl ImageViewDao {
        pub fn new<T: AsRef<Path>>(path: T) -> Self {
            ImageViewDao {
                conn: sqlite::open(path).unwrap(),
            }
        }

        pub fn init_table(&self) {
            self.conn
                .execute(
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

CREATE TABLE IF NOT EXISTS browsesettings (
    meta_id INTEGER PRIMARY KEY,
    browse_type VARCHAR(16),
    home_page BOOLEAN,
    current_path TEXT,
    current_index INTEGER
);
",
                )
                .unwrap();
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

            let mut statement = self
                .conn
                .prepare(
                    "
INSERT INTO imagesmeta (path, title, author, intro, cover, create_time, update_time)
VALUES (:path, :title, :author, :intro, :cover, :create_time, :update_time);
",
                )
                .unwrap();
            statement.bind_by_name(":path", path).unwrap();
            statement.bind_by_name(":title", title).unwrap();
            statement.bind_by_name(":author", author).unwrap();
            statement.bind_by_name(":intro", intro).unwrap();
            statement.bind_by_name(":cover", cover).unwrap();
            statement.bind_by_name(":create_time", timestamp).unwrap();
            statement.bind_by_name(":update_time", timestamp).unwrap();
            statement.next().unwrap();
        }

        pub fn get_images_meta_list(
            &self,
            search: &str,
            page: i64,
            page_size: i64,
        ) -> Result<ImagesMetaList, String> {
            let mut resp: ImagesMetaList = ImagesMetaList {
                list: vec![],
                pagination: Pagination { current: page as i32, page_size: page_size as i32, total: 0 },
            };
            let search = search.trim();

            if search.len() == 0 {
                let mut cursor = self.conn.prepare(
                    "
                    SELECT COUNT(1) FROM imagesmeta;
                    "
                ).unwrap().into_cursor();
                if let Some(row) = cursor.next().unwrap() {
                    resp.pagination.total = row[0].as_integer().unwrap() as i32;
                }
    
                let statement_result = self.conn.prepare(
                    "
        SELECT * FROM imagesmeta ORDER BY update_time desc LIMIT ? OFFSET ?;
        ",
                );
    
                match statement_result {
                    Ok(statement) => {
                        let mut cursor = statement.into_cursor();
                        cursor
                            .bind(&[
                                sqlite::Value::Integer(page_size as i64),
                                sqlite::Value::Integer(((page - 1) * page_size) as i64),
                            ])
                            .unwrap();
    
                        while let Some(row) = cursor.next().unwrap() {
                            resp.list.push(ImagesMeta::from(row));
                        }
                        Ok(resp)
                    }
                    Err(_error) => Ok(resp),
                }
            } else {
                let mut cursor = self.conn.prepare(
                    "
                    SELECT COUNT(1) FROM imagesmeta WHERE PRINTF('%s:#%s', title, author) LIKE ?;
                    "
                ).unwrap().into_cursor();
                cursor.bind(&[sqlite::Value::String(String::from(format!("%{}%", search)))]).unwrap();
                if let Some(row) = cursor.next().unwrap() {
                    resp.pagination.total = row[0].as_integer().unwrap() as i32;
                }
    
                let statement_result = self.conn.prepare(
                    "
        SELECT * FROM imagesmeta WHERE PRINTF('%s:#%s', title, author) LIKE ? ORDER BY update_time desc LIMIT ? OFFSET ?;
        ",
                );
    
                match statement_result {
                    Ok(statement) => {
                        let mut cursor = statement.into_cursor();
                        cursor
                            .bind(&[
                                sqlite::Value::String(String::from(format!("%{}%", search))),
                                sqlite::Value::Integer(page_size as i64),
                                sqlite::Value::Integer(((page - 1) * page_size) as i64),
                            ])
                            .unwrap();
    
                        while let Some(row) = cursor.next().unwrap() {
                            resp.list.push(ImagesMeta::from(row));
                        }
                        Ok(resp)
                    }
                    Err(_error) => Ok(resp),
                }
            }
        }

        pub fn get_images_meta(&self, id: i64) -> Result<ImagesMeta, String> {
            let statement_result = self.conn.prepare(
                "
                SELECT * FROM imagesmeta WHERE id = ?;
                ",
            );
            
            match statement_result {
                Ok(statement) => {
                    let mut cursor = statement.into_cursor();
                    cursor
                        .bind(&[
                            sqlite::Value::Integer(id),
                        ])
                        .unwrap();
                    
                    while let Some(row) = cursor.next().unwrap() {
                        return Ok(ImagesMeta::from(row));
                    }
                    return Err(String::from("找不到图片信息"));
                },
                Err(_error) => return Err(String::from("找不到图片信息"))
            }
        }

        pub fn delete_images_meta(&self, id: i64) -> Result<(), String> {
            let statement_result = self.conn.prepare(
                "
                DELETE FROM imagesmeta WHERE id = ?;
                ",
            );

            match statement_result {
                Ok(statement) => {
                    let mut cursor = statement.into_cursor();
                    cursor
                        .bind(&[
                            sqlite::Value::Integer(id),
                        ])
                        .unwrap();
                    cursor.next().unwrap();
                    return Ok(());
                },
                Err(_error) => return Err(String::from("找不到图片信息"))
            }
        }

        pub fn update_images_meta(
            &self,
            id: i64,
            path: &str,
            title: &str,
            author: &str,
            intro: &str,
            cover: &str,
        ) {
            let dt = Local::now();
            let timestamp = dt.timestamp() as i64;

            let mut statement = self
                .conn
                .prepare(
                    "
UPDATE imagesmeta SET path=:path, title=:title, author=:author, intro=:intro, cover=:cover, update_time=:update_time
WHERE id=:id;
",
                )
                .unwrap();
            statement.bind_by_name(":id", id).unwrap();
            statement.bind_by_name(":path", path).unwrap();
            statement.bind_by_name(":title", title).unwrap();
            statement.bind_by_name(":author", author).unwrap();
            statement.bind_by_name(":intro", intro).unwrap();
            statement.bind_by_name(":cover", cover).unwrap();
            statement.bind_by_name(":update_time", timestamp).unwrap();
            statement.next().unwrap();
        }

        pub fn update_browse_settings(
            &self, 
            meta_id: i64, 
            browse_type: &str, 
            home_page: bool,
            current_path: &str,
            current_index: i64,
        ) {
            let mut statement = self.conn.prepare(
                "
                INSERT INTO browsesettings(meta_id,browse_type,home_page,current_path,current_index)
                    VALUES (:meta_id, :browse_type, :home_page, :current_path, :current_index)
                    ON CONFLICT(meta_id) DO UPDATE SET
                        browse_type=excluded.browse_type,
                        home_page=excluded.home_page,
                        current_path=excluded.current_path,
                        current_index=excluded.current_index;
                "
            ).unwrap();
            statement.bind_by_name(":meta_id", meta_id).unwrap();
            statement.bind_by_name(":browse_type", browse_type).unwrap();
            statement.bind_by_name(":home_page", home_page as i64).unwrap();
            statement.bind_by_name(":current_path", current_path).unwrap();
            statement.bind_by_name(":current_index", current_index).unwrap();
            statement.next().unwrap();
        }
    
        pub fn get_browse_settings(&self, meta_id: i64) -> Result<BrowseSettings, String> {
            let statement_result = self.conn.prepare(
                "
                SELECT * FROM browsesettings WHERE meta_id = ?;
                ",
            );
            
            match statement_result {
                Ok(statement) => {
                    let mut cursor = statement.into_cursor();
                    cursor
                        .bind(&[
                            sqlite::Value::Integer(meta_id),
                        ])
                        .unwrap();
                    
                    while let Some(row) = cursor.next().unwrap() {
                        return Ok(BrowseSettings::from(row));
                    }
                    return Err(String::from("找不到浏览设置"));
                },
                Err(_error) => return Err(String::from("找不到浏览设置"))
            }
        }
    }
}
