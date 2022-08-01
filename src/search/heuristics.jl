apply(f, cube::Cube) = sum([f(piece) for piece in cube.pieces])

manhattan(piece::Piece) = sum(abs.(piece.position .- piece.id))
manhattan(cube::Cube) = apply(manhattan, cube) / 16

euclidean(piece::Piece) = √sum((piece.position .- piece.id)^2)
euclidean(cube::Cube) = apply(manhattan, cube) / 16