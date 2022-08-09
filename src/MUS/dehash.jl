function dehash_permutations(hash::Integer; length::Integer, max::Integer=length)
	decisions = map(1:length) do i
		i = i - 1
		hash % permutations(max, i + 1) ÷ permutations(max, i) + 1
	end

	options = collect(1:max)
	map(decisions) do decision
		n = options[decision]
		deleteat!(options, decision)
		n
	end
end

function dehash(hash::Integer, ::Type{Corners})::Corners
	hash -= 1
	permutations_hash = hash % factorial(8)
	orientation_hash = hash ÷ factorial(8)

	orientations = map(1:7) do i
		(orientation_hash % 3^i) ÷ 3^(i - 1)
	end

	orientations = [orientations; 3 - sum(orientations) % 3]

	permutations = dehash_permutations(permutations_hash; length=8)

	corners = Corners().pieces
	map(zip(permutations, orientations, corners)) do (index, orientation, piece)
		position = corners[index].position
		Piece(piece.id, position, orientation)
	end |> Corners
end

function dehash(hash::Integer, ::Union{Type{Edges}, Type{HalfEdges}})::HalfEdges
	hash -= 1
	permutations_hash = hash ÷ 2^6
	orientation_hash = hash % 2^6 

	orientations = map(1:6) do i
		(orientation_hash % 2^i) ÷ 2^(i - 1)
	end

	permutations = dehash_permutations(permutations_hash; length=6, max=12)

	edges = Edges().pieces

	map(zip(permutations, orientations, edges[1:6])) do (index, orientation, piece)
		position = edges[index].position
		Piece(piece.id, position, orientation)
	end |> HalfEdges
end