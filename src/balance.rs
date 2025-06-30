// this was just for testing purposes previouslyy

// use std::str::FromStr;
// use solana_client::rpc_client::RpcClient;
// use solana_sdk::pubkey::Pubkey;


// #[derive(Debug)]
// pub struct SolanaBalance {
//     pub lamports: u64,
//     pub sol: f64,
// }

// #[derive(Debug)]
// pub struct SolanaError {
//     pub error: String,
// }

// pub enum Cluster {
//     Devnet,
// }

// impl Cluster {
//     fn endpoint(&self) -> &str {
//         match self {
//             &Cluster::Devnet => "https://api.devnet.solana.com",
//         }
//     }
// }

// pub fn get_solana_balance(pubkey: &str, cluster: Cluster) -> Result<SolanaBalance, SolanaError> {
//     let rpc = RpcClient::new(String::from(cluster.endpoint()));
//     let pubkey = match Pubkey::from_str(pubkey) {
//         Ok(key) => key,
//         Err(err) => {
//             return Err(SolanaError {
//                 error: err.to_string(),
//             });
//         }
//     };

//     match rpc.get_account(&pubkey) {
//         Ok(acc) => {
//             let balance: SolanaBalance = SolanaBalance {
//                 lamports: acc.lamports,
//                 sol: (acc.lamports as f64) / 1000000000.0,
//             };
//             Ok(balance)
//         }

//         Err(err) => {
//             return Err(SolanaError {
//                 error: err.to_string(),
//             });
//         }
//     }
// }