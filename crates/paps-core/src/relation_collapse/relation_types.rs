use std::collections::HashMap;

/// 关系类型 —— 人格光谱投射的不同维度
///
/// 这不是"用户与周围人的关系"，而是"这个人格在面对不同类型对象时的参数表现"。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelationType {
    /// 亲密对象 —— 家人、挚友、伴侣
    Intimate,
    /// 陌生人 —— 无任何关系的路人
    Stranger,
    /// 敌对对象 —— 已知的敌人/竞争对手
    Hostile,
    /// 权力上位者 —— 比自己地位高的人
    PowerSuperior,
    /// 权力下位者 —— 比自己地位低的人
    PowerSubordinate,
    /// 依赖对象 —— 自己依赖的人
    Dependent,
    /// 内群体成员 —— 属于自己群体的成员
    IngroupMember,
    /// 外群体成员 —— 不属于自己群体的成员
    OutgroupMember,
}

impl RelationType {
    /// 所有关系类型
    pub fn all() -> Vec<RelationType> {
        vec![
            RelationType::Intimate,
            RelationType::Stranger,
            RelationType::Hostile,
            RelationType::PowerSuperior,
            RelationType::PowerSubordinate,
            RelationType::Dependent,
            RelationType::IngroupMember,
            RelationType::OutgroupMember,
        ]
    }

    /// 关系类型ID
    pub fn id(&self) -> &'static str {
        match self {
            RelationType::Intimate => "intimate",
            RelationType::Stranger => "stranger",
            RelationType::Hostile => "hostile",
            RelationType::PowerSuperior => "power_superior",
            RelationType::PowerSubordinate => "power_subordinate",
            RelationType::Dependent => "dependent",
            RelationType::IngroupMember => "ingroup",
            RelationType::OutgroupMember => "outgroup",
        }
    }

    /// 关系类型中文名
    pub fn name(&self) -> &'static str {
        match self {
            RelationType::Intimate => "亲密对象",
            RelationType::Stranger => "陌生人",
            RelationType::Hostile => "敌对对象",
            RelationType::PowerSuperior => "权力上位者",
            RelationType::PowerSubordinate => "权力下位者",
            RelationType::Dependent => "依赖对象",
            RelationType::IngroupMember => "内群体成员",
            RelationType::OutgroupMember => "外群体成员",
        }
    }

    /// 获取该关系类型的默认参数修饰因子
    ///
    /// modifier > 1.0: 该关系下参数值被放大
    /// modifier < 1.0: 该关系下参数值被压制
    /// modifier = 1.0: 无变化
    ///
    /// 关键参数的关系修饰：
    /// - A009a(内群体痛苦敏感): 对内群体↑, 对外群体↓
    /// - B015a(内群体内疚): 对内群体↑, 对外群体↓
    /// - B021a(内群体情绪传染): 对内群体↑, 对外群体↓
    /// - F061(信任默认值): 亲密↑, 敌对↓
    /// - A008b(社交威胁放大): 敌对↑, 亲密↓
    /// - C031(支配-顺从): 对上位者↓, 对下位者↑
    pub fn default_modifiers(&self) -> HashMap<String, f64> {
        let mut modifiers = HashMap::new();

        match self {
            RelationType::Intimate => {
                modifiers.insert("A009a".into(), 1.3);
                modifiers.insert("B015a".into(), 1.4);
                modifiers.insert("B021a".into(), 1.3);
                modifiers.insert("F061".into(), 1.2);
                modifiers.insert("F061a".into(), 1.3);
                modifiers.insert("A008b".into(), 0.6);
                modifiers.insert("C033".into(), 1.3);
                modifiers.insert("C033a".into(), 1.4);
                modifiers.insert("B022a".into(), 0.7);
                modifiers.insert("F057b".into(), 0.5);
                modifiers.insert("F059b".into(), 0.7);
            }
            RelationType::Stranger => {
                modifiers.insert("F061".into(), 0.8);
                modifiers.insert("F061a".into(), 0.7);
                modifiers.insert("A009c".into(), 0.9);
                modifiers.insert("B015c".into(), 0.8);
                modifiers.insert("F057a".into(), 1.2);
                modifiers.insert("C033c".into(), 0.7);
            }
            RelationType::Hostile => {
                modifiers.insert("A008b".into(), 1.5);
                modifiers.insert("A008".into(), 1.3);
                modifiers.insert("B016c".into(), 1.5);
                modifiers.insert("F061".into(), 0.3);
                modifiers.insert("F061a".into(), 0.2);
                modifiers.insert("A009a".into(), 0.4);
                modifiers.insert("B015a".into(), 0.3);
                modifiers.insert("B021a".into(), 0.3);
                modifiers.insert("C033".into(), 0.3);
                modifiers.insert("B022".into(), 1.5);
                modifiers.insert("D040b".into(), 1.4);
            }
            RelationType::PowerSuperior => {
                modifiers.insert("C031b".into(), 0.5);
                modifiers.insert("C028a".into(), 0.7);
                modifiers.insert("F058a".into(), 1.3);
                modifiers.insert("B022c".into(), 0.8);
                modifiers.insert("D041c".into(), 1.2);
                modifiers.insert("A010".into(), 0.7);
            }
            RelationType::PowerSubordinate => {
                modifiers.insert("C031a".into(), 1.3);
                modifiers.insert("C028a".into(), 1.2);
                modifiers.insert("F058c".into(), 0.7);
                modifiers.insert("B022d".into(), 1.2);
                modifiers.insert("D041c".into(), 0.8);
                modifiers.insert("A010".into(), 1.3);
            }
            RelationType::Dependent => {
                modifiers.insert("C033".into(), 1.2);
                modifiers.insert("C033c".into(), 1.3);
                modifiers.insert("F058".into(), 1.2);
                modifiers.insert("F061".into(), 1.1);
                modifiers.insert("C028".into(), 0.7);
                modifiers.insert("B022a".into(), 0.6);
            }
            RelationType::IngroupMember => {
                modifiers.insert("A009a".into(), 1.2);
                modifiers.insert("B015a".into(), 1.3);
                modifiers.insert("B021a".into(), 1.2);
                modifiers.insert("F061a".into(), 1.2);
                modifiers.insert("C033d".into(), 1.2);
                modifiers.insert("A004a".into(), 1.2);
                modifiers.insert("F056a".into(), 0.8);
                modifiers.insert("C035c".into(), 1.2);
                modifiers.insert("D040a".into(), 0.7);
            }
            RelationType::OutgroupMember => {
                modifiers.insert("A009b".into(), 0.7);
                modifiers.insert("B015b".into(), 0.6);
                modifiers.insert("B021b".into(), 0.7);
                modifiers.insert("F061b".into(), 0.7);
                modifiers.insert("A004b".into(), 0.8);
                modifiers.insert("F056b".into(), 1.3);
                modifiers.insert("C035d".into(), 0.7);
                modifiers.insert("D040b".into(), 1.2);
                modifiers.insert("B016b".into(), 1.2);
            }
        }

        modifiers
    }
}
