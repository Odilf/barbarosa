@testset "Vector rotation" begin
	@test Cube3x3.rotate_180(v(1, 0, 1), X) == [1, 0, -1] # R2
	@test Cube3x3.rotate_90(v(3, 2, 1), Y) == [-1, 2, 3] # Random
	@test Cube3x3.rotate_90(v(1, 1, 1), Z) == [1, -1, 1] # F
	@test Cube3x3.rotate_90(v(1, 1, 1), X) == [1, 1, -1] # R
end

@testset "Move parsing" begin
	@test Move("R2") == Move(R, 2)
	@test Move("B'") == Move(B, -1)
	@test Move("F") == Move(F, 1)
	@test Move("L42'") == Move(L, -42)
	@test_throws ErrorException Move("X2")
end

@testset "Piece movement" begin
	@test move(v(1, 1, 0), Move("R")) == [1, 0, -1]
	@test move(v(1, 0, 1), Move("R2")) == [1, 0, -1]
	@test move(v(1, 1, 1), Move("F")) == [1, -1, 1]
	@test move(v(1, 0, 0), Move("L")) == [1, 0, 0]
	@test move(v(-1, 1, 0), Move("L")) == [-1, 0, 1]
	@test move(v(1, 1, 1), Move("U")) == [-1, 1, 1]
end

@testset "Piece generation" begin
	@test length(makeedges()) == 12
	@test length(makecorners()) == 8
	@test length(Cube().pieces) == 20
	@test Cube().pieces[1].normal == [1, 0, 0]
end

@testset "Getting pieces of plane" begin
	@test isinrange(v(1, 0, 0), v(1, 0, -1)) == true
	@test isinrange(v(0, -1, 0), v(0, -1, 2)) == true
	@test isinrange(v(0, 0, 1), v(1, 1, -1)) == false
	@test isinrange(v(0, 1, 0), v(-1, 0, 1)) == false
end

@testset "Cube movement" begin
	@test move(Cube(), "R2 R2") |> issolved
	@test move(Cube(), "R R'") |> issolved
	@test move(Cube(), "R U R' U' " ^ 6) |> issolved
	@test move(Cube(), "R2 L2") == move(Cube(), "L2 R2") == move(Cube(), "L R L2 R L'")
	@test move(Cube(), repeat(Algs.U * " ", 3)) |> issolved
	@test move(Cube(), repeat(Algs.T * " ", 2)) |> issolved
	positionset(cube::Cube) = Set(map(p -> p.position, cube.pieces))
	@test positionset(Cube()) == positionset(move(Cube(), "R U R' U' D F L2 B4 R'"))
end

@testset "Allocations" begin
	@test (@allocated Piece(v(1, 2, 3), v(1, 2, 3), v(1, 2, 3))) == 0
	@test (@allocated Cube()) == 0
	let m = Move("R2")
		@test (@allocated move(Cube(), m)) < 3000
	end
end

@testset "Scrambler" begin
	@testset "Getting conrners and edges" begin
		@test filter(piece -> sum(abs.(piece.position)) == 3, Cube().pieces) == Cube().pieces[1:8] == Corners().pieces
		@test filter(piece -> sum(abs.(piece.position)) == 2, Cube().pieces) == Cube().pieces[9:20] == Edges().pieces
	end

	@testset "Orientation" begin
		@test orientation(v(1, 0, 1), v(1, 0, 0)) == 0
		@test orientation(v(1, 1, 1), v(1, 0, 0)) == 0
		@test orientation(v(-1, 0, 1), v(-1, 0, 0)) == 0
		@test sum(orientation(Cube())) == 0
		@test orientation(move(Cube(), "R U D2 F L R3 D F' D")) == (edges = 4, corners = 6)
		@test sum(orientation.(move(Cube(), "R F2 L B2 D2 R").pieces)) == 0
		@test sum(orientation.(move(Cube(), "U").pieces)) == 10
	end

	@testset "Scrambles" begin
		@test all([issolvable(scramble()) for _ ∈ 1:1000])
	end
end