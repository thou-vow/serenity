use serenity::model::prelude::*;

pub fn print_ranking() {
    let sizes = [
        ("ActionExecution", std::mem::size_of::<ActionExecution>()),
        ("Activity", std::mem::size_of::<Activity>()),
        ("ActivityAssets", std::mem::size_of::<ActivityAssets>()),
        ("ActivityButton", std::mem::size_of::<ActivityButton>()),
        ("ActivityEmoji", std::mem::size_of::<ActivityEmoji>()),
        ("ActivityFlags", std::mem::size_of::<ActivityFlags>()),
        ("ActivityParty", std::mem::size_of::<ActivityParty>()),
        ("ActivitySecrets", std::mem::size_of::<ActivitySecrets>()),
        ("ActivityTimestamps", std::mem::size_of::<ActivityTimestamps>()),
        ("AffectedRole", std::mem::size_of::<AffectedRole>()),
        ("CommandInteraction", std::mem::size_of::<CommandInteraction>()),
        ("CommandPermissionsUpdateEvent", std::mem::size_of::<CommandPermissionsUpdateEvent>()),
        ("ApplicationFlags", std::mem::size_of::<ApplicationFlags>()),
        ("ApplicationId", std::mem::size_of::<ApplicationId>()),
        ("Attachment", std::mem::size_of::<Attachment>()),
        ("AttachmentId", std::mem::size_of::<AttachmentId>()),
        ("AuditLogEntry", std::mem::size_of::<AuditLogEntry>()),
        ("AuditLogEntryId", std::mem::size_of::<AuditLogEntryId>()),
        ("AuditLogs", std::mem::size_of::<AuditLogs>()),
        ("AutoModActionExecutionEvent", std::mem::size_of::<AutoModActionExecutionEvent>()),
        ("AutoModRuleCreateEvent", std::mem::size_of::<AutoModRuleCreateEvent>()),
        ("AutoModRuleDeleteEvent", std::mem::size_of::<AutoModRuleDeleteEvent>()),
        ("AutoModRuleUpdateEvent", std::mem::size_of::<AutoModRuleUpdateEvent>()),
        ("AutocompleteOption", std::mem::size_of::<AutocompleteOption>()),
        ("Ban", std::mem::size_of::<Ban>()),
        ("BotGateway", std::mem::size_of::<BotGateway>()),
        ("ChannelCreateEvent", std::mem::size_of::<ChannelCreateEvent>()),
        ("ChannelDeleteEvent", std::mem::size_of::<ChannelDeleteEvent>()),
        ("ChannelId", std::mem::size_of::<ChannelId>()),
        ("ChannelMention", std::mem::size_of::<ChannelMention>()),
        ("ChannelPinsUpdateEvent", std::mem::size_of::<ChannelPinsUpdateEvent>()),
        ("ChannelUpdateEvent", std::mem::size_of::<ChannelUpdateEvent>()),
        ("ClientStatus", std::mem::size_of::<ClientStatus>()),
        ("Colour", std::mem::size_of::<Colour>()),
        ("CommandData", std::mem::size_of::<CommandData>()),
        ("CommandDataOption", std::mem::size_of::<CommandDataOption>()),
        ("CommandDataResolved", std::mem::size_of::<CommandDataResolved>()),
        ("CommandId", std::mem::size_of::<CommandId>()),
        ("CommandPermissionId", std::mem::size_of::<CommandPermissionId>()),
        ("CommandVersionId", std::mem::size_of::<CommandVersionId>()),
        ("Connection", std::mem::size_of::<Connection>()),
        ("CurrentApplicationInfo", std::mem::size_of::<CurrentApplicationInfo>()),
        ("CurrentUser", std::mem::size_of::<CurrentUser>()),
        ("Embed", std::mem::size_of::<Embed>()),
        ("EmbedAuthor", std::mem::size_of::<EmbedAuthor>()),
        ("EmbedField", std::mem::size_of::<EmbedField>()),
        ("EmbedFooter", std::mem::size_of::<EmbedFooter>()),
        ("EmbedImage", std::mem::size_of::<EmbedImage>()),
        ("EmbedProvider", std::mem::size_of::<EmbedProvider>()),
        ("EmbedThumbnail", std::mem::size_of::<EmbedThumbnail>()),
        ("EmbedVideo", std::mem::size_of::<EmbedVideo>()),
        ("Emoji", std::mem::size_of::<Emoji>()),
        ("EmojiId", std::mem::size_of::<EmojiId>()),
        ("EmojiIdentifier", std::mem::size_of::<EmojiIdentifier>()),
        ("EmojiIdentifierParseError", std::mem::size_of::<EmojiIdentifierParseError>()),
        ("FollowedChannel", std::mem::size_of::<FollowedChannel>()),
        ("Gateway", std::mem::size_of::<Gateway>()),
        ("GatewayIntents", std::mem::size_of::<GatewayIntents>()),
        ("GenericId", std::mem::size_of::<GenericId>()),
        ("Guild", std::mem::size_of::<Guild>()),
        ("GuildBanAddEvent", std::mem::size_of::<GuildBanAddEvent>()),
        ("GuildBanRemoveEvent", std::mem::size_of::<GuildBanRemoveEvent>()),
        ("GuildChannel", std::mem::size_of::<GuildChannel>()),
        ("GuildCreateEvent", std::mem::size_of::<GuildCreateEvent>()),
        ("GuildDeleteEvent", std::mem::size_of::<GuildDeleteEvent>()),
        ("GuildEmojisUpdateEvent", std::mem::size_of::<GuildEmojisUpdateEvent>()),
        ("GuildId", std::mem::size_of::<GuildId>()),
        ("GuildInfo", std::mem::size_of::<GuildInfo>()),
        ("GuildIntegrationsUpdateEvent", std::mem::size_of::<GuildIntegrationsUpdateEvent>()),
        ("GuildMemberAddEvent", std::mem::size_of::<GuildMemberAddEvent>()),
        ("GuildMemberRemoveEvent", std::mem::size_of::<GuildMemberRemoveEvent>()),
        ("GuildMemberUpdateEvent", std::mem::size_of::<GuildMemberUpdateEvent>()),
        ("GuildMembersChunkEvent", std::mem::size_of::<GuildMembersChunkEvent>()),
        ("GuildPreview", std::mem::size_of::<GuildPreview>()),
        ("GuildPrune", std::mem::size_of::<GuildPrune>()),
        ("GuildRoleCreateEvent", std::mem::size_of::<GuildRoleCreateEvent>()),
        ("GuildRoleDeleteEvent", std::mem::size_of::<GuildRoleDeleteEvent>()),
        ("GuildRoleUpdateEvent", std::mem::size_of::<GuildRoleUpdateEvent>()),
        ("GuildScheduledEventCreateEvent", std::mem::size_of::<GuildScheduledEventCreateEvent>()),
        ("GuildScheduledEventDeleteEvent", std::mem::size_of::<GuildScheduledEventDeleteEvent>()),
        ("GuildScheduledEventUpdateEvent", std::mem::size_of::<GuildScheduledEventUpdateEvent>()),
        ("GuildScheduledEventUserAddEvent", std::mem::size_of::<GuildScheduledEventUserAddEvent>()),
        (
            "GuildScheduledEventUserRemoveEvent",
            std::mem::size_of::<GuildScheduledEventUserRemoveEvent>(),
        ),
        ("GuildStickersUpdateEvent", std::mem::size_of::<GuildStickersUpdateEvent>()),
        ("GuildUpdateEvent", std::mem::size_of::<GuildUpdateEvent>()),
        ("GuildWelcomeChannel", std::mem::size_of::<GuildWelcomeChannel>()),
        ("GuildWelcomeScreen", std::mem::size_of::<GuildWelcomeScreen>()),
        ("GuildWidget", std::mem::size_of::<GuildWidget>()),
        ("Incident", std::mem::size_of::<Incident>()),
        ("IncidentUpdate", std::mem::size_of::<IncidentUpdate>()),
        ("InstallParams", std::mem::size_of::<InstallParams>()),
        ("Integration", std::mem::size_of::<Integration>()),
        ("IntegrationAccount", std::mem::size_of::<IntegrationAccount>()),
        ("IntegrationApplication", std::mem::size_of::<IntegrationApplication>()),
        ("IntegrationCreateEvent", std::mem::size_of::<IntegrationCreateEvent>()),
        ("IntegrationDeleteEvent", std::mem::size_of::<IntegrationDeleteEvent>()),
        ("IntegrationId", std::mem::size_of::<IntegrationId>()),
        ("IntegrationUpdateEvent", std::mem::size_of::<IntegrationUpdateEvent>()),
        ("InteractionResponseFlags", std::mem::size_of::<InteractionResponseFlags>()),
        ("InteractionCreateEvent", std::mem::size_of::<InteractionCreateEvent>()),
        ("InteractionId", std::mem::size_of::<InteractionId>()),
        ("Invite", std::mem::size_of::<Invite>()),
        ("InviteChannel", std::mem::size_of::<InviteChannel>()),
        ("InviteCreateEvent", std::mem::size_of::<InviteCreateEvent>()),
        ("InviteDeleteEvent", std::mem::size_of::<InviteDeleteEvent>()),
        ("InviteGuild", std::mem::size_of::<InviteGuild>()),
        ("InviteStageInstance", std::mem::size_of::<InviteStageInstance>()),
        ("Maintenance", std::mem::size_of::<Maintenance>()),
        ("Member", std::mem::size_of::<Member>()),
        ("Message", std::mem::size_of::<Message>()),
        ("MessageActivity", std::mem::size_of::<MessageActivity>()),
        ("MessageApplication", std::mem::size_of::<MessageApplication>()),
        ("ComponentInteraction", std::mem::size_of::<ComponentInteraction>()),
        ("ComponentInteractionData", std::mem::size_of::<ComponentInteractionData>()),
        ("MessageCreateEvent", std::mem::size_of::<MessageCreateEvent>()),
        ("MessageDeleteBulkEvent", std::mem::size_of::<MessageDeleteBulkEvent>()),
        ("MessageDeleteEvent", std::mem::size_of::<MessageDeleteEvent>()),
        ("MessageFlags", std::mem::size_of::<MessageFlags>()),
        ("MessageFlags", std::mem::size_of::<MessageFlags>()),
        ("MessageId", std::mem::size_of::<MessageId>()),
        ("MessageInteraction", std::mem::size_of::<MessageInteraction>()),
        ("MessageReaction", std::mem::size_of::<MessageReaction>()),
        ("MessageReference", std::mem::size_of::<MessageReference>()),
        ("MessageUpdateEvent", std::mem::size_of::<MessageUpdateEvent>()),
        ("ModalInteraction", std::mem::size_of::<ModalInteraction>()),
        ("ModalInteractionData", std::mem::size_of::<ModalInteractionData>()),
        ("Options", std::mem::size_of::<Options>()),
        ("PartialChannel", std::mem::size_of::<PartialChannel>()),
        ("PartialCurrentApplicationInfo", std::mem::size_of::<PartialCurrentApplicationInfo>()),
        ("PartialGuild", std::mem::size_of::<PartialGuild>()),
        ("PartialGuildChannel", std::mem::size_of::<PartialGuildChannel>()),
        ("PartialMember", std::mem::size_of::<PartialMember>()),
        ("PermissionOverwrite", std::mem::size_of::<PermissionOverwrite>()),
        ("Permissions", std::mem::size_of::<Permissions>()),
        ("PingInteraction", std::mem::size_of::<PingInteraction>()),
        ("Presence", std::mem::size_of::<Presence>()),
        ("PresenceUpdateEvent", std::mem::size_of::<PresenceUpdateEvent>()),
        ("PresenceUser", std::mem::size_of::<PresenceUser>()),
        ("PrivateChannel", std::mem::size_of::<PrivateChannel>()),
        ("Reaction", std::mem::size_of::<Reaction>()),
        ("ReactionAddEvent", std::mem::size_of::<ReactionAddEvent>()),
        ("ReactionConversionError", std::mem::size_of::<ReactionConversionError>()),
        ("ReactionRemoveAllEvent", std::mem::size_of::<ReactionRemoveAllEvent>()),
        ("ReactionRemoveEvent", std::mem::size_of::<ReactionRemoveEvent>()),
        ("Ready", std::mem::size_of::<Ready>()),
        ("ReadyEvent", std::mem::size_of::<ReadyEvent>()),
        ("ResolvedOption", std::mem::size_of::<ResolvedOption>()),
        ("ResumedEvent", std::mem::size_of::<ResumedEvent>()),
        ("RichInvite", std::mem::size_of::<RichInvite>()),
        ("Role", std::mem::size_of::<Role>()),
        ("RoleId", std::mem::size_of::<RoleId>()),
        ("RoleTags", std::mem::size_of::<RoleTags>()),
        ("Rule", std::mem::size_of::<Rule>()),
        ("RuleId", std::mem::size_of::<RuleId>()),
        ("ScheduledEvent", std::mem::size_of::<ScheduledEvent>()),
        ("ScheduledEventId", std::mem::size_of::<ScheduledEventId>()),
        ("ScheduledEventMetadata", std::mem::size_of::<ScheduledEventMetadata>()),
        ("ScheduledEventUser", std::mem::size_of::<ScheduledEventUser>()),
        ("SessionStartLimit", std::mem::size_of::<SessionStartLimit>()),
        ("ShardInfo", std::mem::size_of::<ShardInfo>()),
        ("SkuId", std::mem::size_of::<SkuId>()),
        ("StageInstance", std::mem::size_of::<StageInstance>()),
        ("StageInstanceCreateEvent", std::mem::size_of::<StageInstanceCreateEvent>()),
        ("StageInstanceDeleteEvent", std::mem::size_of::<StageInstanceDeleteEvent>()),
        ("StageInstanceId", std::mem::size_of::<StageInstanceId>()),
        ("StageInstanceUpdateEvent", std::mem::size_of::<StageInstanceUpdateEvent>()),
        ("Sticker", std::mem::size_of::<Sticker>()),
        ("StickerId", std::mem::size_of::<StickerId>()),
        ("StickerItem", std::mem::size_of::<StickerItem>()),
        ("StickerPack", std::mem::size_of::<StickerPack>()),
        ("StickerPackBannerId", std::mem::size_of::<StickerPackBannerId>()),
        ("StickerPackId", std::mem::size_of::<StickerPackId>()),
        ("SystemChannelFlags", std::mem::size_of::<SystemChannelFlags>()),
        ("TargetId", std::mem::size_of::<TargetId>()),
        ("Team", std::mem::size_of::<Team>()),
        ("TeamMember", std::mem::size_of::<TeamMember>()),
        ("ThreadCreateEvent", std::mem::size_of::<ThreadCreateEvent>()),
        ("ThreadDeleteEvent", std::mem::size_of::<ThreadDeleteEvent>()),
        ("ThreadListSyncEvent", std::mem::size_of::<ThreadListSyncEvent>()),
        ("ThreadMember", std::mem::size_of::<ThreadMember>()),
        ("ThreadMemberFlags", std::mem::size_of::<ThreadMemberFlags>()),
        ("ThreadMemberUpdateEvent", std::mem::size_of::<ThreadMemberUpdateEvent>()),
        ("ThreadMembersUpdateEvent", std::mem::size_of::<ThreadMembersUpdateEvent>()),
        ("ThreadMetadata", std::mem::size_of::<ThreadMetadata>()),
        ("ThreadUpdateEvent", std::mem::size_of::<ThreadUpdateEvent>()),
        ("ThreadsData", std::mem::size_of::<ThreadsData>()),
        ("TriggerMetadata", std::mem::size_of::<TriggerMetadata>()),
        ("TypingStartEvent", std::mem::size_of::<TypingStartEvent>()),
        ("UnavailableGuild", std::mem::size_of::<UnavailableGuild>()),
        ("User", std::mem::size_of::<User>()),
        ("UserId", std::mem::size_of::<UserId>()),
        ("UserPublicFlags", std::mem::size_of::<UserPublicFlags>()),
        ("UserUpdateEvent", std::mem::size_of::<UserUpdateEvent>()),
        ("VoiceRegion", std::mem::size_of::<VoiceRegion>()),
        ("VoiceServerUpdateEvent", std::mem::size_of::<VoiceServerUpdateEvent>()),
        ("VoiceState", std::mem::size_of::<VoiceState>()),
        ("VoiceStateUpdateEvent", std::mem::size_of::<VoiceStateUpdateEvent>()),
        ("Webhook", std::mem::size_of::<Webhook>()),
        ("WebhookId", std::mem::size_of::<WebhookId>()),
        ("WebhookUpdateEvent", std::mem::size_of::<WebhookUpdateEvent>()),
        ("ActionType", std::mem::size_of::<ActionType>()),
        ("ActivityType", std::mem::size_of::<ActivityType>()),
        ("AutoModAction", std::mem::size_of::<AutoModAction>()),
        ("Change", std::mem::size_of::<Change>()),
        ("Channel", std::mem::size_of::<Channel>()),
        ("ChannelAction", std::mem::size_of::<ChannelAction>()),
        ("ChannelOverwriteAction", std::mem::size_of::<ChannelOverwriteAction>()),
        ("ChannelType", std::mem::size_of::<ChannelType>()),
        ("CommandDataOptionValue", std::mem::size_of::<CommandDataOptionValue>()),
        ("ConnectionVisibility", std::mem::size_of::<ConnectionVisibility>()),
        ("DefaultMessageNotificationLevel", std::mem::size_of::<DefaultMessageNotificationLevel>()),
        ("EmojiAction", std::mem::size_of::<EmojiAction>()),
        ("EntityType", std::mem::size_of::<EntityType>()),
        ("Event", std::mem::size_of::<Event>()),
        ("ExplicitContentFilter", std::mem::size_of::<ExplicitContentFilter>()),
        ("GatewayEvent", std::mem::size_of::<GatewayEvent>()),
        ("GuildWelcomeChannelEmoji", std::mem::size_of::<GuildWelcomeChannelEmoji>()),
        ("GuildWidgetStyle", std::mem::size_of::<GuildWidgetStyle>()),
        ("IntegrationAction", std::mem::size_of::<IntegrationAction>()),
        ("IntegrationExpireBehaviour", std::mem::size_of::<IntegrationExpireBehaviour>()),
        ("Interaction", std::mem::size_of::<Interaction>()),
        ("InteractionType", std::mem::size_of::<InteractionType>()),
        ("InviteAction", std::mem::size_of::<InviteAction>()),
        ("InviteTargetType", std::mem::size_of::<InviteTargetType>()),
        ("KeywordPresetType", std::mem::size_of::<KeywordPresetType>()),
        ("MemberAction", std::mem::size_of::<MemberAction>()),
        ("MembershipState", std::mem::size_of::<MembershipState>()),
        ("Mention", std::mem::size_of::<Mention>()),
        ("MentionParseError", std::mem::size_of::<MentionParseError>()),
        ("MessageAction", std::mem::size_of::<MessageAction>()),
        ("MessageActivityKind", std::mem::size_of::<MessageActivityKind>()),
        ("MessageType", std::mem::size_of::<MessageType>()),
        ("MfaLevel", std::mem::size_of::<MfaLevel>()),
        ("Nonce", std::mem::size_of::<Nonce>()),
        ("NsfwLevel", std::mem::size_of::<NsfwLevel>()),
        ("OnlineStatus", std::mem::size_of::<OnlineStatus>()),
        ("PermissionOverwriteType", std::mem::size_of::<PermissionOverwriteType>()),
        ("PremiumTier", std::mem::size_of::<PremiumTier>()),
        ("ReactionType", std::mem::size_of::<ReactionType>()),
        ("ResolvedTarget", std::mem::size_of::<ResolvedTarget>()),
        ("ResolvedValue", std::mem::size_of::<ResolvedValue>()),
        ("RoleAction", std::mem::size_of::<RoleAction>()),
        ("ScheduledEventAction", std::mem::size_of::<ScheduledEventAction>()),
        ("ScheduledEventStatus", std::mem::size_of::<ScheduledEventStatus>()),
        ("ScheduledEventType", std::mem::size_of::<ScheduledEventType>()),
        ("Scope", std::mem::size_of::<Scope>()),
        ("StageInstanceAction", std::mem::size_of::<StageInstanceAction>()),
        ("StickerAction", std::mem::size_of::<StickerAction>()),
        ("StickerFormatType", std::mem::size_of::<StickerFormatType>()),
        ("StickerType", std::mem::size_of::<StickerType>()),
        ("ThreadAction", std::mem::size_of::<ThreadAction>()),
        ("Trigger", std::mem::size_of::<Trigger>()),
        ("TriggerType", std::mem::size_of::<TriggerType>()),
        ("Unresolved", std::mem::size_of::<Unresolved>()),
        ("VerificationLevel", std::mem::size_of::<VerificationLevel>()),
        ("VideoQualityMode", std::mem::size_of::<VideoQualityMode>()),
        ("WebhookAction", std::mem::size_of::<WebhookAction>()),
        ("WebhookType", std::mem::size_of::<WebhookType>()),
    ];
    for (i, (name, size)) in sizes.iter().enumerate() {
        println!("#{} {}: {} bytes", i + 1, name, size);
    }
}
