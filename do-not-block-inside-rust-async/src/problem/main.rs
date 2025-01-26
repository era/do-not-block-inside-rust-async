use crossbeam_channel::bounded;
use tokio;
use tokio::task;


// using only 2 threads so we clearly hit the problem faster
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    // so we can only send one message per time
    // it means that if we call send twice
    // without reading from the channel
    // the thread should block
    // waiting for someone to read
    let (s, r) = bounded(1);


    let mut tasks = Vec::with_capacity(5);

    for i in 0..5 {
        let s_clone = s.clone();
        // given that no one is reading the channel,
        // we should see one task starting and finishing
        // and other 2 other threads starging and getting stuck
        // should start after it
        tasks.push(task::spawn(async move {
            println!("task {} starting", i);
            s_clone.send(i).unwrap();
            println!("task {} finished", i);
            i
        }));
    }

    // let's sleep so that we yield control of this thread
    // and leave room for other things to run
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    println!("we woke up");

    // in theory we now reading from the channel after wake up
    // and we shold see other tasks running, because we are emptying the channel
    for i in 0..5 {
        println!("receiving {}", r.recv().unwrap());
    }


    for task in tasks {
        println!("result of the tasks: {}", task.await.unwrap());
    }


    println!("Done :)")


}
