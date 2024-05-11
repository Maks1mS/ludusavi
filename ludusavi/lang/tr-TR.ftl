ludusavi = Ludusavi
language = Dil
game-name = Oyun İsmi
total-games = Oyunlar
file-size = Boyut
file-location = Konum
overall = Genel
status = Durum
cli-unrecognized-games = Bu oyunlar hakkında bilgi bulunamadı:
cli-unable-to-request-confirmation = Unable to request confirmation.
    .winpty-workaround = Eğer bash emulator kullanıyorsan (Git Bash gibi), winpty'i çalıştırmayı dene.
cli-backup-id-with-multiple-games = Oyunların yedeği geri yüklenirken backup ID belirlenemiyor.
cli-invalid-backup-id = Geçersiz backup ID.
badge-failed = BAŞARISIZ
badge-duplicates = KOPYALAR
badge-duplicated = KOPYALANMIŞ
badge-ignored = IGNORED
badge-redirected-from = Buradan: { $path }
badge-redirecting-to = TO: { $path }
some-entries-failed = Some entries failed to process; look for { badge-failed } in the output for details. Double check whether you can access those files or whether their paths are very long.
cli-game-line-item-redirected = Redirected from: { $path }
cli-game-line-item-redirecting = Redirecting to: { $path }
button-backup = Yedekle
button-preview = Önizle
button-restore = Geri Yükle
button-nav-backup = YEDEKLEME MODU
button-nav-restore = GERİ YÜKLEME MODU
button-nav-custom-games = ÖZEL OYUNLAR
button-nav-other = DİĞER
button-add-game = Oyun ekle
button-continue = Devam Et
button-cancel = İptal
button-cancelling = İptal ediliyor...
button-okay = Tamam
button-select-all = Tümünü seç
button-deselect-all = Tüm seçimi kaldır
button-enable-all = Hepsini etkinleştir
button-disable-all = Hepsini devre dışı bırak
button-customize = Kişiselleştir
button-exit = Çıkış
button-comment = Yorum Yap
button-lock = Kilitle
button-unlock = Kilidi aç
# This opens a download page.
button-get-app = Get { $app }
button-validate = Doğrula
no-roots-are-configured = Add some roots to back up even more data.
config-is-invalid = Error: The config file is invalid.
manifest-is-invalid = Error: The manifest file is invalid.
manifest-cannot-be-updated = Error: Unable to check for an update to the manifest file. Is your Internet connection down?
cannot-prepare-backup-target = Error: Unable to prepare backup target (either creating or emptying the folder). If you have the folder open in your file browser, try closing it: { $path }
restoration-source-is-invalid = Error: The restoration source is invalid (either doesn't exist or isn't a directory). Please double check the location: { $path }
registry-issue = Error: Some registry entries were skipped.
unable-to-browse-file-system = Error: Unable to browse on your system.
unable-to-open-directory = Error: Unable to open directory:
unable-to-open-url = Error: Unable to open URL:
unable-to-configure-cloud = Unable to configure cloud.
unable-to-synchronize-with-cloud = Unable to synchronize with cloud.
cloud-synchronize-conflict = Your local and cloud backups are in conflict. Perform an upload or download to resolve this.
command-unlaunched = Command did not launch: { $command }
command-terminated = Command terminated abruptly: { $command }
command-failed = Command failed with code { $code }: { $command }
processed-games =
    { $total-games } { $total-games ->
        [one] game
       *[other] games
    }
processed-games-subset =
    { $processed-games } of { $total-games } { $total-games ->
        [one] game
       *[other] games
    }
processed-size-subset = { $processed-size } of { $total-size }
field-backup-target = Back up to:
field-restore-source = Restore from:
field-custom-files = Paths:
field-custom-registry = Registry:
field-sort = Sort:
field-redirect-source =
    .placeholder = Source (original location)
field-redirect-target =
    .placeholder = Target (new location)
field-roots = Roots:
field-backup-excluded-items = Backup exclusions:
field-redirects = Redirects:
# This appears next to the number of full backups that you'd like to keep.
# A full backup includes all save files for a game.
field-retention-full = Full:
# This appears next to the number of differential backups that you'd like to keep.
# A differential backup includes only the files that have changed since the last full backup.
field-retention-differential = Differential:
field-backup-format = Format:
field-backup-compression = Compression:
# The compression level determines how much compresison we perform.
field-backup-compression-level = Level:
label-manifest = Manifest
# This shows the time when we checked for an update to the manifest.
label-checked = Checked
# This shows the time when we found an update to the manifest.
label-updated = Updated
label-new = New
label-removed = Removed
label-comment = Comment
label-unchanged = Unchanged
label-scan = Scan
label-filter = Filter
label-unique = Unique
label-complete = Complete
label-partial = Partial
label-enabled = Enabled
label-disabled = Disabled
# https://en.wikipedia.org/wiki/Thread_(computing)
label-threads = Threads
label-cloud = Cloud
# A "remote" is what Rclone calls cloud systems like Google Drive.
label-remote = Remote
label-remote-name = Remote name
label-folder = Folder
# An executable file
label-executable = Executable
# Options given to a command line program
label-arguments = Arguments
label-url = URL
# https://en.wikipedia.org/wiki/Host_(network)
label-host = Host
# https://en.wikipedia.org/wiki/Port_(computer_networking)
label-port = Port
label-username = Username
label-password = Password
# This is a specific website or service that provides some cloud functionality.
# For example, Nextcloud and Owncloud are providers of WebDAV services.
label-provider = Provider
label-custom = Custom
label-none = None
label-change-count = Changes: { $total }
label-unscanned = Unscanned
# This refers to a local file on the computer
label-file = File
label-game = Game
# Aliases are alternative titles for the same game.
label-alias = Alias
label-original-name = Original name
store-ea = EA
store-epic = Epic
store-gog = GOG
store-gog-galaxy = GOG Galaxy
store-heroic = Heroic
store-legendary = Legendary
store-lutris = Lutris
store-microsoft = Microsoft
store-origin = Origin
store-prime = Prime Gaming
store-steam = Steam
store-uplay = Uplay
store-other-home = Home folder
# This would be a folder acting as a virtual C: drive, created by Wine.
store-other-wine = Wine prefix
# This would be a folder with typical Windows system folders,
# like "Program Files (x86)" and "Users".
store-other-windows = Windows drive
# This would be a folder with typical Linux system folders,
# like "home" and "opt".
store-other-linux = Linux drive
# This would be a folder with typical Mac system folders,
# like "Applications" and "Users".
store-other-mac = Mac drive
store-other = Other
backup-format-simple = Simple
backup-format-zip = Zip
compression-none = Yok
# "Deflate" is a proper noun: https://en.wikipedia.org/wiki/Deflate
compression-deflate = Deflate
compression-bzip2 = Bzip2
compression-zstd = Zstd
theme = Tema
theme-light = Beyaz
theme-dark = Siyah
redirect-bidirectional = Bidirectional
show-deselected-games = Show deselected games
show-unchanged-games = Show unchanged games
show-unscanned-games = Show unscanned games
override-max-threads = Override max threads
synchronize-automatically = Synchronize automatically
prefer-alias-display = Display alias instead of original name
explanation-for-exclude-store-screenshots = In backups, exclude store-specific screenshots
consider-doing-a-preview =
    If you haven't already, consider doing a preview first so that there
    are no surprises.
confirm-backup =
    Are you sure you want to proceed with the backup? { $path-action ->
        [merge] New save data will be merged into the target folder:
       *[create] The target folder will be created:
    }
confirm-restore =
    Are you sure you want to proceed with the restoration?
    This will overwrite any current files with the backups from here:
confirm-cloud-upload =
    Do you want to replace your cloud files with your local files?
    Your cloud files ({ $cloud-path }) will become an exact copy of your local files ({ $local-path }).
    Files in the cloud will be updated or deleted as necessary.
confirm-cloud-download =
    Do you want to replace your local files with your cloud files?
    Your local files ({ $local-path }) will become an exact copy of your cloud files ({ $cloud-path }).
    Local files will be updated or deleted as necessary.
confirm-add-missing-roots = Add these roots?
no-missing-roots = No additional roots found.
loading = Loading...
preparing-backup-target = Preparing backup directory...
updating-manifest = Updating manifest...
no-cloud-changes = No changes to synchronize
backups-are-valid = Your backups are valid.
backups-are-invalid =
    These games' backups appear to be invalid.
    Do you want to create new full backups for these games?
saves-found = Kayıtlı veri mevcut.
no-saves-found = Kayıtlı veri bulunamadı.
# This is tacked on to form something like "Back up (no confirmation)",
# meaning we would perform an action without asking the user if they're sure.
suffix-no-confirmation = doğrulamasız
# This is shown when a setting will only take effect after closing and reopening Ludusavi.
suffix-restart-required = yeniden başlatma gerekli
prefix-error = Error: { $message }
prefix-warning = Warning: { $message }
cloud-app-unavailable = Cloud backups are disabled because { $app } is not available.
cloud-not-configured = Cloud backups are disabled because no cloud system is configured.
cloud-path-invalid = Cloud backups are disabled because the backup path is invalid.
game-is-unrecognized = Ludusavi does not recognize this game.
game-has-nothing-to-restore = This game does not have a backup to restore.
launch-game-after-error = Launch the game anyway?
game-did-not-launch = Game failed to launch.
back-up-specific-game =
    .confirm = Back up save data for { $game }?
    .failed = Failed to back up save data for { $game }
restore-specific-game =
    .confirm = Restore save data for { $game }?
    .failed = Failed to restore save data for { $game }
