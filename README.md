# Aquiver
Playing video on Minecraft Bedrock.

## Building
1.Install Rust.

2.Open Terminal(cmd), clone(download) this project.
```
cd Aquiver
cargo build --release
```
## Generate a video
```
aquiver -n <packName> -d <packDescription> -w <particleWidth> -h <particleHeight> -p <imgPath>
```
n: Pack's name.The resource files will be saved in ./{name}/

d: Pack's description

w: Particle's width.(best close to 1.0)

h: Particle's height (best close to 1.0)

p: Video's path(Supported format: .gif)


Then open Minecraft, load the pack and enter the world.
Run chat command:
```
function init
```
Place an armor_stand named {packName}.

Give yourself a repeating command block.Input this command into the command block, set at running.
```
function loop
```

