@testset "Hashing" begin
	@test all([scramble() |> Corners |> hash < permutations(Corners) for _ ∈ 1:100])
	@test all([all(scramble() |> HalfEdges |> hash < permutations(Edges)) for _ ∈ 1:100])
	@test hash_permutations(@SVector[1, 2, 3, 4, 5, 6, 7, 8], max=8) == 0
	@test hash_permutations(@SVector[2, 1, 3, 4, 5, 6, 7, 8], max=8) == 1
	@test hash_permutations(@SVector[3, 1, 2, 4, 5, 6, 7, 8], max=8) == 2
	@test hash(Cube()) == (1, 1, 1)
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

@testset "Symmetries" begin
	@test let cube = move(Cube(), Algs.T)
		symmetries = [transform(cube, m) |> MUS.sort for m ∈ MUS.symmetry_matrices]

		rotated = move(Cube(), "F U F' U' F' L F2 U' F' U' F U F' L'")
		rotated ∈ symmetries
	end

	@test let cube = move(Cube(), "R U R' U R U2 R'")
		symmetries = [transform(cube, m) |> MUS.sort for m ∈ MUS.symmetry_matrices]

		# move(Cube(), "F U F' U F U2 F'") ∈ symmetries && 
		move(Cube(), "R F R' F R F2 R'") ∈ symmetries
	end
end