@test all([scramble() |> corners |> hash < corner_permutations for _ in 1:1000])
@test all([all(scramble() |> edges |> hash .< edge_permutations) for _ in 1:1000])
@test permutations_hash(@SVector[1, 2, 3, 4, 5, 6, 7, 8], max=8) == 0
@test permutations_hash(@SVector[2, 1, 3, 4, 5, 6, 7, 8], max=8) == 1
@test permutations_hash(@SVector[3, 1, 2, 4, 5, 6, 7, 8], max=8) == 2
@test cube() |> hash == [1, 1, permutations_hash(@SVector[7, 8, 9, 10, 11, 12], max=12) * 2^6 + 1]