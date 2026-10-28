# Ferro Engine
![Rust Version](https://img.shields.io/badge/rust-2021-orange.svg)
![License](https://img.shields.io/badge/license-Apache--2.0-blue)

Ferro Engine is a 2D game engine that combines performance, good physics, and a user-friendly editor.

## Features
- **Fast 2D Rendering:** Built on top of OpenGL (`glow`).
- **Physics Engine:** Integrated 2D physics simulation.
- **Cross-Platform:** Runs on Linux, Windows, with WebGL/Android targets planned.
- **Built-in Editor:** Intuitive UI for scene composition.
- **ECS-driven Architecture:** Highly modular and efficient.

# Graphics API
Ferro Engine currently supports 3 graphics APIs, with more coming soon. Here is all the information you need:
<details>
  <summary><b>Low-level API</b></summary>

| API | Status | Platforms | Working versions |
| :--- | :--- | :--- | :--- |
| [Vulkan](https://vulkan.org/) | Developing ⏳ | [Windows](https://www.microsoft.com/en-us/windows/), [Linux](https://www.linux.org/), [Android](https://www.android.com/), [ChromeOS](https://chromeos.google/), [BSD Unix](https://en.wikipedia.org/wiki/Berkeley_Software_Distribution), [Fuchsia](https://fuchsia.dev/) | - |
| [DirectX12](https://github.com/microsoft/DirectX-Graphics-Samples) | ❌ | [Windows](https://www.microsoft.com/en-us/windows/), [WSL](https://wsl.dev/), [Windows Server](https://www.microsoft.com/en/windows-server) | - |
| [Metal](https://developer.apple.com/metal/) | ❌ | [macOS](https://www.apple.com/os/macos/), [iOS](https://www.apple.com/os/ios/), [iPadOS](https://www.apple.com/os/ipados/), [watchOS](https://www.apple.com/os/watchos/), [tvOS](https://developer.apple.com/tvos/), [visionOS](https://www.apple.com/os/visionos/) | - |
| [WebGPU](https://webgpu.org/) | ❌ | [Windows](https://www.microsoft.com/en-us/windows/), [macOS](https://www.apple.com/os/macos/), [Linux](https://www.linux.org/), [Android](https://www.android.com/), [iOS](https://www.apple.com/os/ios/), [Web](https://en.wikipedia.org/wiki/Web_browser) | - |
</details>

<details>
  <summary><b>Legacy API</b></summary>
  
| API | Status | Platforms | Working versions |
| :--- | :--- | :--- | :--- |
| [OpenGL](https://www.opengl.org/) | Stable ✅ | [Windows](https://www.microsoft.com/en-us/windows/), [Linux](https://www.linux.org/), [BSD Unix](https://en.wikipedia.org/wiki/Berkeley_Software_Distribution) | 3.3 - 4.6 |
| [OpenGL ES](https://www.khronos.org/opengles/) | Unstable ⚠️ | [Android](https://www.android.com/), [iOS](https://www.apple.com/os/ios/), [iPadOS](https://www.apple.com/os/ipados/), [Linux](https://www.linux.org/) | 2.0 - 3.2 |
| [WebGL](https://www.khronos.org/webgl/) | Unstable ⚠️ | [Windows](https://www.microsoft.com/en-us/windows/), [macOS](https://www.apple.com/os/macos/), [Linux](https://www.linux.org/), [Android](https://www.android.com/), [iOS](https://www.apple.com/os/ios/), [Web](https://en.wikipedia.org/wiki/Web_browser) | 1.0 - 2.0 |
| [DirectX](https://github.com/microsoft/DirectX-Headers) | ❌ | [Windows](https://www.microsoft.com/en-us/windows/) | - |
</details>

# Getting Started
Follow these steps to download stable version of Ferro Engine. Make sure you downloaded [Rust language](https://rust-lang.org/) and [Git](https://git-scm.com/).
  
1. Download our engine:
```bash
git clone https://github.com/rivnarc/ferro-engine.git
```
2. Go inside engine folder:
```bash
cd ferro-engine
```
3. Run ferro engine:
```bash
cargo run --release
```

# Community
Follow our channels, to get announcements and notification of the latest version. You can also report a bug, or ask for help if something goes wrong.
- [Discord](#)
- [Telegram](#)
- [TikTok](#)
- [YouTube](#)
- [X](#)

### Contributing
Contributions are welcome! If you want to contribute:
1. Check existing [Issues](https://github.com/rivnarc/ferro-engine/issues).
2. Join our [Discord](#) to get access

# License
Ferro Engine is licensed under the [Apache License 2.0](LICENSE).
