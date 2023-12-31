pub mod data;
use std::vec;

// sql
use sqlx::{
    mysql::{
        // MySql,
        MySqlPool,
        MySqlPoolOptions,
    },
    Row,
    types::chrono::{DateTime, Utc},
};

use self::data::DbData;

// date

pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn new(db: String) -> Database {
        // 创建连接池
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(db.as_str())
            .await
            .expect("连接数据库失败");

        Database { pool }
    }

    async fn select(&self, query: &str) -> Result<Vec<sqlx::mysql::MySqlRow>, sqlx::Error> {
        Ok(sqlx::query(query).fetch_all(&self.pool).await?)
    }

    /// get all series from database
    /// return a vector of SeriesDb
    /// in case u forgot what is SeriesDb
    /// ```
    /// pub struct SeriesDb {
    ///     name: String,
    ///     update_time: DateTime<Utc>,
    ///     info_path: String,
    /// }
    /// ```
    pub async fn get_all_series(&self) -> Vec<DbData> {
        let query = "SELECT * FROM series";

        let mut data: Vec<DbData> = vec![];

        if let Ok(rows) = self.select(query).await {
            for row in rows {
                let name: String = row.get::<String, _>("name");
                let update_time: DateTime<Utc> = row.get::<DateTime<Utc>, _>("update_time");
                let info_path: String = row.get::<String, _>("info_path");
                data.push(DbData::new_series(name, update_time, info_path));
            }
        }
        return data;
    }


    pub async fn get_all_blog(&self) -> Vec<DbData> {
        let query = "SELECT * FROM blog_series";

        let mut data: Vec<DbData> = vec![];

        if let Ok(rows) = self.select(query).await {
            for row in rows {
                let name: String = row.get::<String, _>("name");
                let series: String = row.get::<String, _>("series");
                let update_time: DateTime<Utc> = row.get::<DateTime<Utc>, _>("update_time");
                let info_path: String = row.get::<String, _>("info_path");
                data.push(DbData::new_blog_series(info_path, update_time, series, name));
            }
        }
        return data;
    }

    pub async fn get_all_tags(&self) -> Vec<DbData> {
        let query = "SELECT DISTINCT tag FROM tag_blog";

        let mut data: Vec<DbData> = vec![];

        if let Ok(rows) = self.select(query).await {
            for row in rows {
                let tag: String = row.get::<String, _>("tag");
                data.push(DbData::new_tag(tag));
            }
        }
        return data;
    }

    pub async fn get_blog_from_series(&self, series: &str) -> Vec<DbData> {
        let query = format!("SELECT * FROM blog_series WHERE series = '{}'", series);

        let mut data: Vec<DbData> = vec![];

        if let Ok(rows) = self.select(query.as_str()).await {
            for row in rows {
                let name: String = row.get::<String, _>("name");
                let series: String = row.get::<String, _>("series");
                let update_time: DateTime<Utc> = row.get::<DateTime<Utc>, _>("update_time");
                let info_path: String = row.get::<String, _>("info_path");
                data.push(DbData::new_blog_series(info_path, update_time, series, name));
            }
        }
        return data;
    }


    pub async fn get_blog_from_tag(&self, tag: &str) -> Vec<DbData> {
        let query = format!("SELECT info_path,update_time,series,name FROM tag_blog,blog_series WHERE tag_blog.blog_id=blog_series.id AND tag='{}';", tag);

        let mut data: Vec<DbData> = vec![];

        if let Ok(rows) = self.select(query.as_str()).await {
            for row in rows {
                let name: String = row.get::<String, _>("name");
                let series: String = row.get::<String, _>("series");
                let update_time: DateTime<Utc> = row.get::<DateTime<Utc>, _>("update_time");
                let info_path: String = row.get::<String, _>("info_path");
                data.push(DbData::new_blog_series(info_path, update_time, series, name));
            }
        }
        return data;
    }

    pub async fn get_blog_from_name(&self, name: &str) -> Option<DbData> {
        let query = format!("SELECT * FROM blog_series WHERE name = '{}'", name);

        if let Ok(rows) = self.select(query.as_str()).await{
          let row = rows.get(0);
          let series: String = row?.get::<String, _>("series");
          let update_time: DateTime<Utc> = row?.get::<DateTime<Utc>, _>("update_time");
          let info_path: String = row?.get::<String, _>("info_path");
          return Some(DbData::new_blog_series(info_path, update_time, series, name.to_string()));
        }
        None
    }
}
