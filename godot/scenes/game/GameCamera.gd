extends Camera2D

# Movement speed of the camera
export (float) var camera_speed := 512.0

# Limit the camera around the borders of the map
var camera_limit_left := limit_left
var camera_limit_right := limit_right
var camera_limit_top := limit_top
var camera_limit_bottom := limit_bottom


func _ready() -> void:
	var viewport = get_viewport().get_visible_rect()

	camera_limit_left = limit_left + viewport.size.x / 2
	camera_limit_right = limit_right - viewport.size.x / 2
	camera_limit_top = limit_top + viewport.size.y / 2
	camera_limit_bottom = limit_bottom - viewport.size.y / 2


func _physics_process(delta) -> void:
	var velocity = input_to_velocity()

	position.x = clamp(lerp(position.x, position.x + velocity.x * camera_speed, delta), camera_limit_left, camera_limit_right)
	position.y = clamp(lerp(position.y, position.y + velocity.y * camera_speed, delta), camera_limit_top, camera_limit_bottom)


func input_to_velocity() -> Vector2:
	var velocity = Vector2(0.0, 0.0)

	if Input.is_action_pressed("move_camera_left"):
		velocity.x -= 1.0
	if Input.is_action_pressed("move_camera_right"):
		velocity.x += 1.0
	if Input.is_action_pressed("move_camera_up"):
		velocity.y -= 1.0
	if Input.is_action_pressed("move_camera_down"):
		velocity.y += 1.0

	return velocity.normalized()
