use std::io::SeekFrom;
use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

pub struct FileQueue {
    file_path: String,
    next_item_position: u64,
}

impl FileQueue {
    pub fn new(file_path: &str) -> Self {
        FileQueue {
            file_path: file_path.to_string(),
            next_item_position: 0,
        }
    }

    pub async fn enqueue(&self, item: &[u8]) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&self.file_path)
            .await?;

        let metadata = file.metadata().await?;
        if metadata.len() != 0 {
            file.seek(SeekFrom::End(0)).await?;
            let offset = file.stream_position().await?;
            println!("starting write offset {}", offset);
        }

        // Write header(item length)
        // TODO:
        //  - include several other information of data (length, timestamp...)
        //  - use JSON format
        let item_length = item.len() as u32;
        println!("item length -> {}", item_length);
        file.write_all(&item_length.to_le_bytes()).await?;

        // Write body(data)
        file.write_all(item).await?;

        // Get current queue size
        let size = self.get_queue_size(&mut file).await?;
        println!("item size -> {}", size);
        // Seek to the start to write the new size
        file.seek(SeekFrom::Start(0)).await?;
        file.write_all(&size.to_le_bytes()).await?;

        Ok(())
    }

    pub async fn dequeue(&mut self) -> io::Result<Option<Vec<u8>>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.file_path)
            .await?;

        println!("read file from -> {}", self.next_item_position);
        let size = self.get_queue_size(&mut file).await?;
        if size == 0 {
            return Ok(None);
        }

        file.seek(SeekFrom::Start(self.next_item_position)).await?;

        // read byte data with size of u32, to figure out the size of data
        // TODO:　Change size to read from 'u32' to appropriate header size when updated
        let mut len_bytes = [0u8; std::mem::size_of::<u32>()];
        file.read_exact(&mut len_bytes).await?;
        let data_len = u32::from_le_bytes(len_bytes) as usize;
        println!("len size -> {}", data_len);

        // write the data from queue, by reading size of data_len
        let mut item = vec![0u8; data_len];
        file.read_exact(&mut item).await?;

        // Get current position after reading the item
        let current_position = file.stream_position().await?;
        // let remaining_bytes = file.metadata()?.len() - current_position
        println!("current_position: {}", current_position);
        self.next_item_position = current_position as u64;

        println!("next_item_position: {}", self.next_item_position);

        // Seek to the next item's length
        file.seek(SeekFrom::Start(self.next_item_position)).await?;

        Ok(Some(item))
    }

    async fn get_queue_size(&self, file: &mut File) -> io::Result<u32> {
        file.seek(SeekFrom::Start(0)).await?;
        let mut size_bytes = [0u8; size_of::<u32>()];
        file.read_exact(&mut size_bytes).await?;
        Ok(u32::from_le_bytes(size_bytes))
    }
}
