use std::convert::Infallible;
use anyhow::Result;

pub mod telegram;

pub trait RoyalnetService {
	async fn run(self) -> Result<Infallible>;
}
