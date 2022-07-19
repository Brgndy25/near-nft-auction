use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::ext_contract;
use near_sdk::PanicOnDefault;
use near_sdk::{env, Gas};
use near_sdk::{near_bindgen, AccountId, Promise};
use std::str::FromStr;

pub const NFT_ACCOUNT: &str = "nft.examples.testnet";
pub const NFT_TRANSFER_GAS: Gas = Gas(5_000_000_000_000);

#[ext_contract(ext_nft)]
trait NonFungibleToken {
    fn nft_transfer(
        &mut self,
        receiver_id: String,
        token_id: String,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    auc_token_id: String,
    owner_id: AccountId,
    currentbid: u128,
    highestbidder: AccountId,
    bids: Vec<Bids>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Bids {
    account: AccountId,
    bid_amount: u128,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, auc_token_id: String) -> Self {
        Self {
            auc_token_id,
            owner_id,
            currentbid: 0,
            highestbidder: "example.near".parse().unwrap(),
            bids: Vec::new(),
        }
    }

    pub fn place_bid(&mut self, amount: u128, bid_account: AccountId) {
        if amount > self.currentbid {
            self.currentbid = amount;
            self.highestbidder = bid_account.clone();
            self.bids.push(Bids {
                account: bid_account,
                bid_amount: amount,
            });
        } else {
            env::panic_str("INVALID AMOUNT");
        }
    }

    pub fn view_highestbid(&mut self) {
        env::log_str(&format!("CURRENT BID IS {}", self.currentbid).to_string());
    }

    #[private]
    pub fn view_bids(&mut self) {
        env::log_str(&format!("BID HISTORY{:#?}", self.bids).to_string());
    }

    /*  pub fn auction_end(&mut self) {
        env::log_str(&format!("AMOUNT DEPOSITED {}", self.currentbid).to_string());
        let deal = Promise::new(self.owner_id.clone())
            .transfer(self.currentbid * 1000000000000000000000000);
        let transfer_nft = ext_nft::nft_transfer(
            self.highestbidder.to_string(),
            self.auc_token_id.to_string(),
            None,
            None,
            AccountId::from_str(NFT_ACCOUNT).unwrap(),
            1,
            NFT_TRANSFER_GAS,
        );
    }
    */
    #[payable]
    pub fn auction_end(&mut self) {
        env::log_str(&format!("AMOUNT DEPOSITED {}", self.currentbid).to_string());
        if env::attached_deposit() < self.currentbid {
            env::panic_str("NOT ENOUGH NEAR");
        } else {
            let deal = Promise::new(self.owner_id.clone()).transfer(env::attached_deposit());
            let transfer_nft = ext_nft::nft_transfer(
                self.highestbidder.to_string(),
                self.auc_token_id.to_string(),
                None,
                None,
                AccountId::from_str(NFT_ACCOUNT).unwrap(),
                1,
                NFT_TRANSFER_GAS,
            );
        }
    }

    pub fn show_owner(&mut self) {
        env::log_str(&format!("{}", self.owner_id).to_string())
    }

    #[private]
    pub fn show_token(&mut self) {
        env::log_str(&format!("{}", self.auc_token_id).to_string())
    }
}
