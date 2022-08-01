using StaticArrays
using .Cube3x3: permutations

const edge_permutations = factorial(12) ÷ factorial(6) * 2^6
const corner_permutations = factorial(8) * 3^7

Cube3x3.permutations(elements::Integer, choose::Integer) = reduce(*, (elements - choose + 1):elements)

function hash_permutations(vector::SVector{N, <:Integer}; max::Integer=N)::Integer where N
	map(enumerate(vector)) do (i, n)
		i = i - 1 # 0-indexing

		# Decision (Eg.: if we have selected 1 and 3, 4 is the second decision)
		n = n - sum(vector[1:i] .< n) - 1

		n * permutations(max, i) # Encode decision
	end |> sum
end

function hash_orientations(orientations::SVector{N, <:Integer}, modulus::Integer)::Integer where N
	map(enumerate(orientations)) do (i, orientation)
		orientation * modulus^(i - 1)
	end |> sum
end

hash_orientations(o::Vector{<:Integer}, m::Integer) = hash_orientations(SVector(o...), m)

# Corner hash
function Base.hash(corners::Corners)::Integer
	permutation_hash = hash_permutations(permutations(corners); max=8)
	orientation_hash = hash_orientations(orientation.(corners.pieces[1:end-1]), 3)

	# Stuff to get the number (1 indexed)
	permutation_hash + orientation_hash * factorial(8) + 1
end

# Edge hash
function Base.hash(half::HalfEdges)::Integer
	permutation_hash = hash_permutations(permutations(half, pool=Edges()); max=12)
	orientation_hash = hash_orientations(orientation.(half.pieces), 2)

	# Stuff to get the number (1 indexed)
	permutation_hash * 2^6 + orientation_hash + 1
end

function Base.hash(edges::Edges)::Vector{Integer}
	halves = [HalfEdges(edges.pieces[1:6]), HalfEdges([Piece(piece.id, piece.position .* -1, piece.normal) for piece in edges.pieces[7:12]])]

	[hash(halves[1]), hash(halves[2])]
end

function Base.hash(cube::Cube)::Vector{Integer}
	[hash(Corners(cube)), hash(Edges(cube))...]
end