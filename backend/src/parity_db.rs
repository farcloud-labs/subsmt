use parity_db::{Db, Options, clear_column};
use std::{fmt, fs, path::PathBuf, u8};

pub struct ParityDb {
    path: PathBuf,
}

#[derive(Debug)]
pub enum StoreError {
    DbError(parity_db::Error),
    InvalidColumnId,
}

// 实现 Display trait
impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::DbError(e) => write!(f, "Database error: {:?}", e),
            StoreError::InvalidColumnId => write!(f, "Invalid column ID"),
        }
    }
}

impl From<parity_db::Error> for StoreError {
    fn from(error: parity_db::Error) -> Self {
        StoreError::DbError(error)
    }
}

impl ParityDb {

    /// create a new ParityDb instance
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Opens an existing database or creates a new one if it doesn't exist
    pub fn open_or_create(path: impl Into<PathBuf>) -> Result<Db, StoreError> {
        let path = path.into();
        let num_columns = u8::MAX;
        let options = Options::with_columns(&path, num_columns);
        
        let db = Db::open_or_create(&options)?;
        Ok(db)
    }

    /// Insert a value into the specified column
    pub fn insert(&self, column: u8, key: &[u8], value: &[u8]) -> Result<(), StoreError> {
        let db = Self::open_or_create(self.path.clone())?;
        db.commit(vec![(column, key.to_vec(), Some(value.to_vec()))])?;
        Ok(())
    }

    /// Delete a value from the specified column
    pub fn delete(&self, column: u8, key: &[u8]) -> Result<(), StoreError> {
        // self.db.commit(vec![(column, key.to_vec(), None)])?;
        let db = Self::open_or_create(self.path.clone())?;
        db.commit(vec![(column, key.to_vec(), None)])?;
        Ok(())
    }

    /// Get a value from the specified column
    pub fn get(&self, column: u8, key: &[u8]) -> Result<Option<Vec<u8>>, StoreError> {
        // Ok(self.db.get(column, key)?)
        let db = Self::open_or_create(self.path.clone())?;
        Ok(db.get(column, key)?)
    }

    /// Delete the entire database by removing all files
    pub fn destroy(self) -> Result<(), StoreError> {
        if self.path.exists() {
            std::fs::remove_dir_all(&self.path)
                .map_err(|e| StoreError::DbError(parity_db::Error::Io(e)))?;
        }
        
        Ok(())
    }

    /// Clear all data in a column without recreating it
    pub fn clear_column(&self, column: u8) -> Result<(), StoreError> {
        // 执行这个之前需要先关闭数据库
        clear_column(&self.path, column).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_basic_operations() {
        let temp_dir = tempdir().unwrap();
        let store = ParityDb::new(temp_dir.path());

        // Test insert
        let key = b"test_key";
        let value = b"test_value";
        store.insert(0, key, value).unwrap();

        // Test get
        let retrieved = store.get(0, key).unwrap();
        assert_eq!(retrieved, Some(value.to_vec()));

        // Test delete
        store.delete(0, key).unwrap();
        let retrieved = store.get(0, key).unwrap();
        assert_eq!(retrieved, None);

        // Test destroy
        store.destroy().unwrap();
        assert!(!temp_dir.path().exists());
    }

    #[test]
    fn test_reset_column() {
        let temp_dir = tempdir().unwrap();
        let store = ParityDb::new(temp_dir.path());

        // Insert data in both columns
        store.insert(0, b"key1", b"value1").unwrap();
        store.insert(1, b"key2", b"value2").unwrap();

        // Clear column 0
        store.clear_column(0).unwrap();

        // Verify data
        assert_eq!(store.get(0, b"key1").unwrap(), None);
        assert_eq!(store.get(1, b"key2").unwrap(), Some(b"value2".to_vec()));
    }

    #[test]
    fn test_clear_column() {
        let temp_dir = tempdir().unwrap();
        let store = ParityDb::new(temp_dir.path());

        // Insert some test data
        for i in 0..100 {
            let key = format!("key{}", i).into_bytes();
            let value = format!("value{}", i).into_bytes();
            store.insert(0, &key, &value).unwrap();
            store.insert(1, &key, &value).unwrap();
        }

        // Clear column 0
        store.clear_column(0).unwrap();

        // Verify column 0 is empty but column 1 still has data
        for i in 0..100 {
            let key = format!("key{}", i).into_bytes();
            assert_eq!(store.get(0, &key).unwrap(), None);
            assert!(store.get(1, &key).unwrap().is_some());
        }
    }
}