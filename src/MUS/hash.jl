using StaticArrays
using .Cube3x3: v, permutations

const edge_permutations = factorial(12) ÷ factorial(6) * 2^6
const corner_permutations = factorial(8) * 3^7

Cube3x3.permutations(elements::Integer, choose::Integer) = reduce(*, (elements - choose + 1):elements)

# Corner hash
function Base.hash(corners::Corners)
	perms = permutations(corners)

	orientations = map(enumerate(corners[1:end-1])) do (i, (pos, piece))
		i = i - 1
		index = orientation(pos, piece.normal)
		index * 3^i
	end

	# Stuff to get the number (1 indexed)
	hash_permutations(perms, max=8) + sum(orientations) * factorial(8) + 1
end

# Edge hash
function Base.hash(edges::Edges)
	halves = [SVector{6}(edges[1:6]), SVector{6}(edges[7:12])]

	map(halves) do half
		perms = permutations(half, edges)

		orientations = map(enumerate(half)) do (i, (pos, piece))
			i = i - 1
			index = orientation(pos, piece.normal)
			index * 2^i
		end

		# Stuff to get the number (1 indexed)
		hash_permutations(SVector{6}(perms), max=12) * 2^6 + sum(orientations) + 1
	end |> Tuple
end

function Base.hash(cube::Cube)
	[hash(cube |> corners), hash(cube |> edges)...]
end

function hash_permutations(vector::SVector{N, <:Integer}; max::Integer=N)::Integer where N
	map(enumerate(vector)) do (i, n)
		i = i - 1 # 0-indexing

		# Decision (Eg.: if we have selected 1 and 3, 4 is the second decision)
		n = n - sum(vector[1:i] .< n) - 1

		n * permutations(max, i) # Encode decision
	end |> sum
end