@dft
Feature: token tx auto scaling storage

  Background:
    Given Reinstall dft canisters
      | key       | name         | symbol | decimals | total_supply | fee_minimum | fee_rate | rate_decimals | owner    |
      | dft_basic | Test Token 1 | TST1   | 18       | 100000000    | 0.01        | 0        | 8             | dft_main |
    And owner "dft_main" set "dft_fee_charger" as fee_to
    And transfer token from "dft_main" to these users
      | token     | user       | amount  |
      | dft_basic | dft_miner  | 1000000 |
      | dft_basic | dft_user1  | 1000000 |
      | dft_basic | dft_user2  | 1000000 |
      | dft_basic | dft_user3  | 1000000 |
      | dft_basic | dft_user4  | 1000000 |
      | dft_basic | dft_user5  | 1000000 |
      | dft_basic | dft_user6  | 1000000 |
      | dft_basic | dft_user7  | 1000000 |
      | dft_basic | dft_user8  | 1000000 |
      | dft_basic | dft_user9  | 1000000 |
      | dft_basic | dft_user10 | 1000000 |
      | dft_basic | dft_user11 | 1000000 |
      | dft_basic | dft_user12 | 1000000 |
      | dft_basic | dft_user13 | 1000000 |


  Scenario:Auto scaling storage when tx count is more than 3000 times
    When Check the storage canisters count is equal to "0" ,by "dft_main"
    Then Transfer token repeat "3010" times
      | token     | from       | to         |
      | dft_basic | dft_main   | dft_miner  |
      | dft_basic | dft_miner  | dft_user1  |
      | dft_basic | dft_user1  | dft_user2  |
      | dft_basic | dft_user2  | dft_user3  |
      | dft_basic | dft_user3  | dft_user4  |
      | dft_basic | dft_user4  | dft_user5  |
      | dft_basic | dft_user5  | dft_user6  |
      | dft_basic | dft_user6  | dft_user7  |
      | dft_basic | dft_user7  | dft_user8  |
      | dft_basic | dft_user8  | dft_user9  |
      | dft_basic | dft_user9  | dft_user10 |
      | dft_basic | dft_user10 | dft_user11 |
      | dft_basic | dft_user11 | dft_user12 |
      | dft_basic | dft_user12 | dft_user13 |
      | dft_basic | dft_user13 | dft_main   |
    Then Check the storage canisters count is equal to "1" ,by "dft_main"
    Then Check the block height "2004" transfer transaction of "dft_basic", the amount should be 1988
    Then Check the block height "999" transfer transaction of "dft_basic", the result should be a forward result
    Then Get the block height "999" transfer transaction of "dft_basic" from archive canister, the amount should be "983"
    Then Check the block height "2010" transfer transaction of "dft_basic", the result should not be a forward result
    Then Check the blocks query of "dft_basic", start block height "3024",size "2", check each transaction is correct
      | amount | fee  |
      | 3008   | 0.01 |
      | 3009   | 0.01 |
    Then Check token "dft_basic"'s archives ,should be
      | start | end  |
      | 0     | 1999 |
