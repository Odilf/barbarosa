include("3x3.jl")

using Random

isedge(piece::Piece) = sum(abs.(piece.position)) == 2

# Kinda ugly
edges(cube::Cube) = filter(pair -> sum(abs.(pair.first)) == 2, cube)
corners(cube::Cube) = filter(pair -> sum(abs.(pair.first)) == 3, cube)

filter(pair -> pair.first == 2, cube())

# I'm so proud of this
function orientation(position::Vector3, normal::Vector3)
	# Represent "loop"
	mask = position .!= 0 
	p = position[mask]
	n = normal[mask]

	# Get parity. If it is negative, the loop is inversed
	parity = reduce(*, p)

	# Count spaces of separation
	if findfirst(i -> abs(i) == 1, n) |> isnothing
		error("What $position, $normal")
	end
	index = findfirst(i -> abs(i) == 1, normal) - 1
	
	# Mod it and apply parity
	n = length(p)
	mod((n - index) * parity, n)
end

scramble() |> orientation

function orientation(cube::Cube)
	edges = 0
	corners = 0

	for (pos, piece) in cube
		n = orientation(pos, piece.normal)
		if isedge(piece)
			edges += n
		else
			corners += n
		end
	end

	(edges = edges, corners = corners)
end

function isoriented(cube::Cube)
	(e, c) = orientation(cube)
	e % 2 == 0 && c % 3 == 0
end

function randomorientation(piece::Piece, position::Vector3)
	mask = position .!= 0
	i = collect(1:3)[mask] |> rand
	n = [0, 0, 0]
	n[i] = position[i]
	Piece(piece.position, n)
end

function randomize(input::Cube)::Cube
	pieces = shuffle(input |> values |> collect)
	pairs = map(zip(keys(input), pieces)) do (pos, piece)
		pos => randomorientation(piece, pos)
	end

	Dict(pairs)
end

function twist(normal::Vector3, position::Vector3, n::Integer = 1)::Vector3
	n -= 1
	i = findfirst(x -> abs(x) == 1, normal)

	for _ in 1:length(normal)
		i = i % 3 + 1

		if (position[i] != 0)
			normal = [0, 0, 0]
			normal[i] = position[i]

			

			return if n > 0
				twist(SVector(normal...), position, n)
			else
				normal
			end
		end
	end

	error("Unreachable, in theory")
end


twist(SVector(0, 1, 0), SVector(1, 1, 0))

function scramble()
	e = randomize(cube() |> edges)
	c = randomize(cube() |> corners)
	(eo, _) = orientation(e)
	(_, co) = orientation(c)

	# Flip edge if orientation is incorrect
	if eo % 2 != 0
		pos = [1, 1, 0]
		p = e[pos]
		e[pos] = Piece(p.position, twist(p.normal, SVector(pos...)))
	end

	# Twist corner if orientation is incorrect
	dif = co % 3
	if dif != 0
		pos = [1, 1, 1]
		p = c[pos]
		c[pos] = Piece(p.position, twist(p.normal, SVector(pos...), dif))
	end

	Dict(e..., c...)
end