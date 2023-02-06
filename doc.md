# Structs
## Simulation
### Fields
* `objects` - A list of all the objects in the simulation.
* `graphics` - The graphics object of the simulation.
* `physics` - The physics object of the simulation.
* `time_step` - How often fixed update is called in microseconds.
### Methods
* `add_object` - Adds an object to the simulation.
* `remove_object` - Removes an object from the simulation.
* `update` - Updates the simulation.
---
&thinsp;
## SimObject
### Fields
* `id` - The unique identifier of the object.
* `name` - The name of the object.
* `x` - The x coordinate of the object.
* `y` - The y coordinate of the object.
* `render_object` - The render object of the object.
* `physics_object` - The physics object of the object.
* `script` - Optional function that is run every update
### Methods
* `update` - Updates the object.
* `fixed_update` - Updates the object every timestep.
* `get_render_object` - Returns the render object of the object.
- Example callback for update and fixed_update:

  ```rust
  Some(|obj: &mut physics_sim::SimObject| {
      // do something
  })
  ```


Example for RenderType mutation
``` rust
let render_obj_mut = obj.get_render_object_mut().unwrap();
match render_obj_mut.render_type {
    RenderType::Line { magnitude } => {
        render_obj_mut.render_type = RenderType::Line { magnitude: magnitude + 1 };
    },
    _ => {},
}
```