using Test

include("../3x3/scrambler.jl")

# Check orientation (I hope this is thorough)
@test orientation(SVector(1, 0, 1), SVector(1, 0, 0)) == 0
@test orientation(SVector(1, 1, 1), SVector(1, 0, 0)) == 0
@test orientation(SVector(-1, 0, 1), SVector(-1, 0, 0)) == 0
@test sum(orientation(cube())) == 0
@test orientation(move(cube(), "R U D2 F L R3 D F' D")) == (edges = 4, corners = 9)
@test sum([orientation(pos, piece.normal) for (pos, piece) in move(cube(), "R F2 L B2 D2 R")]) == 0
@test sum([orientation(pos, piece.normal) for (pos, piece) in move(cube(), "U")]) == 10

# Check twists
@test twist(SVector(0, 1, 0), SVector(1, 1, 0)) == [1, 0, 0]

# Check scrambler 
@test all([isoriented(scramble()) for _ in 1:1000])