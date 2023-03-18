use std::process;

use redis::{Commands, Client, RedisResult, ErrorKind};

fn get_redis_connection() -> RedisResult<redis::Connection> {
    let client = Client::open("redis://127.0.0.1/")?;
    client.get_connection()
}

pub struct Conn {
    redis: redis::Connection,
    conn_id: String
}

impl Conn {
    pub fn new() -> Self {
        let pid = process::id();
        let conn_id = format!("{pid}");
        Self { 
            redis: get_redis_connection().expect("Could not connect to redis"),
            conn_id,
        }
    }

    fn acquire_lock(&mut self, key: &str) -> redis::RedisResult<bool> {
        let lock_key = format!("lock.{key}");
        let lock_acquired = self.redis.set_nx(&lock_key, &self.conn_id)?;
        Ok(lock_acquired)
    }

    fn read_lock(&mut self, key: &str) -> redis::RedisResult<String> {
        let lock_key = format!("lock.{key}");
        self.redis.get(lock_key)
    }

    fn release_lock(&mut self, key: &str) -> redis::RedisResult<()> {
        let lock_key = format!("lock.{key}");
        self.redis.del(lock_key)?;
        Ok(())
    }

    pub fn read_rvc_db(&mut self, key: &str) -> RedisResult<String> {
        let lock_state = self.acquire_lock(key)?;
        if !lock_state {
            return Err((ErrorKind::TryAgain, "Lock is held").into());
        }
        self.redis.get(key)
    }

    pub fn write_rvc_db(&mut self, key: &str, value: &str) -> RedisResult<()> {
        let lock_state = self.read_lock(key)?;
        if lock_state != self.conn_id {
            return Err((ErrorKind::TryAgain, "Lock is not held by this process").into());
        }
        self.redis.set(key, value)?;
        self.release_lock(key)
    }
}
