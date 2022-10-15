mod test;
use anyhow::Result;
use async_std;
pub use test::*;

#[async_std::test]
async fn two_workers() -> Result<()> {
    todo!()
}
