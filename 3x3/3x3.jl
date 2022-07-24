include("algebra.jl")

struct Piece
	position::Vector3
	normal::Vector3
end

function edges()
	positions = []
	for (i, j) in [(1, 1), (1, -1), (-1, -1), (-1, 1)]
		positions = [positions..., [i, j, 0], [i, 0, j], [0, i, j]]
	end
	
	edges = map(positions) do pos
		normal = if pos[1] == 1
			[1, 0, 0]
		elseif pos[1] == -1
			[-1, 0, 0]
		else
			[0, 0, pos[3]]
		end
		Piece(pos, normal)
	end
end

function corners()
	pieces = []
	for i in [1, -1]
		for j in [1, -1]
			for k in [1, -1]
				pieces = [pieces..., 
				Piece([i, j, k], [i, 0, 0])]
			end
		end
	end
	pieces
end

Cube = Dict{Vector3, Piece}

function cube()::Cube
	Dict(
		[edge.position => edge for edge in edges()]...,
		[corner.position => corner for corner in corners()]...,
	)
end

const solved_cube = cube()

function isinrange(position::Vector3, plane::Vector3)::Bool
	i = findfirst(x -> x != 0, plane)
	position[i] == plane[i]
end

function move(cube::Cube, input::Move)::Cube
	moved = []
	for (pos, piece) in cube
		pair = if isinrange(pos, face_planes_dict[input.face])
			move(pos, input) => Piece(piece.position, move(piece.normal, input))
		else
			pos => piece
		end

		moved = [moved..., pair]
	end

	Dict(moved)
end

function move(cube::Cube, alg::String)::Cube
	for input in parsealg(alg)
		cube = move(cube, input)
	end
	cube
end

function name(position::Vector3)
	faces = []
	for (face, plane) in face_planes_dict
		if isinrange(position, plane)
			faces = [faces..., face]
		end
	end

	output = ""
	for (letter, face) in face_letters_dict
		if face ∈ faces
			output *= letter
		end
	end

	length(output) == 2 && (output *= ' ')

	output
end





name(SVector{3}([1, 1, 1]))

Base.show(io::IO, piece::Piece) = print(io, "Piece $(name(piece.position)) with normal $(piece.normal)")

Base.show(io::IO, cube::Cube) = print(io, "3x3 cube" * (issolved(cube) ? " (solved)" : " (scrambled)"))
function Base.show(io::IO, ::MIME"text/plain", cube::Cube)
	print(io, cube)
	print(io, ": ")
	for (pos, piece) in cube
		print(io, "\n  ")
		print(io, piece)
		print(io, " at $(name(pos))")
	end
end

issolved(cube::Cube) = cube == solved_cube