using .Cube3x3

function apply(f, cube::HashSet)
	value = 0
	for (pos, piece) in cube
		value += f(pos, piece)
	end
	value
end

manhattan(pos::Vector3, piece::Piece) = sum(abs.(pos .- piece.position))
manhattan(cube::HashSet) = apply(manhattan, cube) / 16

euclidean(pos::Vector3, piece::Piece) = √sum((pos .- piece.position)^2)
euclidean(cube::HashSet) = apply(manhattan, cube) / 16

# function MUS(corner_cache::Vector{UInt8}, edge_cache::Vector{UInt8}; fallback = manhattan)
# 	function h(corners::Corners) 
# 		v = corner_cache[hash(corners)]
# 		if v != 0
# 			v
# 		else
# 			fallback(corners)
# 		end
# 	end

# 	function h(edges::HalfEdges)
# 		v = edge_cache[hash(edges)]
# 		if v != 0
# 			v
# 		else
# 			fallback(edges)
# 		end
# 	end

# 	function h(cube::Cube)
# 		c = corners(cube)
# 		e1 = SVector{6}(edges(cube)[1:6])
# 		e2 = SVector{6}(edges(cube)[7:12])

# 		max(h(c), h(e1), h(e2))
# 	end

# 	return h
# end