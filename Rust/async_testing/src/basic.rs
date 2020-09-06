use async_std::task;
use futures::future;

async fn count_up(stop: u16) {
    for x in 0..=stop {
        println!("UP: {}", x);
        task::sleep(std::time::Duration::from_millis(1000)).await;
    }
}

async fn count_down(stop: u16) {
    for n in (0..=stop).rev() {
        println!("DOWN: {}", n);
        task::sleep(std::time::Duration::from_millis(500)).await;
    }
}

pub fn run() {
    /*
    let a = task::spawn(count_down(10));
    let b = task::spawn(count_up(20));
    println!("Started task!");
    task::block_on(a);
    task::block_on(b);
    println!("Stopped task!");
    */

    let a = task::spawn(count_down(20));
    let b = task::spawn(count_up(10));
    let vec_items = vec![a, b];
    println!("Started task!");
    let f = future::join_all(vec_items);
    task::block_on(f);
    println!("Stopped task!");

    /*
    let f = future::join(count_down(20), count_up(10));
    println!("Started task!");
    task::block_on(f);
    println!("Stopped task!");
    */
}
