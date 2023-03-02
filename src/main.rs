use std::sync::Arc;
use std::pin::Pin;
use tokio;
use tokio::net::TcpListener;
use tokio::sync::Mutex;


struct WaitGroup{
    count: i32,
    //state: bool,
}

impl WaitGroup{
    pub fn new() -> WaitGroup{
        WaitGroup { count: 0}
    }

    pub fn Add(&mut self){
        self.count += 1;
    }

    pub fn Done(&mut self){
        self.count -= 1;
    }

    pub async fn wait(arcself: Arc<tokio::sync::Mutex<WaitGroup>>){
        while arcself.lock().await.count != 0 {

        }   
    }
}


//start point
#[tokio::main]
async fn main() {
    let wg = Arc::new(Mutex::new(WaitGroup::new()));
    create(10, Arc::clone(&wg)).await;
    WaitGroup::wait(wg.clone()).await;
}

async fn create(c: i32, arcwg: Arc<tokio::sync::Mutex<WaitGroup>>){

    for i in 0..c{
        arcwg.lock().await.Add();
        tokio::spawn(job(i, Arc::clone(&arcwg)));
    }
    
}

async fn job(i :i32, arcwg: Arc<tokio::sync::Mutex<WaitGroup>>){
    println!("{}", i);
    arcwg.lock().await.Done();
}