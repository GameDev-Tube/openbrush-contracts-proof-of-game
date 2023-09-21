// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Extension of [`PSP22`] which supports token wrapping
pub use crate::traits::errors::PSP22Error;
pub use crate::traits::psp22::*;

use openbrush::traits::{
    AccountId,
    Balance,
};

/// The idea of PSP22Wrapper is that it is PSP22 by itself.
/// Wrapper only adds 2 additional methods for depositing and withdrawing.
#[openbrush::wrapper]
pub type PSP22WrapperRef = dyn PSP22Wrapper + PSP22;

#[openbrush::trait_definition]
pub trait PSP22Wrapper: PSP22 {
    /// Allow a user to deposit `amount` of underlying tokens and mint `amount` of the wrapped tokens to `account`
    #[ink(message)]
    fn deposit_for(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    /// Allow a user to burn `amount` of wrapped tokens and withdraw the corresponding number of underlying tokens to `account`
    #[ink(message)]
    fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
}
