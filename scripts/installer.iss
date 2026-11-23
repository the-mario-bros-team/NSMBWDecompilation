; The Windows installer for a release (Inno Setup 6): the launcher in the
; Start menu, an uninstaller in Settings > Apps. Built by
; scripts/make-installer.ps1 from a release folder; the files come from there.
;
; It installs for the current user, without admin rights, to
; %LOCALAPPDATA%\Programs\NSMBW Port: the launcher keeps its settings and
; each version's saves next to itself, and the game's updater replaces the
; programs in place, so the folder has to be the player's to write to.
; Uninstalling removes the programs and shortcuts only: settings and saves
; stay where they are.

#ifndef AppVersion
  #error Pass /DAppVersion=<x.y.z>
#endif
#ifndef Source
  #error Pass /DSource=<release folder>
#endif
#ifndef OutputName
  #define OutputName "nsmbw-port-v" + AppVersion + "-windows-x64-setup"
#endif

[Setup]
AppId={{6E1B5C2A-8F4D-4C7B-9A0E-3D2F7B61C5A9}
AppName=NSMBW Port
AppVersion={#AppVersion}
AppVerName=NSMBW Port {#AppVersion}
AppPublisher=WitherBlitz
AppPublisherURL=https://github.com/the-mario-bros-team/NSMBWDecompilation
DefaultDirName={localappdata}\Programs\NSMBW Port
DefaultGroupName=NSMBW Port
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
UninstallDisplayIcon={app}\nsmbw-launcher.exe
UninstallDisplayName=NSMBW Port
LicenseFile={#Source}\LICENSE.txt
OutputBaseFilename={#OutputName}
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
CloseApplications=yes
RestartApplications=no

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#Source}\nsmbw-launcher.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#Source}\nsmbw-port.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#Source}\LICENSE.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#Source}\README.txt"; DestDir: "{app}"; Flags: ignoreversion isreadme

[Icons]
Name: "{autoprograms}\NSMBW Port"; Filename: "{app}\nsmbw-launcher.exe"; WorkingDir: "{app}"
Name: "{autodesktop}\NSMBW Port"; Filename: "{app}\nsmbw-launcher.exe"; WorkingDir: "{app}"; Tasks: desktopicon

[Run]
Filename: "{app}\nsmbw-launcher.exe"; WorkingDir: "{app}"; Description: "{cm:LaunchProgram,NSMBW Port}"; Flags: nowait postinstall skipifsilent
