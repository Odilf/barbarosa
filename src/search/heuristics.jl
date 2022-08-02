# apply(f, cube::Cube) = sum([f(piece) for piece in cube.pieces])
function apply(f, cube::Cube)
	total = 0
	for piece in cube.pieces
		total += f(piece)
	end
	total
end

manhattan(piece::Piece) = sum(abs.(piece.position .- piece.id))
# manhattan(piece::Piece) = abs(piece.position[1] - piece.id[1]) + abs(piece.position[2] - piece.id[2]) + abs(piece.position[3] - piece.id[3])
manhattan(cube::Cube) = apply(manhattan, cube) / 16

euclidean(piece::Piece) = √sum((piece.position .- piece.id)^2)
euclidean(cube::Cube) = apply(manhattan, cube) / 16