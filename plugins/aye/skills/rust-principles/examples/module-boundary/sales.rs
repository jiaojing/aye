//! 文件模块拥有订单规则、受控值与自己的 Error。

use std::num::{NonZeroU32, NonZeroU64};

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("order ID must be nonzero")]
    InvalidId,
    #[error("order units must be positive")]
    InvalidUnits,
    #[error("unit price must be positive")]
    InvalidPrice,
    #[error("order is already confirmed")]
    AlreadyConfirmed,
    #[error("order total overflows")]
    TotalOverflow,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrderId(NonZeroU64);

impl TryFrom<u64> for OrderId {
    type Error = Error;

    fn try_from(raw: u64) -> std::result::Result<Self, Self::Error> {
        NonZeroU64::new(raw).map(Self).ok_or(Error::InvalidId)
    }
}

/// 模块内部状态；调用方读取确认事实，不依赖内部 enum 的组织。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Draft,
    Confirmed { total_minor: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Order {
    id: OrderId,
    units: NonZeroU32,
    state: State,
}

/// 销售发布的业务事实。私有字段保证只能由确认后的订单产生。
#[derive(Clone, Copy, Debug)]
pub struct Confirmation {
    order_id: OrderId,
    total_minor: u64,
}

impl Confirmation {
    pub fn order_id(&self) -> u64 {
        self.order_id.0.get()
    }

    pub fn total_minor(&self) -> u64 {
        self.total_minor
    }
}

impl Order {
    pub fn draft(id: OrderId, units: u32) -> Result<Self> {
        let units = NonZeroU32::new(units).ok_or(Error::InvalidUnits)?;
        Ok(Self {
            id,
            units,
            state: State::Draft,
        })
    }

    pub fn confirmation(&self) -> Option<Confirmation> {
        match self.state {
            State::Draft => None,
            State::Confirmed { total_minor } => Some(Confirmation {
                order_id: self.id,
                total_minor,
            }),
        }
    }
}

/// 纯规则返回新快照；输入不变，持久化并发控制仍由存储契约负责。
pub fn confirm(order: &Order, unit_price_minor: u64) -> Result<Order> {
    if !matches!(order.state, State::Draft) {
        return Err(Error::AlreadyConfirmed);
    }
    if unit_price_minor == 0 {
        return Err(Error::InvalidPrice);
    }
    let total_minor = unit_price_minor
        .checked_mul(u64::from(order.units.get()))
        .ok_or(Error::TotalOverflow)?;
    Ok(Order {
        state: State::Confirmed { total_minor },
        ..*order
    })
}
