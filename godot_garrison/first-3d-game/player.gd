extends CharacterBody3D

# Emitted when player has been hit by a mob
signal hit

# Player speed [m/s]
@export var speed := 14

# Player falling acceleration [m/(s^2)]
@export var fall_acceleration := 75

# Vertical impulse applied to character upon jumping [m/s]
@export var jump_impulse := 20

# Vertical impulse applied to character upon bouncing over a mob [m/s]
@export var bounce_impulse := 16


var target_velocity := Vector3.ZERO


func _physics_process(delta) -> void:
	# We create a local variable to store the input direction.
	var direction = Vector3.ZERO

	# We check for each move input and update the direction accordingly.
	if Input.is_action_pressed("move_right"):
		direction.x += 1
	if Input.is_action_pressed("move_left"):
		direction.x -= 1
	if Input.is_action_pressed("move_back"):
		# Notice how we are working with the vector's x and z axes.
		# In 3D, the XZ plane is the ground plane.
		direction.z += 1
	if Input.is_action_pressed("move_forward"):
		direction.z -= 1

	if direction != Vector3.ZERO:
		direction = direction.normalized()

		# Set basis property to affect node rotation
		$Pivot.basis = Basis.looking_at(direction)

		# Speed up animation while moving
		$AnimationPlayer.speed_scale = 4
	else:
		$AnimationPlayer.speed_scale = 1

	# Ground velocity
	target_velocity.x = direction.x * speed
	target_velocity.z = direction.z * speed

	# Vertical velocity
	if not is_on_floor():
		# If in air, fall towards floor ((gravity))
		target_velocity.y = target_velocity.y - (fall_acceleration * delta)

	# Move the character
	velocity = target_velocity

	# Jump
	if (is_on_floor() and Input.is_action_pressed("jump")):
		target_velocity.y = jump_impulse

	# Run collision physics
	move_and_slide()

	# Check to see if we've jumped on a mob
	for index in range(get_slide_collision_count()):
		# Get one of the collisions with us (the player)
		var collision := get_slide_collision(index)

		# If we collide more than once with a mob in a single frame,
		# the mob will be deleted by the first collision.
		# Subsequent calls to get_collider will return null.
		# This will lead to a null pointer when calling
		# collision.get_collider().is_in_group("mob")
		if collision.get_collider() == null:
			continue

		# If collider is with a mob
		if collision.get_collider().is_in_group("mob"):
			var mob := collision.get_collider()

			# Check we're (roughly) hitting it from above
			if Vector3.UP.dot(collision.get_normal()) > 0.1:
				# Squash & bounce if so
				mob.squash()
				target_velocity.y = bounce_impulse

				# Prevent duplicate calls
				break

	# Make character model (pivot node) arc when jumping
	$Pivot.rotation.x = PI / 6 * velocity.y / jump_impulse


# Kill player
func die() -> void:
	hit.emit()
	queue_free()


func _on_mob_detector_body_entered(_body: Node3D) -> void:
	die()
