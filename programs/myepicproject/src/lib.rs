use anchor_lang::prelude::*;

declare_id!("DKtz9FqVnawRY1f3kY7aqA3oefFJqH9nup28Nh8VCAi3");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs.

        base_account.total_users = 0;
        Ok(())
    }
    pub fn add_user(ctx: Context<AddUser>, another_value: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            user_rank: 0,
            user_address: *user.to_account_info().key,
            another_value: another_value.to_string(),
        };

        // Add it to the gif_list vector.
        base_account.user_list.push(item);
        base_account.total_users += 1;
        Ok(())
    }
    pub fn del_user(ctx: Context<AddUser>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        // Build the struct.

        // Add it to the gif_list vector.
        base_account.user_list.pop();
        base_account.total_users -= 1;
        Ok(())
    }

    pub fn update_user(ctx: Context<AddUser>, another_value: String, criteria1: u64) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let user_address = *user.to_account_info().key;
        // Find the index of the first GIF with the matching user_address.
        let index = base_account
            .user_list
            .iter()
            .position(|item| item.user_address == user_address);

        // Check if the GIF was found.
        match index {
            Some(i) => {
                // Access the GIF with the matching user_address.
                let user = &mut base_account.user_list[i];
                // Update the another_value field of the GIF.
                user.another_value = another_value.to_string();
                user.user_rank += criteria1;
                // Do something with the updated GIF.
                println!(
                    "Updated GIF with user_address {}: {}",
                    user_address, user.user_rank
                );
            }
            None => {
                // Handle the case where the GIF wasn't found.
                println!("No GIF found with user_address {}", user_address);
            }
        }
        Ok(())
    }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 10000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}
// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub user_rank: u64,
    pub user_address: Pubkey,
    pub another_value: String,
}
// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    // Attach a Vector of type ItemStruct to the account.
    pub user_list: Vec<ItemStruct>,
    pub total_users: u64,
}
