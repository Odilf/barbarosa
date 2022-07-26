include("algebra.jl")

mutable struct Piece
	position::Vector3
	normal::Vector3
end

Base.:(==)(p1::Piece, p2::Piece) = p1.position == p2.position && p1.normal == p2.normal

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
			[0, pos[2], 0]
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

Cube = SVector{20, Pair{Vector3, Piece}}

const solved_cube = let
	c = corners()
	e = edges()
	p = [piece.position => piece for piece in [c..., e...]]
	SVector{20}(p)
end

Base.copy(piece::Piece) = Piece(piece.position, piece.normal)
Base.copy(cube::Cube) = SVector{20}([copy(pos) => copy(piece) for (pos, piece) in cube])

cube()::Cube = solved_cube::Cube

function isinrange(position::Vector3, plane::Vector3)::Bool
	i = findfirst(x -> x != 0, plane)
	position[i] == plane[i]
end

function move(cube::Cube, input::Move)::Cube
	map(cube) do (pos, piece)
		if isinrange(pos, face_planes_dict[input.face])
			move(pos, input) => Piece(piece.position, move(piece.normal, input))
		else
			pos => piece
		end
	end
end

function move(cube::Cube, alg::String)::Cube
	for input in parsealg(alg)
		cube = move(cube, input)
	end
	cube
end

issolved(cube::Cube) = cube == solved_cube
isreallysolved(cube::Cube) = Set(cube) == Set(solved_cube)

const possible_moves = let
	m::Vector{Move} = []
	for face in instances(Face)
		for i in [-1, 1, 2]
			m = [m..., Move(face, i)]
		end
	end
	SVector{18}(m)
end

neighbours(cube::Cube)::SVector{18, Cube} = map(m -> move(cube, m), possible_moves)

# Pretty printing
function name(face::Face)
	for (letter, f) in face_letters_dict
		if f == face 
			return letter
		end
	end
end

function name(position::Vector3)
	faces = []
	for (face, plane) in face_planes_dict
		if isinrange(position, plane)
			faces = [faces..., face]
		end
	end

	output = join([name(face) for face in faces])
	length(output) == 2 && (output *= ' ')
	output
end

function name(move::Move)
	f = name(move.face)

	a = if move.amount == 1
		""
	elseif move.amount == -1
		'\''
	else
		string(move.amount)
	end

	f * a
end

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

Base.show(io::IO, move::Move) = print(io, name(move))
Base.show(io::IO, alg::Vector{Move}) = print(io, join(name.(alg), " "))
Base.show(io::IO, ::MIME"text/plain", alg::Vector{Move}) = print(io, join(name.(alg), " "))