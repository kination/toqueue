use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::Path;
use serde::{Serialize, Deserialize};


struct FileQueue {
    file_path: String,
}

impl FileQueue {
    fn new(file_path: &str) -> Self {
        FileQueue {
            file_path: file_path.to_string(),
        }
    }

    fn enqueue(&self, item: &[u8]) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&self.file_path)?;

        file.seek(SeekFrom::End(0))?;
        let offset = file.stream_position()?;
        println!("write offset {}", offset);
        
        // Write item length and data
        file.write_all(&(item.len() as u32).to_le_bytes())?;
        file.write_all(item)?;
        
        // Update queue size
        // let mut size = self.get_queue_size(&mut file)?;
        // size += 1;

        file.seek(SeekFrom::Start(0))?;
        let item_length = item.len() as u32;
        println!("writing length: {}", item_length);
        file.write_all(&item_length.to_le_bytes())?;

        Ok(())
    }

    fn dequeue(&self) -> io::Result<Option<Vec<u8>>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.file_path)?;

        println!("read file");
        let size = self.get_queue_size(&mut file)?;
        if size == 0 {
            return Ok(None);
        }

        file.seek(SeekFrom::Start(0))?; // Ensure you are at the correct position
        let mut len_bytes = [0u8; size_of::<u32>()];
        file.read_exact(&mut len_bytes)?;
        // TODO: 4 is to go over data about item size. Need to setup appropriate value
        let len = u32::from_le_bytes(len_bytes) as usize + 4;
        let read_len: usize = len - 4;

        // Check if the file has enough data to read the item
        let current_position = file.stream_position()?;
        let remaining_bytes = file.metadata()?.len() - current_position;

        if remaining_bytes < (read_len as u64) {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough data to read the item"));
        }

        let mut item = vec![0u8; read_len];
        file.read_exact(&mut item)?;

        // Update queue size and shift remaining data
        // let new_size = size - 1;
        file.seek(SeekFrom::Start(0))?;
        file.write_all(&size.to_le_bytes())?;
        println!("write_all");

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        file.set_len(size_of::<u32>() as u64 + buffer.len() as u64)?;
        file.seek(SeekFrom::Start(size_of::<u32>() as u64))?;
        file.write_all(&buffer[len..])?; 

        Ok(Some(item))
    }

    fn get_queue_size(&self, file: &mut File) -> io::Result<u32> {
        file.seek(SeekFrom::Start(0))?;
        let mut size_bytes = [0u8; size_of::<u32>()];
        file.read_exact(&mut size_bytes)?;
        Ok(u32::from_le_bytes(size_bytes))
    }
}


fn main() -> Result<(), std::io::Error> { 
    let queue: FileQueue= FileQueue::new("test_queue.bin");
    queue.enqueue(b"Item1").unwrap();
    queue.enqueue(b"Item2").unwrap();
    queue.enqueue(b"Item3").unwrap();
    // queue.dequeue().unwrap();

    println!("Enqueued 3 items");
    // queue.dequeue().unwrap();

    // Dequeue and print items
    for _ in 0..3 {
        match queue.dequeue()? {
            Some(item) => println!("Dequeued: {}", String::from_utf8_lossy(&item)),
            None => println!("Queue is empty"),
        }
    }

    Ok(())
}
