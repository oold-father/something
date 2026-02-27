use super::{event::FileEvent, queue::EventQueue, QueueConfig};
use std::path::PathBuf;
use tokio::runtime::Runtime;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_event_primary_path() {
        let path = PathBuf::from("/test/file.jpg");
        let event = FileEvent::Created { path: path.clone() };

        assert_eq!(event.primary_path(), Some(&path));
    }

    #[test]
    fn test_file_event_is_scan_event() {
        let path = PathBuf::from("/test");
        assert!(FileEvent::ScanStart { path }.is_scan_event());
        assert!(FileEvent::ScanEnd { path, count: 10 }.is_scan_event());
        assert!(!FileEvent::Created { path }.is_scan_event());
    }

    #[test]
    fn test_file_event_is_error() {
        let path = PathBuf::from("/test");
        assert!(FileEvent::Error { path, error: "test".to_string() }.is_error());
        assert!(!FileEvent::Created { path }.is_error());
    }

    #[test]
    fn test_event_queue_creation() {
        let config = QueueConfig {
            max_capacity: 100,
            debounce_delay_ms: 500,
            batch_size: 10,
        };
        let queue = EventQueue::new(config);

        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_event_queue_send_recv() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = QueueConfig::default();
            let queue = EventQueue::new(config);

            let path = PathBuf::from("/test/file.jpg");
            let event = FileEvent::Created { path };

            // 发送事件
            queue.send(event.clone()).await.unwrap();
            assert_eq!(queue.len(), 1);

            // 接收事件
            let received = queue.recv().await.unwrap();
            assert_eq!(matches!(&received, FileEvent::Created { .. }), true);
        });
    }

    #[test]
    fn test_file_event_moved() {
        let from = PathBuf::from("/test/old.jpg");
        let to = PathBuf::from("/test/new.jpg");
        let event = FileEvent::Moved { from, to };

        assert_eq!(event.primary_path(), Some(&from));
    }
}
