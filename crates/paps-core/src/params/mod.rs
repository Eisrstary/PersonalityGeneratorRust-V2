use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::types::{ParameterSpec, ParamId, Domain};

mod domain_a;
mod domain_b;
mod domain_c;
mod domain_d;
mod domain_e;
mod domain_f;
mod domain_g;
mod domain_h;

/// 全局参数注册表
pub struct ParamRegistry {
    /// 所有参数（含叶子参数和可拆参数）
    all_params: HashMap<ParamId, ParameterSpec>,
    /// 仅叶子参数
    leaf_params: Vec<ParamId>,
    /// 仅顶层参数
    top_level_params: Vec<ParamId>,
    /// 按领域索引
    by_domain: HashMap<Domain, Vec<ParamId>>,
}

impl ParamRegistry {
    /// 获取全局单例
    pub fn global() -> &'static ParamRegistry {
        &GLOBAL_REGISTRY
    }

    /// 获取所有参数
    pub fn all_params(&self) -> &HashMap<ParamId, ParameterSpec> {
        &self.all_params
    }

    /// 获取单个参数
    pub fn get(&self, id: &str) -> Option<&ParameterSpec> {
        self.all_params.get(id)
    }

    /// 获取所有叶子参数ID列表
    pub fn leaf_param_ids(&self) -> &[ParamId] {
        &self.leaf_params
    }

    /// 获取所有叶子参数的数量
    pub fn leaf_param_count(&self) -> usize {
        self.leaf_params.len()
    }

    /// 获取所有顶层参数ID列表
    pub fn top_level_param_ids(&self) -> &[ParamId] {
        &self.top_level_params
    }

    /// 按领域获取参数ID列表
    pub fn by_domain(&self, domain: Domain) -> &[ParamId] {
        self.by_domain.get(&domain).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// 获取所有叶子参数的值域信息（用于生成随机值）
    pub fn leaf_param_value_ranges(&self) -> Vec<(&ParamId, &crate::types::ValueRange)> {
        self.leaf_params.iter()
            .filter_map(|id| self.all_params.get(id).map(|p| (id, &p.value_range)))
            .collect()
    }
}

/// 构建全局注册表
fn build_registry() -> ParamRegistry {
    let mut all_params: HashMap<ParamId, ParameterSpec> = HashMap::new();
    let mut leaf_params: Vec<ParamId> = Vec::new();
    let mut top_level_params: Vec<ParamId> = Vec::new();
    let mut by_domain: HashMap<Domain, Vec<ParamId>> = HashMap::new();

    // 收集所有领域的参数
    let domains: Vec<(Domain, Vec<ParameterSpec>)> = vec![
        (Domain::A, domain_a::domain_a_params()),
        (Domain::B, domain_b::domain_b_params()),
        (Domain::C, domain_c::domain_c_params()),
        (Domain::D, domain_d::domain_d_params()),
        (Domain::E, domain_e::domain_e_params()),
        (Domain::F, domain_f::domain_f_params()),
        (Domain::G, domain_g::domain_g_params()),
        (Domain::H, domain_h::domain_h_params()),
    ];

    for (domain, params) in domains {
        let mut domain_ids = Vec::new();
        for param in params {
            let id = param.id.clone();
            let is_leaf = param.is_leaf;
            let is_top = param.parent_id.is_none();

            if is_leaf {
                leaf_params.push(id.clone());
            }
            if is_top {
                top_level_params.push(id.clone());
            }
            domain_ids.push(id.clone());
            all_params.insert(id, param);
        }
        by_domain.insert(domain, domain_ids);
    }

    ParamRegistry {
        all_params,
        leaf_params,
        top_level_params,
        by_domain,
    }
}

static GLOBAL_REGISTRY: Lazy<ParamRegistry> = Lazy::new(build_registry);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_loaded() {
        let reg = ParamRegistry::global();
        assert!(reg.leaf_param_count() > 100, "Should have many leaf params");
        assert!(reg.top_level_params.len() >= 66, "Should have at least 66 top-level params");
    }

    #[test]
    fn test_get_param() {
        let reg = ParamRegistry::global();
        assert!(reg.get("A001").is_some());
        assert!(reg.get("NONEXISTENT").is_none());
    }

    #[test]
    fn test_domains_have_params() {
        let reg = ParamRegistry::global();
        for domain in [Domain::A, Domain::B, Domain::C, Domain::D, Domain::E, Domain::F, Domain::G, Domain::H] {
            assert!(!reg.by_domain(domain).is_empty(), "Domain {:?} should have params", domain);
        }
    }
}
