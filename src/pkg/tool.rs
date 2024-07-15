use std::time::{SystemTime, UNIX_EPOCH};

pub fn time_unix(t: SystemTime) ->i64{
    t.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as i64
}

pub fn time_now_unix() ->i64{
    time_unix(SystemTime::now())
}