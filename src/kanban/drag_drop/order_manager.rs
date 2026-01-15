use std::cmp::Ordering;

/// 排序管理器
#[derive(Debug, Default)]
pub struct SimpleOrderManager;

impl SimpleOrderManager {
    /// 初始排序值
    const INITIAL_ORDER: f64 = 1000.0;

    /// 排序间隔
    const ORDER_INTERVAL: f64 = 1000.0;

    /// 计算新位置
    pub fn calculate_new_position(
        &self,
        before_order: Option<f64>,
        after_order: Option<f64>,
    ) -> f64 {
        match (before_order, after_order) {
            (None, Some(after)) => {
                if after > Self::ORDER_INTERVAL {
                    after - Self::ORDER_INTERVAL
                } else {
                    self.reorder_and_insert(None, Some(after))
                }
            }
            (Some(before), None) => before + Self::ORDER_INTERVAL,
            (Some(before), Some(after)) => {
                let middle = (before + after) / 2.0;
                if middle != before && middle != after {
                    middle
                } else {
                    self.reorder_and_insert(Some(before), Some(after))
                }
            }
            (None, None) => Self::INITIAL_ORDER,
        }
    }

    fn reorder_and_insert(&self, before_order: Option<f64>, after_order: Option<f64>) -> f64 {
        match (before_order, after_order) {
            (Some(before), Some(after)) => (before + after) / 2.0,
            (Some(before), None) => before + Self::ORDER_INTERVAL,
            (None, Some(after)) => after - Self::ORDER_INTERVAL,
            (None, None) => Self::INITIAL_ORDER,
        }
    }

    /// 批量重新排序
    pub fn reorder_all(&self, orders: &mut [f64]) {
        orders.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        for (i, order) in orders.iter_mut().enumerate() {
            *order = Self::INITIAL_ORDER + i as f64 * Self::ORDER_INTERVAL;
        }
    }
}

/// 可排序 trait
pub trait Sortable {
    fn id(&self) -> &str;
    fn order(&self) -> f64;
    fn set_order(&mut self, order: f64);
}
