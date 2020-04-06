use crate::schema::{maps, Map, COLLECTION_SCHEMA, MAP_SCHEMA, SCORE_SCHEMA};
use diesel::{insert_into, prelude::*, sql_query, dsl::sql, SqliteConnection, *};
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
        // join maps, scores, and collections on map id
        let maps = maps::table
            .filter(sql(query))
            .load(&self.conn)
            .map_err(|e| format!("Could not query maps: {}",e))?;
        Ok(maps)
    }
    pub fn delete_maps(&self, maps: &[Map]) -> Result<(), String> {
        Ok(())
    }

    // Collection (ird)
    pub fn insert_collections(&self) -> Result<(), String> {
        Ok(())
    }
    pub fn rename_collection(&self) -> Result<(), String> {
        Ok(())
    }
    pub fn delete_collection(&self) -> Result<(), String> {
        Ok(())
    }

    // Score (id)
    pub fn insert_score(&self) -> Result<(), String> {
        Ok(())
    }
    pub fn delete_score(&self) -> Result<(), String> {
        Ok(())
    }
}
