module TestBarbarosa

using Test

include("../barbarosa/heuristics.jl")
# include("../3x3/main.jl")

using .Cube3x3

@test manhattan(scramble()) != 0

end