# Universal Positioning System(UPS):
UPS is a collection of algorithms for finding optimal routes between star systems in Elite
Dangerous, a space sim game with a 1:1 recreation of the Milky Way Galaxy with over
400 billion star systems.

# Installation:
Clone this repository and change directory to the project.
```
git clone https://github.com/NNNiv/elite-dangerous-pathfinder.git
cd elite-dangerous-pathfinder
```
# Usage:
As this repository follows a mono-repo structure, it contains both the server as well as the website.
This requires running both the server as well as the web-server.<br>
Ensure that you have cargo@1.81 installed and npm@10.8.3 installed before running the code.

## Back-end
```
cd backend 
cargo build
cargo run
```

## Front-end
```
cd frontend 
npm install 
npm run dev 
o + enter  (to open in browser) 
```
In the website, the following parameters are required
- Start System ID 
- Goal System ID 
- Via System ID's (if using)
- Jump Distance (Maximum distance your ship can cover in a single journey)
- Ship Weight (Mass of your ship)
- A* or Dijkstra's algorithm for pathfinding 
