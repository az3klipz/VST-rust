[Setup]
AppId={{YOUR-UNIQUE-GUID-HERE}}
AppName=Antigravity Hybrid Designer
AppVersion=1.0.0
AppPublisher=Your Studio Name
DefaultDirName={commoncf}\VST3\Antigravity
DefaultGroupName=Antigravity Audio
OutputDir=.\target\installer
OutputBaseFilename=Antigravity_Designer_Setup
Compression=lzma2
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64
; Disables the "Select Start Menu Folder" page for a cleaner "Waves-like" feel
DisableProgramGroupPage=yes

[Files]
; VST3 Bundle (Recursive to include metadata)
Source: ".\target\bundled\antigravity_designer.vst3\*"; DestDir: "{commoncf}\VST3\Antigravity_Designer.vst3"; Flags: ignoreversion recursesubdirs
; CLAP Binary
Source: ".\target\bundled\antigravity_designer.clap"; DestDir: "{commoncf}\CLAP"; Flags: ignoreversion

[Icons]
Name: "{group}\Uninstall Antigravity Designer"; Filename: "{unsellexe}"

[Run]
; Optional: Open the manual or a "Success" webpage after install
Filename: "https://yourwebsite.com/welcome"; Description: "Visit Welcome Page"; Flags: postinstall shellexec
