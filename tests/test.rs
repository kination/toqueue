use tokio::fs::File;
use toqueue::FileQueue;


#[tokio::test]
async fn test_enqueue_dequeue() {
    let file_path = "test_queue.bin";
    let mut queue = FileQueue::new(file_path);

    // Enqueue items
    queue.enqueue(b"{name: Item1}").await.unwrap();
    queue.enqueue(b"{name: seconditem}").await.unwrap();
    queue.enqueue(b"{name: thirditem}").await.unwrap();

    // Dequeue items
    let item1 = queue.dequeue().await.unwrap();
    let item2 = queue.dequeue().await.unwrap();
    let item3 = queue.dequeue().await.unwrap();

    assert_eq!(item1, Some(b"{name: Item1}".to_vec()));
    assert_eq!(item2, Some(b"{name: seconditem}".to_vec()));
    assert_eq!(item3, Some(b"{name: thirditem}".to_vec()));

    // Clean up test file
    let _ = File::open(file_path).await;
    let _ = tokio::fs::remove_file(file_path).await;
}

#[tokio::test]
async fn test_dequeue_empty() {
    let file_path = "empty_queue.bin";
    let mut queue = FileQueue::new(file_path);

    // Dequeue from an empty queue
    let item = queue.dequeue().await.unwrap();
    assert_eq!(item, None);

    // Clean up test file
    let _ = tokio::fs::remove_file(file_path).await;
}
