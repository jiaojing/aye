//! 一个可运行的模块边界 showcase：销售文件模块、支付目录模块及 adapter。
//! 约定所有金额都是 USD 最小单位；示例不实现持久化、并发事务或支付幂等。

pub mod adapter;
pub mod payments;
pub mod sales;
