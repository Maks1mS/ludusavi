//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionKerberosMapping")]
    pub struct ASAuthorizationProviderExtensionKerberosMapping;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionKerberosMapping")]
    unsafe impl ClassType for ASAuthorizationProviderExtensionKerberosMapping {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionKerberosMapping")]
unsafe impl NSObjectProtocol for ASAuthorizationProviderExtensionKerberosMapping {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionKerberosMapping")]
    unsafe impl ASAuthorizationProviderExtensionKerberosMapping {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other ticketKeyPath)]
        pub unsafe fn ticketKeyPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTicketKeyPath:)]
        pub unsafe fn setTicketKeyPath(&self, ticket_key_path: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other messageBufferKeyName)]
        pub unsafe fn messageBufferKeyName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMessageBufferKeyName:)]
        pub unsafe fn setMessageBufferKeyName(&self, message_buffer_key_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other realmKeyName)]
        pub unsafe fn realmKeyName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setRealmKeyName:)]
        pub unsafe fn setRealmKeyName(&self, realm_key_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other serviceNameKeyName)]
        pub unsafe fn serviceNameKeyName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setServiceNameKeyName:)]
        pub unsafe fn setServiceNameKeyName(&self, service_name_key_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other clientNameKeyName)]
        pub unsafe fn clientNameKeyName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setClientNameKeyName:)]
        pub unsafe fn setClientNameKeyName(&self, client_name_key_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other encryptionKeyTypeKeyName)]
        pub unsafe fn encryptionKeyTypeKeyName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setEncryptionKeyTypeKeyName:)]
        pub unsafe fn setEncryptionKeyTypeKeyName(
            &self,
            encryption_key_type_key_name: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sessionKeyKeyName)]
        pub unsafe fn sessionKeyKeyName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSessionKeyKeyName:)]
        pub unsafe fn setSessionKeyKeyName(&self, session_key_key_name: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionKerberosMapping")]
    unsafe impl ASAuthorizationProviderExtensionKerberosMapping {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginConfiguration")]
    pub struct ASAuthorizationProviderExtensionLoginConfiguration;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginConfiguration")]
    unsafe impl ClassType for ASAuthorizationProviderExtensionLoginConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginConfiguration")]
unsafe impl NSObjectProtocol for ASAuthorizationProviderExtensionLoginConfiguration {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginConfiguration")]
    unsafe impl ASAuthorizationProviderExtensionLoginConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithClientID:issuer:tokenEndpointURL:jwksEndpointURL:audience:)]
        pub unsafe fn initWithClientID_issuer_tokenEndpointURL_jwksEndpointURL_audience(
            this: Option<Allocated<Self>>,
            client_id: &NSString,
            issuer: &NSString,
            token_endpoint_url: &NSURL,
            jwks_endpoint_url: &NSURL,
            audience: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(configurationWithOpenIDConfigurationURL:clientID:issuer:completion:)]
        pub unsafe fn configurationWithOpenIDConfigurationURL_clientID_issuer_completion(
            open_id_configuration_url: &NSURL,
            client_id: &NSString,
            issuer: Option<&NSString>,
            completion: &Block<
                (
                    *mut ASAuthorizationProviderExtensionLoginConfiguration,
                    *mut NSError,
                ),
                (),
            >,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other invalidCredentialPredicate)]
        pub unsafe fn invalidCredentialPredicate(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setInvalidCredentialPredicate:)]
        pub unsafe fn setInvalidCredentialPredicate(
            &self,
            invalid_credential_predicate: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other accountDisplayName)]
        pub unsafe fn accountDisplayName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAccountDisplayName:)]
        pub unsafe fn setAccountDisplayName(&self, account_display_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other clientID)]
        pub unsafe fn clientID(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other issuer)]
        pub unsafe fn issuer(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other audience)]
        pub unsafe fn audience(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAudience:)]
        pub unsafe fn setAudience(&self, audience: &NSString);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other tokenEndpointURL)]
        pub unsafe fn tokenEndpointURL(&self) -> Id<NSURL>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setTokenEndpointURL:)]
        pub unsafe fn setTokenEndpointURL(&self, token_endpoint_url: &NSURL);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other jwksEndpointURL)]
        pub unsafe fn jwksEndpointURL(&self) -> Id<NSURL>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setJwksEndpointURL:)]
        pub unsafe fn setJwksEndpointURL(&self, jwks_endpoint_url: &NSURL);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other nonceEndpointURL)]
        pub unsafe fn nonceEndpointURL(&self) -> Id<NSURL>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setNonceEndpointURL:)]
        pub unsafe fn setNonceEndpointURL(&self, nonce_endpoint_url: &NSURL);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nonceResponseKeypath)]
        pub unsafe fn nonceResponseKeypath(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNonceResponseKeypath:)]
        pub unsafe fn setNonceResponseKeypath(&self, nonce_response_keypath: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other serverNonceClaimName)]
        pub unsafe fn serverNonceClaimName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setServerNonceClaimName:)]
        pub unsafe fn setServerNonceClaimName(&self, server_nonce_claim_name: &NSString);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURLQueryItem"))]
        #[method_id(@__retain_semantics Other customNonceRequestValues)]
        pub unsafe fn customNonceRequestValues(&self) -> Id<NSArray<NSURLQueryItem>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURLQueryItem"))]
        #[method(setCustomNonceRequestValues:)]
        pub unsafe fn setCustomNonceRequestValues(
            &self,
            custom_nonce_request_values: &NSArray<NSURLQueryItem>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(setCustomAssertionRequestHeaderClaims:returningError:_)]
        pub unsafe fn setCustomAssertionRequestHeaderClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(setCustomAssertionRequestBodyClaims:returningError:_)]
        pub unsafe fn setCustomAssertionRequestBodyClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other additionalScopes)]
        pub unsafe fn additionalScopes(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAdditionalScopes:)]
        pub unsafe fn setAdditionalScopes(&self, additional_scopes: &NSString);

        #[method(includePreviousRefreshTokenInLoginRequest)]
        pub unsafe fn includePreviousRefreshTokenInLoginRequest(&self) -> bool;

        #[method(setIncludePreviousRefreshTokenInLoginRequest:)]
        pub unsafe fn setIncludePreviousRefreshTokenInLoginRequest(
            &self,
            include_previous_refresh_token_in_login_request: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other previousRefreshTokenClaimName)]
        pub unsafe fn previousRefreshTokenClaimName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPreviousRefreshTokenClaimName:)]
        pub unsafe fn setPreviousRefreshTokenClaimName(
            &self,
            previous_refresh_token_claim_name: &NSString,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURLQueryItem"))]
        #[method_id(@__retain_semantics Other customLoginRequestValues)]
        pub unsafe fn customLoginRequestValues(&self) -> Id<NSArray<NSURLQueryItem>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURLQueryItem"))]
        #[method(setCustomLoginRequestValues:)]
        pub unsafe fn setCustomLoginRequestValues(
            &self,
            custom_login_request_values: &NSArray<NSURLQueryItem>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(setCustomLoginRequestHeaderClaims:returningError:_)]
        pub unsafe fn setCustomLoginRequestHeaderClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(setCustomLoginRequestBodyClaims:returningError:_)]
        pub unsafe fn setCustomLoginRequestBodyClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationProviderExtensionKerberosMapping",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other kerberosTicketMappings)]
        pub unsafe fn kerberosTicketMappings(
            &self,
        ) -> Id<NSArray<ASAuthorizationProviderExtensionKerberosMapping>>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationProviderExtensionKerberosMapping",
            feature = "Foundation_NSArray"
        ))]
        #[method(setKerberosTicketMappings:)]
        pub unsafe fn setKerberosTicketMappings(
            &self,
            kerberos_ticket_mappings: &NSArray<ASAuthorizationProviderExtensionKerberosMapping>,
        );
    }
);
