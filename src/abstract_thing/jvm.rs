use std::time::Instant;
use frunk::Generic;
use rayon::prelude::*;
use tokio::task;
use smallvec::SmallVec;

fn compose<F, G,T>(f: F, g: G) -> impl Fn(T) -> T
    where
        F: Fn(T) -> T,
        G: Fn(T) -> T,
{
    move |x| f(g(x))
}

fn f1() {
    println!("{}","f1");
}

fn f2() {
    println!("{}","f2");
}


pub fn sum100(){
    let rst:i64 = (0..100000000).sum();
    println!("{}",rst)
}

#[test]
fn compute_all() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let start = Instant::now();
    let n = 2;
    let barrier = Arc::new(Barrier::new(n));
    let mut handles = Vec::with_capacity(n);

    for _ in 0..n {
        let c = Arc::clone(&barrier);
        // 每个线程都会在这个点停止，直到所有线程都到达这个点
        let handle = thread::spawn(move || {
            //println!("before wait");
            sum100();
            c.wait();
            //println!("after wait");
        });
        handles.push(handle);
    }

    // Wait for other threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

}

#[tokio::test]
async fn test_toki() {
    let start = Instant::now();
    let task1 = task::spawn(async {
        println!("Task 1 start");
        // 模拟耗时操作
        let r:i64 = (0..100000000).sum();
        println!("Task 1 end,{}",r);
    });

    let task2 = task::spawn(async {
        // 异步任务 2
        println!("Task 2 start");
        // 模拟耗时操作
        let r:i64 = (0..100000000).sum();
        println!("Task 2 end,{}",r);
    });

    // 等待所有任务完成
    task1.await.unwrap();
    task2.await.unwrap();

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);


}

#[test]
fn test() {
    let start = Instant::now();
    let  nums = [0,1];
    nums.par_iter().for_each(|x| {
        let r:i64 = (0..100000000).sum();
        println!("{}",r);
    });

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}




#[test]
fn many_fn() {
    let mut v: SmallVec<[fn(); 4]> = SmallVec::new();
    // 如果我们添加的元素数量不超过4个，那么它们都将在堆栈上分配。
    v.push(f1);
    v.push(f2);
    v.par_iter().for_each(|x| x());
}

#[test]
fn test_many() {
    many_fn();
}

#[test]
fn test_compose() {
    let nums = vec![1, 2, 3, 4, 5];

    let compile = |x| x + 2;
    let translate = |x| x * 3;
    let deal = |x| x + 13;

    let result: Vec<_> = nums
        .into_iter()
        .map(compile)
        .map(translate)
        .map(deal)
        .collect();

    println!("{:?}", result); // 输出：[5, 8, 11, 14, 17]
}