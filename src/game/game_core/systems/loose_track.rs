use super::*;

/// Provides a loose tracking system betwen 2 entities, a source and its target.
///
/// This systems resolves in that the source is always facing the target. If the target is outside
/// of a certain radius of the source, it will be "tugged" by the target in a motion as if the
/// source was bound by rope to the target.
pub fn system_loose_tracking<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (id, (track,)) in
        &mut world.query::<(&LooseTracking,)>()
    {
        /* skip if target is not specified or component inactive */
        if track.target.is_none() || !track.is_active {
            continue;
        }

        let target = track.target.unwrap();

        /* get target transform */
        let target_transform =
            world.get::<Transform>(target);
        if target_transform.is_err() {
            continue;
        }
        let target_transform = target_transform.unwrap();

        /* get target motion */
        let target_motion = world.get::<Motion>(target);
        if target_motion.is_err() {
            continue;
        }
        let target_motion = target_motion.unwrap();

        /* get source transform */
        let source_transform =
            world.get_mut::<Transform>(id);
        if source_transform.is_err() {
            continue;
        }
        let mut source_transform =
            source_transform.unwrap();

        /* diff vector from the source to target */
        let diff_vec = target_transform.position
            - source_transform.position;

        /* is inside radius of the track */
        let is_inside = diff_vec.mag() < track.radius;

        /* amount of perpendicular movement the source is going to make */
        let mut perpendicular_mag = track
            .perpendicular_ratio
            * target_motion.vel.mag();

        /* the predicted location of the target in the next frame */
        let prediction = target_transform.position
            + target_motion.vel.rotate_deg(
                target_motion.angular_vel
                    + target_motion.angular_accel
                    + target_transform.rotation,
            );

        /* unit vector in the direction perpendicular to the target's motion */
        let mut z = (prediction
            - target_transform.position)
            .perpendicular();

        /* distance in which the source has to move in the perpendicular direction of the target's
         * motion inorder to form a straight line through the source's position, the target's
         * current position and the target's predicted position in the next frame. In simpler
         * words, "straight behind the target in terms of where the target is going next"
         */
        let mut distance = diff_vec.dot(z);

        /* Sometimes the vec's perpendicular method gives the opposite unit vec which we want. That
         * will result in a negative dot product, here we are reversing z if the dot product is
         * negative
         */
        if distance < 0.0 {
            z = z * -1.0;
            distance = diff_vec.dot(z);
        }

        /* We do not want the source to "swing" past the target, so we are capping it here */
        if perpendicular_mag > distance {
            perpendicular_mag = distance;
        }

        /* The distance remaining for the source to move in the direction perpendicular of the
         * target's motion after this frame inorder to reach the golden straight line
         */
        let opposite = distance - perpendicular_mag;

        /* After the source has moved "distance" in the direction perpendicular to the target's
         * motion, we calculate the movement the source has to take in the direction of the
         * target's motion inorder to maintain a fixed distanced to the target. Here, "side" is the
         * magnitude of the source's position behind the target's current position in the opposite
         * direction of the traget's motion
         */
        let side = (track.radius.powi(2)
            - opposite.powi(2))
        .powf(0.5);

        /* Resolving the calculated magnitude to a vector in real space */
        let side = target_transform.position
            + (prediction - target_transform.position)
                .unit()
                * -side;

        /* if the target is outside of the radius, we track it */
        if !is_inside {
            /* resolve the calculated vectors in real space */
            source_transform.position =
                side + z * (-opposite);
        }

        /* look at target at all times */
        source_transform.rotation = diff_vec.rotation();
    }
}
