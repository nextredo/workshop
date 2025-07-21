extends CharacterBody3D

# Player speed [m/s]
@export var speed = 14

# Player falling acceleration [m/(s^2)]
@export var fall_acceleration = 75

# Vertical impulse applied to character upon jumping [m/s]
@export var jump_impulse = 20


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

	move_and_slide()
