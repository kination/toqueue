use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::Path;
use serde::{Serialize, Deserialize};


struct FileQueue {
    file_path: String,
    next_item_position: u64
}

impl FileQueue {
    fn new(file_path: &str) -> Self {
        FileQueue {
            file_path: file_path.to_string(),
            next_item_position: 0
        }
    }

    fn enqueue(&self, item: &[u8]) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&self.file_path)?;

        if file.metadata()?.len() != 0 {
            file.seek(SeekFrom::End(0))?;
            let offset = file.stream_position()?;
            println!("starting write offset {}", offset);
        }
        
        // Write header(item length)
        let item_length = item.len() as u32;
        println!("item length -> {}", item_length);
        file.write_all(&item_length.to_le_bytes())?;
        // Write body(data)
        file.write_all(item)?;

        
        let size = self.get_queue_size(&mut file)?; // Get current queue size
        println!("item size -> {}", size);
        file.seek(SeekFrom::Start(0))?; // Seek to the start to write the new size
        file.write_all(&size.to_le_bytes())?; 

        Ok(())
    }

    fn dequeue(&mut self) -> io::Result<Option<Vec<u8>>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.file_path)?;

        println!("---- start dequeue ----");
        println!("read file from -> {}", self.next_item_position);
        let size = self.get_queue_size(&mut file)?;
        if size == 0 {
            return Ok(None);
        }

        file.seek(SeekFrom::Start(self.next_item_position))?;

        // read byte data with size of u32, which data length has been storred
        let mut len_bytes = [0u8; std::mem::size_of::<u32>()];
        file.read_exact(&mut len_bytes)?;
        
        // read data length data
        let data_len = u32::from_le_bytes(len_bytes) as usize;
        println!("len size -> {}", data_len);

        // write the data by reading size of data_len
        let mut item = vec![0u8; data_len];
        file.read_exact(&mut item)?;
        println!("result item -> {}", String::from_utf8_lossy(&item));

        // Get current position after reading the item
        let current_position = file.stream_position()?;
        // let remaining_bytes = file.metadata()?.len() - current_position
        println!("current_position: {}", current_position);
        self.next_item_position = current_position as u64;
        // TODO: check correct value
        // if self.next_item_position == 0 {
        //     self.next_item_position += data_len as u64 + 1;
        // } else {
        //     self.next_item_position += current_position as u64 + 1;
        // }
        
        println!("next_item_position: {}", self.next_item_position);

        // Seek to the next item's length
        file.seek(SeekFrom::Start(self.next_item_position))?;

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
    let file_name = "test_queue.bin";
    let mut queue: FileQueue= FileQueue::new(&file_name);
    queue.enqueue(b"{name:Item1}").unwrap();
    queue.enqueue(b"{name:seconditem}").unwrap();
    queue.enqueue(b"{name:thirditem}").unwrap();

    println!("Enqueued 3 items");

    // Dequeue and print items
    for _ in 0..3 {
        match queue.dequeue()? {
            Some(item) => println!("Dequeued: {}", String::from_utf8_lossy(&item)),
            None => println!("Queue is empty"),
        }
    }

    std::fs::remove_file(file_name).unwrap();

    Ok(())
}

