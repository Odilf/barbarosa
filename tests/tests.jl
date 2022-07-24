include("3x3.jl")

using Test

# Check position rotation
@test rotate(SVector(1, 0, 1), X, 2π/2) == [1, 0, -1] # R2
@test rotate(SVector(3, 2, 1), Y, 2π/4) == [1, 2, -3] # Random
@test rotate(SVector(1, 1, 1), Z, 2π/4) == [-1, 1, 1] # F
@test rotate(SVector(1, 1, 1), X, 2π/4) == [1, 1, -1] # R

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
@test move(SVector(1, 1, 0), "R") == [1, 0, -1]
@test move(SVector(1, 0, 1), "R2") == [1, 0, -1]
@test move(SVector(1, 1, 1), "F") == [-1, 1, 1]
@test move(SVector(1, 0, 0), "L") == [1, 0, 0]
@test move(SVector(-1, 1, 0), "L") == [-1, 0, 1]

# Check piece generation
@test length(edges()) == 12
@test length(corners()) == 8
@test length(cube()) == 20
@test cube()[[1, 1, 1]].normal == [1, 0, 0]

# Check getting pieces of plane
@test isinrange(SVector(1, 0, 0), SVector(1, 0, -1)) == true
@test isinrange(SVector(0, -1, 0), SVector(0, -1, 2)) == true
@test isinrange(SVector(0, 0, 1), SVector(1, 1, -1)) == false
@test isinrange(SVector(0, 1, 0), SVector(-1, 0, 1)) == false

# Check movement
include("algs.jl")

@test move(cube(), "R2 R2") == cube()
@test move(cube(), "R R'") == cube()
@test move(cube(), "R U R' U' " ^ 6) == cube()
@test move(cube(), "R2 L2") == move(cube(), "L2 R2") == move(cube(), "L R L2 R L'")
@test move(cube(), repeat(Algs.U * " ", 3)) == cube()
positionset(cube::Cube) = Set(map(x -> x.position, values(cube)))
@test positionset(cube()) == positionset(move(cube(), "R U R' U' D F L2 B4 R'"))
