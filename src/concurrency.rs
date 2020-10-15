use std::thread;
use std::time::Duration;
use std::sync::{mpsc,Mutex,Arc}; //mpsc: multiple producer single consumer.
pub fn fearless_concurrency(){
    println!("################# INSIDE FEARLESS CONCURRENCY #######################");
    let mut list = vec![1,2,3,4,5];
    list.push(6);
    let handle = thread::spawn(move ||{
        for i in 1..10{
            println!("Inside spawning thread {}",i);
            thread::sleep(Duration::from_millis(1));
        }
        println!("Vector elements are:{:#?}",list);
    });
    for i in (1..5+1).rev(){
        println!("Inside Main Thread:{}",i);
        thread::sleep(Duration::from_millis(1));
    }
    //list.push(100); //Does not compile as list is moved into spawned thread using move keyword.
    handle.join().unwrap(); //This wail wait until the spawned thread finishes executing before main thread exits.

    //Channels. Definition:Transmitter sends messages and receiver waits for the receiving message.
    let (tx,rx) = mpsc::channel();
    let tx_stream = mpsc::Sender::clone(&tx);
    let tx_stream1 = mpsc::Sender::clone(&tx);
    thread::spawn(move||{
       let message = String::from("Hello, I am sender");
       tx.send(message).unwrap(); //Here, value is moved to receiver and receiver has ownership of it.
        //println!("Accessing message again:{}",message); //Throw error.
    });
    let received_message = rx.recv().unwrap(); //Here, recv() fun blocks the main thread until it gets the message from sender.
    //let received_message = rx.try_recv().unwrap();//Here, if we use try_recv, main thread will not be blocked.
    // It will panic if sender does not send anything as it is executing before sender.
    println!("Message received is:{}",received_message);

    //Sending multiple messages by the transmitter.
    thread::spawn(move||{
       let message_stream = vec!["Hello","I am","sending","love"];
       for message in message_stream{
           tx_stream.send(message.to_string()).unwrap();
           thread::sleep(Duration::from_millis(1));
       }
       //println!("Tx: I send all my messages");
        //println!("Using message_stream vector again:{:#?}",message_stream); //cannot use here. it is moved in for loop.
    });
    thread::spawn(move||{
        let message_stream = vec!["I'm","second","wife"];
        for message in message_stream{
            tx_stream1.send(message.to_string()).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for recv_message in rx{
        println!("Receiving stream:{}",recv_message);
        //thread::sleep(Duration::from_millis(1));
    }

    //Shared Concurrency. Using Mutexes.
    let mute = Mutex::new(5);
    {
        let mut mute = mute.lock().unwrap();
        *mute = 10;
    } //here, mute which is MutexGuard goes out of scope and automatically, mutex will be released by calling Drop trait for MutexGuard.
    println!("Mutex value is:{:#?}",mute);

    //Sharing a value using mutex between different threads.
    let counter = Arc::new(Mutex::new(0)); //We cannot use, RC:new here because Rust does not allow to be used in threads which is unsafe and not concurrent.
    let mut handles = vec![];
    for i in 0..10{
        let counter = Arc::clone(&counter); //Clone the counter.
        let handle = thread::spawn(move||{
            let mut value = counter.lock().unwrap(); //You can have question: Here, counter is immutable and how we are declaring mutable and modifying it.
            //Answer is from Mutex<T>. Mutex<T> is like a RefCell<T> where it mutates the immutable entity.
            *value+=1;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    println!("Final counter value is:{}",*counter.lock().unwrap());
    println!("Arc counting is:{}",Arc::strong_count(&counter));

    let items = vec![10,20,30,40,50];
    let (tx, rx) = mpsc::channel();
    let th = thread::spawn(move || {
        for i in 1..=items.len() {
            if i%2 == 0 {
                println!("Thread printing even index ele:{}", items[i-1]);
            }
            else {
                tx.send(items[i-1]).unwrap();
                //thread::sleep(Duration::from_secs(1));
            }
        }
    });

    for recv in rx {
        println!("Thread printing odd index ele:{}", recv);
    }

    let vect = vec![11,22,33,44,55];
    let vect_ref = Arc::new(Mutex::new(vect));
    let vect_ref1 = Arc::clone(&vect_ref);
    let vect_ref2 = Arc::clone(&vect_ref);

    let th1 = thread::spawn(move || {
        //thread::sleep(Duration::from_millis(5));
        vect_ref1.lock().unwrap().pop();
    });

    let th2 = thread::spawn(move || {
        vect_ref2.lock().unwrap().push(66);
    });

    th1.join().unwrap();
    th2.join().unwrap();

    println!("Final vector contents are:{:?}", vect_ref.lock().unwrap());
    println!("################# EXITING FEARLESS CONCURRENCY ######################")
}