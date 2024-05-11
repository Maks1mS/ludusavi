//! Identify keyboard keys.
use crate::SmolStr;

/// A key on the keyboard.
///
/// This is mostly the `Key` type found in [`winit`].
///
/// [`winit`]: https://docs.rs/winit/0.29.10/winit/keyboard/enum.Key.html
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key<C = SmolStr> {
    /// A key with an established name.
    Named(Named),

    /// A key string that corresponds to the character typed by the user, taking into account the
    /// user’s current locale setting, and any system-level keyboard mapping overrides that are in
    /// effect.
    Character(C),

    /// An unidentified key.
    Unidentified,
}

impl Key {
    /// Convert `Key::Character(SmolStr)` to `Key::Character(&str)` so you can more easily match on
    /// `Key`. All other variants remain unchanged.
    pub fn as_ref(&self) -> Key<&str> {
        match self {
            Self::Named(named) => Key::Named(*named),
            Self::Character(c) => Key::Character(c.as_ref()),
            Self::Unidentified => Key::Unidentified,
        }
    }
}

/// A named key.
///
/// This is mostly the `NamedKey` type found in [`winit`].
///
/// [`winit`]: https://docs.rs/winit/0.29.10/winit/keyboard/enum.Key.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Named {
    /// The `Alt` (Alternative) key.
    ///
    /// This key enables the alternate modifier function for interpreting concurrent or subsequent
    /// keyboard input. This key value is also used for the Apple <kbd>Option</kbd> key.
    Alt,
    /// The Alternate Graphics (<kbd>AltGr</kbd> or <kbd>AltGraph</kbd>) key.
    ///
    /// This key is used enable the ISO Level 3 shift modifier (the standard `Shift` key is the
    /// level 2 modifier).
    AltGraph,
    /// The `Caps Lock` (Capital) key.
    ///
    /// Toggle capital character lock function for interpreting subsequent keyboard input event.
    CapsLock,
    /// The `Control` or `Ctrl` key.
    ///
    /// Used to enable control modifier function for interpreting concurrent or subsequent keyboard
    /// input.
    Control,
    /// The Function switch `Fn` key. Activating this key simultaneously with another key changes
    /// that key’s value to an alternate character or function. This key is often handled directly
    /// in the keyboard hardware and does not usually generate key events.
    Fn,
    /// The Function-Lock (`FnLock` or `F-Lock`) key. Activating this key switches the mode of the
    /// keyboard to changes some keys' values to an alternate character or function. This key is
    /// often handled directly in the keyboard hardware and does not usually generate key events.
    FnLock,
    /// The `NumLock` or Number Lock key. Used to toggle numpad mode function for interpreting
    /// subsequent keyboard input.
    NumLock,
    /// Toggle between scrolling and cursor movement modes.
    ScrollLock,
    /// Used to enable shift modifier function for interpreting concurrent or subsequent keyboard
    /// input.
    Shift,
    /// The Symbol modifier key (used on some virtual keyboards).
    Symbol,
    SymbolLock,
    // Legacy modifier key. Also called "Super" in certain places.
    Meta,
    // Legacy modifier key.
    Hyper,
    /// Used to enable "super" modifier function for interpreting concurrent or subsequent keyboard
    /// input. This key value is used for the "Windows Logo" key and the Apple `Command` or `⌘` key.
    ///
    /// Note: In some contexts (e.g. the Web) this is referred to as the "Meta" key.
    Super,
    /// The `Enter` or `↵` key. Used to activate current selection or accept current input. This key
    /// value is also used for the `Return` (Macintosh numpad) key. This key value is also used for
    /// the Android `KEYCODE_DPAD_CENTER`.
    Enter,
    /// The Horizontal Tabulation `Tab` key.
    Tab,
    /// Used in text to insert a space between words. Usually located below the character keys.
    Space,
    /// Navigate or traverse downward. (`KEYCODE_DPAD_DOWN`)
    ArrowDown,
    /// Navigate or traverse leftward. (`KEYCODE_DPAD_LEFT`)
    ArrowLeft,
    /// Navigate or traverse rightward. (`KEYCODE_DPAD_RIGHT`)
    ArrowRight,
    /// Navigate or traverse upward. (`KEYCODE_DPAD_UP`)
    ArrowUp,
    /// The End key, used with keyboard entry to go to the end of content (`KEYCODE_MOVE_END`).
    End,
    /// The Home key, used with keyboard entry, to go to start of content (`KEYCODE_MOVE_HOME`).
    /// For the mobile phone `Home` key (which goes to the phone’s main screen), use [`GoHome`].
    ///
    /// [`GoHome`]: Self::GoHome
    Home,
    /// Scroll down or display next page of content.
    PageDown,
    /// Scroll up or display previous page of content.
    PageUp,
    /// Used to remove the character to the left of the cursor. This key value is also used for
    /// the key labeled `Delete` on MacOS keyboards.
    Backspace,
    /// Remove the currently selected input.
    Clear,
    /// Copy the current selection. (`APPCOMMAND_COPY`)
    Copy,
    /// The Cursor Select key.
    CrSel,
    /// Cut the current selection. (`APPCOMMAND_CUT`)
    Cut,
    /// Used to delete the character to the right of the cursor. This key value is also used for the
    /// key labeled `Delete` on MacOS keyboards when `Fn` is active.
    Delete,
    /// The Erase to End of Field key. This key deletes all characters from the current cursor
    /// position to the end of the current field.
    EraseEof,
    /// The Extend Selection (Exsel) key.
    ExSel,
    /// Toggle between text modes for insertion or overtyping.
    /// (`KEYCODE_INSERT`)
    Insert,
    /// The Paste key. (`APPCOMMAND_PASTE`)
    Paste,
    /// Redo the last action. (`APPCOMMAND_REDO`)
    Redo,
    /// Undo the last action. (`APPCOMMAND_UNDO`)
    Undo,
    /// The Accept (Commit, OK) key. Accept current option or input method sequence conversion.
    Accept,
    /// Redo or repeat an action.
    Again,
    /// The Attention (Attn) key.
    Attn,
    Cancel,
    /// Show the application’s context menu.
    /// This key is commonly found between the right `Super` key and the right `Control` key.
    ContextMenu,
    /// The `Esc` key. This key was originally used to initiate an escape sequence, but is
    /// now more generally used to exit or "escape" the current context, such as closing a dialog
    /// or exiting full screen mode.
    Escape,
    Execute,
    /// Open the Find dialog. (`APPCOMMAND_FIND`)
    Find,
    /// Open a help dialog or toggle display of help information. (`APPCOMMAND_HELP`,
    /// `KEYCODE_HELP`)
    Help,
    /// Pause the current state or application (as appropriate).
    ///
    /// Note: Do not use this value for the `Pause` button on media controllers. Use `"MediaPause"`
    /// instead.
    Pause,
    /// Play or resume the current state or application (as appropriate).
    ///
    /// Note: Do not use this value for the `Play` button on media controllers. Use `"MediaPlay"`
    /// instead.
    Play,
    /// The properties (Props) key.
    Props,
    Select,
    /// The ZoomIn key. (`KEYCODE_ZOOM_IN`)
    ZoomIn,
    /// The ZoomOut key. (`KEYCODE_ZOOM_OUT`)
    ZoomOut,
    /// The Brightness Down key. Typically controls the display brightness.
    /// (`KEYCODE_BRIGHTNESS_DOWN`)
    BrightnessDown,
    /// The Brightness Up key. Typically controls the display brightness. (`KEYCODE_BRIGHTNESS_UP`)
    BrightnessUp,
    /// Toggle removable media to eject (open) and insert (close) state. (`KEYCODE_MEDIA_EJECT`)
    Eject,
    LogOff,
    /// Toggle power state. (`KEYCODE_POWER`)
    /// Note: Note: Some devices might not expose this key to the operating environment.
    Power,
    /// The `PowerOff` key. Sometime called `PowerDown`.
    PowerOff,
    /// Initiate print-screen function.
    PrintScreen,
    /// The Hibernate key. This key saves the current state of the computer to disk so that it can
    /// be restored. The computer will then shutdown.
    Hibernate,
    /// The Standby key. This key turns off the display and places the computer into a low-power
    /// mode without completely shutting down. It is sometimes labelled `Suspend` or `Sleep` key.
    /// (`KEYCODE_SLEEP`)
    Standby,
    /// The WakeUp key. (`KEYCODE_WAKEUP`)
    WakeUp,
    /// Initate the multi-candidate mode.
    AllCandidates,
    Alphanumeric,
    /// Initiate the Code Input mode to allow characters to be entered by
    /// their code points.
    CodeInput,
    /// The Compose key, also known as "Multi_key" on the X Window System. This key acts in a
    /// manner similar to a dead key, triggering a mode where subsequent key presses are combined to
    /// produce a different character.
    Compose,
    /// Convert the current input method sequence.
    Convert,
    /// The Final Mode `Final` key used on some Asian keyboards, to enable the final mode for IMEs.
    FinalMode,
    /// Switch to the first character group. (ISO/IEC 9995)
    GroupFirst,
    /// Switch to the last character group. (ISO/IEC 9995)
    GroupLast,
    /// Switch to the next character group. (ISO/IEC 9995)
    GroupNext,
    /// Switch to the previous character group. (ISO/IEC 9995)
    GroupPrevious,
    /// Toggle between or cycle through input modes of IMEs.
    ModeChange,
    NextCandidate,
    /// Accept current input method sequence without
    /// conversion in IMEs.
    NonConvert,
    PreviousCandidate,
    Process,
    SingleCandidate,
    /// Toggle between Hangul and English modes.
    HangulMode,
    HanjaMode,
    JunjaMode,
    /// The Eisu key. This key may close the IME, but its purpose is defined by the current IME.
    /// (`KEYCODE_EISU`)
    Eisu,
    /// The (Half-Width) Characters key.
    Hankaku,
    /// The Hiragana (Japanese Kana characters) key.
    Hiragana,
    /// The Hiragana/Katakana toggle key. (`KEYCODE_KATAKANA_HIRAGANA`)
    HiraganaKatakana,
    /// The Kana Mode (Kana Lock) key. This key is used to enter hiragana mode (typically from
    /// romaji mode).
    KanaMode,
    /// The Kanji (Japanese name for ideographic characters of Chinese origin) Mode key. This key is
    /// typically used to switch to a hiragana keyboard for the purpose of converting input into
    /// kanji. (`KEYCODE_KANA`)
    KanjiMode,
    /// The Katakana (Japanese Kana characters) key.
    Katakana,
    /// The Roman characters function key.
    Romaji,
    /// The Zenkaku (Full-Width) Characters key.
    Zenkaku,
    /// The Zenkaku/Hankaku (full-width/half-width) toggle key. (`KEYCODE_ZENKAKU_HANKAKU`)
    ZenkakuHankaku,
    /// General purpose virtual function key, as index 1.
    Soft1,
    /// General purpose virtual function key, as index 2.
    Soft2,
    /// General purpose virtual function key, as index 3.
    Soft3,
    /// General purpose virtual function key, as index 4.
    Soft4,
    /// Select next (numerically or logically) lower channel. (`APPCOMMAND_MEDIA_CHANNEL_DOWN`,
    /// `KEYCODE_CHANNEL_DOWN`)
    ChannelDown,
    /// Select next (numerically or logically) higher channel. (`APPCOMMAND_MEDIA_CHANNEL_UP`,
    /// `KEYCODE_CHANNEL_UP`)
    ChannelUp,
    /// Close the current document or message (Note: This doesn’t close the application).
    /// (`APPCOMMAND_CLOSE`)
    Close,
    /// Open an editor to forward the current message. (`APPCOMMAND_FORWARD_MAIL`)
    MailForward,
    /// Open an editor to reply to the current message. (`APPCOMMAND_REPLY_TO_MAIL`)
    MailReply,
    /// Send the current message. (`APPCOMMAND_SEND_MAIL`)
    MailSend,
    /// Close the current media, for example to close a CD or DVD tray. (`KEYCODE_MEDIA_CLOSE`)
    MediaClose,
    /// Initiate or continue forward playback at faster than normal speed, or increase speed if
    /// already fast forwarding. (`APPCOMMAND_MEDIA_FAST_FORWARD`, `KEYCODE_MEDIA_FAST_FORWARD`)
    MediaFastForward,
    /// Pause the currently playing media. (`APPCOMMAND_MEDIA_PAUSE`, `KEYCODE_MEDIA_PAUSE`)
    ///
    /// Note: Media controller devices should use this value rather than `"Pause"` for their pause
    /// keys.
    MediaPause,
    /// Initiate or continue media playback at normal speed, if not currently playing at normal
    /// speed. (`APPCOMMAND_MEDIA_PLAY`, `KEYCODE_MEDIA_PLAY`)
    MediaPlay,
    /// Toggle media between play and pause states. (`APPCOMMAND_MEDIA_PLAY_PAUSE`,
    /// `KEYCODE_MEDIA_PLAY_PAUSE`)
    MediaPlayPause,
    /// Initiate or resume recording of currently selected media. (`APPCOMMAND_MEDIA_RECORD`,
    /// `KEYCODE_MEDIA_RECORD`)
    MediaRecord,
    /// Initiate or continue reverse playback at faster than normal speed, or increase speed if
    /// already rewinding. (`APPCOMMAND_MEDIA_REWIND`, `KEYCODE_MEDIA_REWIND`)
    MediaRewind,
    /// Stop media playing, pausing, forwarding, rewinding, or recording, if not already stopped.
    /// (`APPCOMMAND_MEDIA_STOP`, `KEYCODE_MEDIA_STOP`)
    MediaStop,
    /// Seek to next media or program track. (`APPCOMMAND_MEDIA_NEXTTRACK`, `KEYCODE_MEDIA_NEXT`)
    MediaTrackNext,
    /// Seek to previous media or program track. (`APPCOMMAND_MEDIA_PREVIOUSTRACK`,
    /// `KEYCODE_MEDIA_PREVIOUS`)
    MediaTrackPrevious,
    /// Open a new document or message. (`APPCOMMAND_NEW`)
    New,
    /// Open an existing document or message. (`APPCOMMAND_OPEN`)
    Open,
    /// Print the current document or message. (`APPCOMMAND_PRINT`)
    Print,
    /// Save the current document or message. (`APPCOMMAND_SAVE`)
    Save,
    /// Spellcheck the current document or selection. (`APPCOMMAND_SPELL_CHECK`)
    SpellCheck,
    /// The `11` key found on media numpads that
    /// have buttons from `1` ... `12`.
    Key11,
    /// The `12` key found on media numpads that
    /// have buttons from `1` ... `12`.
    Key12,
    /// Adjust audio balance leftward. (`VK_AUDIO_BALANCE_LEFT`)
    AudioBalanceLeft,
    /// Adjust audio balance rightward. (`VK_AUDIO_BALANCE_RIGHT`)
    AudioBalanceRight,
    /// Decrease audio bass boost or cycle down through bass boost states. (`APPCOMMAND_BASS_DOWN`,
    /// `VK_BASS_BOOST_DOWN`)
    AudioBassBoostDown,
    /// Toggle bass boost on/off. (`APPCOMMAND_BASS_BOOST`)
    AudioBassBoostToggle,
    /// Increase audio bass boost or cycle up through bass boost states. (`APPCOMMAND_BASS_UP`,
    /// `VK_BASS_BOOST_UP`)
    AudioBassBoostUp,
    /// Adjust audio fader towards front. (`VK_FADER_FRONT`)
    AudioFaderFront,
    /// Adjust audio fader towards rear. (`VK_FADER_REAR`)
    AudioFaderRear,
    /// Advance surround audio mode to next available mode. (`VK_SURROUND_MODE_NEXT`)
    AudioSurroundModeNext,
    /// Decrease treble. (`APPCOMMAND_TREBLE_DOWN`)
    AudioTrebleDown,
    /// Increase treble. (`APPCOMMAND_TREBLE_UP`)
    AudioTrebleUp,
    /// Decrease audio volume. (`APPCOMMAND_VOLUME_DOWN`, `KEYCODE_VOLUME_DOWN`)
    AudioVolumeDown,
    /// Increase audio volume. (`APPCOMMAND_VOLUME_UP`, `KEYCODE_VOLUME_UP`)
    AudioVolumeUp,
    /// Toggle between muted state and prior volume level. (`APPCOMMAND_VOLUME_MUTE`,
    /// `KEYCODE_VOLUME_MUTE`)
    AudioVolumeMute,
    /// Toggle the microphone on/off. (`APPCOMMAND_MIC_ON_OFF_TOGGLE`)
    MicrophoneToggle,
    /// Decrease microphone volume. (`APPCOMMAND_MICROPHONE_VOLUME_DOWN`)
    MicrophoneVolumeDown,
    /// Increase microphone volume. (`APPCOMMAND_MICROPHONE_VOLUME_UP`)
    MicrophoneVolumeUp,
    /// Mute the microphone. (`APPCOMMAND_MICROPHONE_VOLUME_MUTE`, `KEYCODE_MUTE`)
    MicrophoneVolumeMute,
    /// Show correction list when a word is incorrectly identified. (`APPCOMMAND_CORRECTION_LIST`)
    SpeechCorrectionList,
    /// Toggle between dictation mode and command/control mode.
    /// (`APPCOMMAND_DICTATE_OR_COMMAND_CONTROL_TOGGLE`)
    SpeechInputToggle,
    /// The first generic "LaunchApplication" key. This is commonly associated with launching "My
    /// Computer", and may have a computer symbol on the key. (`APPCOMMAND_LAUNCH_APP1`)
    LaunchApplication1,
    /// The second generic "LaunchApplication" key. This is commonly associated with launching
    /// "Calculator", and may have a calculator symbol on the key. (`APPCOMMAND_LAUNCH_APP2`,
    /// `KEYCODE_CALCULATOR`)
    LaunchApplication2,
    /// The "Calendar" key. (`KEYCODE_CALENDAR`)
    LaunchCalendar,
    /// The "Contacts" key. (`KEYCODE_CONTACTS`)
    LaunchContacts,
    /// The "Mail" key. (`APPCOMMAND_LAUNCH_MAIL`)
    LaunchMail,
    /// The "Media Player" key. (`APPCOMMAND_LAUNCH_MEDIA_SELECT`)
    LaunchMediaPlayer,
    LaunchMusicPlayer,
    LaunchPhone,
    LaunchScreenSaver,
    LaunchSpreadsheet,
    LaunchWebBrowser,
    LaunchWebCam,
    LaunchWordProcessor,
    /// Navigate to previous content or page in current history. (`APPCOMMAND_BROWSER_BACKWARD`)
    BrowserBack,
    /// Open the list of browser favorites. (`APPCOMMAND_BROWSER_FAVORITES`)
    BrowserFavorites,
    /// Navigate to next content or page in current history. (`APPCOMMAND_BROWSER_FORWARD`)
    BrowserForward,
    /// Go to the user’s preferred home page. (`APPCOMMAND_BROWSER_HOME`)
    BrowserHome,
    /// Refresh the current page or content. (`APPCOMMAND_BROWSER_REFRESH`)
    BrowserRefresh,
    /// Call up the user’s preferred search page. (`APPCOMMAND_BROWSER_SEARCH`)
    BrowserSearch,
    /// Stop loading the current page or content. (`APPCOMMAND_BROWSER_STOP`)
    BrowserStop,
    /// The Application switch key, which provides a list of recent apps to switch between.
    /// (`KEYCODE_APP_SWITCH`)
    AppSwitch,
    /// The Call key. (`KEYCODE_CALL`)
    Call,
    /// The Camera key. (`KEYCODE_CAMERA`)
    Camera,
    /// The Camera focus key. (`KEYCODE_FOCUS`)
    CameraFocus,
    /// The End Call key. (`KEYCODE_ENDCALL`)
    EndCall,
    /// The Back key. (`KEYCODE_BACK`)
    GoBack,
    /// The Home key, which goes to the phone’s main screen. (`KEYCODE_HOME`)
    GoHome,
    /// The Headset Hook key. (`KEYCODE_HEADSETHOOK`)
    HeadsetHook,
    LastNumberRedial,
    /// The Notification key. (`KEYCODE_NOTIFICATION`)
    Notification,
    /// Toggle between manner mode state: silent, vibrate, ring, ... (`KEYCODE_MANNER_MODE`)
    MannerMode,
    VoiceDial,
    /// Switch to viewing TV. (`KEYCODE_TV`)
    TV,
    /// TV 3D Mode. (`KEYCODE_3D_MODE`)
    TV3DMode,
    /// Toggle between antenna and cable input. (`KEYCODE_TV_ANTENNA_CABLE`)
    TVAntennaCable,
    /// Audio description. (`KEYCODE_TV_AUDIO_DESCRIPTION`)
    TVAudioDescription,
    /// Audio description mixing volume down. (`KEYCODE_TV_AUDIO_DESCRIPTION_MIX_DOWN`)
    TVAudioDescriptionMixDown,
    /// Audio description mixing volume up. (`KEYCODE_TV_AUDIO_DESCRIPTION_MIX_UP`)
    TVAudioDescriptionMixUp,
    /// Contents menu. (`KEYCODE_TV_CONTENTS_MENU`)
    TVContentsMenu,
    /// Contents menu. (`KEYCODE_TV_DATA_SERVICE`)
    TVDataService,
    /// Switch the input mode on an external TV. (`KEYCODE_TV_INPUT`)
    TVInput,
    /// Switch to component input #1. (`KEYCODE_TV_INPUT_COMPONENT_1`)
    TVInputComponent1,
    /// Switch to component input #2. (`KEYCODE_TV_INPUT_COMPONENT_2`)
    TVInputComponent2,
    /// Switch to composite input #1. (`KEYCODE_TV_INPUT_COMPOSITE_1`)
    TVInputComposite1,
    /// Switch to composite input #2. (`KEYCODE_TV_INPUT_COMPOSITE_2`)
    TVInputComposite2,
    /// Switch to HDMI input #1. (`KEYCODE_TV_INPUT_HDMI_1`)
    TVInputHDMI1,
    /// Switch to HDMI input #2. (`KEYCODE_TV_INPUT_HDMI_2`)
    TVInputHDMI2,
    /// Switch to HDMI input #3. (`KEYCODE_TV_INPUT_HDMI_3`)
    TVInputHDMI3,
    /// Switch to HDMI input #4. (`KEYCODE_TV_INPUT_HDMI_4`)
    TVInputHDMI4,
    /// Switch to VGA input #1. (`KEYCODE_TV_INPUT_VGA_1`)
    TVInputVGA1,
    /// Media context menu. (`KEYCODE_TV_MEDIA_CONTEXT_MENU`)
    TVMediaContext,
    /// Toggle network. (`KEYCODE_TV_NETWORK`)
    TVNetwork,
    /// Number entry. (`KEYCODE_TV_NUMBER_ENTRY`)
    TVNumberEntry,
    /// Toggle the power on an external TV. (`KEYCODE_TV_POWER`)
    TVPower,
    /// Radio. (`KEYCODE_TV_RADIO_SERVICE`)
    TVRadioService,
    /// Satellite. (`KEYCODE_TV_SATELLITE`)
    TVSatellite,
    /// Broadcast Satellite. (`KEYCODE_TV_SATELLITE_BS`)
    TVSatelliteBS,
    /// Communication Satellite. (`KEYCODE_TV_SATELLITE_CS`)
    TVSatelliteCS,
    /// Toggle between available satellites. (`KEYCODE_TV_SATELLITE_SERVICE`)
    TVSatelliteToggle,
    /// Analog Terrestrial. (`KEYCODE_TV_TERRESTRIAL_ANALOG`)
    TVTerrestrialAnalog,
    /// Digital Terrestrial. (`KEYCODE_TV_TERRESTRIAL_DIGITAL`)
    TVTerrestrialDigital,
    /// Timer programming. (`KEYCODE_TV_TIMER_PROGRAMMING`)
    TVTimer,
    /// Switch the input mode on an external AVR (audio/video receiver). (`KEYCODE_AVR_INPUT`)
    AVRInput,
    /// Toggle the power on an external AVR (audio/video receiver). (`KEYCODE_AVR_POWER`)
    AVRPower,
    /// General purpose color-coded media function key, as index 0 (red). (`VK_COLORED_KEY_0`,
    /// `KEYCODE_PROG_RED`)
    ColorF0Red,
    /// General purpose color-coded media function key, as index 1 (green). (`VK_COLORED_KEY_1`,
    /// `KEYCODE_PROG_GREEN`)
    ColorF1Green,
    /// General purpose color-coded media function key, as index 2 (yellow). (`VK_COLORED_KEY_2`,
    /// `KEYCODE_PROG_YELLOW`)
    ColorF2Yellow,
    /// General purpose color-coded media function key, as index 3 (blue). (`VK_COLORED_KEY_3`,
    /// `KEYCODE_PROG_BLUE`)
    ColorF3Blue,
    /// General purpose color-coded media function key, as index 4 (grey). (`VK_COLORED_KEY_4`)
    ColorF4Grey,
    /// General purpose color-coded media function key, as index 5 (brown). (`VK_COLORED_KEY_5`)
    ColorF5Brown,
    /// Toggle the display of Closed Captions. (`VK_CC`, `KEYCODE_CAPTIONS`)
    ClosedCaptionToggle,
    /// Adjust brightness of device, by toggling between or cycling through states. (`VK_DIMMER`)
    Dimmer,
    /// Swap video sources. (`VK_DISPLAY_SWAP`)
    DisplaySwap,
    /// Select Digital Video Rrecorder. (`KEYCODE_DVR`)
    DVR,
    /// Exit the current application. (`VK_EXIT`)
    Exit,
    /// Clear program or content stored as favorite 0. (`VK_CLEAR_FAVORITE_0`)
    FavoriteClear0,
    /// Clear program or content stored as favorite 1. (`VK_CLEAR_FAVORITE_1`)
    FavoriteClear1,
    /// Clear program or content stored as favorite 2. (`VK_CLEAR_FAVORITE_2`)
    FavoriteClear2,
    /// Clear program or content stored as favorite 3. (`VK_CLEAR_FAVORITE_3`)
    FavoriteClear3,
    /// Select (recall) program or content stored as favorite 0. (`VK_RECALL_FAVORITE_0`)
    FavoriteRecall0,
    /// Select (recall) program or content stored as favorite 1. (`VK_RECALL_FAVORITE_1`)
    FavoriteRecall1,
    /// Select (recall) program or content stored as favorite 2. (`VK_RECALL_FAVORITE_2`)
    FavoriteRecall2,
    /// Select (recall) program or content stored as favorite 3. (`VK_RECALL_FAVORITE_3`)
    FavoriteRecall3,
    /// Store current program or content as favorite 0. (`VK_STORE_FAVORITE_0`)
    FavoriteStore0,
    /// Store current program or content as favorite 1. (`VK_STORE_FAVORITE_1`)
    FavoriteStore1,
    /// Store current program or content as favorite 2. (`VK_STORE_FAVORITE_2`)
    FavoriteStore2,
    /// Store current program or content as favorite 3. (`VK_STORE_FAVORITE_3`)
    FavoriteStore3,
    /// Toggle display of program or content guide. (`VK_GUIDE`, `KEYCODE_GUIDE`)
    Guide,
    /// If guide is active and displayed, then display next day’s content. (`VK_NEXT_DAY`)
    GuideNextDay,
    /// If guide is active and displayed, then display previous day’s content. (`VK_PREV_DAY`)
    GuidePreviousDay,
    /// Toggle display of information about currently selected context or media. (`VK_INFO`,
    /// `KEYCODE_INFO`)
    Info,
    /// Toggle instant replay. (`VK_INSTANT_REPLAY`)
    InstantReplay,
    /// Launch linked content, if available and appropriate. (`VK_LINK`)
    Link,
    /// List the current program. (`VK_LIST`)
    ListProgram,
    /// Toggle display listing of currently available live content or programs. (`VK_LIVE`)
    LiveContent,
    /// Lock or unlock current content or program. (`VK_LOCK`)
    Lock,
    /// Show a list of media applications: audio/video players and image viewers. (`VK_APPS`)
    ///
    /// Note: Do not confuse this key value with the Windows' `VK_APPS` / `VK_CONTEXT_MENU` key,
    /// which is encoded as `"ContextMenu"`.
    MediaApps,
    /// Audio track key. (`KEYCODE_MEDIA_AUDIO_TRACK`)
    MediaAudioTrack,
    /// Select previously selected channel or media. (`VK_LAST`, `KEYCODE_LAST_CHANNEL`)
    MediaLast,
    /// Skip backward to next content or program. (`KEYCODE_MEDIA_SKIP_BACKWARD`)
    MediaSkipBackward,
    /// Skip forward to next content or program. (`VK_SKIP`, `KEYCODE_MEDIA_SKIP_FORWARD`)
    MediaSkipForward,
    /// Step backward to next content or program. (`KEYCODE_MEDIA_STEP_BACKWARD`)
    MediaStepBackward,
    /// Step forward to next content or program. (`KEYCODE_MEDIA_STEP_FORWARD`)
    MediaStepForward,
    /// Media top menu. (`KEYCODE_MEDIA_TOP_MENU`)
    MediaTopMenu,
    /// Navigate in. (`KEYCODE_NAVIGATE_IN`)
    NavigateIn,
    /// Navigate to next key. (`KEYCODE_NAVIGATE_NEXT`)
    NavigateNext,
    /// Navigate out. (`KEYCODE_NAVIGATE_OUT`)
    NavigateOut,
    /// Navigate to previous key. (`KEYCODE_NAVIGATE_PREVIOUS`)
    NavigatePrevious,
    /// Cycle to next favorite channel (in favorites list). (`VK_NEXT_FAVORITE_CHANNEL`)
    NextFavoriteChannel,
    /// Cycle to next user profile (if there are multiple user profiles). (`VK_USER`)
    NextUserProfile,
    /// Access on-demand content or programs. (`VK_ON_DEMAND`)
    OnDemand,
    /// Pairing key to pair devices. (`KEYCODE_PAIRING`)
    Pairing,
    /// Move picture-in-picture window down. (`VK_PINP_DOWN`)
    PinPDown,
    /// Move picture-in-picture window. (`VK_PINP_MOVE`)
    PinPMove,
    /// Toggle display of picture-in-picture window. (`VK_PINP_TOGGLE`)
    PinPToggle,
    /// Move picture-in-picture window up. (`VK_PINP_UP`)
    PinPUp,
    /// Decrease media playback speed. (`VK_PLAY_SPEED_DOWN`)
    PlaySpeedDown,
    /// Reset playback to normal speed. (`VK_PLAY_SPEED_RESET`)
    PlaySpeedReset,
    /// Increase media playback speed. (`VK_PLAY_SPEED_UP`)
    PlaySpeedUp,
    /// Toggle random media or content shuffle mode. (`VK_RANDOM_TOGGLE`)
    RandomToggle,
    /// Not a physical key, but this key code is sent when the remote control battery is low.
    /// (`VK_RC_LOW_BATTERY`)
    RcLowBattery,
    /// Toggle or cycle between media recording speeds. (`VK_RECORD_SPEED_NEXT`)
    RecordSpeedNext,
    /// Toggle RF (radio frequency) input bypass mode (pass RF input directly to the RF output).
    /// (`VK_RF_BYPASS`)
    RfBypass,
    /// Toggle scan channels mode. (`VK_SCAN_CHANNELS_TOGGLE`)
    ScanChannelsToggle,
    /// Advance display screen mode to next available mode. (`VK_SCREEN_MODE_NEXT`)
    ScreenModeNext,
    /// Toggle display of device settings screen. (`VK_SETTINGS`, `KEYCODE_SETTINGS`)
    Settings,
    /// Toggle split screen mode. (`VK_SPLIT_SCREEN_TOGGLE`)
    SplitScreenToggle,
    /// Switch the input mode on an external STB (set top box). (`KEYCODE_STB_INPUT`)
    STBInput,
    /// Toggle the power on an external STB (set top box). (`KEYCODE_STB_POWER`)
    STBPower,
    /// Toggle display of subtitles, if available. (`VK_SUBTITLE`)
    Subtitle,
    /// Toggle display of teletext, if available (`VK_TELETEXT`, `KEYCODE_TV_TELETEXT`).
    Teletext,
    /// Advance video mode to next available mode. (`VK_VIDEO_MODE_NEXT`)
    VideoModeNext,
    /// Cause device to identify itself in some manner, e.g., audibly or visibly. (`VK_WINK`)
    Wink,
    /// Toggle between full-screen and scaled content, or alter magnification level. (`VK_ZOOM`,
    /// `KEYCODE_TV_ZOOM_MODE`)
    ZoomToggle,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F1,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F2,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F3,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F4,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F5,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F6,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F7,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F8,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F9,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F10,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F11,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F12,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F13,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F14,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F15,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F16,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F17,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F18,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F19,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F20,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F21,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F22,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F23,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F24,
    /// General-purpose function key.
    F25,
    /// General-purpose function key.
    F26,
    /// General-purpose function key.
    F27,
    /// General-purpose function key.
    F28,
    /// General-purpose function key.
    F29,
    /// General-purpose function key.
    F30,
    /// General-purpose function key.
    F31,
    /// General-purpose function key.
    F32,
    /// General-purpose function key.
    F33,
    /// General-purpose function key.
    F34,
    /// General-purpose function key.
    F35,
}
