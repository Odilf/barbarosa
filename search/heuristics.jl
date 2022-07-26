using .Cube3x3

function apply(f, cube::Cube)
	value = 0
	for (pos, piece) in cube
		value += f(pos, piece)
	end
	value
end

manhattan(pos::Vector3, piece::Piece) = sum(abs.(pos .- piece.position))
manhattan(cube::Cube) = apply(manhattan, cube) / 16

euclidean(pos::Vector3, piece::Piece) = √sum((pos .- piece.position)^2)
euclidean(cube::Cube) = apply(manhattan, cube) / 16