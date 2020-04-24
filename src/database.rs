use crate::schema::*;
use diesel::{dsl::sql, insert_into, prelude::*, sql_query, SqliteConnection, *};
use std::{
    collections::hash_map::{DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

pub struct Database {
    conn: SqliteConnection,
}

impl Database {
    // General
    pub fn connect() -> Result<Database, String> {
        let conn = SqliteConnection::establish("taipo.db").map_err(|e| format!("Could not connect to taipo.db: {}", e))?;
        Ok(Database { conn })
    }
    pub fn exec(&self, query: &str) -> Result<(), String> {
        sql_query(query)
            .execute(&self.conn)
            .map_err(|e| format!("Could not exec statement: {}", e))?;
        Ok(())
    }

    // Tables (cd)
    pub fn create_table(&self, table: &str, schema: &str) -> Result<(), String> {
        self.exec(&format!("CREATE TABLE IF NOT EXISTS {} ({})", table, schema))?;
        Ok(())
    }
    pub fn create_tables(&self) -> Result<(), String> {
        self.create_table("maps", MAP_SCHEMA)?;
        self.create_table("scores", SCORE_SCHEMA)?;
        self.create_table("collections", COLLECTION_SCHEMA)?;
        Ok(())
    }
    pub fn drop_table(&self, table: &str) -> Result<(), String> {
        self.exec(&format!("DROP TABLE IF EXISTS {}", table))?;
        Ok(())
    }
    pub fn drop_tables(&self) -> Result<(), String> {
        self.drop_table("maps")?;
        self.drop_table("scores")?;
        self.drop_table("collections")?;
        Ok(())
    }

    // Map (iqd)
    pub fn insert_maps(&self, maps: &[Map]) -> Result<(), String> {
        insert_into(maps::table)
            .values(maps)
            .execute(&self.conn)
            .map_err(|e| format!("Could not insert maps: {}", e))?;
        Ok(())
    }
    pub fn query_maps(&self, query: &str) -> Result<Vec<Map>, String> {
        allow_tables_to_appear_in_same_query!(maps, scores);
        allow_tables_to_appear_in_same_query!(scores, collections);
        allow_tables_to_appear_in_same_query!(maps, collections);
        // TODO: order by??
        // TODO: only top score?
        let m = maps::table
            .left_join(scores::table.on(maps::id.eq(scores::map)))
            .left_join(collections::table.on(maps::id.eq(collections::map)))
            .filter(sql(if query.len() > 0 { query } else { "TRUE" }))
            .order(maps::dmin.desc())
            .load::<(Map, Option<Score>, Option<Collection>)>(&self.conn)
            .map_err(|e| format!("Could not query maps: {}", e))?
            .iter()
            .map(|j| j.0.clone())
            .collect::<Vec<Map>>();
        Ok(m)
    }
    pub fn delete_maps(&self, maps: &[Map]) -> Result<(), String> {
        Ok(())
    }

    // Collection (irqdu)
    pub fn insert_collections(&self, name: &str, maps: &[Map]) -> Result<(), String> {
        let collections = maps
            .iter()
            .map(|m| {
                let mut s = DefaultHasher::new();
                format!("{}{}", name, m.id).hash(&mut s);
                Collection {
                    id: s.finish().to_string(),
                    name: name.to_string(),
                    map: m.id.clone(),
                }
            })
            .collect::<Vec<Collection>>();
        insert_into(collections::table)
            .values(collections)
            .execute(&self.conn)
            .map_err(|e| format!("Could not insert maps: {}", e))?;
        Ok(())
    }
    pub fn query_collections(&self, query: &str) -> Result<Vec<Collection>, String> {
        let collections = collections::table
            .filter(sql(if query.len() > 0 { query } else { "TRUE" }))
            .load(&self.conn)
            .map_err(|e| format!("Could not query collections: {}", e))?;
        Ok(collections)
    }
    pub fn rename_collection(&self, old: &str, new: &str) -> Result<(), String> {
        if new.len() == 0 {
            self.delete_collection(old)?;
        } else {
            update(collections::table)
                .filter(collections::name.eq(old))
                .set(collections::name.eq(new))
                .execute(&self.conn)
                .map_err(|e| format!("Could not rename collection: {}", e))?;
        }
        Ok(())
    }
    pub fn delete_collection(&self, name: &str) -> Result<(), String> {
        delete(collections::table.filter(collections::name.eq(name)))
            .execute(&self.conn)
            .map_err(|e| format!("Could not delete collection: {}", e))?;
        Ok(())
    }
    pub fn remove_map(&self, map: Map) -> Result<(), String> {
        delete(collections::table.filter(collections::map.eq(map.id)))
            .execute(&self.conn)
            .map_err(|e| format!("Could not delete collection: {}", e))?;
        Ok(())
    }

    // Score (iqd)
    pub fn insert_score(&self, score: Score) -> Result<(), String> {
        insert_into(scores::table)
            .values(score)
            .execute(&self.conn)
            .map_err(|e| format!("Could not insert scores: {}", e))?;
        Ok(())
    }
    pub fn query_scores(&self, query: &str) -> Result<Vec<Score>, String> {
        let scores = scores::table
            .filter(sql(if query.len() > 0 { query } else { "TRUE" }))
            .load(&self.conn)
            .map_err(|e| format!("Could not query scores: {}", e))?;
        Ok(scores)
    }
    pub fn delete_score(&self, score: Score) -> Result<(), String> {
        delete(collections::table.filter(collections::id.eq(score.id)))
            .execute(&self.conn)
            .map_err(|e| format!("Could not delete collection: {}", e))?;
        Ok(())
    }
}
