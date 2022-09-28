# Hapi

This is a crate that helps implement the use of the Hapi protocol in your smart contract on Near blockchain.

The crate has an AML structure that stores:

- account_id: AccountId - the address of the [aml contract](https://github.com/HAPIprotocol/near-proxy-contract);
- pub aml_conditions: UnorderedMap<Category, RiskScore> - a map of categories and corresponding risk levels that you will add there.

>Note
>
>If the risk level was not set for some categories, then is used the risk level for the category **All**, which is set during initialization.

Methods 
  
- get_aml - Returns the aml accountId and vector of added categories with accepted risk levels.

- update_account_id - Updates account id of aml service.

- update_category - Updates or add category with accepted risk score to aml conditions.

- remove_category - Removes category from aml conditions.

- assert_risk - Checks the category according to the set level of risk. If the risk level is not set for this category, it checks the All category.