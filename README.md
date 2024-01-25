# Visualizer
#### Individual project made for the Advanced Programming course (Academic Year 2023-2024) at University of Trento.
This is a visualizer/GUI for a Robot doing stuff in a virtual world.

The Robot is made by a team member via interfaces in a common crate made by the whole class. Said Robot uses tools made by groups of students.

The Robot is then wrapped in order to continuously gather data about the world to then visualize.

The GUI a 2D top-down view with pixelated graphics. I made it using the Bevy engine in Rust.

The GUI shows the world, undiscovered tiles, the items on it, the robot, an energy bar and a minimap.

As soon as the actual runner (robot + world logic) processes a tick the whole screen is updated.

The code will not run because course-related crates are from a private registry.

<img width="1080" alt="image" src="https://github.com/davidepaci/visualizer/assets/23656588/1377af00-29c7-4b2e-b895-df1912444553">
