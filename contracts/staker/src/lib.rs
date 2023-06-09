#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod staker {
    #[ink(storage)]
    pub struct Staker {
        num: i32,
    }

    impl Staker {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { num: 0 }
        }

        #[ink(message)]
        pub fn stake(&mut self, modif: i32) {
            self.num += modif;
        }

        #[ink(message)]
        pub fn get_stake(&self) -> i32 {
            self.num
        }
    }
}
