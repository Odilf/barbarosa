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

function dehash_corners(hash::Integer)
	hash -= 1
	permutations_hash = hash % factorial(8)
	orientation_hash = hash ÷ factorial(8)

	orientations = map(1:7) do i
		(orientation_hash % 3^i) ÷ 3^(i - 1)
	end

	orientations = [orientations; 3 - sum(orientations) % 3]

	permutations = dehash_permutations(permutations_hash; length=8)

	cube_corners = cube() |> corners

	map(zip(permutations, orientations, cube_corners)) do (index, orientation, (_, piece))
		normal = v(cube_corners[index].first[1], 0, 0)
		pair = cube_corners[index].first => Piece(piece.position, normal)
		Cube3x3.twist(pair, (3 - orientation) % 3)
	end
end

function dehash_edges(hashes::NTuple{2, <:Integer})
	zips = map(hashes) do hash
		hash -= 1
		permutations_hash = hash ÷ 2^6
		orientation_hash = hash % 2^6 

		orientations = map(1:6) do i
			(orientation_hash % 2^i) ÷ 2^(i - 1)
		end

		permutations = dehash_permutations(permutations_hash; length=6, max=12)

		zip(permutations, orientations) |> collect
	end

	cube_edges = cube() |> edges

	map(zip(vcat(zips...), cube_edges)) do ((index, orientation), (_, piece))
		
		pos = cube_edges[index].first

		normal = if abs(pos[1]) == 1
			v(cube_edges[index].first[1], 0, 0)
		else
			v(0, cube_edges[index].first[2], 0)
		end

		piece = Piece(piece.position, normal)

		pair = pos => piece
		Cube3x3.twist(pair, orientation % 2)
	end
end