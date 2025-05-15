#[cfg(target_os = "windows")] pub mod windows;
#[cfg(target_os = "windows")] pub(crate) type PlatformGraphicsAdapter = windows::WindowsGraphicsAdapter;
#[cfg(target_os = "windows")] pub(crate) type PlatformGraphicsAdapterOutput = windows::WindowsGraphicsAdapterOutput;
#[cfg(target_os = "windows")] pub(crate) type PlatformGameWindow = windows::WindowsGameWindow;
#[cfg(target_os = "windows")] pub(crate) type PlatformPresentationParameters = windows::WindowsPresentationParameters;
#[cfg(target_os = "windows")] pub(crate) type PlatformGraphicsDevice = windows::WindowsGraphicsDevice;
#[cfg(target_os = "windows")] pub(crate) type PlatformGame = windows::WindowsGame;
#[cfg(target_os = "windows")] pub(crate) type PlatformRenderTarget2D = windows::WindowsRenderTarget2D;
#[cfg(target_os = "windows")] pub(crate) type PlatformScreen = windows::WindowsScreen;

