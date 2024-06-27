
# Smart Road

A traffic control system for cars entering an intersection application written in Rust

![Smart Road](./assets/smart-roadGif.gif)

PS: Screen recording fps is lowered to create 1 minute long GIF



## Instructions

#### **Intersection**

There are various shapes of intersections, we will focus on the widely seen cross intersection, where each lane has a different route:

- `r`, turning right
- `s`, straight ahead
- `l`, turning left

```console
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |r  | s | l |   |   |   |
_______________| ← | ↓ | → |   |   |   |________________
                           |            ↑ r
_______________            |            ________________
                           |            ← s
_______________            |            ________________
                           |            ↓ l
___________________________|____________________________
           l ↑             |
_______________            |            ________________
           s →             |
_______________            |            ________________
           r ↓             |
_______________            |            ________________
               |   |   |   | ← | ↑ | → |
               |   |   |   | l | s | r |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
```

---

#### **Vehicles**

```console
  ______
 /|_||_\`.__
=`-(_)--(_)-'
```
1. AVs driving on a lane with a **given route** must follow the direction of that route; the AVs can't change lanes or routes.

2. AVs must have at least 3 different velocities. Therefore the **smart intersection system** can control the velocity of the vehicle.\
   This will be the way of controlling the current velocity/time/distance (depending on the algorithm you implement) of the AVs.

3. Each AV must respect a safety distance from other AVs.\
   If a vehicle is driving at a high velocity and encounters another vehicle, it must detect that vehicle and keep a safe distance from it. It should not collide!
   You are free to decide the safety distance, but it must be a strictly positive value.

4. Other vehicles, such as emergency vehicles, are not considered in this project.

5. You must implement physics for the vehicle, such as `velocity = distance / time`. Each vehicle must have a :

- `time`: the time that the AV takes to leave the intersection
- `distance`: the distance that the AV takes to leave the intersection
- `velocity`: the speed of the AV at the current time

#### **Commands**

1. The creation of vehicles must be done using the keyboard events
- `Arrow Up`, generate vehicles from south to north.
- `Arrow Down`, generate vehicles from north to south.
- `Arrow Right`, generate vehicles from west to east.
- `Arrow Left`, generate vehicles from east to west.

2. It must also be possible to use the key `R` to continually generate random vehicles (using the game loop).

3. The `Esc` key must finish the simulation and generate a window with all statistics

4. When spamming the same key, the vehicles should not be generated all at the same time. In other words, the vehicles should not be created on top of each other.

---

#### **Statistics**

The statistics must include:

- Max number of vehicles that passed the intersection
- Max velocity of all vehicles (Display the fastest speed achieved)
- Min velocity of all vehicles (Display the slowest speed reached)
- Max time that the vehicles took to pass the intersection (for all vehicles, display the one that took more time)
- Min time that the vehicles took to pass the intersection (for all vehicles, display the one that took less time)
    - The time starts to count whenever the vehicle is detected by the **smart intersection algorithm** until the end of the intersection, which is when the vehicle is removed from the canvas.
- Close calls, this is when both vehicles pass each other with a violation of the safe distance.
## Authors

- Cenk
- Jimmy
- Acki


## Run Locally

Clone the project

```bash
  git clone https://01.gritlab.ax/git/minhtuann/smart-road
```

Go to the project directory

```bash
  cd smart-road
```

Start the app at smart-road/src

```bash
  cargo run --release
```



