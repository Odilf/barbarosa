include("../barbarosa/hash.jl")

@test all([scramble() |> corners |> hash < factorial(8) * 3^7 for _ in 1:1000])