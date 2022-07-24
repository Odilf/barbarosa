include("3x3.jl")

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
	normal = normal[mask]

	# Get parity. If it is negative, the loop is inversed
	parity = reduce(*, p)

	# Count spaces of separation
	index = findfirst(i -> abs(i) == 1, normal) - 1
	
	# Mod it and apply parity
	n = length(p)
	mod((n - index) * parity, n)
end

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