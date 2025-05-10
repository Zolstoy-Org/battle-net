#![forbid(unsafe_code)]

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error")]
    GenericError
}

async fn get_nb_auctions() -> u32 {
    0
}


#[cfg(test)]
mod tests_battle_net {
    use crate::get_nb_auctions;


    #[tokio::test]
    async fn case_01_nb_auctions() -> anyhow::Result<()> {
        
        assert_eq!(0, get_nb_auctions().await);

        Ok(())
    }
}