//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "MEAddressAnnotation.rs"]
mod __MEAddressAnnotation;
#[path = "MEComposeContext.rs"]
mod __MEComposeContext;
#[path = "MEComposeSession.rs"]
mod __MEComposeSession;
#[path = "MEContentBlocker.rs"]
mod __MEContentBlocker;
#[path = "MEDecodedMessage.rs"]
mod __MEDecodedMessage;
#[path = "MEDecodedMessageBanner.rs"]
mod __MEDecodedMessageBanner;
#[path = "MEEmailAddress.rs"]
mod __MEEmailAddress;
#[path = "MEEncodedOutgoingMessage.rs"]
mod __MEEncodedOutgoingMessage;
#[path = "MEExtension.rs"]
mod __MEExtension;
#[path = "MEExtensionManager.rs"]
mod __MEExtensionManager;
#[path = "MEExtensionViewController.rs"]
mod __MEExtensionViewController;
#[path = "MEMessage.rs"]
mod __MEMessage;
#[path = "MEMessageAction.rs"]
mod __MEMessageAction;
#[path = "MEMessageActionDecision.rs"]
mod __MEMessageActionDecision;
#[path = "MEMessageActionHandler.rs"]
mod __MEMessageActionHandler;
#[path = "MEMessageDecoder.rs"]
mod __MEMessageDecoder;
#[path = "MEMessageEncoder.rs"]
mod __MEMessageEncoder;
#[path = "MEMessageEncodingResult.rs"]
mod __MEMessageEncodingResult;
#[path = "MEMessageSecurityHandler.rs"]
mod __MEMessageSecurityHandler;
#[path = "MEMessageSecurityInformation.rs"]
mod __MEMessageSecurityInformation;
#[path = "MEMessageSigner.rs"]
mod __MEMessageSigner;
#[path = "MEOutgoingMessageEncodingStatus.rs"]
mod __MEOutgoingMessageEncodingStatus;

#[cfg(feature = "MailKit_MEAddressAnnotation")]
pub use self::__MEAddressAnnotation::MEAddressAnnotation;
#[cfg(feature = "MailKit_MEComposeContext")]
pub use self::__MEComposeContext::MEComposeContext;
pub use self::__MEComposeContext::{
    MEComposeUserAction, MEComposeUserActionForward, MEComposeUserActionNewMessage,
    MEComposeUserActionReply, MEComposeUserActionReplyAll,
};
#[cfg(feature = "MailKit_MEComposeSession")]
pub use self::__MEComposeSession::MEComposeSession;
pub use self::__MEComposeSession::MEComposeSessionErrorDomain;
pub use self::__MEComposeSession::MEComposeSessionHandler;
pub use self::__MEComposeSession::{
    MEComposeSessionErrorCode, MEComposeSessionErrorCodeInvalidBody,
    MEComposeSessionErrorCodeInvalidHeaders, MEComposeSessionErrorCodeInvalidRecipients,
};
pub use self::__MEContentBlocker::MEContentBlocker;
#[cfg(feature = "MailKit_MEDecodedMessage")]
pub use self::__MEDecodedMessage::MEDecodedMessage;
#[cfg(feature = "MailKit_MEDecodedMessageBanner")]
pub use self::__MEDecodedMessageBanner::MEDecodedMessageBanner;
#[cfg(feature = "MailKit_MEEmailAddress")]
pub use self::__MEEmailAddress::MEEmailAddress;
#[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
pub use self::__MEEncodedOutgoingMessage::MEEncodedOutgoingMessage;
pub use self::__MEExtension::MEExtension;
#[cfg(feature = "MailKit_MEExtensionManager")]
pub use self::__MEExtensionManager::MEExtensionManager;
#[cfg(feature = "MailKit_MEExtensionViewController")]
pub use self::__MEExtensionViewController::MEExtensionViewController;
#[cfg(feature = "MailKit_MEMessage")]
pub use self::__MEMessage::MEMessage;
pub use self::__MEMessage::{
    MEMessageEncryptionState, MEMessageEncryptionStateEncrypted,
    MEMessageEncryptionStateNotEncrypted, MEMessageEncryptionStateUnknown,
};
pub use self::__MEMessage::{
    MEMessageState, MEMessageStateDraft, MEMessageStateReceived, MEMessageStateSending,
};
#[cfg(feature = "MailKit_MEMessageAction")]
pub use self::__MEMessageAction::MEMessageAction;
pub use self::__MEMessageAction::{
    MEMessageActionFlag, MEMessageActionFlagBlue, MEMessageActionFlagDefaultColor,
    MEMessageActionFlagGray, MEMessageActionFlagGreen, MEMessageActionFlagNone,
    MEMessageActionFlagOrange, MEMessageActionFlagPurple, MEMessageActionFlagRed,
    MEMessageActionFlagYellow,
};
pub use self::__MEMessageAction::{
    MEMessageActionMessageColor, MEMessageActionMessageColorBlue, MEMessageActionMessageColorGray,
    MEMessageActionMessageColorGreen, MEMessageActionMessageColorNone,
    MEMessageActionMessageColorOrange, MEMessageActionMessageColorPurple,
    MEMessageActionMessageColorRed, MEMessageActionMessageColorYellow,
};
#[cfg(feature = "MailKit_MEMessageActionDecision")]
pub use self::__MEMessageActionDecision::MEMessageActionDecision;
pub use self::__MEMessageActionHandler::MEMessageActionHandler;
pub use self::__MEMessageDecoder::MEMessageDecoder;
pub use self::__MEMessageEncoder::MEMessageEncoder;
#[cfg(feature = "MailKit_MEMessageEncodingResult")]
pub use self::__MEMessageEncodingResult::MEMessageEncodingResult;
pub use self::__MEMessageSecurityHandler::MEMessageSecurityErrorDomain;
pub use self::__MEMessageSecurityHandler::MEMessageSecurityHandler;
pub use self::__MEMessageSecurityHandler::{
    MEMessageSecurityDecodingError, MEMessageSecurityEncodingError, MEMessageSecurityErrorCode,
};
#[cfg(feature = "MailKit_MEMessageSecurityInformation")]
pub use self::__MEMessageSecurityInformation::MEMessageSecurityInformation;
#[cfg(feature = "MailKit_MEMessageSigner")]
pub use self::__MEMessageSigner::MEMessageSigner;
#[cfg(feature = "MailKit_MEOutgoingMessageEncodingStatus")]
pub use self::__MEOutgoingMessageEncodingStatus::MEOutgoingMessageEncodingStatus;
