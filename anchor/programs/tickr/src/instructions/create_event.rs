use anchor_lang::prelude::*;
use mpl_core::ID as MPL_CORE_ID;
use mpl_core::{
    instructions::CreateCollectionV2CpiBuilder,
    types::{Attribute, Attributes, Plugin, PluginAuthority, PluginAuthorityPair},
};

use crate::state::Manager;

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [b"manager", organizer.key().as_ref()],
        bump = manager.bump
    )]
    pub manager: Account<'info, Manager>,
    #[account(mut)]
    pub event: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub organizer: UncheckedAccount<'info>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateEventArgs {
    pub name: String,
    pub category: String,
    pub uri: String,
    pub city: String,
    pub venue: String,
    pub organizer: String,
    pub date: String,
    pub time: String,
    pub capacity: u32,
    pub is_ticket_transferable: bool,
}

impl<'info> CreateEvent<'info> {
    pub fn create_event(&self, args: CreateEventArgs) -> Result<()> {
        let mut collection_plugin: Vec<PluginAuthorityPair> = vec![];

        let attribute_list: Vec<Attribute> = vec![
            Attribute {
                key: "Category".to_string(),
                value: args.category,
            },
            Attribute {
                key: "City".to_string(),
                value: args.city,
            },
            Attribute {
                key: "Venue".to_string(),
                value: args.venue,
            },
            Attribute {
                key: "Artist".to_string(),
                value: args.organizer,
            },
            Attribute {
                key: "Date".to_string(),
                value: args.date,
            },
            Attribute {
                key: "Time".to_string(),
                value: args.time,
            },
            Attribute {
                key: "Capacity".to_string(),
                value: args.capacity.to_string(),
            },
            Attribute {
                key: "IsTicketTransferable".to_string(),
                value: args.is_ticket_transferable.to_string(),
            },
        ];

        collection_plugin.push(PluginAuthorityPair {
            plugin: Plugin::Attributes(Attributes { attribute_list }),
            authority: Some(PluginAuthority::UpdateAuthority),
        });

        CreateCollectionV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.event.to_account_info())
            .update_authority(Some(&self.manager.to_account_info()))
            .payer(&self.payer.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .plugins(collection_plugin)
            .invoke()?;

        Ok(())
    }
}
