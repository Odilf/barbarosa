using .Cube3x3
using StaticArrays

const edge_permutations = factorial(12) ÷ factorial(6) * 2^6
const corner_permutations = factorial(8) * 3^7

# Corner hash
function Base.hash(cube::SVector{8, Pair{Vector3, Piece}})
	piece_hash = map(enumerate(cube)) do (i, (pos, piece))
		i = i - 1 # 0 indexing, makes my life easier
		index = findfirst(pair -> pair.first == piece.position, cube)
		index * (8 - i)^i
	end

	orientations = map(enumerate(cube[1:end-1])) do (i, (pos, piece))
		i = i - 1
		index = orientation(pos, piece.normal)
		index * 3^i
	end

	# Stuff to get the number
	(sum(piece_hash) - 1) * 3^7 + sum(orientations)
end

# Edge hash
function Base.hash(cube::SVector{12, Pair{Vector3, Piece}})
	halves = [SVector{6}(cube[1:6]), SVector{6}(cube[7:12])]

	hashes = map(halves) do half
		piece_hash = map(enumerate(half)) do (i, (pos, piece))
			i = i - 1 # 0 indexing, makes my life easier
			index = findfirst(pair -> pair.first == piece.position, cube)
			index * (12 - i)^i
		end

		orientations = map(enumerate(half)) do (i, (pos, piece))
			i = i - 1
			index = orientation(pos, piece.normal)
			index * 2^i
		end

		# Stuff to get the number
		(sum(piece_hash) - 1) * 2^6 + sum(orientations)
	end
end