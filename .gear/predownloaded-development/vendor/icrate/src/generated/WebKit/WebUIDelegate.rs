//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_enum!(
    #[underlying(c_uint)]
    #[deprecated]
    pub enum __anonymous__ {
        #[deprecated]
        WebMenuItemTagOpenLinkInNewWindow = 1,
        #[deprecated]
        WebMenuItemTagDownloadLinkToDisk = 2,
        #[deprecated]
        WebMenuItemTagCopyLinkToClipboard = 3,
        #[deprecated]
        WebMenuItemTagOpenImageInNewWindow = 4,
        #[deprecated]
        WebMenuItemTagDownloadImageToDisk = 5,
        #[deprecated]
        WebMenuItemTagCopyImageToClipboard = 6,
        #[deprecated]
        WebMenuItemTagOpenFrameInNewWindow = 7,
        #[deprecated]
        WebMenuItemTagCopy = 8,
        #[deprecated]
        WebMenuItemTagGoBack = 9,
        #[deprecated]
        WebMenuItemTagGoForward = 10,
        #[deprecated]
        WebMenuItemTagStop = 11,
        #[deprecated]
        WebMenuItemTagReload = 12,
        #[deprecated]
        WebMenuItemTagCut = 13,
        #[deprecated]
        WebMenuItemTagPaste = 14,
        #[deprecated]
        WebMenuItemTagSpellingGuess = 15,
        #[deprecated]
        WebMenuItemTagNoGuessesFound = 16,
        #[deprecated]
        WebMenuItemTagIgnoreSpelling = 17,
        #[deprecated]
        WebMenuItemTagLearnSpelling = 18,
        #[deprecated]
        WebMenuItemTagOther = 19,
        #[deprecated]
        WebMenuItemTagSearchInSpotlight = 20,
        #[deprecated]
        WebMenuItemTagSearchWeb = 21,
        #[deprecated]
        WebMenuItemTagLookUpInDictionary = 22,
        #[deprecated]
        WebMenuItemTagOpenWithDefaultApplication = 23,
        #[deprecated]
        WebMenuItemPDFActualSize = 24,
        #[deprecated]
        WebMenuItemPDFZoomIn = 25,
        #[deprecated]
        WebMenuItemPDFZoomOut = 26,
        #[deprecated]
        WebMenuItemPDFAutoSize = 27,
        #[deprecated]
        WebMenuItemPDFSinglePage = 28,
        #[deprecated]
        WebMenuItemPDFFacingPages = 29,
        #[deprecated]
        WebMenuItemPDFContinuous = 30,
        #[deprecated]
        WebMenuItemPDFNextPage = 31,
        #[deprecated]
        WebMenuItemPDFPreviousPage = 32,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    #[deprecated]
    pub enum WebDragDestinationAction {
        #[deprecated]
        WebDragDestinationActionNone = 0,
        #[deprecated]
        WebDragDestinationActionDHTML = 1,
        #[deprecated]
        WebDragDestinationActionEdit = 2,
        #[deprecated]
        WebDragDestinationActionLoad = 4,
        #[deprecated]
        WebDragDestinationActionAny = 4294967295,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    #[deprecated]
    pub enum WebDragSourceAction {
        #[deprecated]
        WebDragSourceActionNone = 0,
        #[deprecated]
        WebDragSourceActionDHTML = 1,
        #[deprecated]
        WebDragSourceActionImage = 2,
        #[deprecated]
        WebDragSourceActionLink = 4,
        #[deprecated]
        WebDragSourceActionSelection = 8,
        #[deprecated]
        WebDragSourceActionAny = 4294967295,
    }
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebOpenPanelResultListener: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method(chooseFilename:)]
        unsafe fn chooseFilename(&self, file_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(chooseFilenames:)]
        unsafe fn chooseFilenames(&self, file_names: Option<&NSArray>);

        #[method(cancel)]
        unsafe fn cancel(&self);
    }

    unsafe impl ProtocolType for dyn WebOpenPanelResultListener {}
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebUIDelegate: NSObjectProtocol {
        #[cfg(all(feature = "Foundation_NSURLRequest", feature = "WebKit_WebView"))]
        #[optional]
        #[method_id(@__retain_semantics Other webView:createWebViewWithRequest:)]
        unsafe fn webView_createWebViewWithRequest(
            &self,
            sender: Option<&WebView>,
            request: Option<&NSURLRequest>,
        ) -> Option<Id<WebView>>;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webViewShow:)]
        unsafe fn webViewShow(&self, sender: Option<&WebView>);

        #[cfg(all(feature = "Foundation_NSURLRequest", feature = "WebKit_WebView"))]
        #[optional]
        #[method_id(@__retain_semantics Other webView:createWebViewModalDialogWithRequest:)]
        unsafe fn webView_createWebViewModalDialogWithRequest(
            &self,
            sender: Option<&WebView>,
            request: Option<&NSURLRequest>,
        ) -> Option<Id<WebView>>;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webViewRunModal:)]
        unsafe fn webViewRunModal(&self, sender: Option<&WebView>);

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webViewClose:)]
        unsafe fn webViewClose(&self, sender: Option<&WebView>);

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webViewFocus:)]
        unsafe fn webViewFocus(&self, sender: Option<&WebView>);

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webViewUnfocus:)]
        unsafe fn webViewUnfocus(&self, sender: Option<&WebView>);

        #[cfg(all(feature = "AppKit_NSResponder", feature = "WebKit_WebView"))]
        #[optional]
        #[method_id(@__retain_semantics Other webViewFirstResponder:)]
        unsafe fn webViewFirstResponder(&self, sender: Option<&WebView>)
            -> Option<Id<NSResponder>>;

        #[cfg(all(feature = "AppKit_NSResponder", feature = "WebKit_WebView"))]
        #[optional]
        #[method(webView:makeFirstResponder:)]
        unsafe fn webView_makeFirstResponder(
            &self,
            sender: Option<&WebView>,
            responder: Option<&NSResponder>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_WebView"))]
        #[optional]
        #[method(webView:setStatusText:)]
        unsafe fn webView_setStatusText(&self, sender: Option<&WebView>, text: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_WebView"))]
        #[optional]
        #[method_id(@__retain_semantics Other webViewStatusText:)]
        unsafe fn webViewStatusText(&self, sender: Option<&WebView>) -> Option<Id<NSString>>;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webViewAreToolbarsVisible:)]
        unsafe fn webViewAreToolbarsVisible(&self, sender: Option<&WebView>) -> bool;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:setToolbarsVisible:)]
        unsafe fn webView_setToolbarsVisible(&self, sender: Option<&WebView>, visible: bool);

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webViewIsStatusBarVisible:)]
        unsafe fn webViewIsStatusBarVisible(&self, sender: Option<&WebView>) -> bool;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:setStatusBarVisible:)]
        unsafe fn webView_setStatusBarVisible(&self, sender: Option<&WebView>, visible: bool);

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webViewIsResizable:)]
        unsafe fn webViewIsResizable(&self, sender: Option<&WebView>) -> bool;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:setResizable:)]
        unsafe fn webView_setResizable(&self, sender: Option<&WebView>, resizable: bool);

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:setFrame:)]
        unsafe fn webView_setFrame(&self, sender: Option<&WebView>, frame: NSRect);

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webViewFrame:)]
        unsafe fn webViewFrame(&self, sender: Option<&WebView>) -> NSRect;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:runJavaScriptAlertPanelWithMessage:initiatedByFrame:)]
        unsafe fn webView_runJavaScriptAlertPanelWithMessage_initiatedByFrame(
            &self,
            sender: Option<&WebView>,
            message: Option<&NSString>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:runJavaScriptConfirmPanelWithMessage:initiatedByFrame:)]
        unsafe fn webView_runJavaScriptConfirmPanelWithMessage_initiatedByFrame(
            &self,
            sender: Option<&WebView>,
            message: Option<&NSString>,
            frame: Option<&WebFrame>,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other webView:runJavaScriptTextInputPanelWithPrompt:defaultText:initiatedByFrame:)]
        unsafe fn webView_runJavaScriptTextInputPanelWithPrompt_defaultText_initiatedByFrame(
            &self,
            sender: Option<&WebView>,
            prompt: Option<&NSString>,
            default_text: Option<&NSString>,
            frame: Option<&WebFrame>,
        ) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:runBeforeUnloadConfirmPanelWithMessage:initiatedByFrame:)]
        unsafe fn webView_runBeforeUnloadConfirmPanelWithMessage_initiatedByFrame(
            &self,
            sender: Option<&WebView>,
            message: Option<&NSString>,
            frame: Option<&WebFrame>,
        ) -> bool;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:runOpenPanelForFileButtonWithResultListener:)]
        unsafe fn webView_runOpenPanelForFileButtonWithResultListener(
            &self,
            sender: Option<&WebView>,
            result_listener: Option<&ProtocolObject<dyn WebOpenPanelResultListener>>,
        );

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:runOpenPanelForFileButtonWithResultListener:allowMultipleFiles:)]
        unsafe fn webView_runOpenPanelForFileButtonWithResultListener_allowMultipleFiles(
            &self,
            sender: Option<&WebView>,
            result_listener: Option<&ProtocolObject<dyn WebOpenPanelResultListener>>,
            allow_multiple_files: bool,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "WebKit_WebView"))]
        #[optional]
        #[method(webView:mouseDidMoveOverElement:modifierFlags:)]
        unsafe fn webView_mouseDidMoveOverElement_modifierFlags(
            &self,
            sender: Option<&WebView>,
            element_information: Option<&NSDictionary>,
            modifier_flags: NSUInteger,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other webView:contextMenuItemsForElement:defaultMenuItems:)]
        unsafe fn webView_contextMenuItemsForElement_defaultMenuItems(
            &self,
            sender: Option<&WebView>,
            element: Option<&NSDictionary>,
            default_menu_items: Option<&NSArray>,
        ) -> Option<Id<NSArray>>;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:validateUserInterfaceItem:defaultValidation:)]
        unsafe fn webView_validateUserInterfaceItem_defaultValidation(
            &self,
            web_view: Option<&WebView>,
            item: Option<&ProtocolObject<dyn NSValidatedUserInterfaceItem>>,
            default_validation: bool,
        ) -> bool;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:shouldPerformAction:fromSender:)]
        unsafe fn webView_shouldPerformAction_fromSender(
            &self,
            web_view: Option<&WebView>,
            action: Option<Sel>,
            sender: Option<&AnyObject>,
        ) -> bool;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:dragDestinationActionMaskForDraggingInfo:)]
        unsafe fn webView_dragDestinationActionMaskForDraggingInfo(
            &self,
            web_view: Option<&WebView>,
            dragging_info: Option<&ProtocolObject<dyn NSDraggingInfo>>,
        ) -> NSUInteger;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:willPerformDragDestinationAction:forDraggingInfo:)]
        unsafe fn webView_willPerformDragDestinationAction_forDraggingInfo(
            &self,
            web_view: Option<&WebView>,
            action: WebDragDestinationAction,
            dragging_info: Option<&ProtocolObject<dyn NSDraggingInfo>>,
        );

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:dragSourceActionMaskForPoint:)]
        unsafe fn webView_dragSourceActionMaskForPoint(
            &self,
            web_view: Option<&WebView>,
            point: NSPoint,
        ) -> NSUInteger;

        #[cfg(all(feature = "AppKit_NSPasteboard", feature = "WebKit_WebView"))]
        #[optional]
        #[method(webView:willPerformDragSourceAction:fromPoint:withPasteboard:)]
        unsafe fn webView_willPerformDragSourceAction_fromPoint_withPasteboard(
            &self,
            web_view: Option<&WebView>,
            action: WebDragSourceAction,
            point: NSPoint,
            pasteboard: Option<&NSPasteboard>,
        );

        #[cfg(all(feature = "WebKit_WebFrameView", feature = "WebKit_WebView"))]
        #[optional]
        #[method(webView:printFrameView:)]
        unsafe fn webView_printFrameView(
            &self,
            sender: Option<&WebView>,
            frame_view: Option<&WebFrameView>,
        );

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webViewHeaderHeight:)]
        unsafe fn webViewHeaderHeight(&self, sender: Option<&WebView>) -> c_float;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webViewFooterHeight:)]
        unsafe fn webViewFooterHeight(&self, sender: Option<&WebView>) -> c_float;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:drawHeaderInRect:)]
        unsafe fn webView_drawHeaderInRect(&self, sender: Option<&WebView>, rect: NSRect);

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:drawFooterInRect:)]
        unsafe fn webView_drawFooterInRect(&self, sender: Option<&WebView>, rect: NSRect);

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_WebView"))]
        #[deprecated]
        #[optional]
        #[method(webView:runJavaScriptAlertPanelWithMessage:)]
        unsafe fn webView_runJavaScriptAlertPanelWithMessage(
            &self,
            sender: Option<&WebView>,
            message: Option<&NSString>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_WebView"))]
        #[deprecated]
        #[optional]
        #[method(webView:runJavaScriptConfirmPanelWithMessage:)]
        unsafe fn webView_runJavaScriptConfirmPanelWithMessage(
            &self,
            sender: Option<&WebView>,
            message: Option<&NSString>,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_WebView"))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other webView:runJavaScriptTextInputPanelWithPrompt:defaultText:)]
        unsafe fn webView_runJavaScriptTextInputPanelWithPrompt_defaultText(
            &self,
            sender: Option<&WebView>,
            prompt: Option<&NSString>,
            default_text: Option<&NSString>,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "WebKit_WebView")]
        #[deprecated]
        #[optional]
        #[method(webView:setContentRect:)]
        unsafe fn webView_setContentRect(&self, sender: Option<&WebView>, frame: NSRect);

        #[cfg(feature = "WebKit_WebView")]
        #[deprecated]
        #[optional]
        #[method(webViewContentRect:)]
        unsafe fn webViewContentRect(&self, sender: Option<&WebView>) -> NSRect;
    }

    unsafe impl ProtocolType for dyn WebUIDelegate {}
);