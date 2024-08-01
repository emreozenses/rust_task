
fn main (){}
#[test]
fn keygen() {
// Create a new keypair
// wallet : ECVHjPdPX2equSJXaGhVCa9NMsiGwxeScVqNX4xEQkhS
let kp: Keypair = Keypair::new();
println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
println!("");
println!("To save your wallet, copy and paste the following into a JSON file:");
println!("{:?}", kp.to_bytes());
}
