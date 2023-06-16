

pub async fn test1() {
    use tokio_stream::{self as stream, StreamExt};

    let a = [1, 2, 3];

    assert!(stream::iter(&a).all(|&x| x > 0).await);

    assert!(!stream::iter(&a).all(|&x| x > 2).await);
}

 use tokio::time::{Duration, Instant, sleep};


 pub async fn test2() {
     let instant = Instant::now();
     let three_secs = Duration::from_secs(3);
     sleep(three_secs).await;
     assert!(instant.elapsed() >= three_secs);
 }

use tokio::task::JoinSet;


pub async fn test3() {
    let mut set = JoinSet::new();

    for i in 0..10 {
        set.spawn(async move { i });
    }

    let mut seen = [false; 10];
    while let Some(res) = set.join_next().await {
        let idx = res.unwrap();
        seen[idx] = true;
    }

    for i in 0..10 {
        assert!(seen[i]);
    }
}
