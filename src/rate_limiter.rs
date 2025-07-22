use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use anyhow::Result;

pub struct RateLimiter {
    requests: Mutex<HashMap<String, Vec<Instant>>>,
    max_requests: usize,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_duration: Duration) -> Self {
        Self {
            requests: Mutex::new(HashMap::new()),
            max_requests,
            window_duration,
        }
    }

    pub fn check_rate_limit(&self, user_id: &str) -> Result<bool> {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();
        
        // 获取用户请求历史
        let user_requests = requests.entry(user_id.to_string()).or_insert_with(Vec::new);
        
        // 清理过期的请求记录
        user_requests.retain(|&time| now.duration_since(time) < self.window_duration);
        
        // 检查是否超过限速
        if user_requests.len() >= self.max_requests {
            return Ok(false);
        }
        
        // 添加当前请求
        user_requests.push(now);
        Ok(true)
    }

    pub fn wait_if_needed(&self, user_id: &str) -> Result<()> {
        while !self.check_rate_limit(user_id)? {
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(10, Duration::from_secs(2))
    }
} 