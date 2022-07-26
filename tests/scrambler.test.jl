using Test

include("../3x3/scrambler.jl")

# Check orientation (I hope this is thorough)
@test orientation(v(1, 0, 1), v(1, 0, 0)) == 0
@test orientation(v(1, 1, 1), v(1, 0, 0)) == 0
@test orientation(v(-1, 0, 1), v(-1, 0, 0)) == 0
@test sum(orientation(cube())) == 0
@test orientation(move(cube(), "R U D2 F L R3 D F' D")) == (edges = 4, corners = 9)
@test sum([orientation(pos, piece.normal) for (pos, piece) in move(cube(), "R F2 L B2 D2 R")]) == 0
@test sum([orientation(pos, piece.normal) for (pos, piece) in move(cube(), "U")]) == 10


orientation.(cube())
cube()

# Check twists
@test twist(v(0, 1, 0), v(1, 1, 0)) == [1, 0, 0]

# Check scrambler 
@test all([isoriented(scramble()) for _ in 1:1000])