using Random

# isedge(piece::Piece) = sum(abs.(piece.position)) == 2


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
	index = findfirst(i -> abs(i) == 1, n) - 1
	
	# Mod it and apply parity
	n = length(p)
	mod((n + index) * parity, n)
end

orientation(piece::Piece) = orientation(piece.position, piece.normal)

function orientation(cube::Cube)
	map(cube.pieces) do piece
		orientation(piece)
	end |> sum
end

orientation(cube::FullCube) = (edges = orientation(Edges(cube)), corners = orientation(Corners(cube)))

function isoriented(cube::FullCube)
	(e, c) = orientation(cube)
	e % 2 == 0 && c % 3 == 0
end

function randomize_positions(cube::Cube)::Vector{Vector3}
	shuffle([piece.position for piece ∈ cube.pieces])
end

function scramble(cube::Cube{N}, mod::Integer; normalize_orientations = true) where N
	positions = randomize_positions(cube)
	orientations = [rand(0:mod - 1) for _ ∈ 1:N]

	if normalize_orientations
		dif = mod - sum(orientations[2:end]) % mod
		orientations[1] = dif % mod
	end

	map(zip(cube.pieces, positions, orientations)) do (piece, position, orientation)
		Piece(piece.id, position, orientation)
	end |> Cube{N}
end

scramble(corners::Corners) = scramble(corners, 3)
scramble(corners::Edges) = scramble(corners, 2)

function scramble(cube::FullCube = Cube())
	corners = scramble(Corners(cube))
	edges = scramble(Edges(cube))

	if countswaps(corners) % 2 != countswaps(edges) % 2
		Cube{20}([
			Piece(corners.pieces[1].id, corners.pieces[2].position, orientation(corners.pieces[2])),
			Piece(corners.pieces[2].id, corners.pieces[1].position, orientation(corners.pieces[1])),
			corners.pieces[3:end]...,
			edges.pieces...
		])
	else
		Cube{20}([
			corners.pieces...,
			edges.pieces...
		])
	end
end

function issolvable(cube::Cube{N}) where N
	if N != 20
		error("Cube unsolvable: not full cube (has $N pieces instead of 20)")
	end

	e, c = orientation(cube)
	if c % 3 != 0
		error("Cube unsolvable: corners aren't oriented (sum is $c)")
	end

	if e % 2 != 0
		error("Cube unsolvable: edges aren't oriented (sum is $e)")
	end


	e, c = countswaps(Corners(cube)), countswaps(Edges(cube))
	if c % 2 != e % 2
		error("Cube unsolvable: swap parity isn't equal (corners $c, edges $e)")
	end

	return true
end

function permutations(cube::Cube{N}; pool::Cube{M}=Cube{N}())::SVector{N, <: Integer} where {N, M}
	map(cube.pieces) do piece
		findfirst(pool_piece -> pool_piece.position == piece.position, pool.pieces)
	end
end

# Ew
function countswaps(v::SVector{N, <:Integer}) where N
	v = MVector(v)
	total = 0
	for i ∈ eachindex(v)
		n = v[i]
		j = findfirst(n -> n == i, v)

		if j != i
			total += 1
			v[j], v[i] = v[i], v[j]
		end
	end

	return total
end

countswaps(cube::Cube{N}) where N = countswaps(permutations(cube))