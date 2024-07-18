use std::thread::sleep;
use std::time;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use actix_rt::time::Instant;

#[test]
fn duration(){
    // Duration/Instant
    let five_sec: Duration = Duration::from_secs(5);
    let five_nanos = Duration::from_nanos(5);
    assert_eq!(five_sec+five_nanos,Duration::new(5,5));
    let now = Instant::now();
    let elapsed_time = now.elapsed();
    println!("Rnning slow took {} seconds.",elapsed_time.as_secs());

    // SystemTime
    let one_sec = Duration::from_secs(1);
    let sys_time = SystemTime::now();
    sleep(one_sec);
    println!("sys_time {:?}",sys_time.elapsed().unwrap());
    assert!(sys_time.elapsed().unwrap() >= one_sec);

    // SystemTimeError later 比 earlier还早时触发报错
    let earlier = SystemTime::now() + Duration::from_secs(10);
    let later = SystemTime::now();
    match later.duration_since(earlier) {
        Ok(duration) => println!("Duration: {:?}", duration),
        Err(e) => {
            println!("Error: {:?}",e);
            println!("Duration since error: {:?}", e.duration());
        }
    }

    // TryFromFloatSecsError
    if let Err(e) = Duration::try_from_secs_f64(-1.0){
        println!("Failed conversion to Duration :{e}");
    }
}