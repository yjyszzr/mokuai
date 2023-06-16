use crossbeam::channel;

//单生产者多消费者
#[test]
fn test_water_pime () {

// 创建一个无缓冲的通道
    let (sender, receiver) = channel::unbounded();

// 生成一些消费者线程
    for _ in 0..10 {
        // 克隆接收者
        let r = receiver.clone();
        std::thread::spawn(move || {
            // 从通道接收消息
            while let Ok(msg) = r.recv() {
                println!("received: {}", msg);
            }
        });
    }

// 生产者发送消息
    for i in 0..100 {
        sender.send(i).unwrap();
    }

// 当所有的发送者都离开（即发送者被丢弃）时，接收者会自动关闭。
    drop(sender);

}