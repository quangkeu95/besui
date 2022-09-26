use ethers::prelude::*;

pub type HttpProvider = Provider<Http>;

pub const TRANSFER_TOPIC: &'static str = "Transfer(address,address,uint256)";
