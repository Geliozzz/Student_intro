use anchor_lang::prelude::*;

declare_id!("DdBQiXzDtANFyCvzVosgUGesiDAeVpbJVNNGRtSsYuCf");

const MAX_NAME_LENGTH: usize = 20;
const MAX_MESSAGE_LENGTH: usize = 50;

#[error_code]
enum StudentIntroError {
    #[msg("Name too long")]
    NameTooLong,
    #[msg("The message too long")]
    MessageTooLong,
}

#[program]
pub mod anchor_student_intro {
    use super::*;

    pub fn add_student_intro(
        ctx: Context<AddStudentIntro>,
        name: String,
        message: String,
    ) -> Result<()> {
        // We require that the name is not longer than 20 characters
        require!(name.len() <= MAX_NAME_LENGTH, StudentIntroError::NameTooLong);
 
        // We require that the message is not longer than 50 characters
        require!(message.len() <= MAX_MESSAGE_LENGTH, StudentIntroError::MessageTooLong);
 
        msg!("Student Intro Account Created");
        msg!("Name: {}", name);
        msg!("Message: {}", message);
 
        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.reviewer = ctx.accounts.initializer.key();
        student_intro.name = name;
        student_intro.message = message;
        Ok(())
    }
    pub fn update_student_intro(
        ctx: Context<UpdateStudentIntro>,
        name: String,
        message: String,
    ) -> Result<()> {
        // We require that the name is not longer than 20 characters
        require!(name.len() <= MAX_NAME_LENGTH, StudentIntroError::NameTooLong);
 
        // We require that the message is not longer than 50 characters
        require!(message.len() <= MAX_MESSAGE_LENGTH, StudentIntroError::MessageTooLong);
 
        msg!("Student Intro Account space reallocated");
        msg!("Name: {}", name);
        msg!("Message: {}", message);
 
        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.reviewer = ctx.accounts.initializer.key();
        student_intro.name = name;
        student_intro.message = message;
        Ok(())
    }
    pub fn delete_student_intro(
        _ctx: Context<DeleteStudentIntro>,
        name: String,
    ) -> Result<()> {
        msg!("Student Intro {} deleted", name);
 
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct StudentAccountState {
    pub reviewer: Pubkey,    // 32
    #[max_len(20)]
    pub name: String,       // 4 + len()
    #[max_len(50)]
    pub message: String, // 4 + len()
}
 
const DISCRIMINATOR: usize = 8;

#[derive(Accounts)]
#[instruction(name:String)]
pub struct AddStudentIntro<'info> {
    #[account(
        init,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = DISCRIMINATOR + StudentAccountState::INIT_SPACE
    )]
    pub student_intro: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name:String)]
pub struct UpdateStudentIntro<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = DISCRIMINATOR + StudentAccountState::INIT_SPACE,
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub student_intro: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(name:String)]
pub struct DeleteStudentIntro<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        close = initializer
    )]
    pub student_intro: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
