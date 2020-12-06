use crate::{
    payloads,
    prelude::Requester,
    requests::{JsonRequest, MultipartRequest},
    types::{
        BotCommand, ChatId, ChatPermissions, InlineQueryResult, InputFile, InputMedia,
        InputSticker, LabeledPrice,
    },
    Bot,
};

impl Requester for Bot {
    type Err = crate::errors::RequestError;

    type GetUpdates = JsonRequest<payloads::GetUpdates>;

    fn get_updates(&self) -> Self::GetUpdates {
        Self::GetUpdates::new(self.clone(), payloads::GetUpdates::new())
    }

    type SetWebhook = JsonRequest<payloads::SetWebhook>;

    fn set_webhook<U, A>(&self, url: U, allowed_updates: A) -> Self::SetWebhook
    where
        U: Into<String>,
        A: IntoIterator<Item = crate::types::AllowedUpdate>,
    {
        Self::SetWebhook::new(self.clone(), payloads::SetWebhook::new(url, allowed_updates))
    }

    type DeleteWebhook = JsonRequest<payloads::DeleteWebhook>;

    fn delete_webhook(&self) -> Self::DeleteWebhook {
        Self::DeleteWebhook::new(self.clone(), payloads::DeleteWebhook::new())
    }

    type GetWebhookInfo = JsonRequest<payloads::GetWebhookInfo>;

    fn get_webhook_info(&self) -> Self::GetWebhookInfo {
        Self::GetWebhookInfo::new(self.clone(), payloads::GetWebhookInfo::new())
    }

    type GetMe = JsonRequest<payloads::GetMe>;

    fn get_me(&self) -> Self::GetMe {
        Self::GetMe::new(self.clone(), payloads::GetMe::new())
    }

    type SendMessage = JsonRequest<payloads::SendMessage>;

    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self::SendMessage::new(self.clone(), payloads::SendMessage::new(chat_id, text))
    }

    type ForwardMessage = JsonRequest<payloads::ForwardMessage>;

    fn forward_message<C, F>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: i32,
    ) -> Self::ForwardMessage
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
    {
        Self::ForwardMessage::new(
            self.clone(),
            payloads::ForwardMessage::new(chat_id, from_chat_id, message_id),
        )
    }

    type SendPhoto = MultipartRequest<payloads::SendPhoto>;

    fn send_photo<Ch, Ca>(&self, chat_id: Ch, photo: InputFile, caption: Ca) -> Self::SendPhoto
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::SendPhoto::new(self.clone(), payloads::SendPhoto::new(chat_id, photo, caption))
    }

    type SendAudio = MultipartRequest<payloads::SendAudio>;

    fn send_audio<Ch, Ca>(&self, chat_id: Ch, audio: InputFile, caption: Ca) -> Self::SendAudio
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::SendAudio::new(self.clone(), payloads::SendAudio::new(chat_id, audio, caption))
    }

    type SendDocument = MultipartRequest<payloads::SendDocument>;

    fn send_document<Ch, Ca>(
        &self,
        chat_id: Ch,
        document: InputFile,
        caption: Ca,
    ) -> Self::SendDocument
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::SendDocument::new(
            self.clone(),
            payloads::SendDocument::new(chat_id, document, caption),
        )
    }

    type SendVideo = MultipartRequest<payloads::SendVideo>;

    fn send_video<Ch, Ca>(&self, chat_id: Ch, video: InputFile, caption: Ca) -> Self::SendVideo
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::SendVideo::new(self.clone(), payloads::SendVideo::new(chat_id, video, caption))
    }

    type SendAnimation = MultipartRequest<payloads::SendAnimation>;

    fn send_animation<Ch, Ca>(
        &self,
        chat_id: Ch,
        animation: InputFile,
        caption: Ca,
    ) -> Self::SendAnimation
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::SendAnimation::new(
            self.clone(),
            payloads::SendAnimation::new(chat_id, animation, caption),
        )
    }

    type SendVoice = MultipartRequest<payloads::SendVoice>;

    fn send_voice<Ch, Ca>(&self, chat_id: Ch, voice: InputFile, caption: Ca) -> Self::SendVoice
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::SendVoice::new(self.clone(), payloads::SendVoice::new(chat_id, voice, caption))
    }

    type SendVideoNote = MultipartRequest<payloads::SendVideoNote>;

    fn send_video_note<C>(&self, chat_id: C, video_note: InputFile) -> Self::SendVideoNote
    where
        C: Into<ChatId>,
    {
        Self::SendVideoNote::new(self.clone(), payloads::SendVideoNote::new(chat_id, video_note))
    }

    type SendMediaGroup = MultipartRequest<payloads::SendMediaGroup>;

    fn send_media_group<C, M>(&self, chat_id: C, media: M) -> Self::SendMediaGroup
    where
        C: Into<ChatId>,
        M: IntoIterator<Item = InputMedia>,
    {
        Self::SendMediaGroup::new(self.clone(), payloads::SendMediaGroup::new(chat_id, media))
    }

    type SendLocation = JsonRequest<payloads::SendLocation>;

    fn send_location<C>(
        &self,
        chat_id: C,
        latitude: f64,
        longitude: f64,
        live_period: u32,
    ) -> Self::SendLocation
    where
        C: Into<ChatId>,
    {
        Self::SendLocation::new(
            self.clone(),
            payloads::SendLocation::new(chat_id, latitude, longitude, live_period),
        )
    }

    type EditMessageLiveLocation = JsonRequest<payloads::EditMessageLiveLocation>;

    fn edit_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: i32,
        latitude: f64,
        longitude: f64,
    ) -> Self::EditMessageLiveLocation
    where
        C: Into<ChatId>,
    {
        Self::EditMessageLiveLocation::new(
            self.clone(),
            payloads::EditMessageLiveLocation::new(chat_id, message_id, latitude, longitude),
        )
    }

    type EditMessageLiveLocationInline = JsonRequest<payloads::EditMessageLiveLocationInline>;

    fn edit_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
        latitude: f64,
        longitude: f64,
    ) -> Self::EditMessageLiveLocationInline
    where
        I: Into<String>,
    {
        Self::EditMessageLiveLocationInline::new(
            self.clone(),
            payloads::EditMessageLiveLocationInline::new(inline_message_id, latitude, longitude),
        )
    }

    type StopMessageLiveLocation = JsonRequest<payloads::StopMessageLiveLocation>;

    fn stop_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: i32,
        latitude: f64,
        longitude: f64,
    ) -> Self::StopMessageLiveLocation
    where
        C: Into<ChatId>,
    {
        Self::StopMessageLiveLocation::new(
            self.clone(),
            payloads::StopMessageLiveLocation::new(chat_id, message_id, latitude, longitude),
        )
    }

    type StopMessageLiveLocationInline = JsonRequest<payloads::StopMessageLiveLocationInline>;

    fn stop_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
        latitude: f64,
        longitude: f64,
    ) -> Self::StopMessageLiveLocationInline
    where
        I: Into<String>,
    {
        Self::StopMessageLiveLocationInline::new(
            self.clone(),
            payloads::StopMessageLiveLocationInline::new(inline_message_id, latitude, longitude),
        )
    }

    type SendVenue = JsonRequest<payloads::SendVenue>;

    fn send_venue<C, T, A>(
        &self,
        chat_id: C,
        latitude: f64,
        longitude: f64,
        title: T,
        address: A,
    ) -> Self::SendVenue
    where
        C: Into<ChatId>,
        T: Into<String>,
        A: Into<String>,
    {
        Self::SendVenue::new(
            self.clone(),
            payloads::SendVenue::new(chat_id, latitude, longitude, title, address),
        )
    }

    type SendContact = JsonRequest<payloads::SendContact>;

    fn send_contact<C>(&self, chat_id: C, phone_number: f64, first_name: f64) -> Self::SendContact
    where
        C: Into<ChatId>,
    {
        Self::SendContact::new(
            self.clone(),
            payloads::SendContact::new(chat_id, phone_number, first_name),
        )
    }

    type SendPoll = JsonRequest<payloads::SendPoll>;

    fn send_poll<C, Q, O>(
        &self,
        chat_id: C,
        question: Q,
        options: O,
        type_: crate::types::PollType,
    ) -> Self::SendPoll
    where
        C: Into<ChatId>,
        Q: Into<String>,
        O: IntoIterator<Item = String>,
    {
        Self::SendPoll::new(
            self.clone(),
            payloads::SendPoll::new(chat_id, question, options, type_),
        )
    }

    type SendDice = JsonRequest<payloads::SendDice>;

    fn send_dice<C>(&self, chat_id: C, emoji: crate::types::DiceEmoji) -> Self::SendDice
    where
        C: Into<ChatId>,
    {
        Self::SendDice::new(self.clone(), payloads::SendDice::new(chat_id, emoji))
    }

    type SendChatAction = JsonRequest<payloads::SendChatAction>;

    fn send_chat_action<C>(
        &self,
        chat_id: C,
        action: crate::types::ChatAction,
    ) -> Self::SendChatAction
    where
        C: Into<ChatId>,
    {
        Self::SendChatAction::new(self.clone(), payloads::SendChatAction::new(chat_id, action))
    }

    type GetUserProfilePhotos = JsonRequest<payloads::GetUserProfilePhotos>;

    fn get_user_profile_photos(&self, user_id: i32) -> Self::GetUserProfilePhotos {
        Self::GetUserProfilePhotos::new(self.clone(), payloads::GetUserProfilePhotos::new(user_id))
    }

    type GetFile = JsonRequest<payloads::GetFile>;

    fn get_file<F>(&self, file_id: F) -> Self::GetFile
    where
        F: Into<String>,
    {
        Self::GetFile::new(self.clone(), payloads::GetFile::new(file_id))
    }

    type KickChatMember = JsonRequest<payloads::KickChatMember>;

    fn kick_chat_member<C>(&self, chat_id: C, user_id: i32) -> Self::KickChatMember
    where
        C: Into<ChatId>,
    {
        Self::KickChatMember::new(self.clone(), payloads::KickChatMember::new(chat_id, user_id))
    }

    type UnbanChatMember = JsonRequest<payloads::UnbanChatMember>;

    fn unban_chat_member<C>(&self, chat_id: C, user_id: i32) -> Self::UnbanChatMember
    where
        C: Into<ChatId>,
    {
        Self::UnbanChatMember::new(self.clone(), payloads::UnbanChatMember::new(chat_id, user_id))
    }

    type RestrictChatMember = JsonRequest<payloads::RestrictChatMember>;

    fn restrict_chat_member<C>(
        &self,
        chat_id: C,
        user_id: i32,
        permissions: ChatPermissions,
    ) -> Self::RestrictChatMember
    where
        C: Into<ChatId>,
    {
        Self::RestrictChatMember::new(
            self.clone(),
            payloads::RestrictChatMember::new(chat_id, user_id, permissions),
        )
    }

    type PromoteChatMember = JsonRequest<payloads::PromoteChatMember>;

    fn promote_chat_member<C>(&self, chat_id: C, user_id: i32) -> Self::PromoteChatMember
    where
        C: Into<ChatId>,
    {
        Self::PromoteChatMember::new(
            self.clone(),
            payloads::PromoteChatMember::new(chat_id, user_id),
        )
    }

    type SetChatAdministratorCustomTitle = JsonRequest<payloads::SetChatAdministratorCustomTitle>;

    fn set_chat_administrator_custom_title<Ch, Cu>(
        &self,
        chat_id: Ch,
        user_id: i32,
        custom_title: Cu,
    ) -> Self::SetChatAdministratorCustomTitle
    where
        Ch: Into<ChatId>,
        Cu: Into<String>,
    {
        Self::SetChatAdministratorCustomTitle::new(
            self.clone(),
            payloads::SetChatAdministratorCustomTitle::new(chat_id, user_id, custom_title),
        )
    }

    type SetChatPermissions = JsonRequest<payloads::SetChatPermissions>;

    fn set_chat_permissions<C>(
        &self,
        chat_id: C,
        permissions: ChatPermissions,
    ) -> Self::SetChatPermissions
    where
        C: Into<ChatId>,
    {
        Self::SetChatPermissions::new(
            self.clone(),
            payloads::SetChatPermissions::new(chat_id, permissions),
        )
    }

    type ExportChatInviteLink = JsonRequest<payloads::ExportChatInviteLink>;

    fn export_chat_invite_link<C>(&self, chat_id: C) -> Self::ExportChatInviteLink
    where
        C: Into<ChatId>,
    {
        Self::ExportChatInviteLink::new(self.clone(), payloads::ExportChatInviteLink::new(chat_id))
    }

    type SetChatPhoto = MultipartRequest<payloads::SetChatPhoto>;

    fn set_chat_photo<C>(&self, chat_id: C, photo: InputFile) -> Self::SetChatPhoto
    where
        C: Into<ChatId>,
    {
        Self::SetChatPhoto::new(self.clone(), payloads::SetChatPhoto::new(chat_id, photo))
    }

    type DeleteChatPhoto = JsonRequest<payloads::DeleteChatPhoto>;

    fn delete_chat_photo<C>(&self, chat_id: C) -> Self::DeleteChatPhoto
    where
        C: Into<ChatId>,
    {
        Self::DeleteChatPhoto::new(self.clone(), payloads::DeleteChatPhoto::new(chat_id))
    }

    type SetChatTitle = JsonRequest<payloads::SetChatTitle>;

    fn set_chat_title<C, T>(&self, chat_id: C, title: T) -> Self::SetChatTitle
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self::SetChatTitle::new(self.clone(), payloads::SetChatTitle::new(chat_id, title))
    }

    type SetChatDescription = JsonRequest<payloads::SetChatDescription>;

    fn set_chat_description<C>(&self, chat_id: C) -> Self::SetChatDescription
    where
        C: Into<ChatId>,
    {
        Self::SetChatDescription::new(self.clone(), payloads::SetChatDescription::new(chat_id))
    }

    type PinChatMessage = JsonRequest<payloads::PinChatMessage>;

    fn pin_chat_message<C>(&self, chat_id: C, message_id: i32) -> Self::PinChatMessage
    where
        C: Into<ChatId>,
    {
        Self::PinChatMessage::new(self.clone(), payloads::PinChatMessage::new(chat_id, message_id))
    }

    type UnpinChatMessage = JsonRequest<payloads::UnpinChatMessage>;

    fn unpin_chat_message<C>(&self, chat_id: C) -> Self::UnpinChatMessage
    where
        C: Into<ChatId>,
    {
        Self::UnpinChatMessage::new(self.clone(), payloads::UnpinChatMessage::new(chat_id))
    }

    type LeaveChat = JsonRequest<payloads::LeaveChat>;

    fn leave_chat<C>(&self, chat_id: C) -> Self::LeaveChat
    where
        C: Into<ChatId>,
    {
        Self::LeaveChat::new(self.clone(), payloads::LeaveChat::new(chat_id))
    }

    type GetChat = JsonRequest<payloads::GetChat>;

    fn get_chat<C>(&self, chat_id: C) -> Self::GetChat
    where
        C: Into<ChatId>,
    {
        Self::GetChat::new(self.clone(), payloads::GetChat::new(chat_id))
    }

    type GetChatAdministrators = JsonRequest<payloads::GetChatAdministrators>;

    fn get_chat_administrators<C>(&self, chat_id: C) -> Self::GetChatAdministrators
    where
        C: Into<ChatId>,
    {
        Self::GetChatAdministrators::new(
            self.clone(),
            payloads::GetChatAdministrators::new(chat_id),
        )
    }

    type GetChatMembersCount = JsonRequest<payloads::GetChatMembersCount>;

    fn get_chat_members_count<C>(&self, chat_id: C) -> Self::GetChatMembersCount
    where
        C: Into<ChatId>,
    {
        Self::GetChatMembersCount::new(self.clone(), payloads::GetChatMembersCount::new(chat_id))
    }

    type GetChatMember = JsonRequest<payloads::GetChatMember>;

    fn get_chat_member<C>(&self, chat_id: C, user_id: i32) -> Self::GetChatMember
    where
        C: Into<ChatId>,
    {
        Self::GetChatMember::new(self.clone(), payloads::GetChatMember::new(chat_id, user_id))
    }

    type SetChatStickerSet = JsonRequest<payloads::SetChatStickerSet>;

    fn set_chat_sticker_set<C, S>(&self, chat_id: C, sticker_set_name: S) -> Self::SetChatStickerSet
    where
        C: Into<ChatId>,
        S: Into<String>,
    {
        Self::SetChatStickerSet::new(
            self.clone(),
            payloads::SetChatStickerSet::new(chat_id, sticker_set_name),
        )
    }

    type DeleteChatStickerSet = JsonRequest<payloads::DeleteChatStickerSet>;

    fn delete_chat_sticker_set<C>(&self, chat_id: C) -> Self::DeleteChatStickerSet
    where
        C: Into<ChatId>,
    {
        Self::DeleteChatStickerSet::new(self.clone(), payloads::DeleteChatStickerSet::new(chat_id))
    }

    type AnswerCallbackQuery = JsonRequest<payloads::AnswerCallbackQuery>;

    fn answer_callback_query<C>(&self, callback_query_id: C) -> Self::AnswerCallbackQuery
    where
        C: Into<String>,
    {
        Self::AnswerCallbackQuery::new(
            self.clone(),
            payloads::AnswerCallbackQuery::new(callback_query_id),
        )
    }

    type SetMyCommands = JsonRequest<payloads::SetMyCommands>;

    fn set_my_commands<C>(&self, commands: C) -> Self::SetMyCommands
    where
        C: IntoIterator<Item = BotCommand>,
    {
        Self::SetMyCommands::new(self.clone(), payloads::SetMyCommands::new(commands))
    }

    type GetMyCommands = JsonRequest<payloads::GetMyCommands>;

    fn get_my_commands(&self) -> Self::GetMyCommands {
        Self::GetMyCommands::new(self.clone(), payloads::GetMyCommands::new())
    }

    type AnswerInlineQuery = JsonRequest<payloads::AnswerInlineQuery>;

    fn answer_inline_query<I, R>(&self, inline_query_id: I, results: R) -> Self::AnswerInlineQuery
    where
        I: Into<String>,
        R: IntoIterator<Item = InlineQueryResult>,
    {
        Self::AnswerInlineQuery::new(
            self.clone(),
            payloads::AnswerInlineQuery::new(inline_query_id, results),
        )
    }

    type EditMessageText = JsonRequest<payloads::EditMessageText>;

    fn edit_message_text<C, T>(&self, chat_id: C, message_id: i32, text: T) -> Self::EditMessageText
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self::EditMessageText::new(
            self.clone(),
            payloads::EditMessageText::new(chat_id, message_id, text),
        )
    }

    type EditMessageTextInline = JsonRequest<payloads::EditMessageTextInline>;

    fn edit_message_text_inline<I, T>(
        &self,
        inline_message_id: I,
        text: T,
    ) -> Self::EditMessageTextInline
    where
        I: Into<String>,
        T: Into<String>,
    {
        Self::EditMessageTextInline::new(
            self.clone(),
            payloads::EditMessageTextInline::new(inline_message_id, text),
        )
    }

    type EditMessageCaption = JsonRequest<payloads::EditMessageCaption>;

    fn edit_message_caption<Ch, Ca>(
        &self,
        chat_id: Ch,
        message_id: i32,
        caption: Ca,
    ) -> Self::EditMessageCaption
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::EditMessageCaption::new(
            self.clone(),
            payloads::EditMessageCaption::new(chat_id, message_id, caption),
        )
    }

    type EditMessageCaptionInline = JsonRequest<payloads::EditMessageCaptionInline>;

    fn edit_message_caption_inline<I, C>(
        &self,
        inline_message_id: I,
        caption: C,
    ) -> Self::EditMessageCaptionInline
    where
        I: Into<String>,
        C: Into<String>,
    {
        Self::EditMessageCaptionInline::new(
            self.clone(),
            payloads::EditMessageCaptionInline::new(inline_message_id, caption),
        )
    }

    type EditMessageMedia = MultipartRequest<payloads::EditMessageMedia>;

    fn edit_message_media<C>(
        &self,
        chat_id: C,
        message_id: i32,
        media: InputMedia,
    ) -> Self::EditMessageMedia
    where
        C: Into<ChatId>,
    {
        Self::EditMessageMedia::new(
            self.clone(),
            payloads::EditMessageMedia::new(chat_id, message_id, media),
        )
    }

    type EditMessageMediaInline = MultipartRequest<payloads::EditMessageMediaInline>;

    fn edit_message_media_inline<I>(
        &self,
        inline_message_id: I,
        media: InputMedia,
    ) -> Self::EditMessageMediaInline
    where
        I: Into<String>,
    {
        Self::EditMessageMediaInline::new(
            self.clone(),
            payloads::EditMessageMediaInline::new(inline_message_id, media),
        )
    }

    type EditMessageReplyMarkup = JsonRequest<payloads::EditMessageReplyMarkup>;

    fn edit_message_reply_markup<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> Self::EditMessageReplyMarkup
    where
        C: Into<ChatId>,
    {
        Self::EditMessageReplyMarkup::new(
            self.clone(),
            payloads::EditMessageReplyMarkup::new(chat_id, message_id),
        )
    }

    type EditMessageReplyMarkupInline = JsonRequest<payloads::EditMessageReplyMarkupInline>;

    fn edit_message_reply_markup_inline<I>(
        &self,
        inline_message_id: I,
    ) -> Self::EditMessageReplyMarkupInline
    where
        I: Into<String>,
    {
        Self::EditMessageReplyMarkupInline::new(
            self.clone(),
            payloads::EditMessageReplyMarkupInline::new(inline_message_id),
        )
    }

    type StopPoll = JsonRequest<payloads::StopPoll>;

    fn stop_poll<C>(&self, chat_id: C, message_id: i32) -> Self::StopPoll
    where
        C: Into<ChatId>,
    {
        Self::StopPoll::new(self.clone(), payloads::StopPoll::new(chat_id, message_id))
    }

    type DeleteMessage = JsonRequest<payloads::DeleteMessage>;

    fn delete_message<C>(&self, chat_id: C, message_id: i32) -> Self::DeleteMessage
    where
        C: Into<ChatId>,
    {
        Self::DeleteMessage::new(self.clone(), payloads::DeleteMessage::new(chat_id, message_id))
    }

    type SendSticker = MultipartRequest<payloads::SendSticker>;

    fn send_sticker<C>(&self, chat_id: C, sticker: InputFile) -> Self::SendSticker
    where
        C: Into<ChatId>,
    {
        Self::SendSticker::new(self.clone(), payloads::SendSticker::new(chat_id, sticker))
    }

    type GetStickerSet = JsonRequest<payloads::GetStickerSet>;

    fn get_sticker_set<N>(&self, name: N) -> Self::GetStickerSet
    where
        N: Into<String>,
    {
        Self::GetStickerSet::new(self.clone(), payloads::GetStickerSet::new(name))
    }

    type UploadStickerFile = MultipartRequest<payloads::UploadStickerFile>;

    fn upload_sticker_file(&self, user_id: i32, png_sticker: InputFile) -> Self::UploadStickerFile where
    {
        Self::UploadStickerFile::new(
            self.clone(),
            payloads::UploadStickerFile::new(user_id, png_sticker),
        )
    }

    type CreateNewStickerSet = JsonRequest<payloads::CreateNewStickerSet>;

    fn create_new_sticker_set<N, T, E>(
        &self,
        user_id: i32,
        name: N,
        title: T,
        emojis: E,
    ) -> Self::CreateNewStickerSet
    where
        N: Into<String>,
        T: Into<String>,
        E: Into<String>,
    {
        Self::CreateNewStickerSet::new(
            self.clone(),
            payloads::CreateNewStickerSet::new(user_id, name, title, emojis),
        )
    }

    type AddStickerToSet = MultipartRequest<payloads::AddStickerToSet>;

    fn add_sticker_to_set<N, E>(
        &self,
        user_id: i32,
        name: N,
        sticker: InputSticker,
        emojis: E,
    ) -> Self::AddStickerToSet
    where
        N: Into<String>,
        E: Into<String>,
    {
        Self::AddStickerToSet::new(
            self.clone(),
            payloads::AddStickerToSet::new(user_id, name, sticker, emojis),
        )
    }

    type SetStickerPositionInSet = JsonRequest<payloads::SetStickerPositionInSet>;

    fn set_sticker_position_in_set<S>(
        &self,
        sticker: S,
        position: u32,
    ) -> Self::SetStickerPositionInSet
    where
        S: Into<String>,
    {
        Self::SetStickerPositionInSet::new(
            self.clone(),
            payloads::SetStickerPositionInSet::new(sticker, position),
        )
    }

    type DeleteStickerFromSet = JsonRequest<payloads::DeleteStickerFromSet>;

    fn delete_sticker_from_set<S>(&self, sticker: S) -> Self::DeleteStickerFromSet
    where
        S: Into<String>,
    {
        Self::DeleteStickerFromSet::new(self.clone(), payloads::DeleteStickerFromSet::new(sticker))
    }

    type SetStickerSetThumb = MultipartRequest<payloads::SetStickerSetThumb>;

    fn set_sticker_set_thumb<N>(&self, name: N, user_id: i32) -> Self::SetStickerSetThumb
    where
        N: Into<String>,
    {
        Self::SetStickerSetThumb::new(
            self.clone(),
            payloads::SetStickerSetThumb::new(name, user_id),
        )
    }

    type SendInvoice = JsonRequest<payloads::SendInvoice>;

    fn send_invoice<T, D, Pa, P, S, C, Pri>(
        &self,
        chat_id: i32,
        title: T,
        description: D,
        payload: Pa,
        provider_token: P,
        start_parameter: S,
        currency: C,
        prices: Pri,
    ) -> Self::SendInvoice
    where
        T: Into<String>,
        D: Into<String>,
        Pa: Into<String>,
        P: Into<String>,
        S: Into<String>,
        C: Into<String>,
        Pri: IntoIterator<Item = LabeledPrice>,
    {
        Self::SendInvoice::new(
            self.clone(),
            payloads::SendInvoice::new(
                chat_id,
                title,
                description,
                payload,
                provider_token,
                start_parameter,
                currency,
                prices,
            ),
        )
    }

    type AnswerShippingQuery = JsonRequest<payloads::AnswerShippingQuery>;

    fn answer_shipping_query<S>(&self, shipping_query_id: S, ok: bool) -> Self::AnswerShippingQuery
    where
        S: Into<String>,
    {
        Self::AnswerShippingQuery::new(
            self.clone(),
            payloads::AnswerShippingQuery::new(shipping_query_id, ok),
        )
    }

    type AnswerPreCheckoutQuery = JsonRequest<payloads::AnswerPreCheckoutQuery>;

    fn answer_pre_checkout_query<P>(
        &self,
        pre_checkout_query_id: P,
        ok: bool,
    ) -> Self::AnswerPreCheckoutQuery
    where
        P: Into<String>,
    {
        Self::AnswerPreCheckoutQuery::new(
            self.clone(),
            payloads::AnswerPreCheckoutQuery::new(pre_checkout_query_id, ok),
        )
    }

    type SetPassportDataErrors = JsonRequest<payloads::SetPassportDataErrors>;

    fn set_passport_data_errors<E>(&self, user_id: i32, errors: E) -> Self::SetPassportDataErrors
    where
        E: IntoIterator<Item = crate::types::PassportElementError>,
    {
        Self::SetPassportDataErrors::new(
            self.clone(),
            payloads::SetPassportDataErrors::new(user_id, errors),
        )
    }
}
