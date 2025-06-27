extends Area2D

signal hit

# Player move speed (pixels/sec)
@export var speed = 400

# Dimensions of the game window
var screen_size: Vector2

# ------------------------------------------------------------------------------
# ------------------------------------------------------------------------------

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	screen_size = get_viewport_rect().size
	# hide()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	# Player movement vector
	var velocity := Vector2.ZERO

	# Check for input
	if Input.is_action_pressed("move_right"):
		velocity.x += 1
	if Input.is_action_pressed("move_left"):
		velocity.x -= 1
	if Input.is_action_pressed("move_down"):
		velocity.y += 1
	if Input.is_action_pressed("move_up"):
		velocity.y -= 1

	# Play corresponding animation if moving
	if velocity.length() > 0:
		velocity = velocity.normalized() * speed
		$AnimatedSprite2D.play()
	else:
		$AnimatedSprite2D.stop()

	# Move in given direction
	# (ensuring player does not go offscreen)
	position += velocity * delta
	position = position.clamp(Vector2.ZERO, screen_size)

	# Adjust animation for the player sprite
	if velocity.x != 0:
		$AnimatedSprite2D.animation = "walk"
		$AnimatedSprite2D.flip_h    = (velocity.x < 0)
	elif velocity.y != 0:
		$AnimatedSprite2D.animation = "up"

	if velocity.length() != 0:
		$AnimatedSprite2D.flip_v = (velocity.y > 0)


func _on_body_entered(_body: Node2D) -> void:
	hide()
	hit.emit()

	# Must be deferred, as we shouldn't change physics properties
	# in a physics callback
	$CollisionShape2D.set_deferred("disabled", true)


func start(pos: Vector2) -> void:
	position = pos
	show()
	$CollisionShape2D.disabled = false
