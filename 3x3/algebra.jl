using StaticArrays

Vector3 = SVector{3, <:Int}

@enum Axis X Y Z

rotation_matrices_dict = Dict(
	X => θ -> [
		1   0       0
		0 cos(θ)  sin(θ)
		0 -sin(θ) cos(θ) 
	],

	Y => θ -> [
		cos(θ)  0 -sin(θ)
		   0    1   0
		sin(θ) 0 cos(θ)
	],

	Z => θ -> [
		cos(θ)   sin(θ) 0
		-sin(θ)  cos(θ) 0
		   0       0    1
	]
)

function rotate(position::Vector3, axis::Axis, θ::Real)::Vector3	
	m = rotation_matrices_dict[axis](θ)
	round.(Int, m * position)
end

@enum Face R U F L D B

struct Move
	face::Face
	amount::Integer
end

const face_letters_dict = Dict(
	'R' => R,
	'U' => U,
	'F' => F,
	'L' => L,
	'D' => D,
	'B' => B,
)

const face_planes_dict = Dict{Face, Vector3}(
	R => [1, 0, 0],
	L => [-1, 0, 0],
	U => [0, 1, 0],
	D => [0, -1, 0],
	F => [0, 0, 1],
	B => [0, 0, -1],
)

face_letters_dict['R']

function parsemove(input::AbstractString)::Move
	input[1] ∉ keys(face_letters_dict) && error("Unknown face letter ($(input[1]))")
	face = face_letters_dict[input[1]]
	amount = if length(input) == 1
		1
	elseif input[2] == '\''
		-1
	else
		parse(Int, input[2:end])
	end

	Move(face, amount)
end

function parsealg(input::AbstractString)::Vector{Move}
	[parsemove(i) for i in split(rstrip(input), ' ')]
end

function movedata(move::Move)
	axis = if move.face ∈ [R, L]
		X
	elseif move.face ∈ [U, D]
		Y
	else
		Z
	end

	angle = move.amount * π/2 * ((move.face ∈ [R, U, F]) ? 1 : -1)

	(angle, axis)
end

function move(position::Vector3, input::Move)
	angle, axis = movedata(input)
	rotate(position, axis, angle)
end