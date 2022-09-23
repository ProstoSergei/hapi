use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{AccountId, BorshStorageKey};
use serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Category {
    // for all unspecified categories
    All,
    None,
    // Wallet service - custodial or mixed wallets
    WalletService,
    // Merchant service
    MerchantService,
    // Mining pool
    MiningPool,
    // Low risk exchange - Exchange with high KYC standards
    LowRiskExchange,
    // Medium risk exchange
    MediumRiskExchange,
    // DeFi application
    DeFi,
    // OTC Broker
    OTCBroker,
    // Cryptocurrency ATM
    ATM,
    // Gambling
    Gambling,
    // Illicit organization
    IllicitOrganization,
    // Mixer
    Mixer,
    // Darknet market or service
    DarknetService,
    // Scam
    Scam,
    // Ransomware
    Ransomware,
    // Theft - stolen funds
    Theft,
    // Counterfeit - fake assets
    Counterfeit,
    // Terrorist financing
    TerroristFinancing,
    // Sanctions
    Sanctions,
    // Child abuse and porn materials
    ChildAbuse,
}

pub type RiskScore = u8;

pub const INITIAL_MAX_RISK_LEVEL: RiskScore = 10;

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    AmlCategory,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AML {
    pub account_id: AccountId,
    pub aml_conditions: UnorderedMap<Category, RiskScore>,
}

pub trait AmlManager {
    fn get_aml(&self) -> (&AccountId, Vec<(Category, RiskScore)>);

    fn update_account_id(&mut self, aml_account_id: AccountId);

    fn update_category(&mut self, category: Category, accepted_risk_score: RiskScore);
    fn remove_category(&mut self, category: Category);
}

impl AmlManager for AML {
    fn get_aml(&self) -> (&AccountId, Vec<(Category, RiskScore)>) {
        (
            &self.account_id,
            self.aml_conditions
                .iter()
                .map(|(id, acc)| (id, acc))
                .collect(),
        )
    }

    fn update_account_id(&mut self, account_id: AccountId) {
        self.account_id = account_id;
    }

    fn update_category(&mut self, category: Category, accepted_risk_score: RiskScore) {
        assert!(
            accepted_risk_score <= INITIAL_MAX_RISK_LEVEL,
            "ERR_RISK_SCORE_IS_INVALID"
        );
        assert!(accepted_risk_score > 0, "ERR_RISK_SCORE_IS_INVALID");
        self.aml_conditions.insert(&category, &accepted_risk_score);
    }

    fn remove_category(&mut self, category: Category) {
        assert!(category != Category::All);
        self.aml_conditions.remove(&category);
    }
}

impl AML {
    pub fn new(account_id: AccountId, category: Category, accepted_risk_score: RiskScore) -> AML {
        let mut aml_conditions = UnorderedMap::new(StorageKey::AmlCategory);
        aml_conditions.insert(&category, &accepted_risk_score);
        Self {
            account_id,
            aml_conditions,
        }
    }
}
