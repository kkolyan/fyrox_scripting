# About
Rudimental Game made with Fyrox Engine and https://github.com/kkolyan/fyrox_lite.

C# port of https://github.com/kkolyan/fyrox_guards

Explored Fyrox APIs:
* Node graph.
* RigidBody (3D)
* Ray Casting
* Prefabs & Editor. Everything dynamic in game created in Editor and instantiated via prefabs, except UI, which is coded.

# How to run a game

1. Get sources (both game and Fyrox C# SDK):
    ```sh
    git clone --recursive https://github.com/kkolyan/fyrox_lite
   ``` 
2. Install [.Net 8.0](https://dotnet.microsoft.com/en-us/download/dotnet/thank-you/sdk-8.0.410-windows-x64-installer) (later versions probably will work fine too, but not tested)
3. Install [Rust](https://rustup.rs/).
4. Run editor:
    ```sh
    chmod +x editor_cs.sh
    ./editor_cs.sh
    ```
5. When it asks for the directory project to open, choose this folder. Answer "OK" when it asks to create project files.
6. If you have IDE for C#, it will be opened automatically, or you will be asked how to open "*.sln" files. Ignore this if you don't want to use IDE now.
7. Fyrox editor will fail to open scene and alert about missing C# assembly - that's ok.
8. Click "Build" button in Fyrox Editor.
9. Open `data/scene.rgs` by double-click in Fyrox Editor project tree and click "Play" button in Fyrox Editor.
10. Alternative - run game from IDE. For Rider there is predefined run profile. If you use different IDE, note that working directory should be the game project root, not the build directory.

# How to play
Use WASD and mouse to shoot enemies and optionally avoid their attacks.

# How to edit scenes
WIP

# How to edit scripts
WIP

# Screenshots
![gameplay.png](gameplay.png)