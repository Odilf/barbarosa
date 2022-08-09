@testset "Hashing" begin
	@test all([scramble() |> Corners |> hash < corner_permutations for _ ∈ 1:100])
	@test all([all(scramble() |> HalfEdges |> hash < edge_permutations) for _ ∈ 1:100])
	@test hash_permutations(@SVector[1, 2, 3, 4, 5, 6, 7, 8], max=8) == 0
	@test hash_permutations(@SVector[2, 1, 3, 4, 5, 6, 7, 8], max=8) == 1
	@test hash_permutations(@SVector[3, 1, 2, 4, 5, 6, 7, 8], max=8) == 2
	@test hash(Cube()) == [1, 1, 1]
end

@testset "Dehashing" begin
	@test dehash_permutations(hash_permutations(@SVector[1, 4, 3, 2, 6, 5]); length=6) == [1, 4, 3, 2, 6, 5]
	@test dehash_permutations(hash_permutations(@SVector[1, 4, 3, 2, 6, 5]; max=12); length=6, max=12) == [1, 4, 3, 2, 6, 5]

	@test map(1:100) do _
		c = scramble(Corners())
		dehash(hash(c), Corners) == c
	end |> all

	@test map(1:100) do _
		c = scramble() |> HalfEdges
		dehash(hash(c), HalfEdges)  == HalfEdges(c.pieces[1:6])
	end |> all
end