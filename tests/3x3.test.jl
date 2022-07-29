# Check position rotation
@test rotate(v(1, 0, 1), X, 2π/2) == [1, 0, -1] # R2
@test rotate(v(3, 2, 1), Y, 2π/4) == [-1, 2, 3] # Random
@test rotate(v(1, 1, 1), Z, 2π/4) == [1, -1, 1] # F
@test rotate(v(1, 1, 1), X, 2π/4) == [1, 1, -1] # R

# Check move parsing
@test parsemove("R2") == Move(R, 2)
@test parsemove("B'") == Move(B, -1)
@test parsemove("F") == Move(F, 1)
@test parsemove("L42") == Move(L, 42)
@test_throws ErrorException parsemove("X2")

# Check data retrieval
@test movedata(parsemove("R2")) == (2π/2, X)
@test movedata(parsemove("L'")) == (2π/4, X)
@test movedata(parsemove("D4")) == (-2π, Y)
@test movedata(parsemove("B2")) == (-2π/2, Z)

# Check moving pieces
@test move(v(1, 1, 0), parsemove("R")) == [1, 0, -1]
@test move(v(1, 0, 1), parsemove("R2")) == [1, 0, -1]
@test move(v(1, 1, 1), parsemove("F")) == [1, -1, 1]
@test move(v(1, 0, 0), parsemove("L")) == [1, 0, 0]
@test move(v(-1, 1, 0), parsemove("L")) == [-1, 0, 1]
@test move(v(1, 1, 1), parsemove("U")) == [-1, 1, 1]

# Check piece generation
@test length(makeedges()) == 12
@test length(makecorners()) == 8
@test length(cube()) == 20
@test cube()[1].second.normal == [1, 0, 0]

# Check getting pieces of plane
@test isinrange(v(1, 0, 0), v(1, 0, -1)) == true
@test isinrange(v(0, -1, 0), v(0, -1, 2)) == true
@test isinrange(v(0, 0, 1), v(1, 1, -1)) == false
@test isinrange(v(0, 1, 0), v(-1, 0, 1)) == false

# Check movement
@test move(cube(), "R2 R2") |> issolved
@test move(cube(), "R R'") |> issolved
@test move(cube(), "R U R' U' " ^ 6) |> issolved
@test move(cube(), "R2 L2") == move(cube(), "L2 R2") == move(cube(), "L R L2 R L'")
@test move(cube(), repeat(Algs.U * " ", 3)) |> issolved
@test move(cube(), repeat(Algs.T * " ", 2)) |> issolved
positionset(cube::Cube) = Set(map(p -> p.second.position, cube))
@test positionset(cube()) == positionset(move(cube(), "R U R' U' D F L2 B4 R'"))

# Check allocations
@test (@allocated Piece(v(1, 2, 3), v(4, 5, 6))) == 0
@test (@allocated cube()) == 0
@test (@allocated move(cube(), "R2")) < 3000


#=
SCRABMLER
=#

@test filter(pair -> sum(abs.(pair.first)) == 3, cube()) == cube()[1:8] == cube() |> corners
@test filter(pair -> sum(abs.(pair.first)) == 2, cube()) == cube()[9:20] == cube() |> edges

# Check orientation (I hope this is thorough)
@test orientation(v(1, 0, 1), v(1, 0, 0)) == 0
@test orientation(v(1, 1, 1), v(1, 0, 0)) == 0
@test orientation(v(-1, 0, 1), v(-1, 0, 0)) == 0
@test sum(orientation(cube())) == 0
@test orientation(move(cube(), "R U D2 F L R3 D F' D")) == (edges = 4, corners = 9)
@test sum([orientation(pos, piece.normal) for (pos, piece) in move(cube(), "R F2 L B2 D2 R")]) == 0
@test sum([orientation(pos, piece.normal) for (pos, piece) in move(cube(), "U")]) == 10

# Check twists
@test twist(v(0, 1, 0), v(1, 1, 0)) == [1, 0, 0]

# Check scrambler 
@test all([isoriented(scramble()) for _ in 1:1000])