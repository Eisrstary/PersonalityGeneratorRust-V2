use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// 种子码 —— 确定人格生成的核心
///
/// 同一种子码保证生成完全相同的人格参数。
/// 支持从 u64 或字符串创建。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SeedCode(u64);

impl SeedCode {
    /// 从 u64 创建种子码
    pub fn from_u64(seed: u64) -> Self {
        SeedCode(seed)
    }

    /// 从字符串创建种子码（通过哈希）
    pub fn from_str(s: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        SeedCode(hasher.finish())
    }

    /// 获取内部 u64 值
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// 从此种子码创建确定性 RNG
    pub fn create_rng(&self) -> ChaCha20Rng {
        ChaCha20Rng::seed_from_u64(self.0)
    }
}

impl From<u64> for SeedCode {
    fn from(seed: u64) -> Self {
        SeedCode(seed)
    }
}

impl From<&str> for SeedCode {
    fn from(s: &str) -> Self {
        SeedCode::from_str(s)
    }
}

impl std::fmt::Display for SeedCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
