use parity_db::{clear_column, Db, Options};
use std::{fmt, path::PathBuf};

pub struct ParityDb {
    db: Option<Db>,
    path: PathBuf,
    num_columns: u8,
}

#[derive(Debug)]
pub enum StoreError {
    DbError(parity_db::Error),
    InvalidColumnId,
}

// Display trait
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
    pub fn new(path: impl Into<PathBuf> + Clone, num_columns: u8) -> Self {
        let options = Options::with_columns(&path.clone().into(), num_columns);
        let db = Db::open_or_create(&options).unwrap();
        Self {
            path: path.into(),
            num_columns,
            db: Some(db),
        }
    }

    fn ensure_db_exsist(&mut self) -> Result<(), StoreError> {
        if self.db.is_none() {
            let db = self.open_or_create()?;
            self.db = Some(db);
        }

        Ok(())
    }

    /// Opens an existing database or creates a new one if it doesn't exist
    pub fn open_or_create(&self) -> Result<Db, StoreError> {
        let options = Options::with_columns(&self.path, self.num_columns);
        let db = Db::open_or_create(&options)?;
        Ok(db)
    }

    fn check_column(&self, column: u8) -> Result<(), StoreError> {
        if column >= self.num_columns {
            return Err(StoreError::InvalidColumnId);
        }
        Ok(())
    }

    /// Insert a value into the specified column
    pub fn insert(&mut self, column: u8, key: &[u8], value: &[u8]) -> Result<(), StoreError> {
        self.check_column(column)?;
        self.ensure_db_exsist()?;
        // let db = self.open_or_create()?;
        self.db.as_mut().unwrap().commit(vec![(column, key.to_vec(), Some(value.to_vec()))])?;
        Ok(())
    }

    /// Delete a value from the specified column
    pub fn delete(&mut self, column: u8, key: &[u8]) -> Result<(), StoreError> {
        self.check_column(column)?;
        self.ensure_db_exsist()?;
        // let db = self.open_or_create()?;
        self.db.as_mut().unwrap().commit(vec![(column, key.to_vec(), None)])?;
        Ok(())
    }

    /// Get a value from the specified column
    pub fn get(&mut self, column: u8, key: &[u8]) -> Result<Option<Vec<u8>>, StoreError> {
        self.check_column(column)?;
        self.ensure_db_exsist()?;
        // let db = self.open_or_create()?;
        Ok(self.db.as_mut().unwrap().get(column, key)?)
    }

    /// Delete the entire database by removing all files
    pub fn destroy(self) -> Result<(), StoreError> {
        if self.path.exists() {
            std::fs::remove_dir_all(&self.path)
                .map_err(|e| StoreError::DbError(parity_db::Error::Io(e)))?;
        }

        Ok(())
    }

    /// Ensure the database is properly closed before operations like clear_column
    fn ensure_closed(&mut self) -> Result<(), StoreError> {
        self.db = None;
        let db = self.open_or_create()?;
        drop(db);
        Ok(())
    }

    /// Clear all data in a column without recreating it
    pub fn clear_column(&mut self, column: u8) -> Result<(), StoreError> {
        self.check_column(column)?;
        self.ensure_closed()?;
        clear_column(&self.path, column)?;
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
        let mut store = ParityDb::new(temp_dir.path(), 2);

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
        let mut store = ParityDb::new(temp_dir.path(), 2);

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
        let mut store = ParityDb::new(temp_dir.path(), 2);

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

    #[test]
    fn test_column_bounds() {
        let temp_dir = tempdir().unwrap();
        let mut store = ParityDb::new(temp_dir.path(), 2);

        // Test inserting to invalid column
        let result = store.insert(2, b"key", b"value");
        assert!(result.is_err());

        // Test getting from invalid column
        let result = store.get(2, b"key");
        assert!(result.is_err());

        // Test deleting from invalid column
        let result = store.delete(2, b"key");
        assert!(result.is_err());
    }
}
