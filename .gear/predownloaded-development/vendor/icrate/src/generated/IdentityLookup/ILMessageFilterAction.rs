//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ILMessageFilterAction {
        ILMessageFilterActionNone = 0,
        ILMessageFilterActionAllow = 1,
        ILMessageFilterActionJunk = 2,
        #[deprecated]
        ILMessageFilterActionFilter = ILMessageFilterActionJunk,
        ILMessageFilterActionPromotion = 3,
        ILMessageFilterActionTransaction = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ILMessageFilterSubAction {
        ILMessageFilterSubActionNone = 0,
        ILMessageFilterSubActionTransactionalOthers = 10000,
        ILMessageFilterSubActionTransactionalFinance = 10001,
        ILMessageFilterSubActionTransactionalOrders = 10002,
        ILMessageFilterSubActionTransactionalReminders = 10003,
        ILMessageFilterSubActionTransactionalHealth = 10004,
        ILMessageFilterSubActionTransactionalWeather = 10005,
        ILMessageFilterSubActionTransactionalCarrier = 10006,
        ILMessageFilterSubActionTransactionalRewards = 10007,
        ILMessageFilterSubActionTransactionalPublicServices = 10008,
        ILMessageFilterSubActionPromotionalOthers = 20000,
        ILMessageFilterSubActionPromotionalOffers = 20001,
        ILMessageFilterSubActionPromotionalCoupons = 20002,
    }
);
