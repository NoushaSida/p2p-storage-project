
#[cfg(test)]
mod tests {
   // use super::*;

use storage::Storage;

   #[test]
    fn create_storage() {
        async {
            let storage = Storage::new().await;
            storage.close();
        };
    }

    #[test]
    fn initialize_storage() {
        async {
            let mut storage = Storage::new().await;
            let _ = storage.initialize().await;
            storage.close();
        };
    }
}