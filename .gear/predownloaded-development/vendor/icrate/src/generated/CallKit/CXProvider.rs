//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CXCallEndedReason {
        CXCallEndedReasonFailed = 1,
        CXCallEndedReasonRemoteEnded = 2,
        CXCallEndedReasonUnanswered = 3,
        CXCallEndedReasonAnsweredElsewhere = 4,
        CXCallEndedReasonDeclinedElsewhere = 5,
    }
);

extern_protocol!(
    pub unsafe trait CXProviderDelegate: NSObjectProtocol {
        #[cfg(feature = "CallKit_CXProvider")]
        #[method(providerDidReset:)]
        unsafe fn providerDidReset(&self, provider: &CXProvider);

        #[cfg(feature = "CallKit_CXProvider")]
        #[optional]
        #[method(providerDidBegin:)]
        unsafe fn providerDidBegin(&self, provider: &CXProvider);

        #[cfg(all(feature = "CallKit_CXProvider", feature = "CallKit_CXTransaction"))]
        #[optional]
        #[method(provider:executeTransaction:)]
        unsafe fn provider_executeTransaction(
            &self,
            provider: &CXProvider,
            transaction: &CXTransaction,
        ) -> bool;

        #[cfg(all(feature = "CallKit_CXProvider", feature = "CallKit_CXStartCallAction"))]
        #[optional]
        #[method(provider:performStartCallAction:)]
        unsafe fn provider_performStartCallAction(
            &self,
            provider: &CXProvider,
            action: &CXStartCallAction,
        );

        #[cfg(all(feature = "CallKit_CXAnswerCallAction", feature = "CallKit_CXProvider"))]
        #[optional]
        #[method(provider:performAnswerCallAction:)]
        unsafe fn provider_performAnswerCallAction(
            &self,
            provider: &CXProvider,
            action: &CXAnswerCallAction,
        );

        #[cfg(all(feature = "CallKit_CXEndCallAction", feature = "CallKit_CXProvider"))]
        #[optional]
        #[method(provider:performEndCallAction:)]
        unsafe fn provider_performEndCallAction(
            &self,
            provider: &CXProvider,
            action: &CXEndCallAction,
        );

        #[cfg(all(
            feature = "CallKit_CXProvider",
            feature = "CallKit_CXSetHeldCallAction"
        ))]
        #[optional]
        #[method(provider:performSetHeldCallAction:)]
        unsafe fn provider_performSetHeldCallAction(
            &self,
            provider: &CXProvider,
            action: &CXSetHeldCallAction,
        );

        #[cfg(all(
            feature = "CallKit_CXProvider",
            feature = "CallKit_CXSetMutedCallAction"
        ))]
        #[optional]
        #[method(provider:performSetMutedCallAction:)]
        unsafe fn provider_performSetMutedCallAction(
            &self,
            provider: &CXProvider,
            action: &CXSetMutedCallAction,
        );

        #[cfg(all(
            feature = "CallKit_CXProvider",
            feature = "CallKit_CXSetGroupCallAction"
        ))]
        #[optional]
        #[method(provider:performSetGroupCallAction:)]
        unsafe fn provider_performSetGroupCallAction(
            &self,
            provider: &CXProvider,
            action: &CXSetGroupCallAction,
        );

        #[cfg(all(
            feature = "CallKit_CXPlayDTMFCallAction",
            feature = "CallKit_CXProvider"
        ))]
        #[optional]
        #[method(provider:performPlayDTMFCallAction:)]
        unsafe fn provider_performPlayDTMFCallAction(
            &self,
            provider: &CXProvider,
            action: &CXPlayDTMFCallAction,
        );

        #[cfg(all(feature = "CallKit_CXAction", feature = "CallKit_CXProvider"))]
        #[optional]
        #[method(provider:timedOutPerformingAction:)]
        unsafe fn provider_timedOutPerformingAction(
            &self,
            provider: &CXProvider,
            action: &CXAction,
        );

        #[cfg(all(feature = "AVFAudio_AVAudioSession", feature = "CallKit_CXProvider"))]
        #[optional]
        #[method(provider:didActivateAudioSession:)]
        unsafe fn provider_didActivateAudioSession(
            &self,
            provider: &CXProvider,
            audio_session: &AVAudioSession,
        );

        #[cfg(all(feature = "AVFAudio_AVAudioSession", feature = "CallKit_CXProvider"))]
        #[optional]
        #[method(provider:didDeactivateAudioSession:)]
        unsafe fn provider_didDeactivateAudioSession(
            &self,
            provider: &CXProvider,
            audio_session: &AVAudioSession,
        );
    }

    unsafe impl ProtocolType for dyn CXProviderDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXProvider")]
    pub struct CXProvider;

    #[cfg(feature = "CallKit_CXProvider")]
    unsafe impl ClassType for CXProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CallKit_CXProvider")]
unsafe impl NSObjectProtocol for CXProvider {}

extern_methods!(
    #[cfg(feature = "CallKit_CXProvider")]
    unsafe impl CXProvider {
        #[cfg(feature = "CallKit_CXProviderConfiguration")]
        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Option<Allocated<Self>>,
            configuration: &CXProviderConfiguration,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(
            feature = "CallKit_CXCallUpdate",
            feature = "Foundation_NSError",
            feature = "Foundation_NSUUID"
        ))]
        #[method(reportNewIncomingCallWithUUID:update:completion:)]
        pub unsafe fn reportNewIncomingCallWithUUID_update_completion(
            &self,
            uuid: &NSUUID,
            update: &CXCallUpdate,
            completion: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(feature = "CallKit_CXCallUpdate", feature = "Foundation_NSUUID"))]
        #[method(reportCallWithUUID:updated:)]
        pub unsafe fn reportCallWithUUID_updated(&self, uuid: &NSUUID, update: &CXCallUpdate);

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSUUID"))]
        #[method(reportCallWithUUID:endedAtDate:reason:)]
        pub unsafe fn reportCallWithUUID_endedAtDate_reason(
            &self,
            uuid: &NSUUID,
            date_ended: Option<&NSDate>,
            ended_reason: CXCallEndedReason,
        );

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSUUID"))]
        #[method(reportOutgoingCallWithUUID:startedConnectingAtDate:)]
        pub unsafe fn reportOutgoingCallWithUUID_startedConnectingAtDate(
            &self,
            uuid: &NSUUID,
            date_started_connecting: Option<&NSDate>,
        );

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSUUID"))]
        #[method(reportOutgoingCallWithUUID:connectedAtDate:)]
        pub unsafe fn reportOutgoingCallWithUUID_connectedAtDate(
            &self,
            uuid: &NSUUID,
            date_connected: Option<&NSDate>,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSError"))]
        #[method(reportNewIncomingVoIPPushPayload:completion:)]
        pub unsafe fn reportNewIncomingVoIPPushPayload_completion(
            dictionary_payload: &NSDictionary,
            completion: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(feature = "CallKit_CXProviderConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Id<CXProviderConfiguration>;

        #[cfg(feature = "CallKit_CXProviderConfiguration")]
        #[method(setConfiguration:)]
        pub unsafe fn setConfiguration(&self, configuration: &CXProviderConfiguration);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(all(feature = "CallKit_CXTransaction", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other pendingTransactions)]
        pub unsafe fn pendingTransactions(&self) -> Id<NSArray<CXTransaction>>;

        #[cfg(all(
            feature = "CallKit_CXCallAction",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSUUID"
        ))]
        #[method_id(@__retain_semantics Other pendingCallActionsOfClass:withCallUUID:)]
        pub unsafe fn pendingCallActionsOfClass_withCallUUID(
            &self,
            call_action_class: &AnyClass,
            call_uuid: &NSUUID,
        ) -> Id<NSArray<CXCallAction>>;
    }
);
