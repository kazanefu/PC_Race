# PC Racing Game

## Overview

this is a 3D racing game. the car's status (max speed, weight, amount of fuel, etc.) is depends on PC performance (CPU clock, GPU clock, RAM used size, CPU temperature, GPU temperature, etc.).

## Gameplay

the player can control the car using keyboard and mouse. there are 2 modes:

1. Time Attack Mode (this mode is the main mode, the time will be recorded and compared with other players)
2. Race Mode (with AI cars)

## The sequence of the game

1. home screen (player can see the ranking of the time attack mode)
2. select mode (time attack mode or race mode)
3. select course (player can see the courses)
4. select car (which decides the car's base status)
5. Measure PC performance (CPU clock, GPU clock, RAM used size, CPU temperature, GPU temperature, etc.) and calculate the car's status
6. warp to the game screen
7. start the game (3 2 1 GO!)
8. the game is over when the car crashes or runs out of fuel
9. show result screen (show the time and ranking)
10. get XP and coins (depends on the time and ranking) (which can be used to upgrade the car and buy new cars)
11. warp to the home screen
12. the ranking is updated

## Car Status

the car's status is calculated based on the PC performance. the car's status includes:

1. max speed
2. weight
3. fuel consumption
4. handling
5. acceleration
6. braking
7. grip
8. aerodynamics (DRS acceleration & max speed)
10. fuel capacity

## Calculate the car's status

the car's status depends on the Car type (Divided so that the total is 100)

base status:
1. CPU Impact rate
2. GPU Impact rate
3. RAM Impact rate
4. temperature Impact rate
5. SSD Impact rate
6. Base max speed
7. Base weight
8. Base fuel consumption
9. Base handling
10. Base acceleration
11. Base braking
12. Base grip
13. Base aerodynamics
14. Base fuel capacity

PC status:
1. CPU clock
2. GPU clock
3. RAM used size
4. RAM available size
5. CPU temperature(real time)
6. GPU temperature(real time)
7. SSD available size
8. CPU usage rate(real time)
9. GPU usage rate(real time)

calculation:
```
max speed = base max speed * (1 + CPU Impact rate * CPU clock *(1 + CPU usage rate)) * const
fuel capacity = base fuel capacity * (1 + RAM Impact rate * RAM used size + SSD Impact rate * SSD available size) * const
weight = base weight * (1 + Remaining fuel) * const
fuel consumption = base fuel consumption * (1 + temperature Impact rate * (CPU temperature + GPU temperature) * GPU Impact rate * GPU clock) * const
acceleration = base acceleration * ((1 + GPU Impact rate * GPU clock * (1 + GPU usage rate)) * if gear is appropriate then 2.0 else 1.0 / weight) * const
grip = base grip * (1 + RAM Impact rate * RAM available size) * const
handling = base handling * grip / weight * const
aerodynamics = base aerodynamics * (1 + GPU Impact rate * GPU clock * (1 + GPU usage rate)) * const
DRS acceleration = acceleration + aerodynamics * const
DRS max speed = max speed + aerodynamics * const
braking = base braking * grip / weight * const
speed = min(max speed, integral(acceleration)dt - integral(braking)dt)
rotating speed = handling * const
if CPU temperature + GPU temperature > const, the car will overheat and gameover
if DRS on, acceleration = DRS acceleration, max speed = DRS max speed, grip = grip * const(< 1.0)
if remaining fuel < 0, the car will run out of fuel and gameover
XP = const / time
coins = const / time
if course out, penalty is applied to the time and the penalty > threshold, the car will crash and gameover
appropriate gear = gear that is closest to the ratio of speed / max speed
gravity = const (based on the course)
```

## Control
default:

Accelerate: W
Brake: S
Steer left: A
Steer right: D
DRS On: Arrow Up or Mouse right on click
DRS Off: Arrow Down or Mouse right up click
Gear: 1, 2, 3, 4, 5, 6 change by Mouse scroll or Arrow <- ->
Select car: Arrow and GUI
Select course: Arrow and GUI
Get PC status && calculate the car's status: Enter or Mouse left on click

keyconfig is saved in the config folder

## Development

### dependencies

bevy = "0.18.0"
sysinfo = "0.37.2"
bevy_rapier3d = "0.32.0"
...(other dependencies)

### Priority

1. Home screen
2. Select mode
3. Select course
4. Select car
5. Measure PC performance
6. Calculate the car's status
7. Time attack mode Game screen
8. Time attack mode Result screen
9. Race mode Game screen
10. Race mode Result screen

### Environment

- Windows 11

### Entities

- Car
- Course
- Player
- UI
- AI Car

### Share race results(Optional)

if I can use free cloud storage, I will share the race results and ranking.

- Ranking
- XP
- Coins

### Graphics

default:
- fullscreen
- 60fps
- if DLSS is available, use it
- if ray tracing is available, use it

optional:
- 1080p
- 1440p
- 4K
- 30fps
- 120fps
- max available fps
- windowed

### platform

- Windows 11 (recommended)
- Web(Optional)
- Android(Optional)
- iOS(Optional)
- Linux(Optional)
- macOS(Optional)

### Designed to be resistant to modifications

### Localization

- English
- Japanese
