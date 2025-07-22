use anchor_lang::prelude::*;

declare_id!("BHxAy5hDEo4KK3bMX35KrKoq8dpbuWtzLqKu4LA1sJ8c");

const MAX_ENERGY: u64 = 5;
const TIME_TO_REFIL_ENERGY: i64 = 30;
const MAX_FIRE_TOKEN_SUPPLY: u64 = 1;
const WOOD_TO_FIRE_RATIO: u64 = 1; // 100 wood = 1 fire token (makes it rare!)

#[error_code]
pub enum ErrorCode{
    #[msg("Not Enough Energy")]
    NotEnoughEnergy,
    #[msg("Not Enough Wood")]
    NotEnoughWood,
    #[msg("Fire Token Supply Exhausted")]
    FireTokenSupplyExhausted,
}

#[program]
pub mod lumberjack {
    use super::*;

    pub fn init_game_state(ctx: Context<InitGameState>) -> Result<()> {
        ctx.accounts.game_state.total_fire_tokens_minted = 0;
        ctx.accounts.game_state.max_supply = MAX_FIRE_TOKEN_SUPPLY;
        msg!("Game state initialized. Max fire token supply: {}", MAX_FIRE_TOKEN_SUPPLY);
        Ok(())
    }

    pub fn init_player(ctx: Context<InitPlayer>, name: String) -> Result<()> {
        ctx.accounts.player.name = name;
        ctx.accounts.player.level = 1;
        ctx.accounts.player.xp = 0;
        ctx.accounts.player.wood = 0;
        ctx.accounts.player.energy = MAX_ENERGY;
        ctx.accounts.player.fire_tokens = 0;
        ctx.accounts.player.last_login = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn chop_tree(mut ctx: Context<ChopTree>) -> Result<()> {
        let account = &mut ctx.accounts;
        update_energy(account)?;

        if ctx.accounts.player.energy == 0 {
            return err!(ErrorCode::NotEnoughEnergy);
        }

        ctx.accounts.player.wood = ctx.accounts.player.wood + 1;
        ctx.accounts.player.energy = ctx.accounts.player.energy - 1;
        ctx.accounts.player.xp = ctx.accounts.player.xp + 1;
        
        // Level up logic (every 100 XP)
        let new_level = (ctx.accounts.player.xp / 100) + 1;
        if new_level > ctx.accounts.player.level as u64 {
            ctx.accounts.player.level = new_level as u8;
            msg!("Level up! You are now level {}", ctx.accounts.player.level);
        }

        msg!("You chopped a tree and got 1 wood. You have {} wood and {} energy left", 
             ctx.accounts.player.wood, ctx.accounts.player.energy);
        Ok(())
    }

    pub fn burn_wood_for_fire_tokens(ctx: Context<BurnWoodForFireTokens>, wood_amount: u64) -> Result<()> {
        msg!("Starting burn process. Player wood: {}, Wood to burn: {}", ctx.accounts.player.wood, wood_amount);
        
        // Check if player has enough wood
        if ctx.accounts.player.wood < wood_amount {
            return err!(ErrorCode::NotEnoughWood);
        }

        // Calculate fire tokens to mint (wood_amount / WOOD_TO_FIRE_RATIO)
        let fire_tokens_to_mint = wood_amount / WOOD_TO_FIRE_RATIO;
        msg!("Fire tokens to mint: {}", fire_tokens_to_mint);
        
        // Check if we have remaining supply
        let remaining_supply = ctx.accounts.game_state.max_supply - ctx.accounts.game_state.total_fire_tokens_minted;
        msg!("Remaining supply: {}", remaining_supply);
        
        if fire_tokens_to_mint > remaining_supply {
            return err!(ErrorCode::FireTokenSupplyExhausted);
        }

        // Only proceed if we can mint at least 1 fire token
        if fire_tokens_to_mint > 0 {
            // Burn the wood (exact amount used for fire tokens)
            let wood_used = fire_tokens_to_mint * WOOD_TO_FIRE_RATIO;
            
            msg!("Before update - Player wood: {}, Player fire tokens: {}, Global minted: {}", 
                 ctx.accounts.player.wood, ctx.accounts.player.fire_tokens, ctx.accounts.game_state.total_fire_tokens_minted);
            
            // Update player data
            ctx.accounts.player.wood = ctx.accounts.player.wood - wood_used;
            ctx.accounts.player.fire_tokens = ctx.accounts.player.fire_tokens + fire_tokens_to_mint;
            
            // Update global supply
            ctx.accounts.game_state.total_fire_tokens_minted = ctx.accounts.game_state.total_fire_tokens_minted + fire_tokens_to_mint;

            msg!("After update - Player wood: {}, Player fire tokens: {}, Global minted: {}", 
                 ctx.accounts.player.wood, ctx.accounts.player.fire_tokens, ctx.accounts.game_state.total_fire_tokens_minted);

            msg!("Successfully burned {} wood and received {} fire tokens!", wood_used, fire_tokens_to_mint);
        } else {
            msg!("Need at least {} wood to get 1 fire token. You tried to burn {} wood.", 
                 WOOD_TO_FIRE_RATIO, wood_amount);
        }

        Ok(())
    }

    // View function to check player stats
    pub fn get_player_stats(ctx: Context<GetPlayerStats>) -> Result<()> {
        let player = &ctx.accounts.player;
        msg!("=== PLAYER STATS ===");
        msg!("Name: {}", player.name);
        msg!("Level: {}", player.level);
        msg!("XP: {}", player.xp);
        msg!("Wood: {}", player.wood);
        msg!("Energy: {}", player.energy);
        msg!("Fire Tokens: {}", player.fire_tokens);
        msg!("Last Login: {}", player.last_login);
        Ok(())
    }

    // View function to check game state
    pub fn get_game_stats(ctx: Context<GetGameStats>) -> Result<()> {
        let game_state = &ctx.accounts.game_state;
        msg!("=== GAME STATS ===");
        msg!("Total Fire Tokens Minted: {}", game_state.total_fire_tokens_minted);
        msg!("Max Supply: {}", game_state.max_supply);
        msg!("Remaining Supply: {}", game_state.max_supply - game_state.total_fire_tokens_minted);
        Ok(())
    }
}

pub fn update_energy(ctx: &mut ChopTree) -> Result<()> {
    let mut time_passed: i64 = &Clock::get()?.unix_timestamp - &ctx.player.last_login;
    let mut time_spent: i64 = 0;
    
    while time_passed > TIME_TO_REFIL_ENERGY {
        ctx.player.energy = ctx.player.energy + 1;
        time_passed -= TIME_TO_REFIL_ENERGY;
        time_spent += TIME_TO_REFIL_ENERGY;
        if ctx.player.energy >= MAX_ENERGY {
            break;
        }
    }

    if ctx.player.energy >= MAX_ENERGY {
        ctx.player.last_login = Clock::get()?.unix_timestamp;
    } else {
        ctx.player.last_login += time_spent;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct InitGameState<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 8 + 8, // discriminator + total_fire_tokens_minted + max_supply
        seeds = [b"game_state"],
        bump,
    )]
    pub game_state: Account<'info, GameState>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitPlayer<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 1 + 8 + 8 + 8 + 8 + 8, // discriminator + name + level + xp + wood + energy + fire_tokens + last_login
        seeds = [b"player", signer.key().as_ref()],
        bump,
    )]
    pub player: Account<'info, PlayerData>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChopTree<'info> {
    #[account(
        mut,
        seeds = [b"player", signer.key().as_ref()],
        bump,
    )]
    pub player: Account<'info, PlayerData>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct BurnWoodForFireTokens<'info> {
    #[account(
        mut,
        seeds = [b"player", signer.key().as_ref()],
        bump,
    )]
    pub player: Account<'info, PlayerData>,

    #[account(
        mut,
        seeds = [b"game_state"],
        bump,
    )]
    pub game_state: Account<'info, GameState>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetPlayerStats<'info> {
    #[account(
        seeds = [b"player", signer.key().as_ref()],
        bump,
    )]
    pub player: Account<'info, PlayerData>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetGameStats<'info> {
    #[account(
        seeds = [b"game_state"],
        bump,
    )]
    pub game_state: Account<'info, GameState>,
    pub signer: Signer<'info>,
}

#[account]
pub struct GameState {
    pub total_fire_tokens_minted: u64,
    pub max_supply: u64,
}

#[account]
pub struct PlayerData {
    pub name: String,
    pub level: u8,
    pub xp: u64,
    pub wood: u64,
    pub energy: u64,
    pub fire_tokens: u64,
    pub last_login: i64,
}
