use na::Point3;
use rapier3d::dynamics::{JointSet, RigidBodyBuilder, RigidBodySet};
use rapier3d::geometry::{ColliderBuilder, ColliderSet};
use rapier_testbed3d::Testbed;

pub fn init_world(testbed: &mut Testbed) {
    /*
     * World
     */
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let joints = JointSet::new();

    /*
     * Ground
     */
    let ground_size = 100.1;
    let ground_height = 2.1;

    let rigid_body = RigidBodyBuilder::new_static()
        .translation(0.0, -ground_height, 0.0)
        .build();
    let handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(ground_size, ground_height, ground_size).build();
    colliders.insert(collider, handle, &mut bodies);

    /*
     * Create the primitives
     */
    let num = 8;
    let rad = 1.0;

    let shiftx = rad * 2.0 + rad;
    let shifty = rad * 2.0 + rad;
    let shiftz = rad * 2.0 + rad;
    let centerx = shiftx * (num / 2) as f32;
    let centery = shifty / 2.0;
    let centerz = shiftz * (num / 2) as f32;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

    for j in 0usize..20 {
        for i in 0..num {
            for k in 0usize..num {
                let x = i as f32 * shiftx - centerx + offset;
                let y = j as f32 * shifty + centery + 3.0;
                let z = k as f32 * shiftz - centerz + offset;

                // Build the rigid body.
                let rigid_body = RigidBodyBuilder::new_dynamic().translation(x, y, z).build();
                let handle = bodies.insert(rigid_body);

                let collider = match j % 5 {
                    0 => ColliderBuilder::cuboid(rad, rad, rad).build(),
                    1 => ColliderBuilder::ball(rad).build(),
                    // Rounded cylinders are much more efficient that cylinder, even if the
                    // rounding margin is small.
                    2 => ColliderBuilder::round_cylinder(rad, rad, rad / 10.0).build(),
                    3 => ColliderBuilder::cone(rad, rad).build(),
                    _ => ColliderBuilder::capsule_y(rad, rad).build(),
                };

                colliders.insert(collider, handle, &mut bodies);
            }
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }

    /*
     * Set up the testbed.
     */
    testbed.set_world(bodies, colliders, joints);
    testbed.look_at(Point3::new(100.0, 100.0, 100.0), Point3::origin());
}
