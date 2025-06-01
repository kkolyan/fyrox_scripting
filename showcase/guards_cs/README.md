# About
Rudimental Game made with Fyrox Engine and https://github.com/kkolyan/fyrox_lite.

C# port of https://github.com/kkolyan/fyrox_guards

Explored Fyrox APIs:
* Node graph.
* RigidBody (3D)
* Ray Casting
* Prefabs & Editor. Everything dynamic in game created in Editor and instantiated via prefabs, except UI, which is coded.

# How to run a game
1. Install [.Net 8.0](https://dotnet.microsoft.com/en-us/download/dotnet/thank-you/sdk-8.0.410-windows-x64-installer) (later versions probably will work fine too)
2. Run editor, pointing to a game directory (using CLI or UI chooser)
3. Press "Play" button at the top panel

# How to play
Use WASD and mouse to shoot enemies and optionally avoid their attacks.

# How to edit scene
[Editor-related part of Fyrox book](https://fyrox-book.github.io/beginning/editor_overview.html) is completely relevant here.

# How to edit scripts
1. Use any text editor to open `*.cs` files. IDE is not required.
2. Point your favourite IDE to `*.sln` file generated automatically by editor inside project directory. This way you can run game via IDE and debug it.

# Screenshots
![gameplay.png](gameplay.png)
